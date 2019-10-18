//! Supported Drone crates.

/// Drone platform crates.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Platform {
    CortexM,
}

/// Drone register and interrupt binding crates.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Bindings {
    Nrf,
    Stm32,
}

impl Platform {
    /// Returns the name in kebab-case.
    pub fn kebab_name(&self) -> &str {
        match self {
            Self::CortexM => "cortex-m",
        }
    }

    /// Returns the name in underscore-case.
    pub fn underscore_name(&self) -> &str {
        match self {
            Self::CortexM => "cortex_m",
        }
    }
}

impl Bindings {
    /// Returns the name in kebab-case.
    pub fn kebab_name(&self) -> &str {
        match self {
            Self::Nrf => "nrf",
            Self::Stm32 => "stm32",
        }
    }

    /// Returns the name in underscore-case.
    pub fn underscore_name(&self) -> &str {
        match self {
            Self::Nrf => "nrf",
            Self::Stm32 => "stm32",
        }
    }
}
