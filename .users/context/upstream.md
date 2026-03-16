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
No strict import ordering enforced — varies across files. Follow rustfmt default grouping.

### Error Handling
- Library code: `Result<T, WhisperError>` — never panic
- build.rs: `panic!()` or `.expect("reason")` for unrecoverable errors
- build.rs anti-pattern: `.ok()?` or silent Option returns — never swallow errors

### Anti-patterns (signals "AI-generated code" to upstream maintainer)

Detected in PR #275 and #272 rejections:

- **Decorative comment separators** (`// ─── Section ───`) — upstream uses zero
- **Capitalized emphasis** (`// NOT`, `// MUST`, `// CRITICAL`) — upstream uses neutral tone
- **Hardcoded version-specific paths** (`/usr/lib/llvm-18`, `llvm-17`) — snapshot of one machine
- **Manual editing of generated files** (`sys/src/bindings.rs`) — regeneration wipes all edits
- **Two approaches to the same problem** (cmake crate + raw `Command::new("cmake")`) — unclear intent
- **Over-documentation** — upstream density is minimal
- **Shipping unverified code** (`// NOTE: untested`) — upstream expects tested contributions
- **Re-solving upstream's solutions** (`has_libclang()` when `WHISPER_DONT_GENERATE_BINDINGS` exists)

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

**Anti-pattern**: NEVER use `#[cfg(target_os = "linux")]` in src/ or sys/src/. Use `#[cfg(unix)]` — it covers Linux, macOS, BSD. `target_os = "linux"` excludes macOS/BSD and breaks those platforms. Exception: build.rs may use `cfg!(target_os)` for link flags where the distinction matters.

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
- **NEVER manually edit** `sys/src/bindings.rs`. Platform differences must be solved in build.rs or src/ wrapper code.

### CMake
- Static build (`BUILD_SHARED_LIBS=OFF`)
- `WHISPER_*`, `GGML_*`, `CMAKE_*` env vars forwarded
- Always links: whisper, ggml, ggml-base, ggml-cpu
- Windows: `/utf-8` cxxflag + advapi32
- **Consistency rule**: ALL cmake invocations through cmake crate. No raw `Command::new("cmake")`.

### Features
- Default: none
- GPU: cuda, metal, vulkan, hipblas, coreml, intel-sycl, openblas
- Utility: raw-api, log_backend, tracing_backend, openmp, force-debug, test-with-tiny-model

## Testing

- `test-with-tiny-model` feature enables integration tests with real whisper.cpp model
- Every new feature or platform fix MUST include tests
- No shipping untested code — `// NOTE: untested` is not acceptable (cited in PR #275 rejection)

## Re-solving Anti-pattern

Do NOT add new mechanisms for problems upstream already solved. Example: upstream provides `WHISPER_DONT_GENERATE_BINDINGS` for skipping bindgen. Adding `has_libclang()` auto-detection re-solves the same problem with a more fragile approach. Always check upstream.yaml build_system section first.

## Windows Support (upstream)

- **Status**: Partial, fragile
- **Prerequisites**: Visual Studio C++, cmake, LLVM/clang
- **Known issues**: Linux-only bindings, HipBLAS panics, no Windows CI

## Git Conventions

- No strict commit message convention (mixed styles)
- Merge commits with PR number: `(#273)`
