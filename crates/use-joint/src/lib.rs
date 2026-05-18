#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robot joint vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty joint name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JointName(String);

impl JointName {
    /// Creates a joint name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`JointTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, JointTextError> {
        non_empty_joint_text(value).map(Self)
    }

    /// Returns the joint name text.
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

impl AsRef<str> for JointName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for JointName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for JointName {
    type Err = JointTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive robot joint vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JointKind {
    /// Revolute joint.
    Revolute,
    /// Prismatic joint.
    Prismatic,
    /// Fixed joint.
    Fixed,
    /// Continuous joint.
    Continuous,
    /// Spherical joint.
    Spherical,
    /// Planar joint.
    Planar,
    /// Floating joint.
    Floating,
    /// Unknown joint kind.
    Unknown,
    /// Caller-defined joint kind text.
    Custom(String),
}

impl fmt::Display for JointKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Revolute => "revolute",
            Self::Prismatic => "prismatic",
            Self::Fixed => "fixed",
            Self::Continuous => "continuous",
            Self::Spherical => "spherical",
            Self::Planar => "planar",
            Self::Floating => "floating",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for JointKind {
    type Err = JointKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(JointKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "revolute" => Ok(Self::Revolute),
            "prismatic" => Ok(Self::Prismatic),
            "fixed" => Ok(Self::Fixed),
            "continuous" => Ok(Self::Continuous),
            "spherical" => Ok(Self::Spherical),
            "planar" => Ok(Self::Planar),
            "floating" => Ok(Self::Floating),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Optional descriptive joint limits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointLimit {
    minimum: Option<f64>,
    maximum: Option<f64>,
}

impl JointLimit {
    /// Creates optional joint limits from finite numeric bounds.
    ///
    /// # Errors
    ///
    /// Returns [`JointLimitError::NonFinite`] when a bound is not finite, or
    /// [`JointLimitError::Inverted`] when the minimum is greater than the maximum.
    pub fn new(minimum: Option<f64>, maximum: Option<f64>) -> Result<Self, JointLimitError> {
        if minimum.is_some_and(|value| !value.is_finite())
            || maximum.is_some_and(|value| !value.is_finite())
        {
            return Err(JointLimitError::NonFinite);
        }

        if let (Some(minimum), Some(maximum)) = (minimum, maximum)
            && minimum > maximum
        {
            return Err(JointLimitError::Inverted);
        }

        Ok(Self { minimum, maximum })
    }

    /// Creates unbounded descriptive joint limits.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            minimum: None,
            maximum: None,
        }
    }

    /// Returns the optional minimum limit.
    #[must_use]
    pub const fn minimum(self) -> Option<f64> {
        self.minimum
    }

    /// Returns the optional maximum limit.
    #[must_use]
    pub const fn maximum(self) -> Option<f64> {
        self.maximum
    }
}

/// Descriptive joint axis labels.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JointAxis {
    /// X axis label.
    X,
    /// Y axis label.
    Y,
    /// Z axis label.
    Z,
    /// Unknown axis label.
    Unknown,
    /// Caller-defined axis label.
    Custom(String),
}

impl fmt::Display for JointAxis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for JointAxis {
    type Err = JointAxisParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(JointAxisParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "x" | "x-axis" => Ok(Self::X),
            "y" | "y-axis" => Ok(Self::Y),
            "z" | "z-axis" => Ok(Self::Z),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A zero-based joint index label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JointIndex(usize);

impl JointIndex {
    /// Creates a zero-based joint index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the index value.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

impl fmt::Display for JointIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Errors returned while constructing joint text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JointTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for JointTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("joint text cannot be empty"),
        }
    }
}

impl Error for JointTextError {}

/// Error returned when parsing joint kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JointKindParseError {
    /// The joint kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for JointKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("joint kind cannot be empty"),
        }
    }
}

impl Error for JointKindParseError {}

/// Errors returned while constructing joint limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JointLimitError {
    /// Limit values must be finite.
    NonFinite,
    /// The minimum limit was greater than the maximum limit.
    Inverted,
}

impl fmt::Display for JointLimitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("joint limit values must be finite"),
            Self::Inverted => formatter.write_str("joint minimum limit cannot exceed maximum"),
        }
    }
}

impl Error for JointLimitError {}

/// Error returned when parsing joint axes fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JointAxisParseError {
    /// The axis label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for JointAxisParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("joint axis cannot be empty"),
        }
    }
}

impl Error for JointAxisParseError {}

fn non_empty_joint_text(value: impl AsRef<str>) -> Result<String, JointTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(JointTextError::Empty)
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
        JointIndex, JointKind, JointKindParseError, JointLimit, JointLimitError, JointName,
        JointTextError,
    };

    #[test]
    fn constructs_valid_joint_name() -> Result<(), JointTextError> {
        let name = JointName::new("  shoulder-pan  ")?;

        assert_eq!(name.as_str(), "shoulder-pan");
        Ok(())
    }

    #[test]
    fn rejects_empty_joint_name() {
        assert_eq!(JointName::new(""), Err(JointTextError::Empty));
    }

    #[test]
    fn displays_and_parses_joint_kind() -> Result<(), JointKindParseError> {
        assert_eq!("revolute".parse::<JointKind>()?, JointKind::Revolute);
        assert_eq!(JointKind::Prismatic.to_string(), "prismatic");
        Ok(())
    }

    #[test]
    fn stores_custom_joint_kind() -> Result<(), JointKindParseError> {
        assert_eq!(
            "parallel-elastic".parse::<JointKind>()?,
            JointKind::Custom("parallel-elastic".to_string())
        );
        Ok(())
    }

    #[test]
    fn constructs_joint_limits() -> Result<(), JointLimitError> {
        let limit = JointLimit::new(Some(-1.0), Some(1.0))?;

        assert_eq!(limit.minimum(), Some(-1.0));
        assert_eq!(limit.maximum(), Some(1.0));
        Ok(())
    }

    #[test]
    fn constructs_joint_index() {
        let index = JointIndex::new(0);

        assert_eq!(index.get(), 0);
        assert_eq!(index.to_string(), "0");
    }
}
