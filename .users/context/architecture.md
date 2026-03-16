<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Architecture — whisper-rs-nx

Rust bindings for whisper.cpp — Nextain fork with cuda-dynamic and Windows MSVC support.

**Korean mirror**: `.users/context/ko/architecture.md`
**AI context (SoT)**: `.agents/context/architecture.yaml`

## Project Identity

- **Name**: whisper-rs-nx
- **Nature**: Rust FFI bindings for whisper.cpp
- **Upstream**: [codeberg.org/tazz4843/whisper-rs](https://codeberg.org/tazz4843/whisper-rs) (Unlicense)
- **Fork reason**: Upstream does not accept AI-assisted contributions
- **Ecosystem**: Part of [Naia OS](https://github.com/nextain/naia-os) — provides speech-to-text for the AI companion

## Versioning

- **Current**: 0.15.1 (fork diverged at upstream 0.15.0)
- **Upstream latest**: 0.16.0
- **Strategy**: Track upstream versions — bump patch for fork-only changes. Next upstream sync should update to 0.16.x, then add fork patch (e.g., 0.16.1-nx).

## Crate Structure

```
whisper-rs-nx/              # High-level Rust API (crate: whisper-rs-nx)
├── src/                    # WhisperContext, FullParams, etc.
├── sys/                    # FFI bindings (crate: whisper-rs-nx-sys)
│   ├── build.rs            # cmake + bindgen build script
│   ├── src/bindings.rs     # Pre-generated bindings (Linux, fallback only)
│   └── whisper.cpp/        # git submodule — upstream whisper.cpp
├── examples/               # Usage examples
├── .agents/                # AI context (YAML, CC-BY-SA-4.0)
└── .users/                 # Human-readable mirrors (Markdown, CC-BY-SA-4.0)
```

## Build System

The build uses **cmake + bindgen + cargo**:

1. cmake compiles whisper.cpp C/C++ code
2. bindgen generates Rust FFI bindings from whisper.h
3. cargo links the compiled static library

### Features

| Feature | Description |
|---------|-------------|
| default | CPU-only build |
| cuda-dynamic | Runtime CUDA loading (no CUDA SDK needed at build time) |

### Requirements

| Platform | Requirements |
|----------|-------------|
| All | cmake, C++ compiler |
| Linux | libclang-dev |
| Windows | LLVM (`winget install LLVM.LLVM`), VS Build Tools 2022 |

## Key Improvements over Upstream

1. **cuda-dynamic** (`83911ba`): Runtime CUDA backend loading via ggml's dlopen infrastructure. Build CPU-only, load GPU at runtime if available.
2. **Windows MSVC** (`ab97b92`): Windows DLL search patterns for cuda-dynamic, bindgen 0.72 with rust_target for platform-safe bindings.
3. **CPU fallback**: GPU unavailable → automatic CPU inference.

## License

- **Source code**: Apache 2.0
- **AI context** (`.agents/`, `.users/`, `CLAUDE.md`): CC-BY-SA 4.0
- **Original upstream code**: Unlicense (public domain)
