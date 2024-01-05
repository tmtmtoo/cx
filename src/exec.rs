pub mod tokio_impl;

#[derive(derive_new::new, Debug, Clone, PartialEq, derive_getters::Getters)]
pub struct Exit {
    code: i32,
}

#[async_trait::async_trait]
pub trait PipedCmdExecutor: Send + Sync {
    async fn piped_exec(&self, command: &str) -> anyhow::Result<Exit>;
}
