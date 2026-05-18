# use-end-effector

Primitive end-effector vocabulary for RustUse robotics.

This crate describes end-effector names, kinds, grip states, and tool mounts. It does not command grippers, implement tool changing logic, implement force control, or simulate contact.

## Example

```rust
use use_end_effector::{EndEffectorKind, EndEffectorName, GripState, ToolMount};

let name = EndEffectorName::new("parallel-gripper")?;
let kind = "gripper".parse::<EndEffectorKind>()?;
let state = "open".parse::<GripState>()?;
let mount = ToolMount::new("tool0")?;

assert_eq!(name.as_str(), "parallel-gripper");
assert_eq!(kind.to_string(), "gripper");
assert_eq!(state.to_string(), "open");
assert_eq!(mount.as_str(), "tool0");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The values are vocabulary only. They do not control hardware.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
