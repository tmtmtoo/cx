mod child_process;
mod process;

use crate::prelude::*;
use child_process::*;
use process::*;

#[derive(new, Debug, Clone, PartialEq, Getters)]
pub struct Exit {
    code: i32,

    #[new(default)]
    #[getter(skip)]
    stdout_wrote_length: Option<u64>,

    #[new(default)]
    #[getter(skip)]
    stderr_wrote_length: Option<u64>,
}

trait AsyncReadUnpin: tokio::io::AsyncRead + Unpin + Send {}

trait AsyncWriteUnpin: tokio::io::AsyncWrite + Unpin + Send {}

trait ChildProcess: std::future::Future<Output = Result<i32>> + Unpin + Send {
    fn stdout(&mut self) -> Result<Box<dyn AsyncReadUnpin>>;
    fn stderr(&mut self) -> Result<Box<dyn AsyncReadUnpin>>;
}

trait SpawnChild {
    fn spawn(&self, command: &str) -> Result<Box<dyn ChildProcess + Send>>;
}

trait Process {
    fn stdout(&self) -> Box<dyn AsyncWriteUnpin>;
    fn stderr(&self) -> Box<dyn AsyncWriteUnpin>;
}

#[async_trait]
pub trait PipedCmdExecutor: Send + Sync {
    async fn piped_exec(&self, command: &str) -> Result<Exit>;
}

pub struct TokioPipedCmdExecutor {
    process: Box<dyn Process + Send + Sync>,
    cmd_executor: Box<dyn SpawnChild + Send + Sync>,
}

impl TokioPipedCmdExecutor {
    pub fn new() -> Self {
        Self {
            process: Box::new(TokioProcess),
            cmd_executor: Box::new(TokioCmdExecutor),
        }
    }
}

#[async_trait]
impl PipedCmdExecutor for TokioPipedCmdExecutor {
    async fn piped_exec(&self, command: &str) -> Result<Exit> {
        let mut child = self.cmd_executor.spawn(command)?;

        let mut child_stdout = child.stdout()?;
        let mut process_stdout = self.process.stdout();
        let handle_stdout = tokio::io::copy(&mut child_stdout, &mut process_stdout);

        let mut child_stderr = child.stderr()?;
        let mut process_stderr = self.process.stderr();
        let handle_stderr = tokio::io::copy(&mut child_stderr, &mut process_stderr);

        let (code, stdout_wrote_length, stderr_wrote_length) =
            tokio::join!(child, handle_stdout, handle_stderr);

        Ok(Exit {
            code: code?,
            stdout_wrote_length: stdout_wrote_length.ok(),
            stderr_wrote_length: stderr_wrote_length.ok(),
        })
    }
}

#[cfg(test)]
pub struct StubPipedCmdExecutor {
    pub output: Box<dyn Fn() -> Result<Exit> + Send + Sync>,
}

#[async_trait]
#[cfg(test)]
impl PipedCmdExecutor for StubPipedCmdExecutor {
    async fn piped_exec(&self, _: &str) -> Result<Exit> {
        (*self.output)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn expect_stdout_3byte_stderr_4byte() {
        let executor = TokioPipedCmdExecutor {
            process: Box::new(StubProcess {
                stdout: Vec::new(),
                stderr: Vec::new(),
            }),
            cmd_executor: Box::new(StubCmdExecutor {
                child_stdout: vec![0x00, 0x01, 0x02],
                child_stderr: vec![0x03, 0x04, 0x05, 0x06],
            }),
        };

        let actual = executor.piped_exec("dummy").await.unwrap();
        let expected = Exit {
            code: 0,
            stdout_wrote_length: Some(3),
            stderr_wrote_length: Some(4),
        };

        assert_eq!(actual, expected);
    }
}
