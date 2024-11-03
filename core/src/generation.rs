#[derive(Clone, Copy, Debug)]
pub struct FluxGeneration(u64);

impl From<usize> for FluxGeneration {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}
