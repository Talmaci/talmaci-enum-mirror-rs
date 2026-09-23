# talmaci-enum-mirror

Derive bidirectional `From` conversions between enums with matching unit variants.

`EnumMirror` is primarily intended for architectural boundaries where two layers
need equivalent enums but should not share the same type.

A common example is hexagonal or clean architecture: a persistence adapter may
have a database-specific enum with SQLx derives, while the domain or application
layer exposes its own independent enum.

Without a mapping helper, keeping those types separate usually introduces
repetitive `From` implementations containing identical `match` expressions.

`EnumMirror` generates those conversions at compile time.

The generated conversions require no allocations and introduce no runtime
dependencies.

## Installation

```toml
[dependencies]
talmaci-enum-mirror = "0.1"
```

## Example

```rust
// Domain/application crate: no persistence dependencies.
mod application {
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Active,
        Disabled,
    }
}

// Persistence crate, which depends on the application crate.
mod persistence {
    use talmaci_enum_mirror::EnumMirror;

    #[derive(EnumMirror)]
    #[enum_mirror(crate::application::Status)]
    pub enum PgStatus {
        Active,
        Disabled,
    }
}

# fn main() {
let status: application::Status = persistence::PgStatus::Active.into();
assert_eq!(status, application::Status::Active);
let pg_status: persistence::PgStatus = status.into();
# }
```

The modules above stand in for separate crates.

With separate crates, the persistence type can reference the application/domain
enum directly:

```rust,ignore
#[derive(EnumMirror)]
#[enum_mirror(application::Status)]
pub enum PgStatus {
    Active,
    Disabled,
}
```

The application crate needs neither `talmaci-enum-mirror` nor SQLx. The mapping
implementation is generated entirely from the persistence side.

## Why?


Consider a persistence adapter using a PostgreSQL enum.

The persistence representation may need database-specific derives and metadata:

```rust,ignore
#[derive(sqlx::Type)]
#[sqlx(type_name = "types.status", rename_all = "snake_case")]
pub enum PgStatus {
    Active,
    Disabled,
}
```

The domain or application layer should not need to depend on SQLx simply because
the database represents the same concept.

Keeping a separate type preserves the dependency boundary:

```text
PostgreSQL / SQLx
       ↓
  PgStatus
       ↓
persistence adapter
       ↓
   From / Into
       ↓
application::Status
```

Normally this requires maintaining boilerplate such as:

```rust,ignore
impl From<PgStatus> for Status {
    fn from(value: PgStatus) -> Self {
        match value {
            PgStatus::Active => Self::Active,
            PgStatus::Disabled => Self::Disabled,
        }
    }
}

impl From<Status> for PgStatus {
    fn from(value: Status) -> Self {
        match value {
            Status::Active => Self::Active,
            Status::Disabled => Self::Disabled,
        }
    }
}
```

`EnumMirror` generates both implementations from the enum definitions instead.

## SQLx example


`EnumMirror` has no dependency on SQLx and does not implement SQLx traits.

You can combine it with `sqlx::Type` on the persistence enum:

```rust,ignore
use talmaci_enum_mirror::EnumMirror;

#[derive(sqlx::Type, EnumMirror)]
#[sqlx(type_name = "types.status", rename_all = "snake_case")]
#[enum_mirror(application::Status)]
pub enum PgStatus {
    Active,
    Disabled,
}
```

Decode database values as the persistence type and convert them at the adapter
boundary:

```rust,ignore
let pg_status: PgStatus = row.try_get("status")?;
let status: application::Status = pg_status.into();
```

`EnumMirror` only generates Rust conversions. It does not change database
encoding, decoding, or schema behavior.

## Constraints

- Both enums must contain matching variant names.
- Only unit variants are supported.
- Declaration order may differ.
- Explicit discriminant values may differ.
- Conversions consume the value; `Copy` and `Clone` are not required.
- Derive `EnumMirror` on only one enum in each pair. Deriving it on both produces
  conflicting implementations.
- Generic source enums are not supported.
- Variant renaming is not supported.
- Fallible mappings are not supported.
- Missing target variants produce compile errors.
- Extra target variants produce compile errors because the generated reverse
  conversion must exhaustively match the target enum.
- An external `#[non_exhaustive]` target enum cannot be exhaustively mirrored.

## Development

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
