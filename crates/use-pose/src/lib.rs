#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robot pose vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty pose name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PoseName(String);

impl PoseName {
    /// Creates a pose name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PoseTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PoseTextError> {
        non_empty_pose_text(value).map(Self)
    }

    /// Returns the pose name text.
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

impl AsRef<str> for PoseName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PoseName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PoseName {
    type Err = PoseTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive pose kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PoseKind {
    /// Home pose label.
    Home,
    /// Ready pose label.
    Ready,
    /// Rest pose label.
    Rest,
    /// Tool pose label.
    Tool,
    /// Target pose label.
    Target,
    /// Waypoint pose label.
    Waypoint,
    /// Calibration pose label.
    Calibration,
    /// Unknown pose kind.
    Unknown,
    /// Caller-defined pose kind text.
    Custom(String),
}

impl fmt::Display for PoseKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Home => "home",
            Self::Ready => "ready",
            Self::Rest => "rest",
            Self::Tool => "tool",
            Self::Target => "target",
            Self::Waypoint => "waypoint",
            Self::Calibration => "calibration",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for PoseKind {
    type Err = PoseKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(PoseKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "home" => Ok(Self::Home),
            "ready" => Ok(Self::Ready),
            "rest" => Ok(Self::Rest),
            "tool" => Ok(Self::Tool),
            "target" => Ok(Self::Target),
            "waypoint" => Ok(Self::Waypoint),
            "calibration" => Ok(Self::Calibration),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A non-empty 2D pose label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pose2Label(String);

impl Pose2Label {
    /// Creates a 2D pose label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PoseTextError::Empty`] when the trimmed label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PoseTextError> {
        non_empty_pose_text(value).map(Self)
    }

    /// Returns the pose label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the label and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Pose2Label {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Pose2Label {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Pose2Label {
    type Err = PoseTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty 3D pose label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pose3Label(String);

impl Pose3Label {
    /// Creates a 3D pose label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PoseTextError::Empty`] when the trimmed label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PoseTextError> {
        non_empty_pose_text(value).map(Self)
    }

    /// Returns the pose label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the label and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Pose3Label {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Pose3Label {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Pose3Label {
    type Err = PoseTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive orientation representation labels.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrientationKind {
    /// Euler orientation label.
    Euler,
    /// Quaternion orientation label.
    Quaternion,
    /// Axis-angle orientation label.
    AxisAngle,
    /// Rotation-matrix orientation label.
    RotationMatrix,
    /// Unknown orientation kind.
    Unknown,
    /// Caller-defined orientation kind text.
    Custom(String),
}

impl fmt::Display for OrientationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Euler => "euler",
            Self::Quaternion => "quaternion",
            Self::AxisAngle => "axis-angle",
            Self::RotationMatrix => "rotation-matrix",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for OrientationKind {
    type Err = OrientationKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(OrientationKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "euler" => Ok(Self::Euler),
            "quaternion" => Ok(Self::Quaternion),
            "axis-angle" => Ok(Self::AxisAngle),
            "rotation-matrix" => Ok(Self::RotationMatrix),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while constructing pose text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoseTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PoseTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pose text cannot be empty"),
        }
    }
}

impl Error for PoseTextError {}

/// Error returned when parsing pose kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoseKindParseError {
    /// The pose kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PoseKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pose kind cannot be empty"),
        }
    }
}

impl Error for PoseKindParseError {}

/// Error returned when parsing orientation kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrientationKindParseError {
    /// The orientation kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for OrientationKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("orientation kind cannot be empty"),
        }
    }
}

impl Error for OrientationKindParseError {}

fn non_empty_pose_text(value: impl AsRef<str>) -> Result<String, PoseTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(PoseTextError::Empty)
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
        OrientationKind, OrientationKindParseError, PoseKind, PoseKindParseError, PoseName,
        PoseTextError,
    };

    #[test]
    fn constructs_valid_pose_name() -> Result<(), PoseTextError> {
        let name = PoseName::new("  home  ")?;

        assert_eq!(name.as_str(), "home");
        Ok(())
    }

    #[test]
    fn rejects_empty_pose_name() {
        assert_eq!(PoseName::new(""), Err(PoseTextError::Empty));
    }

    #[test]
    fn displays_and_parses_pose_kind() -> Result<(), PoseKindParseError> {
        assert_eq!("waypoint".parse::<PoseKind>()?, PoseKind::Waypoint);
        assert_eq!(PoseKind::Calibration.to_string(), "calibration");
        Ok(())
    }

    #[test]
    fn displays_and_parses_orientation_kind() -> Result<(), OrientationKindParseError> {
        assert_eq!(
            "axis angle".parse::<OrientationKind>()?,
            OrientationKind::AxisAngle
        );
        assert_eq!(
            OrientationKind::RotationMatrix.to_string(),
            "rotation-matrix"
        );
        Ok(())
    }

    #[test]
    fn stores_custom_pose_kind() -> Result<(), PoseKindParseError> {
        assert_eq!(
            "inspection-hover".parse::<PoseKind>()?,
            PoseKind::Custom("inspection-hover".to_string())
        );
        Ok(())
    }
}
