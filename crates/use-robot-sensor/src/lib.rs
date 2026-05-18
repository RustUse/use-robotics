#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Primitive robot sensor vocabulary.

use core::{fmt, str::FromStr};
use std::error::Error;

/// A non-empty robot sensor name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RobotSensorName(String);

impl RobotSensorName {
    /// Creates a sensor name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotSensorTextError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotSensorTextError> {
        non_empty_sensor_text(value).map(Self)
    }

    /// Returns the sensor name text.
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

impl AsRef<str> for RobotSensorName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RobotSensorName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RobotSensorName {
    type Err = RobotSensorTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive robot sensor kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RobotSensorKind {
    /// Camera sensor.
    Camera,
    /// Lidar sensor.
    Lidar,
    /// Radar sensor.
    Radar,
    /// Inertial measurement unit.
    Imu,
    /// Encoder sensor.
    Encoder,
    /// Force-torque sensor.
    ForceTorque,
    /// Proximity sensor.
    Proximity,
    /// Ultrasonic sensor.
    Ultrasonic,
    /// GPS receiver.
    Gps,
    /// Microphone sensor.
    Microphone,
    /// Unknown sensor kind.
    Unknown,
    /// Caller-defined sensor kind text.
    Custom(String),
}

impl fmt::Display for RobotSensorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Camera => "camera",
            Self::Lidar => "lidar",
            Self::Radar => "radar",
            Self::Imu => "imu",
            Self::Encoder => "encoder",
            Self::ForceTorque => "force-torque",
            Self::Proximity => "proximity",
            Self::Ultrasonic => "ultrasonic",
            Self::Gps => "gps",
            Self::Microphone => "microphone",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for RobotSensorKind {
    type Err = RobotSensorKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(RobotSensorKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "camera" => Ok(Self::Camera),
            "lidar" | "laser-scanner" => Ok(Self::Lidar),
            "radar" => Ok(Self::Radar),
            "imu" | "inertial-measurement-unit" => Ok(Self::Imu),
            "encoder" => Ok(Self::Encoder),
            "force-torque" | "ft" => Ok(Self::ForceTorque),
            "proximity" => Ok(Self::Proximity),
            "ultrasonic" => Ok(Self::Ultrasonic),
            "gps" | "gnss" => Ok(Self::Gps),
            "microphone" | "mic" => Ok(Self::Microphone),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// A non-empty descriptive sensor mount label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SensorMount(String);

impl SensorMount {
    /// Creates a sensor mount label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`RobotSensorTextError::Empty`] when the trimmed mount label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, RobotSensorTextError> {
        non_empty_sensor_text(value).map(Self)
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

impl AsRef<str> for SensorMount {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SensorMount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SensorMount {
    type Err = RobotSensorTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Descriptive sensor reading kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SensorReadingKind {
    /// Image reading label.
    Image,
    /// Point cloud reading label.
    PointCloud,
    /// Distance reading label.
    Distance,
    /// Acceleration reading label.
    Acceleration,
    /// Angular velocity reading label.
    AngularVelocity,
    /// Position reading label.
    Position,
    /// Force reading label.
    Force,
    /// Torque reading label.
    Torque,
    /// Contact reading label.
    Contact,
    /// Unknown reading kind.
    Unknown,
    /// Caller-defined reading kind text.
    Custom(String),
}

impl fmt::Display for SensorReadingKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Image => "image",
            Self::PointCloud => "point-cloud",
            Self::Distance => "distance",
            Self::Acceleration => "acceleration",
            Self::AngularVelocity => "angular-velocity",
            Self::Position => "position",
            Self::Force => "force",
            Self::Torque => "torque",
            Self::Contact => "contact",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for SensorReadingKind {
    type Err = SensorReadingKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SensorReadingKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "image" => Ok(Self::Image),
            "point-cloud" => Ok(Self::PointCloud),
            "distance" => Ok(Self::Distance),
            "acceleration" => Ok(Self::Acceleration),
            "angular-velocity" => Ok(Self::AngularVelocity),
            "position" => Ok(Self::Position),
            "force" => Ok(Self::Force),
            "torque" => Ok(Self::Torque),
            "contact" => Ok(Self::Contact),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while constructing robot sensor text values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotSensorTextError {
    /// The value was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotSensorTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot sensor text cannot be empty"),
        }
    }
}

impl Error for RobotSensorTextError {}

/// Error returned when parsing sensor kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotSensorKindParseError {
    /// The sensor kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for RobotSensorKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("robot sensor kind cannot be empty"),
        }
    }
}

impl Error for RobotSensorKindParseError {}

/// Error returned when parsing reading kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensorReadingKindParseError {
    /// The reading kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for SensorReadingKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("sensor reading kind cannot be empty"),
        }
    }
}

impl Error for SensorReadingKindParseError {}

fn non_empty_sensor_text(value: impl AsRef<str>) -> Result<String, RobotSensorTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(RobotSensorTextError::Empty)
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
        RobotSensorKind, RobotSensorKindParseError, RobotSensorName, RobotSensorTextError,
        SensorReadingKind, SensorReadingKindParseError,
    };

    #[test]
    fn constructs_valid_sensor_name() -> Result<(), RobotSensorTextError> {
        let name = RobotSensorName::new("  wrist-camera  ")?;

        assert_eq!(name.as_str(), "wrist-camera");
        Ok(())
    }

    #[test]
    fn rejects_empty_sensor_name() {
        assert_eq!(RobotSensorName::new(""), Err(RobotSensorTextError::Empty));
    }

    #[test]
    fn displays_and_parses_sensor_kind() -> Result<(), RobotSensorKindParseError> {
        assert_eq!(
            "force torque".parse::<RobotSensorKind>()?,
            RobotSensorKind::ForceTorque
        );
        assert_eq!(RobotSensorKind::Camera.to_string(), "camera");
        Ok(())
    }

    #[test]
    fn displays_and_parses_reading_kind() -> Result<(), SensorReadingKindParseError> {
        assert_eq!(
            "point cloud".parse::<SensorReadingKind>()?,
            SensorReadingKind::PointCloud
        );
        assert_eq!(
            SensorReadingKind::AngularVelocity.to_string(),
            "angular-velocity"
        );
        Ok(())
    }

    #[test]
    fn stores_custom_sensor_kind() -> Result<(), RobotSensorKindParseError> {
        assert_eq!(
            "event-camera".parse::<RobotSensorKind>()?,
            RobotSensorKind::Custom("event-camera".to_string())
        );
        Ok(())
    }
}
