use core::ops::Range;

use tenaxfs_core::{Flash, Geometry, StorageError};

/// Errors reported by the host NOR simulator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimFlashError {
    Storage(StorageError),
    ProgramWouldSetBit,
}

impl From<StorageError> for SimFlashError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

/// In-memory NOR flash simulator.
#[derive(Clone, Debug)]
pub struct SimNorFlash {
    geometry: Geometry,
    bytes: Vec<u8>,
}

impl SimNorFlash {
    /// Creates an erased simulated NOR image.
    pub fn new(geometry: Geometry) -> Result<Self, StorageError> {
        geometry.validate()?;

        let length =
            usize::try_from(geometry.total_size).map_err(|_| StorageError::InvalidGeometry)?;

        Ok(Self {
            geometry,
            bytes: vec![0xff; length],
        })
    }

    /// Returns the raw simulated flash image.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn byte_range(
        &self,
        offset: u64,
        length: usize,
        out_of_bounds: StorageError,
    ) -> Result<Range<usize>, SimFlashError> {
        let length = u64::try_from(length).map_err(|_| StorageError::InvalidGeometry)?;
        let end = offset.checked_add(length).ok_or(out_of_bounds)?;

        if end > self.geometry.total_size {
            return Err(out_of_bounds.into());
        }

        let start = usize::try_from(offset).map_err(|_| StorageError::InvalidGeometry)?;
        let end = usize::try_from(end).map_err(|_| StorageError::InvalidGeometry)?;

        Ok(start..end)
    }

    fn require_program_alignment(&self, offset: u64, length: usize) -> Result<(), SimFlashError> {
        let program_size = self.geometry.program_size as u64;
        let length = u64::try_from(length).map_err(|_| StorageError::InvalidGeometry)?;

        if offset % program_size != 0 || length % program_size != 0 {
            return Err(StorageError::ProgramAlignment.into());
        }

        Ok(())
    }

    fn require_erase_alignment(&self, range: &Range<u64>) -> Result<(), SimFlashError> {
        let erase_size = self.geometry.erase_size as u64;

        if range.start % erase_size != 0 || range.end % erase_size != 0 || range.start >= range.end
        {
            return Err(StorageError::EraseAlignment.into());
        }

        Ok(())
    }
}

impl Flash for SimNorFlash {
    type Error = SimFlashError;

    fn geometry(&self) -> Geometry {
        self.geometry
    }

    fn read(&mut self, offset: u64, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let range = self.byte_range(offset, buffer.len(), StorageError::ReadOutOfBounds)?;
        buffer.copy_from_slice(&self.bytes[range]);
        Ok(())
    }

    fn program(&mut self, offset: u64, data: &[u8]) -> Result<(), Self::Error> {
        self.require_program_alignment(offset, data.len())?;
        let range = self.byte_range(offset, data.len(), StorageError::ProgramOutOfBounds)?;

        for (stored, new) in self.bytes[range].iter_mut().zip(data.iter().copied()) {
            if (*stored | new) != *stored {
                return Err(SimFlashError::ProgramWouldSetBit);
            }

            *stored &= new;
        }

        Ok(())
    }

    fn erase(&mut self, range: Range<u64>) -> Result<(), Self::Error> {
        self.require_erase_alignment(&range)?;
        let byte_range = self.byte_range(
            range.start,
            usize::try_from(range.end - range.start).map_err(|_| StorageError::EraseOutOfBounds)?,
            StorageError::EraseOutOfBounds,
        )?;

        self.bytes[byte_range].fill(0xff);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tenaxfs_core::{Flash, Geometry, StorageError};

    use super::{SimFlashError, SimNorFlash};

    fn flash() -> SimNorFlash {
        match SimNorFlash::new(Geometry::new(1, 4, 16, 64)) {
            Ok(flash) => flash,
            Err(error) => panic!("unexpected simulator construction error: {error:?}"),
        }
    }

    #[test]
    fn new_flash_is_erased() {
        let flash = flash();
        assert!(flash.as_bytes().iter().all(|byte| *byte == 0xff));
    }

    #[test]
    fn program_changes_only_one_bits_to_zero_bits() {
        let mut flash = flash();
        assert_eq!(flash.program(0, &[0xf0, 0x0f, 0xaa, 0x55]), Ok(()));

        let mut buffer = [0; 4];
        assert_eq!(flash.read(0, &mut buffer), Ok(()));
        assert_eq!(buffer, [0xf0, 0x0f, 0xaa, 0x55]);

        assert_eq!(
            flash.program(0, &[0xff, 0xff, 0xff, 0xff]),
            Err(SimFlashError::ProgramWouldSetBit)
        );
    }

    #[test]
    fn erase_restores_one_bits() {
        let mut flash = flash();
        assert_eq!(flash.program(0, &[0, 0, 0, 0]), Ok(()));
        assert_eq!(flash.erase(0..16), Ok(()));
        assert!(flash.as_bytes()[0..16].iter().all(|byte| *byte == 0xff));
    }

    #[test]
    fn alignment_is_enforced() {
        let mut flash = flash();
        assert_eq!(
            flash.program(1, &[0, 0, 0, 0]),
            Err(SimFlashError::Storage(StorageError::ProgramAlignment))
        );
        assert_eq!(
            flash.erase(4..16),
            Err(SimFlashError::Storage(StorageError::EraseAlignment))
        );
    }
}
