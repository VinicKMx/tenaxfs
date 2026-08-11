use tenaxfs_core::{Flash, Geometry, StorageResult};
use tenaxfs_flash::SimNorFlash;

/// Default host simulation geometry used by examples and smoke tests.
pub const DEFAULT_SIM_GEOMETRY: Geometry = Geometry::new(1, 256, 4096, 4096 * 16);

/// Summary returned by a simulator smoke run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SmokeReport {
    pub total_size: u64,
    pub segment_count: u64,
    pub erase_size: u32,
    pub program_size: u32,
}

/// Runs the foundation simulator smoke scenario.
pub fn smoke() -> StorageResult<SmokeReport> {
    let flash = SimNorFlash::new(DEFAULT_SIM_GEOMETRY)?;
    let geometry = flash.geometry();

    Ok(SmokeReport {
        total_size: geometry.total_size,
        segment_count: geometry.segment_count(),
        erase_size: geometry.erase_size,
        program_size: geometry.program_size,
    })
}

#[cfg(test)]
mod tests {
    use super::{smoke, SmokeReport, DEFAULT_SIM_GEOMETRY};

    #[test]
    fn smoke_report_matches_default_geometry() {
        assert_eq!(
            smoke(),
            Ok(SmokeReport {
                total_size: DEFAULT_SIM_GEOMETRY.total_size,
                segment_count: DEFAULT_SIM_GEOMETRY.segment_count(),
                erase_size: DEFAULT_SIM_GEOMETRY.erase_size,
                program_size: DEFAULT_SIM_GEOMETRY.program_size,
            })
        );
    }
}
