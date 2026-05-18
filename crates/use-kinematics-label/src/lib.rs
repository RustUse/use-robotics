#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive kinematics terminology labels.

use core::{fmt, num::NonZeroUsize, str::FromStr};
use std::error::Error;

/// Descriptive kinematics terminology.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KinematicsKind {
    /// Forward kinematics label.
    Forward,
    /// Inverse kinematics label.
    Inverse,
    /// Differential kinematics label.
    Differential,
    /// Velocity kinematics label.
    Velocity,
    /// Position kinematics label.
    Position,
    /// Unknown kinematics kind.
    Unknown,
    /// Caller-defined kinematics kind text.
    Custom(String),
}

impl fmt::Display for KinematicsKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Forward => "forward",
            Self::Inverse => "inverse",
            Self::Differential => "differential",
            Self::Velocity => "velocity",
            Self::Position => "position",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for KinematicsKind {
    type Err = KinematicsKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(KinematicsKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "forward" | "forward-kinematics" => Ok(Self::Forward),
            "inverse" | "inverse-kinematics" => Ok(Self::Inverse),
            "differential" | "differential-kinematics" => Ok(Self::Differential),
            "velocity" | "velocity-kinematics" => Ok(Self::Velocity),
            "position" | "position-kinematics" => Ok(Self::Position),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A non-empty kinematic chain name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KinematicChainName(String);

impl KinematicChainName {
    /// Creates a kinematic chain name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`KinematicsTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, KinematicsTextError> {
        non_empty_kinematics_text(value).map(Self)
    }

    /// Returns the chain name text.
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

impl AsRef<str> for KinematicChainName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for KinematicChainName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for KinematicChainName {
    type Err = KinematicsTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-zero degree-of-freedom count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DegreeOfFreedom(NonZeroUsize);

impl DegreeOfFreedom {
    /// Creates a non-zero degree-of-freedom count.
    ///
    /// # Errors
    ///
    /// Returns [`DegreeOfFreedomError::Zero`] when `value` is zero.
    pub const fn new(value: usize) -> Result<Self, DegreeOfFreedomError> {
        match NonZeroUsize::new(value) {
            Some(value) => Ok(Self(value)),
            None => Err(DegreeOfFreedomError::Zero),
        }
    }

    /// Returns the degree-of-freedom count.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl TryFrom<usize> for DegreeOfFreedom {
    type Error = DegreeOfFreedomError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for DegreeOfFreedom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// A non-empty link name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LinkName(String);

impl LinkName {
    /// Creates a link name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`KinematicsTextError::Empty`] when the trimmed link name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, KinematicsTextError> {
        non_empty_kinematics_text(value).map(Self)
    }

    /// Returns the link name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the link name and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for LinkName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LinkName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LinkName {
    type Err = KinematicsTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Error returned when parsing kinematics kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KinematicsKindParseError {
    /// The kinematics kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for KinematicsKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("kinematics kind cannot be empty"),
        }
    }
}

impl Error for KinematicsKindParseError {}

/// Errors returned while constructing kinematics text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KinematicsTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for KinematicsTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("kinematics text cannot be empty"),
        }
    }
}

impl Error for KinematicsTextError {}

/// Errors returned while constructing degrees of freedom.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DegreeOfFreedomError {
    /// Degree-of-freedom counts must be non-zero.
    Zero,
}

impl fmt::Display for DegreeOfFreedomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("degree of freedom must be non-zero"),
        }
    }
}

impl Error for DegreeOfFreedomError {}

fn non_empty_kinematics_text(value: impl AsRef<str>) -> Result<String, KinematicsTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(KinematicsTextError::Empty)
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
        DegreeOfFreedom, DegreeOfFreedomError, KinematicChainName, KinematicsKind,
        KinematicsKindParseError, KinematicsTextError,
    };

    #[test]
    fn displays_and_parses_kinematics_kind() -> Result<(), KinematicsKindParseError> {
        assert_eq!(
            "forward kinematics".parse::<KinematicsKind>()?,
            KinematicsKind::Forward
        );
        assert_eq!(KinematicsKind::Differential.to_string(), "differential");
        Ok(())
    }

    #[test]
    fn stores_custom_kinematics_kind() -> Result<(), KinematicsKindParseError> {
        assert_eq!(
            "redundancy-resolution".parse::<KinematicsKind>()?,
            KinematicsKind::Custom("redundancy-resolution".to_string())
        );
        Ok(())
    }

    #[test]
    fn constructs_valid_chain_name() -> Result<(), KinematicsTextError> {
        let name = KinematicChainName::new("  arm-chain  ")?;

        assert_eq!(name.as_str(), "arm-chain");
        Ok(())
    }

    #[test]
    fn rejects_empty_chain_name() {
        assert_eq!(KinematicChainName::new(""), Err(KinematicsTextError::Empty));
    }

    #[test]
    fn constructs_valid_degree_of_freedom() -> Result<(), DegreeOfFreedomError> {
        let dof = DegreeOfFreedom::new(6)?;

        assert_eq!(dof.get(), 6);
        assert_eq!(dof.to_string(), "6");
        Ok(())
    }

    #[test]
    fn rejects_zero_degree_of_freedom() {
        assert_eq!(DegreeOfFreedom::new(0), Err(DegreeOfFreedomError::Zero));
    }
}
