use crate::prelude::*;

pub struct TokioChildProcess(tokio::process::Child);

impl super::AsyncReadUnpin for tokio::process::ChildStdout {}

impl super::AsyncReadUnpin for tokio::process::ChildStderr {}

impl Future for TokioChildProcess {
    type Output = Result<i32>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use futures::prelude::*;

        self.0.wait().boxed()
            .poll_unpin(cx)
            .map(|status| match status.map(|status| status.code()) {
                anyhow::Result::Ok(Some(code)) => Ok(code),
                _ => Err(anyhow!(
                    "failed to start child process or terminated abnormally"
                )),
            })
    }
}

impl super::ChildProcess for TokioChildProcess {
    fn stdout(&mut self) -> Result<Box<dyn super::AsyncReadUnpin>> {
        let boxed_stdout = self
            .0
            .stdout
            .take()
            .map(Box::new)
            .ok_or_else(|| anyhow!("failed to take stdout"))?;

        Ok(boxed_stdout)
    }

    fn stderr(&mut self) -> Result<Box<dyn super::AsyncReadUnpin>> {
        let boxed_stderr = self
            .0
            .stderr
            .take()
            .map(Box::new)
            .ok_or_else(|| anyhow!("failed to take stderr"))?;

        Ok(boxed_stderr)
    }
}

pub struct TokioCmdExecutor;

impl TokioCmdExecutor {
    fn parse(command: &str) -> (String, Vec<String>) {
        let mut elements = command.split(' ').map(Into::into).collect::<Vec<_>>();

        let options = elements.drain(1..).collect::<Vec<_>>();

        let command = match elements.get(0) {
            Some(_) => elements.remove(0),
            None => String::new(),
        };

        (command, options)
    }
}

impl super::SpawnChild for TokioCmdExecutor {
    fn spawn(&self, command: &str) -> Result<Box<dyn super::ChildProcess + Send>> {
        let (command, options) = Self::parse(command);

        let mut cmd = tokio::process::Command::new(command);

        let child = cmd
            .args(options)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        Ok(Box::new(TokioChildProcess(child)))
    }
}

#[cfg(test)]
pub struct StubChildProcess {
    stdout: std::io::Cursor<Vec<u8>>,
    stderr: std::io::Cursor<Vec<u8>>,
}

#[cfg(test)]
impl super::AsyncReadUnpin for std::io::Cursor<Vec<u8>> {}

#[cfg(test)]
impl Future for StubChildProcess {
    type Output = Result<i32>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(Ok(0))
    }
}

#[cfg(test)]
impl super::ChildProcess for StubChildProcess {
    fn stdout(&mut self) -> Result<Box<dyn super::AsyncReadUnpin>> {
        Ok(Box::new(self.stdout.clone()))
    }

    fn stderr(&mut self) -> Result<Box<dyn super::AsyncReadUnpin>> {
        Ok(Box::new(self.stderr.clone()))
    }
}

#[cfg(test)]
pub struct StubCmdExecutor {
    pub child_stdout: Vec<u8>,
    pub child_stderr: Vec<u8>,
}

#[cfg(test)]
impl super::SpawnChild for StubCmdExecutor {
    fn spawn(&self, _: &str) -> Result<Box<dyn super::ChildProcess + Send>> {
        Ok(Box::new(StubChildProcess {
            stdout: std::io::Cursor::new(self.child_stdout.clone()),
            stderr: std::io::Cursor::new(self.child_stderr.clone()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let (command, options) = TokioCmdExecutor::parse("ping 8.8.8.8");
        assert_eq!(command, "ping".to_string());
        assert_eq!(options, vec!["8.8.8.8".to_string()]);
    }
}
