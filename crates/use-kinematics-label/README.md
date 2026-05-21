# use-kinematics-label

Primitive kinematics terminology labels for `RustUse` robotics.

This crate provides descriptive kinematics kinds, kinematic chain names, link names, and non-zero degrees of freedom. It does not implement forward kinematics, inverse kinematics, Jacobians, solvers, or robot dynamics.

## Example

```rust
use use_kinematics_label::{DegreeOfFreedom, KinematicChainName, KinematicsKind, LinkName};

let kind = "forward".parse::<KinematicsKind>()?;
let chain = KinematicChainName::new("arm-chain")?;
let link = LinkName::new("wrist-link")?;
let dof = DegreeOfFreedom::new(6)?;

assert_eq!(kind.to_string(), "forward");
assert_eq!(chain.as_str(), "arm-chain");
assert_eq!(link.as_str(), "wrist-link");
assert_eq!(dof.get(), 6);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The values are labels only. They do not solve kinematics.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
