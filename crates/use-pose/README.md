# use-pose

Primitive robot pose vocabulary for `RustUse` robotics.

This crate provides pose names, pose-kind labels, 2D and 3D pose labels, and orientation-kind labels. It does not solve transforms, implement coordinate geometry, plan motion, interpolate poses, or duplicate `use-geometry`.

## Example

```rust
use use_pose::{OrientationKind, Pose3Label, PoseKind, PoseName};

let name = PoseName::new("home")?;
let kind = "waypoint".parse::<PoseKind>()?;
let label = Pose3Label::new("pick-ready")?;
let orientation = "quaternion".parse::<OrientationKind>()?;

assert_eq!(name.as_str(), "home");
assert_eq!(kind.to_string(), "waypoint");
assert_eq!(label.as_str(), "pick-ready");
assert_eq!(orientation.to_string(), "quaternion");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The labels describe robot-domain concepts only. They are not transforms or vector/matrix math.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
