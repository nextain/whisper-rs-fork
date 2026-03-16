<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 아키텍처 — whisper-rs-nx

whisper.cpp를 위한 Rust 바인딩 — cuda-dynamic과 Windows MSVC 지원이 포함된 Nextain 포크.

**영문 미러**: `.users/context/architecture.md`
**AI 컨텍스트 (SoT)**: `.agents/context/architecture.yaml`

## 프로젝트 정체성

- **이름**: whisper-rs-nx
- **성격**: whisper.cpp를 위한 Rust FFI 바인딩
- **업스트림**: [codeberg.org/tazz4843/whisper-rs](https://codeberg.org/tazz4843/whisper-rs) (Unlicense)
- **포크 이유**: 업스트림에서 AI 지원 기여를 받지 않음
- **생태계**: [Naia OS](https://github.com/nextain/naia-os)의 일부 — AI 컴패니언을 위한 음성-텍스트 변환 제공

## 버전 전략

- **현재**: 0.15.1 (upstream 0.15.0 시점에서 포크)
- **upstream 최신**: 0.16.0
- **전략**: upstream 버전을 추적하고 포크 전용 변경에는 패치 버전 증가. 다음 upstream 동기화 시 0.16.x로 업데이트 후 포크 패치 추가 (예: 0.16.1-nx).

## 크레이트 구조

```
whisper-rs-nx/              # 고수준 Rust API (crate: whisper-rs-nx)
├── src/                    # WhisperContext, FullParams 등
├── sys/                    # FFI 바인딩 (crate: whisper-rs-nx-sys)
│   ├── build.rs            # cmake + bindgen 빌드 스크립트
│   ├── src/bindings.rs     # 사전 생성된 바인딩 (Linux, 폴백 전용)
│   └── whisper.cpp/        # git 서브모듈 — 업스트림 whisper.cpp
├── examples/               # 사용 예제
├── .agents/                # AI 컨텍스트 (YAML, CC-BY-SA-4.0)
└── .users/                 # 사람이 읽을 수 있는 미러 (Markdown, CC-BY-SA-4.0)
```

## 빌드 시스템

**cmake + bindgen + cargo** 사용:

1. cmake가 whisper.cpp C/C++ 코드를 컴파일
2. bindgen이 whisper.h에서 Rust FFI 바인딩을 생성
3. cargo가 컴파일된 정적 라이브러리를 링크

### 피처

| 피처 | 설명 |
|------|------|
| default | CPU 전용 빌드 |
| cuda-dynamic | 런타임 CUDA 로딩 (빌드 시 CUDA SDK 불필요) |

### 요구사항

| 플랫폼 | 요구사항 |
|---------|---------|
| 전체 | cmake, C++ 컴파일러 |
| Linux | libclang-dev |
| Windows | LLVM (`winget install LLVM.LLVM`), VS Build Tools 2022 |

## 업스트림 대비 주요 개선사항

1. **cuda-dynamic** (`83911ba`): ggml의 dlopen 인프라를 통한 런타임 CUDA 백엔드 로딩. CPU 전용으로 빌드하고, 가능하면 런타임에 GPU 로드.
2. **Windows MSVC** (`ab97b92`): cuda-dynamic용 Windows DLL 검색 패턴, 플랫폼 안전 바인딩을 위한 bindgen 0.72 + rust_target.
3. **CPU 폴백**: GPU 사용 불가 시 자동 CPU 추론.

## 라이선스

- **소스 코드**: Apache 2.0
- **AI 컨텍스트** (`.agents/`, `.users/`, `CLAUDE.md`): CC-BY-SA 4.0
- **원본 업스트림 코드**: Unlicense (퍼블릭 도메인)
