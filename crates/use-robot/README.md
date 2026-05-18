# use-robot

Primitive robot vocabulary for RustUse.

This crate provides descriptive robot names, kinds, models, and manufacturers. It does not create robot registries, connect to robots, control robots, or implement robot configuration systems.

## Example

```rust
use use_robot::{RobotKind, RobotModel, RobotName};

let name = RobotName::new("inspection-arm")?;
let model = RobotModel::new("RX-4")?;
let kind = "collaborative".parse::<RobotKind>()?;

assert_eq!(name.as_str(), "inspection-arm");
assert_eq!(model.as_str(), "RX-4");
assert_eq!(kind.to_string(), "collaborative");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under either the MIT license or Apache License, Version 2.0.
