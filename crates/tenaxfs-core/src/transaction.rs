use crate::maintenance::MaintenanceDebt;

/// Persistent transaction identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionId(pub u64);

impl TransactionId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// Resources declared before a transaction is accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransactionSpec {
    pub max_records: u16,
    pub max_payload_bytes: u32,
}

impl TransactionSpec {
    #[must_use]
    pub const fn new(max_records: u16, max_payload_bytes: u32) -> Self {
        Self {
            max_records,
            max_payload_bytes,
        }
    }
}

/// Admission result for a foreground operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionDecision {
    Accepted,
    MaintenanceRequired { debt: MaintenanceDebt },
    InsufficientReservedSpace,
    ReadOnlyPressureMode,
}

#[cfg(test)]
mod tests {
    use super::TransactionId;

    #[test]
    fn transaction_ids_advance_monotonically() {
        assert_eq!(TransactionId::new(1041).next(), TransactionId::new(1042));
    }
}
