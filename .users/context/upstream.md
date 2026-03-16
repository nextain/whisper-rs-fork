# Upstream whisper-rs Reference

> Mirror of `.agents/context/upstream.yaml`

Pure analysis of the upstream whisper-rs project. All code in this fork MUST conform to these conventions.

## Project Info

- **Repo**: https://codeberg.org/tazz4843/whisper-rs
- **License**: Unlicense
- **Maintainer**: Niko (@niko.lgbt)
- **Rust edition**: 2021, MSRV 1.88.0
- **Versions**: whisper-rs 0.16.0, whisper-rs-sys 0.15.0

## Contribution Rules

PR checklist (upstream enforced):
- Self-review performed
- Code is legible and maintainable by others
- Hard-to-understand areas commented
- Documentation updated where necessary
- `cargo fmt` and `cargo clippy` run

Process: Issue first → PR → Merge with PR number reference.

## Code Style

### Formatting
- **rustfmt** with default config (`.rustfmt.toml` is empty)
- **cargo clippy** clean
- Crate-level: `#![allow(clippy::uninlined_format_args)]`
- Sys crate: `#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]`

### Naming
- Types: `CamelCase` (WhisperContext, FullParams)
- Functions: `snake_case` (path_to_bytes, get_cpp_link_stdlib)
- Constants: `UPPER_SNAKE_CASE` (WHISPER_CPP_VERSION)
- Features: `kebab-case` (cuda-dynamic, force-debug)

### Comments
- Minimal — only where logic isn't self-evident
- Doc comments on public API items with C++ equivalent pattern
- Neutral technical tone

### Imports
No strict import ordering enforced — varies across files. Follow rustfmt default grouping. Do not impose stricter order than upstream.

### Error Handling
- Library code: `Result<T, WhisperError>` — never panic
- build.rs: `panic!()` acceptable for unrecoverable errors

## Platform Patterns

**Principle**: Minimal platform code in Rust. whisper.cpp handles most via CMake.

### cfg Patterns

Unix vs non-Unix:
```rust
#[cfg(unix)]
fn func() { /* unix-specific */ }
#[cfg(not(unix))]
fn func() { /* fallback */ }
```

MSVC enum repr:
```rust
#[cfg_attr(any(not(windows), target_env = "gnu"), repr(u32))]
#[cfg_attr(all(windows, not(target_env = "gnu")), repr(i32))]
pub enum CEnum { ... }
```

### Platform code locations (4 total)
- `src/whisper_ctx.rs:7-25` — path_to_bytes
- `src/common_logging.rs:51-54` — GGMLLogLevel repr
- `src/whisper_grammar.rs:8-9` — WhisperGrammarElementType repr
- `sys/build.rs:193-196` — /utf-8 + advapi32

## Build System

### Binding Generation
- `WHISPER_DONT_GENERATE_BINDINGS` env var → skip bindgen, use pre-generated
- Without env var → run bindgen → on error, fallback to pre-generated
- Pre-generated: `sys/src/bindings.rs` (5964 lines, Linux-only, has glibc types)
- **Cross-platform issue**: glibc types fail on Windows (size assertion overflow)

### CMake
- Static build (`BUILD_SHARED_LIBS=OFF`)
- `WHISPER_*`, `GGML_*`, `CMAKE_*` env vars forwarded
- Always links: whisper, ggml, ggml-base, ggml-cpu
- Windows: `/utf-8` cxxflag + advapi32

### Features
- Default: none
- GPU: cuda, metal, vulkan, hipblas, coreml, intel-sycl, openblas
- Utility: raw-api, log_backend, tracing_backend, openmp, force-debug, test-with-tiny-model

## Windows Support (upstream)

- **Status**: Partial, fragile
- **Prerequisites**: Visual Studio C++, cmake, LLVM/clang
- **Known issues**: Linux-only bindings, HipBLAS panics, no Windows CI

## Git Conventions

- No strict commit message convention (mixed styles)
- Merge commits with PR number: `(#273)`
