#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robot subsystem vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty robot subsystem name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotSubsystemName(String);

impl RobotSubsystemName {
    /// Creates a robot subsystem name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotSubsystemTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotSubsystemTextError> {
        non_empty_subsystem_text(value).map(Self)
    }

    /// Returns the subsystem name text.
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

impl AsRef<str> for RobotSubsystemName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotSubsystemName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotSubsystemName {
    type Err = RobotSubsystemTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive robot subsystem kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RobotSubsystemKind {
    /// Locomotion subsystem label.
    Locomotion,
    /// Manipulation subsystem label.
    Manipulation,
    /// Perception subsystem label.
    Perception,
    /// Power subsystem label.
    Power,
    /// Control subsystem label.
    Control,
    /// Communication subsystem label.
    Communication,
    /// Navigation subsystem label.
    Navigation,
    /// Safety subsystem label.
    Safety,
    /// Human-interface subsystem label.
    HumanInterface,
    /// Unknown subsystem kind.
    Unknown,
    /// Caller-defined subsystem kind text.
    Custom(String),
}

impl fmt::Display for RobotSubsystemKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Locomotion => "locomotion",
            Self::Manipulation => "manipulation",
            Self::Perception => "perception",
            Self::Power => "power",
            Self::Control => "control",
            Self::Communication => "communication",
            Self::Navigation => "navigation",
            Self::Safety => "safety",
            Self::HumanInterface => "human-interface",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for RobotSubsystemKind {
    type Err = RobotSubsystemKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(RobotSubsystemKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "locomotion" => Ok(Self::Locomotion),
            "manipulation" => Ok(Self::Manipulation),
            "perception" => Ok(Self::Perception),
            "power" => Ok(Self::Power),
            "control" => Ok(Self::Control),
            "communication" | "communications" => Ok(Self::Communication),
            "navigation" => Ok(Self::Navigation),
            "safety" => Ok(Self::Safety),
            "human-interface" | "hmi" => Ok(Self::HumanInterface),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Descriptive subsystem lifecycle or status vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubsystemState {
    /// Offline subsystem state.
    Offline,
    /// Starting subsystem state.
    Starting,
    /// Ready subsystem state.
    Ready,
    /// Active subsystem state.
    Active,
    /// Degraded subsystem state.
    Degraded,
    /// Faulted subsystem state.
    Faulted,
    /// Stopped subsystem state.
    Stopped,
    /// Unknown subsystem state.
    Unknown,
    /// Caller-defined subsystem state text.
    Custom(String),
}

impl fmt::Display for SubsystemState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Offline => "offline",
            Self::Starting => "starting",
            Self::Ready => "ready",
            Self::Active => "active",
            Self::Degraded => "degraded",
            Self::Faulted => "faulted",
            Self::Stopped => "stopped",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for SubsystemState {
    type Err = SubsystemStateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SubsystemStateParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "offline" => Ok(Self::Offline),
            "starting" => Ok(Self::Starting),
            "ready" => Ok(Self::Ready),
            "active" => Ok(Self::Active),
            "degraded" => Ok(Self::Degraded),
            "faulted" | "fault" => Ok(Self::Faulted),
            "stopped" | "stop" => Ok(Self::Stopped),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while constructing subsystem text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotSubsystemTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotSubsystemTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot subsystem text cannot be empty"),
        }
    }
}

impl Error for RobotSubsystemTextError {}

/// Error returned when parsing subsystem kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotSubsystemKindParseError {
    /// The subsystem kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotSubsystemKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot subsystem kind cannot be empty"),
        }
    }
}

impl Error for RobotSubsystemKindParseError {}

/// Error returned when parsing subsystem states fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubsystemStateParseError {
    /// The subsystem state was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for SubsystemStateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("subsystem state cannot be empty"),
        }
    }
}

impl Error for SubsystemStateParseError {}

fn non_empty_subsystem_text(value: impl AsRef<str>) -> Result<String, RobotSubsystemTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(RobotSubsystemTextError::Empty)
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
        RobotSubsystemKind, RobotSubsystemKindParseError, RobotSubsystemName,
        RobotSubsystemTextError, SubsystemState, SubsystemStateParseError,
    };

    #[test]
    fn constructs_valid_subsystem_name() -> Result<(), RobotSubsystemTextError> {
        let name = RobotSubsystemName::new("  arm  ")?;

        assert_eq!(name.as_str(), "arm");
        Ok(())
    }

    #[test]
    fn rejects_empty_subsystem_name() {
        assert_eq!(
            RobotSubsystemName::new(""),
            Err(RobotSubsystemTextError::Empty)
        );
    }

    #[test]
    fn displays_and_parses_subsystem_kind() -> Result<(), RobotSubsystemKindParseError> {
        assert_eq!(
            "human interface".parse::<RobotSubsystemKind>()?,
            RobotSubsystemKind::HumanInterface
        );
        assert_eq!(RobotSubsystemKind::Perception.to_string(), "perception");
        Ok(())
    }

    #[test]
    fn displays_and_parses_subsystem_state() -> Result<(), SubsystemStateParseError> {
        assert_eq!("ready".parse::<SubsystemState>()?, SubsystemState::Ready);
        assert_eq!(SubsystemState::Degraded.to_string(), "degraded");
        Ok(())
    }

    #[test]
    fn stores_custom_subsystem_kind() -> Result<(), RobotSubsystemKindParseError> {
        assert_eq!(
            "payload-handling".parse::<RobotSubsystemKind>()?,
            RobotSubsystemKind::Custom("payload-handling".to_string())
        );
        Ok(())
    }
}
