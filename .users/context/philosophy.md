<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Philosophy — whisper-rs-nx

Core principles guiding whisper-rs-nx development.

**Korean mirror**: `.users/context/ko/philosophy.md`
**AI context (SoT)**: `.agents/context/philosophy.yaml`

## Principles

### AI-Native Development

AI-assisted contributions are welcome alongside human contributions. AI tools are part of the development workflow, with transparent attribution. Rich `.agents/` context ensures high-quality AI contributions. AI context files are first-class project infrastructure.

### Privacy First

Local execution by default — cloud is opt-in. whisper.cpp runs locally with no cloud API needed. cuda-dynamic allows GPU acceleration without mandatory CUDA install. CPU fallback ensures the library works everywhere.

### Transparency

Open source — verify by reading the code. Source code is Apache 2.0. AI context is open and forkable (CC-BY-SA 4.0). Upstream attribution chain preserved (Unlicense → Apache 2.0).

### Assembly over Invention

Compose from proven components — don't reinvent. whisper.cpp is the proven STT engine; we provide Rust bindings. bindgen + cmake for build system — standard Rust FFI toolchain. Follow upstream whisper.cpp releases, don't diverge unnecessarily.

### Ecosystem Integration

Part of the Naia OS ecosystem — speech-to-text for AI companions. Designed for embedding in larger Rust applications. Single-binary deployment via cuda-dynamic avoids CUDA SDK dependency at runtime.

### Context as Infrastructure

Structured AI context files improve contribution quality. `.agents/` directories encode project conventions and architecture. Context quality determines AI collaboration quality. Triple-mirror architecture: AI-optimized + English + Korean. CC-BY-SA licensing preserves the contribution chain.
