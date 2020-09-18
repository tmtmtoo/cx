impl super::AsyncWriteUnpin for tokio::io::Stdout {}

impl super::AsyncWriteUnpin for tokio::io::Stderr {}

#[derive(new)]
pub struct TokioProcess;

impl super::Process for TokioProcess {
    fn stdout(&self) -> Box<dyn super::AsyncWriteUnpin> {
        Box::new(tokio::io::stdout())
    }

    fn stderr(&self) -> Box<dyn super::AsyncWriteUnpin> {
        Box::new(tokio::io::stderr())
    }
}

#[cfg(test)]
impl super::AsyncWriteUnpin for Vec<u8> {}

#[cfg(test)]
pub struct StubProcess {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

#[cfg(test)]
impl super::Process for StubProcess {
    fn stdout(&self) -> Box<dyn super::AsyncWriteUnpin> {
        Box::new(self.stdout.clone())
    }

    fn stderr(&self) -> Box<dyn super::AsyncWriteUnpin> {
        Box::new(self.stderr.clone())
    }
}
