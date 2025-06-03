mod code_judger {
    tonic::include_proto!("code_judger");
}

use tonic::Request;
use code_judger::code_judger_client::CodeJudgerClient;
use code_judger::JudgeRequest;
use code_judger::JudgeStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut client = CodeJudgerClient::connect("http://code-judger:50052").await?;

    // 1. Test simple Python code (정답 케이스)
    let request = Request::new(JudgeRequest {
        code: r#"
print("Hello, World!")
for i in range(3):
    print(f"Number: {i}")
"#.to_string(),
        language: "python".to_string(),
        version: "3.12".to_string(),
        timeout_seconds: 10,
        memory_limit_mb: 128,
        input: vec![],
        expected_output: vec![
            "Hello, World!".to_string(),
            "Number: 0".to_string(),
            "Number: 1".to_string(),
            "Number: 2".to_string(),
        ],
    });

    let response = client.judge_code(request).await?;
    println!("\n[CASE 1] 정답 케이스");
    print_pretty_response(&response, "original_response");

    // 2. 오답 케이스
    let request = Request::new(JudgeRequest {
        code: r#"
print("Hello, World!")
for i in range(2):
    print(f"Number: {i}")
"#.to_string(),
        language: "python".to_string(),
        version: "3.12".to_string(),
        timeout_seconds: 10,
        memory_limit_mb: 128,
        input: vec![],
        expected_output: vec![
            "Hello, World!".to_string(),
            "Number: 0".to_string(),
            "Number: 1".to_string(),
            "Number: 2".to_string(),
        ],
    });

    let response = client.judge_code(request).await?;
    println!("\n[CASE 2] 오답 케이스");
    print_pretty_response(&response, "original_response");

    // 3. stdin 입력 케이스
    let request = Request::new(JudgeRequest {
        code: r#"
a = input()
b = input()
print(f"A: {a}, B: {b}")
"#.to_string(),
        language: "python".to_string(),
        version: "3.12".to_string(),
        timeout_seconds: 10,
        memory_limit_mb: 128,
        input: vec!["hello".to_string(), "world".to_string()],
        expected_output: vec!["A: hello, B: world".to_string()],
    });

    let response = client.judge_code(request).await?;
    println!("\n[CASE 3] stdin 입력 케이스");
    print_pretty_response(&response, "original_response");

    // 4. Timeout 케이스
    let request = Request::new(JudgeRequest {
        code: r#"
import time
time.sleep(15)
print('done')
"#.to_string(),
        language: "python".to_string(),
        version: "3.12".to_string(),
        timeout_seconds: 2, // 짧은 시간 제한
        memory_limit_mb: 128,
        input: vec![],
        expected_output: vec!["done".to_string()],
    });
    let response = client.judge_code(request).await?;
    println!("\n[CASE 4] Timeout 케이스");
    print_pretty_response(&response, "original_response");
    let status = response.get_ref().status;
    if status == JudgeStatus::JudgeTimeout as i32 {
        println!("[Timeout 감지됨]");
    }

    // 5. Memory Limit Exceeded 케이스
    let request = Request::new(JudgeRequest {
        code: r#"
a = []
while True:
    a.append(' ' * 10**6)  # 계속 메모리 할당
"#.to_string(),
        language: "python".to_string(),
        version: "3.12".to_string(),
        timeout_seconds: 10,
        memory_limit_mb: 16, // 매우 작은 메모리 제한
        input: vec![],
        expected_output: vec![],
    });
    let response = client.judge_code(request).await?;
    println!("\n[CASE 5] Memory Limit Exceeded 케이스");
    print_pretty_response(&response, "original_response");
    let status = response.get_ref().status;
    if status == JudgeStatus::JudgeMemory as i32 {
        println!("[Memory Limit Exceeded 감지됨]");
    }

    Ok(())
}

fn print_pretty_response(response: &tonic::Response<code_judger::JudgeResponse>, label: &str) {
    println!("{}:", label);
    println!("{:?}", response);

    let message = response.get_ref();
    println!("\nPretty Print:");
    println!("==================");
    println!("Correct: {}", message.correct);
    println!("Actual Output: {}", message.actual_output);
    println!("Expected Output: {}", message.expected_output);
    println!("Stdout:\n{}", message.stdout);
    println!("Stderr:\n{}", message.stderr);
    println!("Execution time: {:.2}ms", message.execution_time_ms);
    println!("Memory used: {} KB", message.memory_used_kb);
    println!("Error message: {}", message.error_message);
    // Print JudgeStatus as string
    let status_str = match message.status {
        x if x == JudgeStatus::JudgeUnknown as i32 => "JUDGE_UNKNOWN",
        x if x == JudgeStatus::JudgeCorrect as i32 => "JUDGE_CORRECT",
        x if x == JudgeStatus::JudgeWrong as i32 => "JUDGE_WRONG",
        x if x == JudgeStatus::JudgeTimeout as i32 => "JUDGE_TIMEOUT",
        x if x == JudgeStatus::JudgeMemory as i32 => "JUDGE_MEMORY",
        x if x == JudgeStatus::JudgeError as i32 => "JUDGE_ERROR",
        _ => "UNKNOWN",
    };
    println!("JudgeStatus: {}", status_str);
} 