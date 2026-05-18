# use-robotics

`use-robotics` is the RustUse set for small, composable robotics vocabulary primitives.

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

This workspace describes robot-domain concepts: robots, joints, actuators, sensors, poses, frames, end effectors, kinematics labels, and robot subsystems. It does not move, control, simulate, plan, connect to, or operate real robots.

## Scope

`use-robotics` owns descriptive robotics vocabulary.

- robots and robot identity
- joints and descriptive joint labels
- actuators as vocabulary, not motor control
- robot sensors as vocabulary, not sensor payload processing
- pose names and pose labels, not transforms or interpolation
- robotics frame names and parent/child labels, not transform trees
- end-effector names, kinds, mount labels, and grip states
- kinematics terminology labels, not solvers
- robot subsystem names, kinds, and lifecycle states

## Non-goals

`use-robotics` is not a robotics framework, ROS replacement, simulator, hardware abstraction layer, firmware framework, motion planner, control system, kinematics solver, navigation stack, SLAM system, robot runtime, command-line app, async runtime, path planner, hardware driver, embedded HAL, sensor processing engine, or distributed robot middleware.

## Boundaries

- `use-physics` owns physical concepts such as force, torque, motion, momentum, rotation, and rigid bodies.
- `use-geometry` owns geometric concepts such as points, vectors, rays, planes, polygons, solids, and transforms when present.
- `use-control` owns control primitives.
- `use-electronics` owns electronic components, pins, boards, and circuits.

`use-robotics` complements `use-physics`, `use-geometry`, `use-control`, `use-electronics`, `use-signal`, `use-data`, and `use-validate` by staying focused on robot-domain naming and descriptive labels.

## Crates

| Crate                  | Purpose                                                                  |
| ---------------------- | ------------------------------------------------------------------------ |
| `use-robotics`         | Thin facade over the focused robotics vocabulary crates.                 |
| `use-robot`            | Robot names, kinds, models, and manufacturers.                           |
| `use-robot-id`         | Robot IDs, serial numbers, and instance IDs.                             |
| `use-joint`            | Joint names, kinds, limits, axes, and indexes.                           |
| `use-actuator`         | Actuator names, kinds, states, and descriptive ratings.                  |
| `use-robot-sensor`     | Robot sensor names, kinds, mounts, and reading-kind labels.              |
| `use-pose`             | Pose names, pose labels, and orientation-kind labels.                    |
| `use-frame`            | Robotics frame names, kinds, references, and parent/child relations.     |
| `use-end-effector`     | End-effector names, kinds, grip states, and tool mounts.                 |
| `use-kinematics-label` | Kinematics terminology, chain names, link names, and degrees of freedom. |
| `use-robot-subsystem`  | Robot subsystem names, kinds, and lifecycle states.                      |

## Example

```rust
use use_robotics::{
    actuator, end_effector, frame, joint, kinematics_label, pose, robot, robot_sensor,
    robot_subsystem,
};

let robot_name = robot::RobotName::new("inspection-arm")?;
let robot_kind = robot::RobotKind::Arm;
let joint_name = joint::JointName::new("shoulder-pan")?;
let joint_kind = joint::JointKind::Revolute;
let actuator_name = actuator::ActuatorName::new("shoulder-servo")?;
let sensor_name = robot_sensor::RobotSensorName::new("wrist-camera")?;
let frame_name = frame::FrameName::new("tool0")?;
let pose_label = pose::Pose3Label::new("pick-ready")?;
let end_effector = end_effector::EndEffectorKind::Gripper;
let kinematics = kinematics_label::KinematicsKind::Forward;
let subsystem_state = robot_subsystem::SubsystemState::Ready;

assert_eq!(robot_name.as_str(), "inspection-arm");
assert_eq!(robot_kind.to_string(), "arm");
assert_eq!(joint_name.as_str(), "shoulder-pan");
assert_eq!(joint_kind.to_string(), "revolute");
assert_eq!(actuator_name.as_str(), "shoulder-servo");
assert_eq!(sensor_name.as_str(), "wrist-camera");
assert_eq!(frame_name.as_str(), "tool0");
assert_eq!(pose_label.as_str(), "pick-ready");
assert_eq!(end_effector.to_string(), "gripper");
assert_eq!(kinematics.to_string(), "forward");
assert_eq!(subsystem_state.to_string(), "ready");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The example composes labels and vocabulary that downstream crates can store, compare, serialize, validate, or map into their own systems. It does not move, control, simulate, plan, connect to, or operate anything.

## Development

Run the standard workspace checks from this directory:

```sh
cargo fmt
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either the MIT license or Apache License, Version 2.0.
