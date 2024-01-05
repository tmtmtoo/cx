use super::*;

pub struct TokioPipedCmdExecutor;

impl TokioPipedCmdExecutor {
    fn parse_command(command: &str) -> (String, Vec<String>) {
        let mut elements = command.split(' ').map(Into::into).collect::<Vec<_>>();

        let options = elements.drain(1..).collect::<Vec<_>>();

        let program = match elements.get(0) {
            Some(_) => elements.remove(0),
            None => String::new(),
        };

        (program, options)
    }
}

#[async_trait::async_trait]
impl PipedCmdExecutor for TokioPipedCmdExecutor {
    async fn piped_exec(&self, command: &str) -> anyhow::Result<Exit> {
        let (program, options) = Self::parse_command(command);

        let mut child = tokio::process::Command::new(program)
            .args(options)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let mut child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("failed to take stdout"))?;

        let mut child_stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("failed to take stderr"))?;

        let mut process_stdout = tokio::io::stdout();

        let mut process_stderr = tokio::io::stderr();

        let (exit_status, _, _) = tokio::join!(
            child.wait(),
            tokio::io::copy(&mut child_stdout, &mut process_stdout),
            tokio::io::copy(&mut child_stderr, &mut process_stderr)
        );

        let code = exit_status?.code().ok_or_else(|| {
            anyhow::anyhow!("failed to start child process or terminated abnormally")
        })?;

        Ok(super::Exit { code })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_success_given_suitable_command() {
        let actual = TokioPipedCmdExecutor.piped_exec("echo abcd").await.unwrap();
        let expected = Exit { code: 0 };
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn should_failure_when_command_not_found() {
        let actual = TokioPipedCmdExecutor.piped_exec("failed").await.is_err();
        assert!(actual);
    }

    #[tokio::test]
    async fn should_success_when_exit_not_zero() {
        let actual = TokioPipedCmdExecutor
            .piped_exec("cat non_existent_file")
            .await
            .unwrap();
        assert_ne!(actual, Exit { code: 0 });
    }
}
