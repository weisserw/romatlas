//! Platform-neutral structural records. Addresses do not assume a CPU width or bank size.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address {
    pub space: String,
    pub offset: u64,
    pub context: std::collections::BTreeMap<String, String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provenance {
    Observed,
    Static,
    Manual,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionKind {
    Code,
    Data,
    Text,
    Padding,
    Image,
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    pub start: Address,
    pub length: u64,
    pub kind: RegionKind,
    pub provenance: Provenance,
    pub confidence: f32,
}
/// Compile-time adapter boundary; implementations own address translation and decoding.
pub trait ImageAdapter {
    fn platform(&self) -> &str;
    fn image_size(&self) -> u64;
    fn physical_address(&self, offset: u64) -> Option<Address>;
    fn peek(&self, address: &Address, length: usize) -> Result<Vec<u8>, String>;
}

/// Platform command adapter. The session layer routes calls without hardware assumptions.
/// Decoding, address translation and platform backend selection stay in the adapter.
pub trait CommandAdapter {
    fn platform(&self) -> &str;
    fn methods(&self) -> Vec<String>;
    fn command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}
impl ApiError {
    pub fn unsupported(method: &str) -> Self {
        Self {
            code: -32601,
            message: format!("unsupported capability: {method}"),
        }
    }
    pub fn invalid(message: impl Into<String>) -> Self {
        Self {
            code: -32602,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventKind {
    Instruction,
    Fetch,
    Read,
    Write,
    ControlFlow,
    Interrupt,
    BankSwitch,
    Dma,
    Frame,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceEvent {
    pub kind: EventKind,
    pub cycle: u64,
    pub address: Address,
    pub target: Option<Address>,
    pub value: Option<u64>,
    pub context: serde_json::Value,
}

/// Platform-decoded structural instruction. No opcode or CPU width is imposed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructuralInstruction {
    pub length: u64,
    pub successors: Vec<Address>,
    pub references: Vec<Reference>,
    pub diagnostics: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reference {
    pub source: Address,
    pub target: Option<Address>,
    pub kind: String,
    pub context: serde_json::Value,
}
pub trait DecodeAdapter: ImageAdapter {
    /// Remove traversal-only state before comparing coverage or map regions.
    fn canonical_address(&self, address: &Address) -> Address {
        address.clone()
    }
    fn decode_structure(&self, address: &Address) -> Result<StructuralInstruction, String>;
}

pub mod execution;
