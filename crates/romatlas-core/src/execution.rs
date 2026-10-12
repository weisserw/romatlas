//! Typed execution boundary. State, inputs, snapshots and trace configuration belong to adapters.
use crate::{Address, ApiError, TraceEvent};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepUnit {
    Operation,
    Cycle,
    Frame,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepRequest {
    pub unit: StepUnit,
    pub count: u64,
    pub max_operations: u64,
    pub timeout_ms: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressSpace {
    pub name: String,
    pub size: Option<u64>,
    pub context: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionCapabilities {
    pub platform: String,
    pub clock: String,
    pub units: Vec<StepUnit>,
    pub address_spaces: Vec<AddressSpace>,
    pub snapshots: bool,
    pub instrumentation: bool,
    pub inputs: bool,
}
pub trait ExecutionAdapter: Sized {
    type LoadOptions;
    type State: Serialize;
    type Snapshot: Clone;
    type Input;
    type Instrumentation;
    fn load(image: &std::path::Path, options: Self::LoadOptions) -> Result<Self, ApiError>;
    fn capabilities(&self) -> ExecutionCapabilities;
    fn state(&self) -> Self::State;
    fn execute(&mut self, request: StepRequest) -> Result<Self::State, ApiError>;
    fn reset(&mut self) -> Result<(), ApiError>;
    fn input(&mut self, input: Self::Input) -> Result<(), ApiError>;
    fn snapshot(&self) -> Result<Self::Snapshot, ApiError>;
    fn restore(&mut self, snapshot: &Self::Snapshot) -> Result<(), ApiError>;
    fn translate(&self, address: &Address) -> Result<Option<Address>, ApiError>;
    fn inspect(&mut self, address: &Address, length: usize) -> Result<Vec<u8>, ApiError>;
    fn configure_instrumentation(&mut self, config: Self::Instrumentation) -> Result<(), ApiError>;
    fn events(&self, offset: usize, limit: usize) -> Result<Vec<TraceEvent>, ApiError>;
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Wide {
        pc: u64,
    }
    impl ExecutionAdapter for Wide {
        type LoadOptions = ();
        type State = u64;
        type Snapshot = u64;
        type Input = u64;
        type Instrumentation = ();
        fn load(_: &std::path::Path, _: ()) -> Result<Self, ApiError> {
            Ok(Self { pc: 1 << 40 })
        }
        fn capabilities(&self) -> ExecutionCapabilities {
            ExecutionCapabilities {
                platform: "wide-mock".into(),
                clock: "mock-ticks".into(),
                units: vec![StepUnit::Operation],
                address_spaces: vec![AddressSpace {
                    name: "wide-code".into(),
                    size: None,
                    context: serde_json::json!({"cpu":"secondary"}),
                }],
                snapshots: true,
                instrumentation: false,
                inputs: true,
            }
        }
        fn state(&self) -> u64 {
            self.pc
        }
        fn execute(&mut self, r: StepRequest) -> Result<u64, ApiError> {
            if r.unit != StepUnit::Operation {
                return Err(ApiError::unsupported("step unit"));
            }
            if r.count > r.max_operations {
                return Err(ApiError::invalid("operation budget"));
            }
            self.pc += r.count;
            Ok(self.pc)
        }
        fn reset(&mut self) -> Result<(), ApiError> {
            self.pc = 1 << 40;
            Ok(())
        }
        fn input(&mut self, input: u64) -> Result<(), ApiError> {
            self.pc = input;
            Ok(())
        }
        fn snapshot(&self) -> Result<u64, ApiError> {
            Ok(self.pc)
        }
        fn restore(&mut self, s: &u64) -> Result<(), ApiError> {
            self.pc = *s;
            Ok(())
        }
        fn translate(&self, a: &Address) -> Result<Option<Address>, ApiError> {
            if a.space == "wide-code" {
                Ok(Some(a.clone()))
            } else {
                Err(ApiError::unsupported("space"))
            }
        }
        fn inspect(&mut self, a: &Address, n: usize) -> Result<Vec<u8>, ApiError> {
            if a.space != "wide-code" {
                return Err(ApiError::unsupported("space"));
            }
            Ok(vec![a.offset as u8; n])
        }
        fn configure_instrumentation(&mut self, _: ()) -> Result<(), ApiError> {
            Err(ApiError::unsupported("instrumentation"))
        }
        fn events(&self, _: usize, _: usize) -> Result<Vec<TraceEvent>, ApiError> {
            Ok(vec![])
        }
    }
    fn replay<A: ExecutionAdapter>(adapter: &mut A) -> Result<(A::State, A::State), ApiError> {
        let snapshot = adapter.snapshot()?;
        let request = StepRequest {
            unit: StepUnit::Operation,
            count: 3,
            max_operations: 3,
            timeout_ms: 100,
        };
        let first = adapter.execute(request.clone())?;
        adapter.restore(&snapshot)?;
        let second = adapter.execute(request)?;
        Ok((first, second))
    }
    #[test]
    fn generic_replay_keeps_wide_platform_types() {
        let mut adapter = Wide::load(std::path::Path::new("mock"), ()).unwrap();
        let (a, b) = replay(&mut adapter).unwrap();
        assert_eq!(a, b);
        assert!(a > u32::MAX as u64);
        assert_eq!(adapter.capabilities().clock, "mock-ticks");
        assert_eq!(
            adapter.configure_instrumentation(()).unwrap_err().code,
            -32601
        );
    }
}
