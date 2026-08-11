/// Current persistent format version.
pub const FORMAT_VERSION: u16 = 1;

/// Segment header magic bytes.
pub const SEGMENT_MAGIC: [u8; 8] = *b"TNXSEG01";

/// Record header magic bytes.
pub const RECORD_MAGIC: [u8; 8] = *b"TNXREC01";

/// Persistent segment state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SegmentState {
    Free = 0x01,
    Active = 0x02,
    Closed = 0x03,
    Reclaimable = 0x04,
    Suspect = 0x05,
    Retired = 0x06,
}

impl SegmentState {
    /// Decodes a persistent state byte.
    #[must_use]
    pub const fn from_byte(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::Free),
            0x02 => Some(Self::Active),
            0x03 => Some(Self::Closed),
            0x04 => Some(Self::Reclaimable),
            0x05 => Some(Self::Suspect),
            0x06 => Some(Self::Retired),
            _ => None,
        }
    }
}

/// Persistent record kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RecordKind {
    Object = 0x01,
    Commit = 0x02,
    Checkpoint = 0x03,
    Epoch = 0x04,
    Gc = 0x05,
}

impl RecordKind {
    /// Decodes a persistent record kind byte.
    #[must_use]
    pub const fn from_byte(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::Object),
            0x02 => Some(Self::Commit),
            0x03 => Some(Self::Checkpoint),
            0x04 => Some(Self::Epoch),
            0x05 => Some(Self::Gc),
            _ => None,
        }
    }
}

/// Encodes `value` as little-endian bytes.
#[must_use]
pub const fn le_u16(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

/// Encodes `value` as little-endian bytes.
#[must_use]
pub const fn le_u32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

/// Encodes `value` as little-endian bytes.
#[must_use]
pub const fn le_u64(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::{RecordKind, SegmentState, FORMAT_VERSION};

    #[test]
    fn persistent_enums_reject_unknown_values() {
        assert_eq!(SegmentState::from_byte(0x06), Some(SegmentState::Retired));
        assert_eq!(SegmentState::from_byte(0xff), None);
        assert_eq!(RecordKind::from_byte(0x02), Some(RecordKind::Commit));
        assert_eq!(RecordKind::from_byte(0xff), None);
    }

    #[test]
    fn format_version_starts_at_one() {
        assert_eq!(FORMAT_VERSION, 1);
    }
}
