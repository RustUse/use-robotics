# use-actuator

Primitive actuator vocabulary for RustUse robotics.

This crate describes actuator names, kinds, states, and ratings. It does not command actuators, implement motor control, implement PID control, or provide hardware drivers.

## Example

```rust
use use_actuator::{ActuatorKind, ActuatorName, ActuatorRating, ActuatorState};

let name = ActuatorName::new("shoulder-servo")?;
let kind = "servo".parse::<ActuatorKind>()?;
let state = "enabled".parse::<ActuatorState>()?;
let rating = ActuatorRating::new("rated torque")?.with_value(12.0)?.with_unit("N m")?;

assert_eq!(name.as_str(), "shoulder-servo");
assert_eq!(kind.to_string(), "servo");
assert_eq!(state.to_string(), "enabled");
assert_eq!(rating.label(), "rated torque");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The rating is descriptive metadata only. It does not imply motor control behavior.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
