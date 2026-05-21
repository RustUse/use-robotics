#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for `RustUse` robotics vocabulary primitive crates.

pub use use_actuator as actuator;
pub use use_end_effector as end_effector;
pub use use_frame as frame;
pub use use_joint as joint;
pub use use_kinematics_label as kinematics_label;
pub use use_pose as pose;
pub use use_robot as robot;
pub use use_robot_id as robot_id;
pub use use_robot_sensor as robot_sensor;
pub use use_robot_subsystem as robot_subsystem;

/// Common robotics vocabulary types from the focused crates.
pub mod prelude {
    pub use crate::actuator::{ActuatorKind, ActuatorName, ActuatorRating, ActuatorState};
    pub use crate::end_effector::{EndEffectorKind, EndEffectorName, GripState, ToolMount};
    pub use crate::frame::{FrameKind, FrameName, FrameRef, FrameRelation};
    pub use crate::joint::{JointAxis, JointIndex, JointKind, JointLimit, JointName};
    pub use crate::kinematics_label::{
        DegreeOfFreedom, KinematicChainName, KinematicsKind, LinkName,
    };
    pub use crate::pose::{OrientationKind, Pose2Label, Pose3Label, PoseKind, PoseName};
    pub use crate::robot::{RobotKind, RobotManufacturer, RobotModel, RobotName};
    pub use crate::robot_id::{RobotId, RobotInstanceId, RobotSerialNumber};
    pub use crate::robot_sensor::{
        RobotSensorKind, RobotSensorName, SensorMount, SensorReadingKind,
    };
    pub use crate::robot_subsystem::{RobotSubsystemKind, RobotSubsystemName, SubsystemState};
}

#[cfg(test)]
mod tests {
    use super::{
        actuator, end_effector, frame, joint, kinematics_label, pose, robot, robot_id,
        robot_sensor, robot_subsystem,
    };

    #[test]
    fn facade_exposes_composable_robotics_primitives() -> Result<(), Box<dyn std::error::Error>> {
        let robot_name = robot::RobotName::new("inspection-arm")?;
        let serial = robot_id::RobotSerialNumber::new("SN-42A")?;
        let joint_name = joint::JointName::new("shoulder-pan")?;
        let limit = joint::JointLimit::new(Some(-1.57), Some(1.57))?;
        let actuator_name = actuator::ActuatorName::new("shoulder-servo")?;
        let sensor_name = robot_sensor::RobotSensorName::new("wrist-camera")?;
        let frame_ref =
            frame::FrameRef::new(frame::FrameName::new("tool0")?, frame::FrameKind::Tool);
        let pose_label = pose::Pose3Label::new("pick-ready")?;
        let end_effector = end_effector::EndEffectorKind::Gripper;
        let dof = kinematics_label::DegreeOfFreedom::new(6)?;
        let subsystem_state = robot_subsystem::SubsystemState::Ready;

        assert_eq!(robot_name.as_str(), "inspection-arm");
        assert_eq!(serial.as_str(), "SN-42A");
        assert_eq!(joint_name.as_str(), "shoulder-pan");
        assert_eq!(limit.minimum(), Some(-1.57));
        assert_eq!(actuator_name.as_str(), "shoulder-servo");
        assert_eq!(sensor_name.as_str(), "wrist-camera");
        assert_eq!(frame_ref.kind(), &frame::FrameKind::Tool);
        assert_eq!(pose_label.as_str(), "pick-ready");
        assert_eq!(end_effector.to_string(), "gripper");
        assert_eq!(dof.get(), 6);
        assert_eq!(subsystem_state.to_string(), "ready");
        Ok(())
    }
}
