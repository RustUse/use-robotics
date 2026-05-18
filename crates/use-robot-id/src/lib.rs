#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robot identity vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty robot identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotId(String);

impl RobotId {
    /// Creates a robot ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotIdError::Empty`] when the trimmed identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotIdError> {
        non_empty_identifier(value).map(Self)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the ID and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RobotId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotId {
    type Err = RobotIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty robot serial number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotSerialNumber(String);

impl RobotSerialNumber {
    /// Creates a serial number from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotIdError::Empty`] when the trimmed serial number is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotIdError> {
        non_empty_identifier(value).map(Self)
    }

    /// Returns the serial number text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the serial number and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RobotSerialNumber {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotSerialNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotSerialNumber {
    type Err = RobotIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty robot instance identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotInstanceId(String);

impl RobotInstanceId {
    /// Creates an instance ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotIdError::Empty`] when the trimmed instance ID is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotIdError> {
        non_empty_identifier(value).map(Self)
    }

    /// Returns the instance ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the instance ID and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RobotInstanceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotInstanceId {
    type Err = RobotIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing robot identity values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotIdError {
    /// The identifier was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot identifier cannot be empty"),
        }
    }
}

impl Error for RobotIdError {}

fn non_empty_identifier(value: impl AsRef<str>) -> Result<String, RobotIdError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(RobotIdError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{RobotId, RobotIdError, RobotInstanceId, RobotSerialNumber};

    #[test]
    fn constructs_valid_robot_id() -> Result<(), RobotIdError> {
        let id = RobotId::new("robot:A-17")?;

        assert_eq!(id.as_str(), "robot:A-17");
        Ok(())
    }

    #[test]
    fn rejects_empty_robot_id() {
        assert_eq!(RobotId::new("  "), Err(RobotIdError::Empty));
    }

    #[test]
    fn constructs_serial_number() -> Result<(), RobotIdError> {
        let serial = RobotSerialNumber::new("SN-2026-A")?;

        assert_eq!(serial.as_str(), "SN-2026-A");
        Ok(())
    }

    #[test]
    fn constructs_instance_id() -> Result<(), RobotIdError> {
        let instance = RobotInstanceId::new("cell-4/arm-1")?;

        assert_eq!(instance.as_str(), "cell-4/arm-1");
        Ok(())
    }

    #[test]
    fn displays_identifiers() -> Result<(), RobotIdError> {
        let id = RobotId::new("Robot.ID:42")?;

        assert_eq!(id.to_string(), "Robot.ID:42");
        Ok(())
    }
}
