<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Harness Engineering — whisper-rs-nx

Mechanical enforcement of project rules via hooks and progress files.

**Korean mirror**: `.users/context/ko/harness.md`
**AI context (SoT)**: `.agents/context/harness.yaml`

## Concept

"Engineer the environment so the AI never repeats a mistake."

Text rules get forgotten; mechanical enforcement doesn't.

## Limitations

Harness catches **style violations only** (formatting, linting, file guards, naming). It CANNOT catch:
- Design quality (wrong abstraction, poor generality)
- Performance regressions
- Semantic correctness (logic errors)
- Scope creep (too many changes in one commit)

Do NOT trust harness passage as proof of upstream quality. Phase 3 (critical review) exists specifically to catch what harness cannot.

## Pillars

1. **Hooks**: Claude Code hooks that intercept edits and commands in real-time
2. **Progress files**: JSON session handoff files that survive context compaction and session boundaries

## Hooks

Located in `.claude/hooks/`, registered in `.claude/settings.json`.

### Trigger Timing Principle

- **PreToolUse** = runs BEFORE tool execution. Can block (`decision: 'block'`).
- **PostToolUse** = runs AFTER tool execution. Advisory only — cannot undo.

Design rule: enforce with PreToolUse, remind with PostToolUse. A PostToolUse hook that tries to block is ineffective.

### Hook Authoring Rule

When writing hooks, NEVER guess Claude Code API format. Read existing working hooks first (e.g., naia-os `.claude/hooks/`). Common bugs from guessing: ESM vs CJS imports, stdin reading pattern, output format schema, settings.json structure.

### upstream-style-check.js

- **Trigger**: PreToolUse on Bash (git commit)
- **Purpose**: Block commits that fail cargo fmt or cargo clippy
- **Behavior**: Runs cargo fmt --check + cargo clippy, blocks on failure, shows semantic review reminder on pass

### cascade-check.js

- **Trigger**: PostToolUse on Edit|Write
- **Purpose**: Remind agent about triple-mirror updates + guard generated files
- **Behavior**: Advisory reminders when `.agents/` or `.users/` files are modified. Warns when `sys/src/bindings.rs` is edited (generated file).

### sync-entry-points.js

- **Trigger**: PostToolUse on Edit|Write
- **Purpose**: Auto-sync CLAUDE.md, AGENTS.md, GEMINI.md
- **Behavior**: When any entry point file is edited, copies content to the others (if they exist)

## Progress Files

- **Location**: `.agents/progress/` (gitignored)
- **Format**: JSON
- **Purpose**: Session handoff — survives context compaction and session boundaries

### Schema

```json
{
  "issue": "#5",
  "title": "Short description",
  "project": "whisper-rs-nx",
  "current_phase": "build",
  "decisions": [{ "decision": "...", "rationale": "...", "date": "..." }],
  "surprises": [],
  "blockers": [],
  "updated_at": "2026-03-15T14:30Z"
}
```
