mod tokio_impl;

#[derive(derive_new::new, Debug, Clone, PartialEq, derive_getters::Getters)]
pub struct Exit {
    code: i32,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait PipedCmdExecutor {
    async fn piped_exec(&self, command: &str) -> anyhow::Result<Exit>;
}
