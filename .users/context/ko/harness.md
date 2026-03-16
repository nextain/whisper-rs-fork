<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 하네스 엔지니어링 — whisper-rs-nx

훅과 진행 파일을 통한 프로젝트 규칙의 기계적 강제.

**영문 미러**: `.users/context/harness.md`
**AI 컨텍스트 (SoT)**: `.agents/context/harness.yaml`

## 개념

"환경을 엔지니어링하여 AI가 같은 실수를 반복하지 않게 한다."

텍스트 규칙은 잊히지만, 기계적 강제는 그렇지 않습니다.

## 한계

하네스는 **스타일 위반만** 잡습니다 (포맷팅, 린팅, 파일 가드, 네이밍). 잡을 수 없는 것:
- 설계 품질 (잘못된 추상화, 부족한 범용성)
- 성능 회귀
- 의미론적 정확성 (로직 에러)
- 범위 초과 (한 커밋에 너무 많은 변경)

하네스 통과를 upstream 품질의 증거로 신뢰하지 마세요. Phase 3 (비판적 리뷰)이 하네스가 잡지 못하는 것을 잡기 위해 존재합니다.

## 핵심 요소

1. **훅**: 실시간으로 편집과 명령을 가로채는 Claude Code 훅
2. **진행 파일**: 컨텍스트 압축과 세션 경계를 넘어 유지되는 JSON 세션 핸드오프 파일

## 훅

`.claude/hooks/`에 위치, `.claude/settings.json`에 등록.

### 트리거 타이밍 원칙

- **PreToolUse** = 도구 실행 전 발동. 차단 가능 (`decision: 'block'`).
- **PostToolUse** = 도구 실행 후 발동. 알림만 가능 — 되돌릴 수 없음.

설계 규칙: 강제는 PreToolUse, 알림은 PostToolUse. PostToolUse로 차단하려는 것은 무효.

### 훅 작성 규칙

훅 작성 시 Claude Code API 형식을 절대 추측하지 말 것. 기존 동작하는 훅(예: naia-os `.claude/hooks/`)을 먼저 읽는다. 추측으로 발생하는 흔한 버그: ESM vs CJS import, stdin 읽기 패턴, 출력 형식 스키마, settings.json 구조.

### upstream-style-check.js

- **트리거**: Bash(git commit)에 대한 PreToolUse
- **목적**: cargo fmt 또는 cargo clippy 실패 시 커밋 차단
- **동작**: cargo fmt --check + cargo clippy 실행, 실패 시 차단, 통과 시 의미론적 리뷰 알림

### cascade-check.js

- **트리거**: Edit|Write에 대한 PostToolUse
- **목적**: 트리플 미러 업데이트 알림 + 생성된 파일 가드
- **동작**: `.agents/` 또는 `.users/` 파일 수정 시 알림. `sys/src/bindings.rs` 편집 시 경고 (생성된 파일).

### sync-entry-points.js

- **트리거**: Edit|Write에 대한 PostToolUse
- **목적**: CLAUDE.md, AGENTS.md, GEMINI.md 자동 동기화
- **동작**: 엔트리 포인트 파일이 편집되면 다른 파일로 내용 복사 (존재하는 경우)

## 진행 파일

- **위치**: `.agents/progress/` (gitignored)
- **형식**: JSON
- **목적**: 세션 핸드오프 — 컨텍스트 압축과 세션 경계를 넘어 유지

### 스키마

```json
{
  "issue": "#5",
  "title": "간단한 설명",
  "project": "whisper-rs-nx",
  "current_phase": "build",
  "decisions": [{ "decision": "...", "rationale": "...", "date": "..." }],
  "surprises": [],
  "blockers": [],
  "updated_at": "2026-03-15T14:30Z"
}
```
