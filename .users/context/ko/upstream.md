# Upstream whisper-rs 레퍼런스

> `.agents/context/upstream.yaml` 미러

upstream whisper-rs 프로젝트 분석. 이 포크의 모든 코드는 이 컨벤션을 따라야 합니다.

## 프로젝트 정보

- **저장소**: https://codeberg.org/tazz4843/whisper-rs
- **라이선스**: Unlicense
- **메인테이너**: Niko (@niko.lgbt)
- **Rust 에디션**: 2021, MSRV 1.88.0
- **버전**: whisper-rs 0.16.0, whisper-rs-sys 0.15.0

## 기여 규칙

PR 체크리스트 (upstream 강제):
- 셀프 리뷰 완료
- 다른 사람이 읽고 유지보수 가능한 코드
- 이해하기 어려운 부분에 코멘트
- 필요한 문서 업데이트
- `cargo fmt` + `cargo clippy` 실행

프로세스: 이슈 먼저 → PR → PR 번호 참조하여 머지.

## 코드 스타일

### 포맷팅
- **rustfmt** 기본 설정 (`.rustfmt.toml` 비어있음)
- **cargo clippy** 클린
- 크레이트 레벨: `#![allow(clippy::uninlined_format_args)]`
- sys 크레이트: `#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]`

### 네이밍
- 타입: `CamelCase` (WhisperContext, FullParams)
- 함수: `snake_case` (path_to_bytes, get_cpp_link_stdlib)
- 상수: `UPPER_SNAKE_CASE` (WHISPER_CPP_VERSION)
- 피처: `kebab-case` (cuda-dynamic, force-debug)

### 코멘트
- 최소한 — 로직이 자명하지 않은 경우에만
- 공개 API에 doc comment, C++ 동등 패턴 포함
- 중립적 기술 용어

### Import 순서
엄격한 import 순서 규칙 없음 — 파일마다 다름. rustfmt 기본 그룹핑을 따른다.

### 에러 처리
- 라이브러리 코드: `Result<T, WhisperError>` — panic 금지
- build.rs: `panic!()` 또는 `.expect("이유")` 허용
- build.rs 안티패턴: `.ok()?` 등 조용한 에러 삼킴 금지

### 안티패턴 (upstream 메인테이너가 "AI 생성 코드"로 감지하는 패턴)

PR #275, #272 거절에서 발견:

- **장식적 코멘트 구분선** (`// ─── Section ───`) — upstream은 사용하지 않음
- **대문자 강조** (`// NOT`, `// MUST`, `// CRITICAL`) — upstream은 중립 톤
- **하드코딩된 버전별 경로** (`/usr/lib/llvm-18`, `llvm-17`) — 한 머신의 스냅샷
- **생성된 파일 수동 편집** (`sys/src/bindings.rs`) — 재생성 시 모든 편집 소실
- **같은 문제에 두 가지 접근** (cmake crate + raw `Command::new("cmake")`) — 의도 불명확
- **과도한 문서화** — upstream은 최소한
- **미검증 코드 출시** (`// NOTE: untested`) — upstream은 테스트된 기여를 기대
- **upstream이 해결한 문제 재해결** (`has_libclang()` — `WHISPER_DONT_GENERATE_BINDINGS`가 이미 있음)

## 플랫폼 패턴

**원칙**: Rust 래퍼의 플랫폼 코드는 최소화. whisper.cpp가 CMake로 대부분 처리.

### cfg 패턴

Unix vs non-Unix:
```rust
#[cfg(unix)]
fn func() { /* unix 전용 */ }
#[cfg(not(unix))]
fn func() { /* fallback */ }
```

MSVC enum repr:
```rust
#[cfg_attr(any(not(windows), target_env = "gnu"), repr(u32))]
#[cfg_attr(all(windows, not(target_env = "gnu")), repr(i32))]
pub enum CEnum { ... }
```

**안티패턴**: src/ 또는 sys/src/에서 `#[cfg(target_os = "linux")]` 절대 사용 금지. `#[cfg(unix)]` 사용 — Linux, macOS, BSD 포괄. `target_os = "linux"`은 macOS/BSD를 제외하여 해당 플랫폼을 깨뜨림. 예외: build.rs에서 Linux와 macOS 구분이 필요한 링크 플래그.

### 플랫폼 코드 위치 (4곳)
- `src/whisper_ctx.rs:7-25` — path_to_bytes
- `src/common_logging.rs:51-54` — GGMLLogLevel repr
- `src/whisper_grammar.rs:8-9` — WhisperGrammarElementType repr
- `sys/build.rs:193-196` — /utf-8 + advapi32

## 빌드 시스템

### 바인딩 생성
- `WHISPER_DONT_GENERATE_BINDINGS` 환경변수 → bindgen 건너뛰고 사전 생성 바인딩 사용
- 환경변수 없으면 → bindgen 실행 → 에러 시 사전 생성 바인딩으로 fallback
- 사전 생성: `sys/src/bindings.rs` (5964줄, Linux 전용, glibc 타입 포함)
- **크로스 플랫폼 이슈**: glibc 타입이 Windows에서 실패 (사이즈 assertion overflow)
- **`sys/src/bindings.rs` 절대 수동 편집 금지**. 플랫폼 차이는 build.rs 또는 src/ 래퍼 코드에서 해결.

### CMake
- 정적 빌드 (`BUILD_SHARED_LIBS=OFF`)
- `WHISPER_*`, `GGML_*`, `CMAKE_*` 환경변수 전달
- 항상 링크: whisper, ggml, ggml-base, ggml-cpu
- Windows: `/utf-8` cxxflag + advapi32
- **일관성 규칙**: 모든 cmake 호출은 cmake crate를 통해서. raw `Command::new("cmake")` 금지.

### 피처
- 기본: 없음
- GPU: cuda, metal, vulkan, hipblas, coreml, intel-sycl, openblas
- 유틸리티: raw-api, log_backend, tracing_backend, openmp, force-debug, test-with-tiny-model

## 테스트

- `test-with-tiny-model` feature로 실제 whisper.cpp 모델을 사용한 통합 테스트
- 모든 새 기능/플랫폼 수정에 테스트 필수
- 미검증 코드 출시 금지 — `// NOTE: untested`는 허용 불가 (PR #275 거절에서 지적됨)

## 재해결 안티패턴

upstream이 이미 해결한 문제에 새 메커니즘을 추가하지 마라. 예: upstream이 bindgen 건너뛰기용 `WHISPER_DONT_GENERATE_BINDINGS`를 제공. `has_libclang()` 자동 감지 추가는 같은 문제를 더 취약한 방식으로 재해결.

## Windows 지원 (upstream)

- **상태**: 부분적, 취약
- **필수 도구**: Visual Studio C++, cmake, LLVM/clang
- **알려진 이슈**: Linux 전용 바인딩, HipBLAS 패닉, Windows CI 없음

## Git 컨벤션

- 엄격한 커밋 메시지 컨벤션 없음 (혼합 스타일)
- 머지 커밋에 PR 번호: `(#273)`
