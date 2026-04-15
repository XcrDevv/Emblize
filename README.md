# Emblize

A compact binary format for embedded systems, with `no_std` support.

> [!NOTE]
> Emblize is a personal project under active development. The format and API may change between versions.

## Features

- `no_std` by default — works without a global allocator
- `heapless` support for fixed-capacity buffers
- `alloc` / `std` features for dynamic containers
- Serde integration via `#[derive(Serialize, Deserialize)]`
- Built-in types for embedded use: `Vec2`, `Vec3`, `Vec4`, `Quat`, timestamps, durations

## Usage

```toml
[dependencies]
emblize = "0.1"

# with std support by default
emblize = { version = "0.1" }
```
### Serialize / Deserialize

```rust
use emblize::serde::{from_bytes, to_heapless};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Sensor {
    id: u8,
    temperature: f32,
    active: bool,
}

let sensor = Sensor { id: 1, temperature: 23.5, active: true };

let bytes = to_heaplessvec(&sensor).unwrap();
let decoded: Sensor = from_bytes(&bytes).unwrap();

assert_eq!(sensor, decoded);
```

### Dynamic builder (requires `alloc`)

```rust
use emblize::dynamic::StructBuilder;

let token = StructBuilder::new_root()
    .u8("id", 1)
    .f32("temperature", 23.5)
    .bool("active", true)
    .map("other", |b| { // nested struct
        b.vec3("data", &[1.0, 2.0, 3.0])
    })
    .build();
```

## `no_std` setup

```toml
[dependencies]
emblize = { version = "0.1", default-features = false }
```

## License

MIT