//! Platform-neutral backend contract, byte comparison, and bounded tool processes.
use anyhow::Result;
use std::path::Path;
mod process;
pub use process::Budget;
/// A backend owns mapping syntax, tools, source format, and address diagnostics.
pub trait DisassemblyBackend {
    type Mapping;
    fn generate(
        &self,
        image: &Path,
        mapping: &Self::Mapping,
        out: &Path,
        timeout_ms: u64,
    ) -> Result<()>;
    fn verify(&self, image: &Path, out: &Path, timeout_ms: u64) -> Result<()>;
}
pub fn generate<B: DisassemblyBackend>(
    backend: &B,
    image: &Path,
    mapping: &B::Mapping,
    out: &Path,
    timeout_ms: u64,
) -> Result<()> {
    backend.generate(image, mapping, out, timeout_ms)
}
pub fn verify<B: DisassemblyBackend>(
    backend: &B,
    image: &Path,
    out: &Path,
    timeout_ms: u64,
) -> Result<()> {
    backend.verify(image, out, timeout_ms)
}
#[derive(Debug, PartialEq, Eq)]
pub struct ByteMismatch {
    pub offset: u64,
    pub expected: Option<u8>,
    pub actual: Option<u8>,
    pub expected_len: usize,
    pub actual_len: usize,
}
pub fn first_mismatch(expected: &[u8], actual: &[u8]) -> Option<ByteMismatch> {
    (0..expected.len().max(actual.len()))
        .find(|&i| expected.get(i) != actual.get(i))
        .map(|i| ByteMismatch {
            offset: i as u64,
            expected: expected.get(i).copied(),
            actual: actual.get(i).copied(),
            expected_len: expected.len(),
            actual_len: actual.len(),
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Mock;
    impl DisassemblyBackend for Mock {
        type Mapping = u64;
        fn generate(&self, _: &Path, mapping: &u64, out: &Path, _: u64) -> Result<()> {
            std::fs::write(out, mapping.to_le_bytes())?;
            Ok(())
        }
        fn verify(&self, image: &Path, out: &Path, _: u64) -> Result<()> {
            anyhow::ensure!(
                first_mismatch(&std::fs::read(image)?, &std::fs::read(out)?).is_none(),
                "mock mismatch"
            );
            Ok(())
        }
    }
    #[test]
    fn comparison_has_no_bank_assumption() {
        let m = first_mismatch(&[1, 2, 3], &[1, 2]).unwrap();
        assert_eq!(m.offset, 2);
        assert_eq!(m.actual, None);
        assert!(first_mismatch(&[], &[]).is_none());
    }
    #[test]
    fn mock_backend_uses_its_own_mapping() {
        let base = std::env::temp_dir().join(format!("romatlas-backend-{}", std::process::id()));
        std::fs::create_dir(&base).unwrap();
        let image = base.join("image");
        let out = base.join("out");
        std::fs::write(&image, 0x123456789abcdef0u64.to_le_bytes()).unwrap();
        generate(&Mock, &image, &0x123456789abcdef0, &out, 1000).unwrap();
        verify(&Mock, &image, &out, 1000).unwrap();
        std::fs::remove_dir_all(base).unwrap();
    }
}
