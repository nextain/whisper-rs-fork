/**
 * upstream-style-check.js — Harness hook for upstream whisper-rs style compliance
 *
 * Trigger: PreToolUse (Bash with "git commit")
 * Purpose: Before committing, run cargo fmt/clippy and block if non-compliant.
 *
 * All code changes MUST follow upstream whisper-rs style,
 * NOT naia-os style or any other project's conventions.
 */

import { execSync } from "node:child_process";

const input = JSON.parse(await new Promise((r) => {
	let d = "";
	process.stdin.on("data", (c) => (d += c));
	process.stdin.on("end", () => r(d));
}));

const toolName = input.tool_name;
const toolInput = input.tool_input || {};
const command = toolInput.command || "";

if (toolName !== "Bash" || !command.includes("git commit")) {
	process.exit(0);
}

// Check for staged Rust files
let stagedFiles = "";
try {
	stagedFiles = execSync("git diff --cached --name-only", { encoding: "utf8" });
} catch {
	process.exit(0);
}

const rustFiles = stagedFiles.split("\n").filter(
	(f) => f.endsWith(".rs") || f === "Cargo.toml" || f === "sys/Cargo.toml"
);

if (rustFiles.length === 0) {
	process.exit(0);
}

// Run cargo fmt --check
let fmtOk = true;
try {
	execSync("cargo fmt --check", { encoding: "utf8", stdio: "pipe" });
} catch {
	fmtOk = false;
}

// Run cargo clippy
let clippyOk = true;
try {
	execSync("cargo clippy --all-targets -- -D warnings 2>&1", { encoding: "utf8", stdio: "pipe" });
} catch {
	clippyOk = false;
}

if (!fmtOk || !clippyOk) {
	const result = {
		decision: "block",
		reason: `[Upstream Style Check] BLOCKED — ${!fmtOk ? "cargo fmt --check failed" : ""}${!fmtOk && !clippyOk ? " + " : ""}${!clippyOk ? "cargo clippy failed" : ""}. Fix before committing.`
	};
	process.stdout.write(JSON.stringify(result));
	process.exit(0);
}

// Mechanical checks passed — output semantic review reminder
const checklist = `
[Upstream Style Check] cargo fmt + clippy PASSED. Staged: ${rustFiles.join(", ")}

Semantic review reminder (see .agents/context/upstream.yaml):
  - Platform cfg uses upstream patterns exactly?
  - Comments minimal, neutral tone?
  - Error handling: Result not panic?
  - No naia-os conventions leaked in Rust code?
  - Would upstream maintainer accept this style?
`;

process.stdout.write(checklist);
