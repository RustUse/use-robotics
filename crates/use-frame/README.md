# use-frame

Primitive coordinate frame vocabulary for `RustUse` robotics.

This crate provides frame names, frame kinds, frame references, and parent/child frame relations. It does not implement TF trees, transform buffers, coordinate conversion, or geometry math.

## Example

```rust
use use_frame::{FrameKind, FrameName, FrameRef, FrameRelation};

let parent = FrameRef::new(FrameName::new("base_link")?, FrameKind::Base);
let child = FrameRef::new(FrameName::new("tool0")?, FrameKind::Tool);
let relation = FrameRelation::new(parent, child);

assert_eq!(relation.parent().name().as_str(), "base_link");
assert_eq!(relation.child().kind(), &FrameKind::Tool);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The relation labels a parent and child. It does not compute transforms.

## License

Licensed under either the MIT license or Apache License, Version 2.0.
