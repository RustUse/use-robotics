# use-robot-subsystem

Primitive robot subsystem vocabulary for `RustUse` robotics.

This crate describes robot subsystem names, subsystem kinds, and lifecycle/status labels. It does not orchestrate subsystems, implement runtime supervision, implement health monitoring frameworks, implement robot middleware, or model distributed systems behavior.

## Example

```rust
use use_robot_subsystem::{RobotSubsystemKind, RobotSubsystemName, SubsystemState};

let name = RobotSubsystemName::new("arm")?;
let kind = "manipulation".parse::<RobotSubsystemKind>()?;
let state = "ready".parse::<SubsystemState>()?;

assert_eq!(name.as_str(), "arm");
assert_eq!(kind.to_string(), "manipulation");
assert_eq!(state.to_string(), "ready");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The state is vocabulary only. It does not start, stop, supervise, or monitor anything.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
