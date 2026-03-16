# Building whisper-rs-nx

This fork of [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) adds cuda-dynamic and Windows MSVC support.

## Prerequisites (all platforms)

- CMake
- C++ compiler
- Rust toolchain (MSRV 1.88.0)

## Linux (Arch)

```bash
sudo pacman -Syy llvm clang cmake
cargo build
```

## Linux (Ubuntu/Debian)

```bash
sudo apt install libclang-dev cmake build-essential
cargo build
```

## Windows using MSVC (recommended)

Make sure you have installed:

- Visual Studio C++ Build Tools
- CMake (add to PATH)
- LLVM/Clang (for bindgen — `winget install LLVM.LLVM`)

```bash
cargo build
```

> **Note**: Once platform-specific pre-generated bindings are available (planned),
> LLVM will no longer be required for default builds.

### With CUDA (static linking)

1. Install [CUDA Toolkit](https://developer.nvidia.com/cuda-downloads?target_os=Windows)
2. Set `CUDA_PATH` environment variable
3. `cargo build --features cuda`

### With cuda-dynamic (runtime GPU loading)

No CUDA SDK needed at build time. GPU is detected at runtime.

```bash
cargo build --features cuda-dynamic
```

Place `libggml-cuda.so` (Linux) or `ggml-cuda.dll` (Windows) next to the binary for GPU acceleration. Without them, CPU-only fallback is used automatically.

## Windows using MSYS2/MinGW

1. Install MSYS2 following [VS Code MinGW guide](https://code.visualstudio.com/docs/cpp/config-mingw)
2. Install toolchain: `pacman -S --needed base-devel mingw-w64-x86_64-toolchain`
3. Add `C:\msys64\ucrt64\bin` to PATH
4. Install make: `pacman -S make`
5. Set Rust target: `rustup toolchain install stable-x86_64-pc-windows-gnu`
6. Create `.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-gnu]
linker = "C:\\msys64\\ucrt64\\bin\\gcc.exe"
ar = "C:\\msys64\\ucrt64\\bin\\ar.exe"
```

7. `cargo build`

## macOS (Apple Silicon)

Add to your project's `.cargo/config.toml`:

```toml
[target.aarch64-apple-darwin]
rustflags = "-lc++ -l framework=Accelerate"
```

Install CMake:

```bash
brew install cmake
```

Then `cargo build`.

## Skipping bindgen (pre-generated bindings)

If LLVM/libclang is not available:

```bash
WHISPER_DONT_GENERATE_BINDINGS=1 cargo build
```

This uses bundled `sys/src/bindings.rs`. The bundled bindings include platform-specific gates for Linux and Windows.
