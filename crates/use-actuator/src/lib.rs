#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive actuator vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty actuator name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActuatorName(String);

impl ActuatorName {
    /// Creates an actuator name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ActuatorTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ActuatorTextError> {
        non_empty_actuator_text(value).map(Self)
    }

    /// Returns the actuator name text.
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

impl AsRef<str> for ActuatorName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ActuatorName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ActuatorName {
    type Err = ActuatorTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive actuator kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActuatorKind {
    /// Electric motor actuator.
    ElectricMotor,
    /// Servo actuator.
    Servo,
    /// Stepper motor actuator.
    StepperMotor,
    /// Linear actuator.
    LinearActuator,
    /// Hydraulic actuator.
    Hydraulic,
    /// Pneumatic actuator.
    Pneumatic,
    /// Solenoid actuator.
    Solenoid,
    /// Unknown actuator kind.
    Unknown,
    /// Caller-defined actuator kind text.
    Custom(String),
}

impl fmt::Display for ActuatorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ElectricMotor => "electric-motor",
            Self::Servo => "servo",
            Self::StepperMotor => "stepper-motor",
            Self::LinearActuator => "linear-actuator",
            Self::Hydraulic => "hydraulic",
            Self::Pneumatic => "pneumatic",
            Self::Solenoid => "solenoid",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ActuatorKind {
    type Err = ActuatorKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ActuatorKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "electric-motor" | "motor" => Ok(Self::ElectricMotor),
            "servo" => Ok(Self::Servo),
            "stepper-motor" | "stepper" => Ok(Self::StepperMotor),
            "linear-actuator" => Ok(Self::LinearActuator),
            "hydraulic" => Ok(Self::Hydraulic),
            "pneumatic" => Ok(Self::Pneumatic),
            "solenoid" => Ok(Self::Solenoid),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Descriptive actuator state vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActuatorState {
    /// Idle actuator state.
    Idle,
    /// Enabled actuator state.
    Enabled,
    /// Disabled actuator state.
    Disabled,
    /// Moving actuator state.
    Moving,
    /// Faulted actuator state.
    Faulted,
    /// Unknown actuator state.
    Unknown,
    /// Caller-defined actuator state text.
    Custom(String),
}

impl fmt::Display for ActuatorState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Idle => "idle",
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Moving => "moving",
            Self::Faulted => "faulted",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ActuatorState {
    type Err = ActuatorStateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ActuatorStateParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "idle" => Ok(Self::Idle),
            "enabled" | "enable" => Ok(Self::Enabled),
            "disabled" | "disable" => Ok(Self::Disabled),
            "moving" => Ok(Self::Moving),
            "faulted" | "fault" => Ok(Self::Faulted),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A descriptive actuator rating label with optional numeric metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct ActuatorRating {
    label: String,
    value: Option<f64>,
    unit: Option<String>,
}

impl ActuatorRating {
    /// Creates an actuator rating from a non-empty label.
    ///
    /// # Errors
    ///
    /// Returns [`ActuatorTextError::Empty`] when the trimmed label is empty.
    pub fn new(label: impl AsRef<str>) -> Result<Self, ActuatorTextError> {
        Ok(Self {
            label: non_empty_actuator_text(label)?,
            value: None,
            unit: None,
        })
    }

    /// Returns this rating with a finite numeric value attached.
    ///
    /// # Errors
    ///
    /// Returns [`ActuatorRatingError::NonFinite`] when `value` is not finite.
    pub fn with_value(mut self, value: f64) -> Result<Self, ActuatorRatingError> {
        if !value.is_finite() {
            return Err(ActuatorRatingError::NonFinite);
        }

        self.value = Some(value);
        Ok(self)
    }

    /// Returns this rating with a non-empty unit label attached.
    ///
    /// # Errors
    ///
    /// Returns [`ActuatorTextError::Empty`] when the unit is empty after trimming.
    pub fn with_unit(mut self, unit: impl AsRef<str>) -> Result<Self, ActuatorTextError> {
        self.unit = Some(non_empty_actuator_text(unit)?);
        Ok(self)
    }

    /// Returns the rating label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the optional numeric value.
    #[must_use]
    pub const fn value(&self) -> Option<f64> {
        self.value
    }

    /// Returns the optional unit label.
    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}

/// Errors returned while constructing actuator text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActuatorTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ActuatorTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("actuator text cannot be empty"),
        }
    }
}

impl Error for ActuatorTextError {}

/// Error returned when parsing actuator kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActuatorKindParseError {
    /// The actuator kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ActuatorKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("actuator kind cannot be empty"),
        }
    }
}

impl Error for ActuatorKindParseError {}

/// Error returned when parsing actuator states fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActuatorStateParseError {
    /// The actuator state was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ActuatorStateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("actuator state cannot be empty"),
        }
    }
}

impl Error for ActuatorStateParseError {}

/// Errors returned while constructing actuator ratings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActuatorRatingError {
    /// Rating values must be finite.
    NonFinite,
}

impl fmt::Display for ActuatorRatingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("actuator rating value must be finite"),
        }
    }
}

impl Error for ActuatorRatingError {}

fn non_empty_actuator_text(value: impl AsRef<str>) -> Result<String, ActuatorTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(ActuatorTextError::Empty)
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
        ActuatorKind, ActuatorKindParseError, ActuatorName, ActuatorRating, ActuatorState,
        ActuatorStateParseError, ActuatorTextError,
    };

    #[test]
    fn constructs_valid_actuator_name() -> Result<(), ActuatorTextError> {
        let name = ActuatorName::new("  shoulder-servo  ")?;

        assert_eq!(name.as_str(), "shoulder-servo");
        Ok(())
    }

    #[test]
    fn rejects_empty_actuator_name() {
        assert_eq!(ActuatorName::new(""), Err(ActuatorTextError::Empty));
    }

    #[test]
    fn displays_and_parses_actuator_kind() -> Result<(), ActuatorKindParseError> {
        assert_eq!(
            "stepper motor".parse::<ActuatorKind>()?,
            ActuatorKind::StepperMotor
        );
        assert_eq!(ActuatorKind::LinearActuator.to_string(), "linear-actuator");
        Ok(())
    }

    #[test]
    fn displays_and_parses_actuator_state() -> Result<(), ActuatorStateParseError> {
        assert_eq!("enabled".parse::<ActuatorState>()?, ActuatorState::Enabled);
        assert_eq!(ActuatorState::Faulted.to_string(), "faulted");
        Ok(())
    }

    #[test]
    fn stores_custom_actuator_kind() -> Result<(), ActuatorKindParseError> {
        assert_eq!(
            "shape-memory".parse::<ActuatorKind>()?,
            ActuatorKind::Custom("shape-memory".to_string())
        );
        Ok(())
    }

    #[test]
    fn constructs_descriptive_rating() -> Result<(), Box<dyn std::error::Error>> {
        let rating = ActuatorRating::new("rated torque")?
            .with_value(12.0)?
            .with_unit("N m")?;

        assert_eq!(rating.label(), "rated torque");
        assert_eq!(rating.value(), Some(12.0));
        assert_eq!(rating.unit(), Some("N m"));
        Ok(())
    }
}
