#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive end-effector vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty end-effector name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EndEffectorName(String);

impl EndEffectorName {
    /// Creates an end-effector name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`EndEffectorTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, EndEffectorTextError> {
        non_empty_end_effector_text(value).map(Self)
    }

    /// Returns the end-effector name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for EndEffectorName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for EndEffectorName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EndEffectorName {
    type Err = EndEffectorTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive end-effector kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EndEffectorKind {
    /// Gripper end effector.
    Gripper,
    /// Vacuum gripper end effector.
    VacuumGripper,
    /// Welder tool.
    Welder,
    /// Cutter tool.
    Cutter,
    /// Drill tool.
    Drill,
    /// Nozzle tool.
    Nozzle,
    /// Suction cup end effector.
    SuctionCup,
    /// Tool changer end effector.
    ToolChanger,
    /// Unknown end-effector kind.
    Unknown,
    /// Caller-defined end-effector kind text.
    Custom(String),
}

impl fmt::Display for EndEffectorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Gripper => "gripper",
            Self::VacuumGripper => "vacuum-gripper",
            Self::Welder => "welder",
            Self::Cutter => "cutter",
            Self::Drill => "drill",
            Self::Nozzle => "nozzle",
            Self::SuctionCup => "suction-cup",
            Self::ToolChanger => "tool-changer",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for EndEffectorKind {
    type Err = EndEffectorKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(EndEffectorKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "gripper" => Ok(Self::Gripper),
            "vacuum-gripper" => Ok(Self::VacuumGripper),
            "welder" => Ok(Self::Welder),
            "cutter" => Ok(Self::Cutter),
            "drill" => Ok(Self::Drill),
            "nozzle" => Ok(Self::Nozzle),
            "suction-cup" => Ok(Self::SuctionCup),
            "tool-changer" => Ok(Self::ToolChanger),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Descriptive grip state vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GripState {
    /// Open grip state.
    Open,
    /// Closed grip state.
    Closed,
    /// Holding grip state.
    Holding,
    /// Released grip state.
    Released,
    /// Unknown grip state.
    Unknown,
    /// Caller-defined grip state text.
    Custom(String),
}

impl fmt::Display for GripState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Holding => "holding",
            Self::Released => "released",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for GripState {
    type Err = GripStateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(GripStateParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "open" => Ok(Self::Open),
            "closed" => Ok(Self::Closed),
            "holding" => Ok(Self::Holding),
            "released" | "release" => Ok(Self::Released),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A non-empty descriptive tool mount label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ToolMount(String);

impl ToolMount {
    /// Creates a tool mount label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`EndEffectorTextError::Empty`] when the trimmed label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, EndEffectorTextError> {
        non_empty_end_effector_text(value).map(Self)
    }

    /// Returns the mount label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the mount label and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ToolMount {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ToolMount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ToolMount {
    type Err = EndEffectorTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing end-effector text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndEffectorTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for EndEffectorTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("end-effector text cannot be empty"),
        }
    }
}

impl Error for EndEffectorTextError {}

/// Error returned when parsing end-effector kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndEffectorKindParseError {
    /// The end-effector kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for EndEffectorKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("end-effector kind cannot be empty"),
        }
    }
}

impl Error for EndEffectorKindParseError {}

/// Error returned when parsing grip states fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GripStateParseError {
    /// The grip state was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for GripStateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("grip state cannot be empty"),
        }
    }
}

impl Error for GripStateParseError {}

fn non_empty_end_effector_text(value: impl AsRef<str>) -> Result<String, EndEffectorTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(EndEffectorTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|character| match character {
            '_' | ' ' => '-',
            other => other.to_ascii_lowercase(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        EndEffectorKind, EndEffectorKindParseError, EndEffectorName, EndEffectorTextError,
        GripState, GripStateParseError,
    };

    #[test]
    fn constructs_valid_end_effector_name() -> Result<(), EndEffectorTextError> {
        let name = EndEffectorName::new("  parallel-gripper  ")?;

        assert_eq!(name.as_str(), "parallel-gripper");
        Ok(())
    }

    #[test]
    fn rejects_empty_end_effector_name() {
        assert_eq!(EndEffectorName::new(""), Err(EndEffectorTextError::Empty));
    }

    #[test]
    fn displays_and_parses_end_effector_kind() -> Result<(), EndEffectorKindParseError> {
        assert_eq!(
            "vacuum gripper".parse::<EndEffectorKind>()?,
            EndEffectorKind::VacuumGripper
        );
        assert_eq!(EndEffectorKind::ToolChanger.to_string(), "tool-changer");
        Ok(())
    }

    #[test]
    fn displays_and_parses_grip_state() -> Result<(), GripStateParseError> {
        assert_eq!("open".parse::<GripState>()?, GripState::Open);
        assert_eq!(GripState::Holding.to_string(), "holding");
        Ok(())
    }

    #[test]
    fn stores_custom_end_effector_kind() -> Result<(), EndEffectorKindParseError> {
        assert_eq!(
            "magnetic-gripper".parse::<EndEffectorKind>()?,
            EndEffectorKind::Custom("magnetic-gripper".to_string())
        );
        Ok(())
    }
}
