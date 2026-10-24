//! Bounded observation and conditional execution through platform adapters only.
use romatlas_core::{ApiError, CommandAdapter};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, time::Instant};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    name: String,
    method: String,
    #[serde(default = "empty")]
    params: Value,
    /// Optional byte-indexed address: base + byte at pointer_address + offset.
    indirect: Option<Indirect>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Indirect {
    pointer_address: u64,
    base: u64,
    offset: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Condition {
    query: String,
    pointer: String,
    value: Value,
    #[serde(default)]
    op: Comparison,
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Comparison {
    #[default]
    Eq,
    Ne,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    queries: Vec<Query>,
    #[serde(default)]
    conditions: Vec<Condition>,
    max_operations: Option<u64>,
    timeout_ms: Option<u64>,
    cpu: Option<String>,
}
fn empty() -> Value {
    json!({})
}
fn invalid(message: &str) -> ApiError {
    ApiError::invalid(message)
}
fn observe(adapter: &mut impl CommandAdapter, queries: &[Query]) -> Result<Value, ApiError> {
    let mut observations = serde_json::Map::new();
    for query in queries {
        let mut params = query.params.clone();
        if let Some(indirect) = &query.indirect {
            let mut pointer_params = params.clone();
            pointer_params["address"] = json!(indirect.pointer_address);
            pointer_params["length"] = json!(1);
            let read = adapter.command("emulator.memory", pointer_params)?;
            let index = read["bytes"][0]
                .as_u64()
                .ok_or_else(|| invalid("indirect pointer byte unavailable"))?;
            let address = indirect
                .base
                .checked_add(index)
                .and_then(|a| a.checked_add(indirect.offset))
                .ok_or_else(|| invalid("indirect address overflow"))?;
            params["address"] = json!(address);
        }
        observations.insert(query.name.clone(), adapter.command(&query.method, params)?);
    }
    Ok(Value::Object(observations))
}

pub(crate) fn execute(
    adapter: &mut impl CommandAdapter,
    method: &str,
    params: Value,
) -> Result<Value, ApiError> {
    let request: Request =
        serde_json::from_value(params).map_err(|e| ApiError::invalid(e.to_string()))?;
    if request.queries.is_empty() || request.queries.len() > 32 || request.conditions.len() > 32 {
        return Err(invalid("require 1..32 queries and at most 32 conditions"));
    }
    let mut names = BTreeMap::new();
    for q in &request.queries {
        if q.name.is_empty()
            || names.insert(&q.name, ()).is_some()
            || !q.params.is_object()
            || !matches!(
                q.method.as_str(),
                "emulator.memory" | "emulator.physical_memory" | "emulator.state"
            )
            || (q.indirect.is_some() && q.method != "emulator.memory")
        {
            return Err(invalid(
                "queries need unique names, object params and read-only inspection methods",
            ));
        }
        if q.method != "emulator.state"
            && !(1..=4096).contains(&q.params.get("length").and_then(Value::as_u64).unwrap_or(1))
        {
            return Err(invalid("query length must be 1..4096"));
        }
    }
    for c in &request.conditions {
        if !names.contains_key(&c.query) || (!c.pointer.is_empty() && !c.pointer.starts_with('/')) {
            return Err(invalid("condition needs a named query and JSON pointer"));
        }
    }
    let running = method == "emulator.run_until";
    let limit = request.max_operations.unwrap_or(100_000);
    let timeout = request.timeout_ms.unwrap_or(1000);
    if limit > 1_000_000
        || !(1..=30_000).contains(&timeout)
        || (running && request.conditions.is_empty())
    {
        return Err(invalid(
            "run_until requires conditions, <=1000000 operations and timeout_ms 1..30000",
        ));
    }
    let start = Instant::now();
    let mut operations = 0;
    loop {
        let observations = observe(adapter, &request.queries)?;
        let mut matched = true;
        for c in &request.conditions {
            let actual = observations[&c.query].pointer(&c.pointer).ok_or_else(|| {
                invalid("condition observation missing; invalid JSON pointer or unavailable field")
            })?;
            matched &= match c.op {
                Comparison::Eq => actual == &c.value,
                Comparison::Ne => actual != &c.value,
            };
        }
        let reason = if !running {
            Some("observed")
        } else if matched {
            Some("matched")
        } else if operations == limit {
            Some("operation_limit")
        } else if start.elapsed().as_millis() >= u128::from(timeout) {
            Some("timeout")
        } else {
            None
        };
        if let Some(reason) = reason {
            return Ok(
                json!({"reason":reason,"matched":matched,"operations":operations,"observations":observations}),
            );
        }
        let remaining = timeout
            .saturating_sub(start.elapsed().as_millis() as u64)
            .max(1);
        let mut step =
            json!({"unit":"operation","count":1,"max_operations":100_000,"timeout_ms":remaining});
        if let Some(cpu) = &request.cpu {
            step["cpu"] = json!(cpu);
        }
        adapter.command("emulator.step", step)?;
        operations += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        tick: u64,
    }
    impl CommandAdapter for Mock {
        fn platform(&self) -> &str {
            "mock"
        }
        fn methods(&self) -> Vec<String> {
            vec![]
        }
        fn command(&mut self, method: &str, params: Value) -> Result<Value, ApiError> {
            match method {
                "emulator.step" => {
                    self.tick += 1;
                    Ok(json!({}))
                }
                "emulator.state" => Ok(json!({"tick":self.tick})),
                "emulator.memory" => {
                    Ok(json!({"bytes":[if params["address"] == 42 {7} else {self.tick}]}))
                }
                _ => Err(ApiError::unsupported(method)),
            }
        }
    }
    fn request(value: u64, limit: u64) -> Value {
        json!({"queries":[{"name":"state","method":"emulator.state"}],
            "conditions":[{"query":"state","pointer":"/tick","value":value}],"max_operations":limit})
    }
    #[test]
    fn transient_match_and_explicit_exhaustion() {
        let mut adapter = Mock { tick: 0 };
        let result = execute(&mut adapter, "emulator.run_until", request(3, 10)).unwrap();
        assert_eq!(result["operations"], 3);
        assert_eq!(adapter.tick, 3);
        let result = execute(&mut adapter, "emulator.run_until", request(20, 2)).unwrap();
        assert_eq!(result["reason"], "operation_limit");
        assert_eq!(result["observations"]["state"]["tick"], 5);
        let result = execute(&mut adapter, "emulator.run_until", request(5, 0)).unwrap();
        assert_eq!(result["operations"], 0);
    }
    #[test]
    fn reject_mutation_and_missing_observation_without_stepping() {
        let mut adapter = Mock { tick: 0 };
        let mut p = request(3, 10);
        p["queries"][0]["method"] = json!("emulator.memory.write");
        assert!(execute(&mut adapter, "emulator.run_until", p).is_err());
        let mut p = request(3, 10);
        p["conditions"][0]["pointer"] = json!("/missing");
        assert!(execute(&mut adapter, "emulator.run_until", p).is_err());
        assert_eq!(adapter.tick, 0);
    }
    #[test]
    fn inequality_requires_a_present_field_and_stops_on_change() {
        let mut adapter = Mock { tick: 0 };
        let mut p = request(0, 5);
        p["conditions"][0]["op"] = json!("ne");
        let result = execute(&mut adapter, "emulator.run_until", p.clone()).unwrap();
        assert_eq!(result["operations"], 1);
        assert_eq!(adapter.tick, 1);
        p["conditions"][0]["pointer"] = json!("/absent");
        assert!(execute(&mut adapter, "emulator.run_until", p.clone()).is_err());
        assert_eq!(adapter.tick, 1);
        p["conditions"][0]["op"] = json!("invalid");
        assert!(execute(&mut adapter, "emulator.run_until", p).is_err());
    }
    #[test]
    fn indirect_read_does_not_advance() {
        let mut adapter = Mock { tick: 0 };
        let p = json!({"queries":[{"name":"stack","method":"emulator.memory","params":{"length":1},
            "indirect":{"pointer_address":42,"base":256,"offset":3}}]});
        let result = execute(&mut adapter, "emulator.observe", p).unwrap();
        assert_eq!(result["observations"]["stack"]["bytes"][0], 0);
        assert_eq!(adapter.tick, 0);
    }
}
