# `ephemeral-dir`

## Overview

The `EphemeralDir` struct allows you to create temporary directory that is automatically cleaned up (deleted) when the `EphemeralDir` instance goes out of scope. This is useful for tests or temporary file operations.

## Why not `tempfile`?

[`tempfile`](https://github.com/Stebalien/tempfile), should be preferred as it is more complete. The reason for `ephemeral_dir` to exist is that I wanted to have temporary directory with a fixed name (without any random bytes in the name).

## Usage

Use `EphemeralDir::new(<path>)` to create an ephemeral dir -- this would panic if the directory at `<path>` already exists.

Use `EphemeralDir::new_forced(<path>)` to create an ephemeral dir, deleting the existing directory at `<path>`.

```rust
use ephemeral_dir::EphemeralDir;
use std::path::Path;

fn main() -> eyre::Result<()> {
    let temp_path = Path::new("/tmp/my_temp_dir");
    let ep_dir = EphemeralDir::new(temp_path)?;
    // Directory "/tmp/my_temp_dir" now exists.

    // Do something with ep_dir.path()

    // When ep_dir goes out of scope, "/tmp/my_temp_dir" will be deleted.
    Ok(())
}
```
