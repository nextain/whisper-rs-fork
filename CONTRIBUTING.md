# Contributing to whisper-rs-nx

Thank you for considering a contribution.

## Code Style

All code must follow upstream whisper-rs conventions:

- Run `cargo fmt` (default rustfmt config)
- Run `cargo clippy --all-targets -- -D warnings`
- Minimal comments — only where logic isn't self-evident
- Use existing platform cfg patterns (see `.agents/context/upstream.yaml`)

## Process

1. Open or pick a GitHub Issue
2. Fork and create a branch: `issue-{N}-{description}`
3. Write code following upstream style
4. Run `cargo fmt` and `cargo clippy`
5. Submit a PR referencing the issue

## AI-Assisted Contributions

AI-assisted contributions are welcome. If you used AI tools, note it in the PR with an `Assisted-by: <tool>` line. This is encouraged for transparency, not a requirement.

## Context Contributions

Changes to `.agents/` or `.users/` context files must follow the triple-mirror rule:
- `.agents/context/*.yaml` (AI-optimized)
- `.users/context/*.md` (English)
- `.users/context/ko/*.md` (Korean)

Context contributions (`.agents/`, `.users/`) are licensed CC-BY-SA 4.0.
Source code is licensed Apache 2.0.
