# Syscaller

[![Crates.io](https://img.shields.io/crates/v/syscaller.svg)](https://crates.io/crates/syscaller)
[![Documentation](https://docs.rs/syscaller/badge.svg)](https://docs.rs/syscaller)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE.md)

A no-std Rust library providing direct system call interfaces for Linux x86_64 systems with hand-optimized assembly implementations and procedural macros for type-safe syscall wrapper generation.

## Features

- **Direct Assembly**: Hand-written x86_64 syscall implementations (0-6 arguments)
- **Type Safety**: Procedural macros for generating safe wrappers from C signatures
- **No-std Compatible**: Minimal dependencies for embedded/kernel development
- **Zero Dependencies**: Core library has no external dependencies

## Target Audience

- Kernel developers requiring direct syscall access
- Systems programmers bypassing libc overhead  
- Security researchers analyzing syscall behavior

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
syscaller = { version = "0.1", features = ["macro"] }
```

### Direct Assembly Usage

```rust
use syscaller::*;

unsafe {
    // Direct syscall with raw numbers and arguments
    let result = syscall1(1, b"Hello, World!\n".as_ptr() as usize); // write syscall
}
```

### Procedural Macro Usage

```rust
use syscaller::wrap_syscall;

wrap_syscall! {
    1 : ssize_t write(int fd, void *buf, size_t count),
    57 : int fork(),
    59 : int execve(const char *path, char *const *argv, char *const *envp),
    319 : int memfd_create(const char *name, unsigned int flags),
}

// Now you can use type-safe wrappers
unsafe {
    let bytes_written = write(1, b"Hello from syscaller!\n".as_ptr() as *const _, 22);
    let pid = fork();
}
```

## Architecture

### Core Library (`syscaller`)

The core library provides hand-written x86_64 assembly implementations:

- `syscall0` through `syscall6` - Support 0-6 arguments
- Follows System V ABI calling conventions
- `#![no_std]` compatible with zero dependencies
- Optimized for minimal overhead

### Procedural Macro (`syscaller-wrap-macro`)

The macro crate provides type-safe wrapper generation:

- Parses C-like function signatures
- Generates Rust wrapper functions with proper types
- Handles type conversions (pointers, integers, etc.)
- Comprehensive error handling for malformed signatures

## Safety

⚠️ **This library provides direct access to system calls without safety guarantees.**

- All functions are marked `unsafe` as they can cause undefined behavior
- Caller must ensure syscall numbers and arguments are valid
- Improper usage can crash your program or system
- Intended for advanced systems programming where direct control is needed

## Platform Support

Currently supports:
- **Linux x86_64** - Full support with hand-optimized assembly

Planned support:
- Linux ARM64
- Linux x86

## Examples

See the [`syscaller-wrap-macro`](syscaller-wrap-macro/) directory for detailed usage examples.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.