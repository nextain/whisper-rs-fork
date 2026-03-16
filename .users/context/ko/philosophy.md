<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 철학 — whisper-rs-nx

whisper-rs-nx 개발을 이끄는 핵심 원칙.

**영문 미러**: `.users/context/philosophy.md`
**AI 컨텍스트 (SoT)**: `.agents/context/philosophy.yaml`

## 원칙

### AI 네이티브 개발

AI 지원 기여는 인간 기여와 함께 환영합니다. AI 도구는 투명한 어트리뷰션과 함께 개발 워크플로우의 일부입니다. 풍부한 `.agents/` 컨텍스트가 높은 품질의 AI 기여를 보장합니다. AI 컨텍스트 파일은 일급 프로젝트 인프라입니다.

### 프라이버시 우선

기본적으로 로컬 실행 — 클라우드는 선택 사항입니다. whisper.cpp는 클라우드 API 없이 로컬에서 실행됩니다. cuda-dynamic을 통해 필수 CUDA 설치 없이 GPU 가속이 가능합니다. CPU 폴백으로 라이브러리가 어디서나 작동합니다.

### 투명성

오픈 소스 — 코드를 읽어서 검증하세요. 소스 코드는 Apache 2.0입니다. AI 컨텍스트는 개방적이고 포크 가능합니다 (CC-BY-SA 4.0). 업스트림 어트리뷰션 체인이 보존됩니다 (Unlicense → Apache 2.0).

### 조립 vs 발명

검증된 컴포넌트로 구성 — 재발명하지 않습니다. whisper.cpp는 검증된 STT 엔진이고, 우리는 Rust 바인딩을 제공합니다. bindgen + cmake로 빌드 시스템 구성 — 표준 Rust FFI 툴체인. 업스트림 whisper.cpp 릴리스를 따르며 불필요하게 분기하지 않습니다.

### 생태계 통합

Naia OS 생태계의 일부 — AI 컴패니언을 위한 음성-텍스트 변환. 더 큰 Rust 애플리케이션에 임베딩하도록 설계되었습니다. cuda-dynamic을 통한 단일 바이너리 배포로 런타임 CUDA SDK 의존성을 피합니다.

### 인프라로서의 컨텍스트

구조화된 AI 컨텍스트 파일이 기여 품질을 향상시킵니다. `.agents/` 디렉토리는 프로젝트 컨벤션과 아키텍처를 인코딩합니다. 컨텍스트 품질이 AI 협업 품질을 결정합니다. 트리플 미러 아키텍처: AI 최적화 + 영문 + 한국어. CC-BY-SA 라이선스가 기여 체인을 보존합니다.
