# use-joint

Primitive robot joint vocabulary for RustUse.

This crate provides descriptive joint names, kinds, limits, axes, and indexes. It does not solve kinematics, enforce robot-specific constraints, control motors, or simulate motion.

## Example

```rust
use use_joint::{JointAxis, JointIndex, JointKind, JointLimit, JointName};

let name = JointName::new("shoulder-pan")?;
let kind = "revolute".parse::<JointKind>()?;
let axis = "z".parse::<JointAxis>()?;
let limit = JointLimit::new(Some(-1.57), Some(1.57))?;
let index = JointIndex::new(0);

assert_eq!(name.as_str(), "shoulder-pan");
assert_eq!(kind.to_string(), "revolute");
assert_eq!(axis.to_string(), "z");
assert_eq!(limit.maximum(), Some(1.57));
assert_eq!(index.get(), 0);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under either the MIT license or Apache License, Version 2.0.
