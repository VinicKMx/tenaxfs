use crate::error::{StorageError, StorageResult};

/// Physical flash geometry known to TenaxFS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Geometry {
    pub read_size: u32,
    pub program_size: u32,
    pub erase_size: u32,
    pub total_size: u64,
}

impl Geometry {
    /// Creates a geometry value. Call [`Self::validate`] before using it for I/O.
    #[must_use]
    pub const fn new(read_size: u32, program_size: u32, erase_size: u32, total_size: u64) -> Self {
        Self {
            read_size,
            program_size,
            erase_size,
            total_size,
        }
    }

    /// Returns the number of erase-sized segments.
    #[must_use]
    pub const fn segment_count(self) -> u64 {
        if self.erase_size == 0 {
            return 0;
        }

        self.total_size / self.erase_size as u64
    }

    /// Validates alignment relationships required by the NOR-first model.
    pub const fn validate(self) -> StorageResult<()> {
        if self.read_size == 0
            || self.program_size == 0
            || self.erase_size == 0
            || self.total_size == 0
        {
            return Err(StorageError::InvalidGeometry);
        }

        if self.program_size % self.read_size != 0 {
            return Err(StorageError::InvalidGeometry);
        }

        if self.erase_size % self.program_size != 0 {
            return Err(StorageError::InvalidGeometry);
        }

        if self.total_size % self.erase_size as u64 != 0 {
            return Err(StorageError::InvalidGeometry);
        }

        Ok(())
    }
}

/// Static configuration for a TenaxFS mount.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Config {
    pub geometry: Geometry,
    pub minimum_erased_segments: u32,
}

impl Config {
    /// Creates a configuration value.
    #[must_use]
    pub const fn new(geometry: Geometry, minimum_erased_segments: u32) -> Self {
        Self {
            geometry,
            minimum_erased_segments,
        }
    }

    /// Validates geometry and reservation settings.
    pub const fn validate(self) -> StorageResult<()> {
        match self.geometry.validate() {
            Ok(()) => {}
            Err(error) => return Err(error),
        }

        if self.minimum_erased_segments as u64 >= self.geometry.segment_count() {
            return Err(StorageError::InvalidGeometry);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Geometry};
    use crate::error::StorageError;

    #[test]
    fn geometry_requires_nested_alignment() {
        let geometry = Geometry::new(1, 256, 4096, 4096 * 8);
        assert_eq!(geometry.validate(), Ok(()));
        assert_eq!(geometry.segment_count(), 8);
    }

    #[test]
    fn geometry_rejects_non_multiple_total_size() {
        let geometry = Geometry::new(1, 256, 4096, 4097);
        assert_eq!(geometry.validate(), Err(StorageError::InvalidGeometry));
    }

    #[test]
    fn config_reserves_less_than_all_segments() {
        let geometry = Geometry::new(1, 256, 4096, 4096 * 4);
        let config = Config::new(geometry, 4);
        assert_eq!(config.validate(), Err(StorageError::InvalidGeometry));
    }
}
