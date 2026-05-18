#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robotics frame vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty robotics frame name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FrameName(String);

impl FrameName {
    /// Creates a frame name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FrameTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FrameTextError> {
        non_empty_frame_text(value).map(Self)
    }

    /// Returns the frame name text.
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

impl AsRef<str> for FrameName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FrameName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FrameName {
    type Err = FrameTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive robotics frame kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrameKind {
    /// World frame label.
    World,
    /// Map frame label.
    Map,
    /// Odometry frame label.
    Odom,
    /// Robot base frame label.
    Base,
    /// Tool frame label.
    Tool,
    /// Sensor frame label.
    Sensor,
    /// Joint frame label.
    Joint,
    /// Link frame label.
    Link,
    /// Unknown frame kind.
    Unknown,
    /// Caller-defined frame kind text.
    Custom(String),
}

impl fmt::Display for FrameKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::World => "world",
            Self::Map => "map",
            Self::Odom => "odom",
            Self::Base => "base",
            Self::Tool => "tool",
            Self::Sensor => "sensor",
            Self::Joint => "joint",
            Self::Link => "link",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for FrameKind {
    type Err = FrameKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(FrameKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "world" => Ok(Self::World),
            "map" => Ok(Self::Map),
            "odom" | "odometry" => Ok(Self::Odom),
            "base" | "base-link" => Ok(Self::Base),
            "tool" => Ok(Self::Tool),
            "sensor" => Ok(Self::Sensor),
            "joint" => Ok(Self::Joint),
            "link" => Ok(Self::Link),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A named frame reference with a descriptive kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FrameRef {
    name: FrameName,
    kind: FrameKind,
}

impl FrameRef {
    /// Creates a frame reference from a name and kind.
    #[must_use]
    pub const fn new(name: FrameName, kind: FrameKind) -> Self {
        Self { name, kind }
    }

    /// Creates a frame reference with an unknown kind.
    #[must_use]
    pub const fn named(name: FrameName) -> Self {
        Self {
            name,
            kind: FrameKind::Unknown,
        }
    }

    /// Returns the frame name.
    #[must_use]
    pub const fn name(&self) -> &FrameName {
        &self.name
    }

    /// Returns the frame kind.
    #[must_use]
    pub const fn kind(&self) -> &FrameKind {
        &self.kind
    }
}

impl fmt::Display for FrameRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.kind, self.name)
    }
}

/// A descriptive parent/child frame relation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FrameRelation {
    parent: FrameRef,
    child: FrameRef,
}

impl FrameRelation {
    /// Creates a parent/child frame relation label.
    #[must_use]
    pub const fn new(parent: FrameRef, child: FrameRef) -> Self {
        Self { parent, child }
    }

    /// Returns the parent frame reference.
    #[must_use]
    pub const fn parent(&self) -> &FrameRef {
        &self.parent
    }

    /// Returns the child frame reference.
    #[must_use]
    pub const fn child(&self) -> &FrameRef {
        &self.child
    }
}

/// Errors returned while constructing frame text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for FrameTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("frame text cannot be empty"),
        }
    }
}

impl Error for FrameTextError {}

/// Error returned when parsing frame kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameKindParseError {
    /// The frame kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for FrameKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("frame kind cannot be empty"),
        }
    }
}

impl Error for FrameKindParseError {}

fn non_empty_frame_text(value: impl AsRef<str>) -> Result<String, FrameTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(FrameTextError::Empty)
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
        FrameKind, FrameKindParseError, FrameName, FrameRef, FrameRelation, FrameTextError,
    };

    #[test]
    fn constructs_valid_frame_name() -> Result<(), FrameTextError> {
        let name = FrameName::new("  base_link  ")?;

        assert_eq!(name.as_str(), "base_link");
        Ok(())
    }

    #[test]
    fn rejects_empty_frame_name() {
        assert_eq!(FrameName::new(""), Err(FrameTextError::Empty));
    }

    #[test]
    fn displays_and_parses_frame_kind() -> Result<(), FrameKindParseError> {
        assert_eq!("base link".parse::<FrameKind>()?, FrameKind::Base);
        assert_eq!(FrameKind::Odom.to_string(), "odom");
        Ok(())
    }

    #[test]
    fn stores_custom_frame_kind() -> Result<(), FrameKindParseError> {
        assert_eq!(
            "fixture".parse::<FrameKind>()?,
            FrameKind::Custom("fixture".to_string())
        );
        Ok(())
    }

    #[test]
    fn constructs_parent_child_relation() -> Result<(), FrameTextError> {
        let parent = FrameRef::new(FrameName::new("base_link")?, FrameKind::Base);
        let child = FrameRef::new(FrameName::new("tool0")?, FrameKind::Tool);
        let relation = FrameRelation::new(parent, child);

        assert_eq!(relation.parent().name().as_str(), "base_link");
        assert_eq!(relation.child().kind(), &FrameKind::Tool);
        Ok(())
    }
}
