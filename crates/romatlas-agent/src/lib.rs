//! Line-delimited JSON-RPC 2.0. No console-specific dependencies.
use romatlas_core::{ApiError, CommandAdapter};
use serde_json::{Value, json};
use std::io::{self, BufRead, Read, Write};
pub const MAX_REQUEST_BYTES: usize = 1024 * 1024;
pub const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
struct ResponseCounter {
    bytes: usize,
}
impl Write for ResponseCounter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_RESPONSE_BYTES - self.bytes {
            return Err(io::Error::other("response exceeds byte budget"));
        }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn bounded_response(response: Value) -> Value {
    if serde_json::to_writer(ResponseCounter { bytes: 0 }, &response).is_ok() {
        return response;
    }
    let message = ApiError {code:-32001,message:"response exceeds 1 MiB; command may have completed; request a smaller page/range or file-only export".into()};
    let small = error(
        response.get("id").cloned().unwrap_or(Value::Null),
        message.clone(),
    );
    if serde_json::to_writer(ResponseCounter { bytes: 0 }, &small).is_ok() {
        small
    } else {
        error(Value::Null, message)
    }
}
pub struct Session<A: CommandAdapter> {
    pub adapter: A,
}
impl<A: CommandAdapter> Session<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }
    pub fn handle(&mut self, request: Value) -> Option<Value> {
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let valid_id = id.is_null() || id.is_string() || id.is_number();
        if !request.is_object()
            || request.get("jsonrpc") != Some(&json!("2.0"))
            || !request.get("method").is_some_and(Value::is_string)
            || !valid_id
        {
            return Some(error(
                Value::Null,
                ApiError {
                    code: -32600,
                    message: "invalid JSON-RPC 2.0 request".into(),
                },
            ));
        }
        let notification = request.get("id").is_none();
        let method = request["method"].as_str().unwrap();
        let params = request.get("params").cloned().unwrap_or(json!({}));
        let result = if !params.is_object() {
            Err(ApiError::invalid("params must be an object"))
        } else if method == "api.describe" {
            Ok(
                json!({"protocol":"romatlas/1","platform":self.adapter.platform(),"methods":self.adapter.methods(),"max_request_bytes":MAX_REQUEST_BYTES,"max_response_bytes":MAX_RESPONSE_BYTES}),
            )
        } else {
            self.adapter.command(method, params)
        };
        if notification {
            None
        } else {
            Some(bounded_response(match result {
                Ok(value) => json!({"jsonrpc":"2.0","id":id,"result":value}),
                Err(e) => error(id, e),
            }))
        }
    }
    pub fn serve(&mut self, mut input: impl BufRead, mut output: impl Write) -> io::Result<()> {
        loop {
            let mut bytes = Vec::new();
            let n = input
                .by_ref()
                .take((MAX_REQUEST_BYTES + 1) as u64)
                .read_until(b'\n', &mut bytes)?;
            if n == 0 {
                break;
            }
            let response = if n > MAX_REQUEST_BYTES {
                // Drain this request without growing memory, then accept the next line.
                if bytes.last() != Some(&b'\n') {
                    loop {
                        let buf = input.fill_buf()?;
                        if buf.is_empty() {
                            break;
                        }
                        let end = buf.iter().position(|b| *b == b'\n');
                        let len = end.map_or(buf.len(), |i| i + 1);
                        input.consume(len);
                        if end.is_some() {
                            break;
                        }
                    }
                }
                Some(error(
                    Value::Null,
                    ApiError {
                        code: -32600,
                        message: "request exceeds byte budget".into(),
                    },
                ))
            } else {
                match serde_json::from_slice(&bytes) {
                    Ok(v) => self.handle(v),
                    Err(_) => Some(error(
                        Value::Null,
                        ApiError {
                            code: -32700,
                            message: "invalid JSON".into(),
                        },
                    )),
                }
            };
            if let Some(response) = response {
                serde_json::to_writer(&mut output, &response)?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
        }
        Ok(())
    }
}
fn error(id: Value, e: ApiError) -> Value {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":e.code,"message":e.message}})
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        count: u64,
    }
    impl CommandAdapter for Mock {
        fn platform(&self) -> &str {
            "wide-mock"
        }
        fn methods(&self) -> Vec<String> {
            vec!["mock.step".into()]
        }
        fn command(&mut self, m: &str, p: Value) -> Result<Value, ApiError> {
            if m == "mock.large" {
                self.count += 1;
                return Ok(json!({"text":"\0".repeat(MAX_RESPONSE_BYTES / 2)}));
            }
            if m != "mock.step" {
                return Err(ApiError::unsupported(m));
            }
            self.count += p["count"]
                .as_u64()
                .ok_or_else(|| ApiError::invalid("count"))?;
            Ok(json!({"address":0x1_0000_0000u64+self.count}))
        }
    }
    #[test]
    fn persistent_mock_and_notifications() {
        let mut s = Session::new(Mock { count: 0 });
        assert!(
            s.handle(json!({"jsonrpc":"2.0","method":"mock.step","params":{"count":4}}))
                .is_none()
        );
        let r = s
            .handle(json!({"jsonrpc":"2.0","id":2,"method":"mock.step","params":{"count":3}}))
            .unwrap();
        assert_eq!(r["result"]["address"], 0x1_0000_0007u64);
        assert_eq!(
            s.handle(json!({"jsonrpc":"2.0","id":3,"method":"gb.peek"}))
                .unwrap()["error"]["code"],
            -32601
        );
    }
    #[test]
    fn bounded_stream_recovers() {
        let mut input = vec![b'x'; MAX_REQUEST_BYTES + 5];
        input.extend_from_slice(
            b"\n{bad\n{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"api.describe\"}\n",
        );
        let mut output = Vec::new();
        Session::new(Mock { count: 0 })
            .serve(io::Cursor::new(input), &mut output)
            .unwrap();
        let rows: Vec<Value> = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0]["error"]["code"], -32600);
        assert_eq!(rows[1]["error"]["code"], -32700);
        assert_eq!(rows[2]["result"]["platform"], "wide-mock");
    }
    #[test]
    fn oversized_escaped_response_keeps_stream_usable() {
        let input = b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"mock.large\"}\n{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"mock.step\",\"params\":{\"count\":3}}\n";
        let mut output = Vec::new();
        let mut session = Session::new(Mock { count: 0 });
        session.serve(io::Cursor::new(input), &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        let lines: Vec<_> = text.lines().collect();
        assert!(lines.iter().all(|line| line.len() <= MAX_RESPONSE_BYTES));
        let first: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(first["id"], 1);
        assert_eq!(first["error"]["code"], -32001);
        let next: Value = serde_json::from_str(lines[1]).unwrap();
        assert_eq!(next["result"]["address"], 0x1_0000_0004u64);
        let huge_id = bounded_response(json!({"id":"x".repeat(MAX_RESPONSE_BYTES),"result":0}));
        assert!(huge_id["id"].is_null());
    }
}
