/// Explicit budget for one maintenance slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaintenanceBudget {
    pub max_copy_bytes: u32,
    pub max_program_ops: u16,
    pub max_erase_ops: u16,
    pub allow_erase: bool,
}

impl MaintenanceBudget {
    /// No maintenance work may be performed.
    pub const NONE: Self = Self {
        max_copy_bytes: 0,
        max_program_ops: 0,
        max_erase_ops: 0,
        allow_erase: false,
    };

    #[must_use]
    pub const fn from_ops(max_copy_bytes: u32, max_program_ops: u16, allow_erase: bool) -> Self {
        Self {
            max_copy_bytes,
            max_program_ops,
            max_erase_ops: if allow_erase { 1 } else { 0 },
            allow_erase,
        }
    }

    #[must_use]
    pub const fn permits_erase(self) -> bool {
        self.allow_erase && self.max_erase_ops > 0
    }
}

/// Observable storage pressure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressureState {
    Normal,
    Elevated,
    High,
    Critical,
    MaintenanceRequired,
}

/// Work required to restore configured free-space and wear policies.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MaintenanceDebt {
    pub segments: u32,
    pub copy_bytes: u64,
    pub erase_ops: u32,
}

/// Objective flash health data exposed to applications and host tooling.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HealthSnapshot {
    pub total_erases: u64,
    pub minimum_erase_count: u32,
    pub maximum_erase_count: u32,
    pub retired_segments: u32,
    pub free_segments: u32,
    pub reclaimable_segments: u32,
    pub maintenance_debt: MaintenanceDebt,
}

#[cfg(test)]
mod tests {
    use super::MaintenanceBudget;

    #[test]
    fn erase_requires_permission_and_budget() {
        assert!(!MaintenanceBudget::NONE.permits_erase());
        assert!(!MaintenanceBudget::from_ops(256, 2, false).permits_erase());
        assert!(MaintenanceBudget::from_ops(256, 2, true).permits_erase());
    }
}
