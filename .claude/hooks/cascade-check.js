/**
 * cascade-check.js — Triple-mirror + generated file guard for whisper-rs-nx
 *
 * Trigger: PostToolUse (Edit|Write)
 * Purpose:
 *   1. Remind to update all three mirror layers when context files change
 *   2. WARN when editing generated files (sys/src/bindings.rs)
 */

const input = JSON.parse(await new Promise((r) => {
	let d = "";
	process.stdin.on("data", (c) => (d += c));
	process.stdin.on("end", () => r(d));
}));

const toolName = input.tool_name;
if (toolName !== "Edit" && toolName !== "Write") {
	process.exit(0);
}

const filePath = (input.tool_input?.file_path || "").replace(/\\/g, "/");

// Guard: sys/src/bindings.rs is GENERATED — never edit manually
if (filePath.includes("sys/src/bindings.rs")) {
	process.stdout.write(
		"[BLOCKED] sys/src/bindings.rs is a GENERATED file. NEVER edit manually. " +
		"Platform differences must be solved in build.rs (blocklist, post-processing) " +
		"or in src/ wrapper code (per-enum cfg_attr). See .agents/context/upstream.yaml pre_generated_bindings.must_not_edit."
	);
	process.exit(0);
}

const isAgentsContext = filePath.includes(".agents/context/");
const isUsersContext = filePath.includes(".users/context/") && !filePath.includes("/ko/");
const isKoContext = filePath.includes(".users/context/ko/");

if (!isAgentsContext && !isUsersContext && !isKoContext) {
	process.exit(0);
}

// Extract the base filename
const basename = filePath.split("/").pop().replace(/\.(yaml|json|md)$/, "");

let msg = `[Harness] You edited ${filePath}. Triple-mirror rule: also update `;
const updates = [];

if (isAgentsContext) {
	updates.push(`.users/context/${basename}.md`);
	updates.push(`.users/context/ko/${basename}.md`);
} else if (isUsersContext) {
	updates.push(`.agents/context/${basename}.yaml (or .json)`);
	updates.push(`.users/context/ko/${basename}.md`);
} else if (isKoContext) {
	updates.push(`.agents/context/${basename}.yaml (or .json)`);
	updates.push(`.users/context/${basename}.md`);
}

msg += updates.join(" and ") + " if they exist.";
process.stdout.write(msg);
