//! Not the guy from Kwik-E-Mart.

#[derive(Debug, Clone)]
pub struct Apu {}

impl Apu {
    pub fn new() -> Self {
        tracing::warn!("apu not yet implemented");
        Self {}
    }
}
