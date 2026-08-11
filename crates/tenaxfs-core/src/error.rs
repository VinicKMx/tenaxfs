/// Result alias used by TenaxFS core operations.
pub type StorageResult<T> = Result<T, StorageError>;

/// Structural errors reported by the storage core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageError {
    CorruptRecord,
    CorruptSegment,
    InvalidGeometry,
    UnsupportedOldFormat,
    UnsupportedFutureFormat,
    OutOfSpace,
    MaintenanceRequired,
    ReserveViolation,
    MediaFailure,
    TransactionTooLarge,
    EpochConflict,
    RecoveryRequired,
    ReadOnlyMode,
    ProgramAlignment,
    EraseAlignment,
    ReadOutOfBounds,
    ProgramOutOfBounds,
    EraseOutOfBounds,
}

impl StorageError {
    /// Stable symbolic name suitable for logs and host tools.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::CorruptRecord => "corrupt_record",
            Self::CorruptSegment => "corrupt_segment",
            Self::InvalidGeometry => "invalid_geometry",
            Self::UnsupportedOldFormat => "unsupported_old_format",
            Self::UnsupportedFutureFormat => "unsupported_future_format",
            Self::OutOfSpace => "out_of_space",
            Self::MaintenanceRequired => "maintenance_required",
            Self::ReserveViolation => "reserve_violation",
            Self::MediaFailure => "media_failure",
            Self::TransactionTooLarge => "transaction_too_large",
            Self::EpochConflict => "epoch_conflict",
            Self::RecoveryRequired => "recovery_required",
            Self::ReadOnlyMode => "read_only_mode",
            Self::ProgramAlignment => "program_alignment",
            Self::EraseAlignment => "erase_alignment",
            Self::ReadOutOfBounds => "read_out_of_bounds",
            Self::ProgramOutOfBounds => "program_out_of_bounds",
            Self::EraseOutOfBounds => "erase_out_of_bounds",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StorageError;

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(
            StorageError::MaintenanceRequired.code(),
            "maintenance_required"
        );
        assert_eq!(
            StorageError::UnsupportedFutureFormat.code(),
            "unsupported_future_format"
        );
    }
}
