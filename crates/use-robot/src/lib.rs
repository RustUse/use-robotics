#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robot vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty robot name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotName(String);

impl RobotName {
    /// Creates a robot name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotTextError> {
        non_empty_robot_text(value).map(Self)
    }

    /// Returns the robot name text.
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

impl AsRef<str> for RobotName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotName {
    type Err = RobotTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Broad robot vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RobotKind {
    /// Robot arm.
    Arm,
    /// Mobile robot.
    Mobile,
    /// Humanoid robot.
    Humanoid,
    /// Flying robot or drone.
    Drone,
    /// Quadruped robot.
    Quadruped,
    /// Manipulator robot.
    Manipulator,
    /// Collaborative robot.
    Collaborative,
    /// Industrial robot.
    Industrial,
    /// Educational robot.
    Educational,
    /// Unknown robot kind.
    Unknown,
    /// Caller-defined robot kind text.
    Custom(String),
}

impl fmt::Display for RobotKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Arm => "arm",
            Self::Mobile => "mobile",
            Self::Humanoid => "humanoid",
            Self::Drone => "drone",
            Self::Quadruped => "quadruped",
            Self::Manipulator => "manipulator",
            Self::Collaborative => "collaborative",
            Self::Industrial => "industrial",
            Self::Educational => "educational",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for RobotKind {
    type Err = RobotKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(RobotKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "arm" | "robot-arm" => Ok(Self::Arm),
            "mobile" | "mobile-robot" => Ok(Self::Mobile),
            "humanoid" => Ok(Self::Humanoid),
            "drone" | "uav" => Ok(Self::Drone),
            "quadruped" => Ok(Self::Quadruped),
            "manipulator" => Ok(Self::Manipulator),
            "collaborative" | "cobot" => Ok(Self::Collaborative),
            "industrial" => Ok(Self::Industrial),
            "educational" => Ok(Self::Educational),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A non-empty robot model label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotModel(String);

impl RobotModel {
    /// Creates a robot model from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotTextError::Empty`] when the trimmed model is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotTextError> {
        non_empty_robot_text(value).map(Self)
    }

    /// Returns the model text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the model and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RobotModel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotModel {
    type Err = RobotTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive robot manufacturer label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotManufacturer(String);

impl RobotManufacturer {
    /// Creates a manufacturer label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotTextError::Empty`] when the trimmed manufacturer is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotTextError> {
        non_empty_robot_text(value).map(Self)
    }

    /// Returns the manufacturer text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the manufacturer and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for RobotManufacturer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotManufacturer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotManufacturer {
    type Err = RobotTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing robot text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot text cannot be empty"),
        }
    }
}

impl Error for RobotTextError {}

/// Error returned when parsing robot kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotKindParseError {
    /// The robot kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot kind cannot be empty"),
        }
    }
}

impl Error for RobotKindParseError {}

fn non_empty_robot_text(value: impl AsRef<str>) -> Result<String, RobotTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(RobotTextError::Empty)
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
    use super::{RobotKind, RobotKindParseError, RobotModel, RobotName, RobotTextError};

    #[test]
    fn constructs_valid_robot_name() -> Result<(), RobotTextError> {
        let name = RobotName::new("  Atlas  ")?;

        assert_eq!(name.as_str(), "Atlas");
        assert_eq!(name.to_string(), "Atlas");
        Ok(())
    }

    #[test]
    fn rejects_empty_robot_name() {
        assert_eq!(RobotName::new("   "), Err(RobotTextError::Empty));
    }

    #[test]
    fn displays_and_parses_robot_kind() -> Result<(), RobotKindParseError> {
        assert_eq!("robot arm".parse::<RobotKind>()?, RobotKind::Arm);
        assert_eq!(RobotKind::Collaborative.to_string(), "collaborative");
        Ok(())
    }

    #[test]
    fn stores_custom_robot_kind() -> Result<(), RobotKindParseError> {
        assert_eq!(
            "pipe-crawler".parse::<RobotKind>()?,
            RobotKind::Custom("pipe-crawler".to_string())
        );
        Ok(())
    }

    #[test]
    fn constructs_robot_model() -> Result<(), RobotTextError> {
        let model = RobotModel::new("RX-4")?;

        assert_eq!(model.as_str(), "RX-4");
        Ok(())
    }
}
