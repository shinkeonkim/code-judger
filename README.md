# Code Judger

Code Judger는 Code Executor를 활용하여 제출된 코드의 정답 여부를 자동으로 판정하는 gRPC 기반 채점 서비스입니다.

## 주요 기능
- 다양한 언어(Python, Ruby 등) 코드 채점 지원
- 입력(문자열 목록), 기대 출력(문자열 목록) 기반 채점
- Code Executor를 통해 실제 코드 실행 및 결과 수집
- 시간/메모리 제한, 표준 입력/출력, 에러 메시지 등 지원
- 출력 비교 정책(공백/개행 처리) 적용
- gRPC API 제공

## 요구 사항
- Docker, docker-compose
- Rust (빌드 및 개발)
- protoc, grpcio-tools (gRPC proto 코드 생성)
- Code Executor 서비스가 함께 실행되어야 함

## 빌드 및 실행 방법

### docker-compose로 실행

1. 다음 명령어로 실행합니다:

```sh
docker-compose up --build
```

- code-judger는 gRPC 서버를 `localhost:50052`에서 실행합니다.
- code-executor가 반드시 함께 실행되어야 정상 동작합니다.

2. 중지하려면:
```sh
docker-compose down
```

## gRPC API 요약
- proto 파일: `src/proto/judger.proto`
- 주요 서비스: `CodeJudger`
- 주요 메서드: `JudgeCode`

### JudgeCode 요청 예시
```protobuf
message JudgeRequest {
  string code = 1;
  string language = 2;
  string version = 3;
  int32 timeout_seconds = 4;
  int32 memory_limit_mb = 5;
  repeated string input = 6;
  repeated string expected_output = 7;
}
```

### JudgeCode 응답 예시
```protobuf
message JudgeResponse {
  JudgeStatus status = 1;
  bool correct = 2;
  repeated string actual_output = 3;
  string stdout = 4;
  string stderr = 5;
  string error_message = 6;
}
```

### JudgeStatus Enum
- JUDGE_PENDING
- JUDGE_RUNNING
- JUDGE_CORRECT
- JUDGE_WRONG
- JUDGE_TIMEOUT
- JUDGE_MEMORY
- JUDGE_ERROR

## 출력 비교 정책
- 출력 결과 비교 시 **맨 끝의 공백/개행 문자는 무시**
- **맨 앞의 공백/개행 문자는 고려**
- **중간의 공백/개행 문자는 모두 고려**

## 개발 참고 사항
- Code Executor와 gRPC로 연동하여 코드 실행 결과를 받아옴
- proto/gRPC 관련 코드는 `src/proto/`에 위치
- 테스트 및 예제 클라이언트는 `src/examples/test_client.rs` 참고
