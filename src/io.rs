mod tokio_impl;

pub use tokio_impl::*;

#[derive(derive_new::new, Debug, Clone, PartialEq, derive_getters::Getters)]
pub struct Exit {
    code: i32,
}

#[async_trait::async_trait]
pub trait PipedCmdExecute {
    async fn piped_exec(&self, command: &str) -> std::io::Result<Exit>;
}
