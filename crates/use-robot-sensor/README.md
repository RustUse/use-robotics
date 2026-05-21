# use-robot-sensor

Primitive robot sensor vocabulary for `RustUse` robotics.

This crate describes robot sensor names, kinds, mounts, and reading-kind labels. It does not read sensors, process images, process point clouds, perform SLAM, or implement sensor fusion.

## Example

```rust
use use_robot_sensor::{RobotSensorKind, RobotSensorName, SensorMount, SensorReadingKind};

let name = RobotSensorName::new("wrist-camera")?;
let kind = "camera".parse::<RobotSensorKind>()?;
let mount = SensorMount::new("wrist")?;
let reading = "image".parse::<SensorReadingKind>()?;

assert_eq!(name.as_str(), "wrist-camera");
assert_eq!(kind.to_string(), "camera");
assert_eq!(mount.as_str(), "wrist");
assert_eq!(reading.to_string(), "image");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The reading kind is a label only. This crate does not model sensor payloads.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
