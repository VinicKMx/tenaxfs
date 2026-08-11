use core::ops::Range;

use crate::config::Geometry;

/// Synchronous flash operations required by the TenaxFS core.
///
/// Implementations are expected to preserve NOR semantics:
///
/// - erase changes bits to `1`;
/// - program may only change bits from `1` to `0`;
/// - program and erase operations honor the reported geometry.
pub trait Flash {
    type Error;

    fn geometry(&self) -> Geometry;
    fn read(&mut self, offset: u64, buffer: &mut [u8]) -> Result<(), Self::Error>;
    fn program(&mut self, offset: u64, data: &[u8]) -> Result<(), Self::Error>;
    fn erase(&mut self, range: Range<u64>) -> Result<(), Self::Error>;
}
