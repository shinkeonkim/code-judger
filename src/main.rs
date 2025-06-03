use tonic::{transport::Server, Request, Response, Status};
use anyhow::Result;
use std::io::Write;

pub mod code_judger {
    tonic::include_proto!("code_judger");
}
pub mod code_executor {
    tonic::include_proto!("code_executor");
}

use code_judger::code_judger_server::{CodeJudger, CodeJudgerServer};
use code_judger::{JudgeRequest, JudgeResponse};
use code_executor::code_executor_client::CodeExecutorClient;
use code_executor::{ExecuteRequest, ExecuteResponse};

#[derive(Debug, Default)]
pub struct CodeJudgerService;

#[tonic::async_trait]
impl CodeJudger for CodeJudgerService {
    async fn judge_code(
        &self,
        request: Request<JudgeRequest>,
    ) -> Result<Response<JudgeResponse>, Status> {
        let req = request.into_inner();
        // Call CodeExecutor gRPC
        let mut client = CodeExecutorClient::connect("http://code-executor:50051")
            .await
            .map_err(|e| Status::internal(format!("executor connect: {e}")))?;
        let exec_req = ExecuteRequest {
            code: req.code.clone(),
            language: req.language.clone(),
            version: req.version.clone(),
            timeout_seconds: req.timeout_seconds,
            memory_limit_mb: req.memory_limit_mb,
            input: req.input.clone(),
        };
        let exec_resp = client.execute_code(Request::new(exec_req)).await
            .map_err(|e| Status::internal(format!("executor error: {e}")))?;
        let exec_resp = exec_resp.into_inner();
        // [DEBUG] Log CodeExecutor response
        println!("==== [DEBUG] CodeExecutor 응답 ====");
        println!("status: {:?}", exec_resp.status);
        println!("stdout: {:?}", exec_resp.stdout);
        println!("stderr: {:?}", exec_resp.stderr);
        println!("error_message: {:?}", exec_resp.error_message);
        println!("memory_used_kb: {:?}", exec_resp.memory_used_kb);
        println!("execution_time_ms: {:?}", exec_resp.execution_time_ms);
        println!("===============================");
        std::io::stdout().flush().unwrap();

        // Compare output
        let actual_lines: Vec<_> = exec_resp.stdout.lines().map(|l| l.trim_end()).collect();
        let expected_lines: Vec<_> = req.expected_output.iter().map(|l| l.trim_end()).collect();
        let correct = actual_lines == expected_lines;

        use code_judger::JudgeStatus;
        use code_executor::ExecutionStatus;
        let status = match exec_resp.status {
            x if x == ExecutionStatus::Timeout as i32 => JudgeStatus::JudgeTimeout as i32,
            x if x == ExecutionStatus::MemoryLimitExceeded as i32 => JudgeStatus::JudgeMemory as i32,
            x if x == ExecutionStatus::Failed as i32 || x == ExecutionStatus::RuntimeError as i32 => JudgeStatus::JudgeError as i32,
            x if x == ExecutionStatus::Completed as i32 => {
                if correct {
                    JudgeStatus::JudgeCorrect as i32
                } else {
                    JudgeStatus::JudgeWrong as i32
                }
            }
            _ => JudgeStatus::JudgeUnknown as i32,
        };

        Ok(Response::new(JudgeResponse {
            correct,
            actual_output: exec_resp.stdout.clone(),
            expected_output: req.expected_output.join("\n"),
            stdout: exec_resp.stdout,
            stderr: exec_resp.stderr,
            execution_time_ms: exec_resp.execution_time_ms,
            memory_used_kb: exec_resp.memory_used_kb,
            error_message: exec_resp.error_message,
            status,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50052".parse()?;
    let service = CodeJudgerService::default();
    println!("CodeJudger server listening on {}", addr);
    Server::builder()
        .add_service(CodeJudgerServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
