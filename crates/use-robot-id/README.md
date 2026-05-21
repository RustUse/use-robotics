# use-robot-id

Primitive robot identity vocabulary for `RustUse`.

This crate stores robot IDs, serial numbers, and instance IDs as stable owned strings. It preserves caller casing and punctuation and rejects empty identifiers. It does not generate UUIDs, create inventory management systems, connect to asset systems, or fetch robot metadata.

## Example

```rust
use use_robot_id::{RobotId, RobotInstanceId, RobotSerialNumber};

let id = RobotId::new("robot:A-17")?;
let serial = RobotSerialNumber::new("SN-2026-A")?;
let instance = RobotInstanceId::new("cell-4/arm-1")?;

assert_eq!(id.as_str(), "robot:A-17");
assert_eq!(serial.to_string(), "SN-2026-A");
assert_eq!(instance.as_str(), "cell-4/arm-1");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under either the MIT license or Apache License, Version 2.0.
