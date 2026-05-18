# use-robotics

Facade crate for the RustUse robotics vocabulary set.

This crate re-exports the focused child crates behind explicit module names. It is documentation and composition glue only; implementation lives in the focused crates.

`use-robotics` is not a robotics framework, ROS replacement, simulator, hardware abstraction layer, motion planner, control system, kinematics solver, navigation stack, SLAM system, robot runtime, firmware framework, embedded HAL, hardware driver, sensor processing engine, or async runtime.

## Modules

- `robot` for robot names, kinds, models, and manufacturers.
- `robot_id` for robot IDs, serial numbers, and instance IDs.
- `joint` for joint names, kinds, limits, axes, and indexes.
- `actuator` for actuator names, kinds, states, and descriptive ratings.
- `robot_sensor` for robot sensor names, kinds, mounts, and reading labels.
- `pose` for pose names, pose labels, and orientation-kind labels.
- `frame` for robotics frame names, kinds, references, and parent/child relations.
- `end_effector` for end-effector names, kinds, grip states, and tool mounts.
- `kinematics_label` for kinematics terminology, chain names, link names, and degrees of freedom.
- `robot_subsystem` for robot subsystem names, kinds, and lifecycle states.

## Example

```rust
use use_robotics::{
    actuator, end_effector, frame, joint, kinematics_label, pose, robot, robot_sensor,
    robot_subsystem,
};

let robot_name = robot::RobotName::new("inspection-arm")?;
let robot_kind = robot::RobotKind::Arm;
let joint_name = joint::JointName::new("shoulder-pan")?;
let actuator_name = actuator::ActuatorName::new("shoulder-servo")?;
let sensor_name = robot_sensor::RobotSensorName::new("wrist-camera")?;
let frame_ref = frame::FrameRef::new(frame::FrameName::new("tool0")?, frame::FrameKind::Tool);
let pose_label = pose::Pose3Label::new("pick-ready")?;
let end_effector = end_effector::EndEffectorKind::Gripper;
let kinematics = kinematics_label::KinematicsKind::Forward;
let subsystem_state = robot_subsystem::SubsystemState::Ready;

assert_eq!(robot_name.as_str(), "inspection-arm");
assert_eq!(robot_kind.to_string(), "arm");
assert_eq!(joint_name.as_str(), "shoulder-pan");
assert_eq!(actuator_name.as_str(), "shoulder-servo");
assert_eq!(sensor_name.as_str(), "wrist-camera");
assert_eq!(frame_ref.name().as_str(), "tool0");
assert_eq!(pose_label.as_str(), "pick-ready");
assert_eq!(end_effector.to_string(), "gripper");
assert_eq!(kinematics.to_string(), "forward");
assert_eq!(subsystem_state.to_string(), "ready");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The example composes robotics vocabulary. It does not move, control, simulate, plan, connect to, or operate a robot.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
