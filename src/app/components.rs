mod cmd_executor;
mod cmd_not_found;
mod wait;

pub use cmd_executor::*;
pub use cmd_not_found::*;
pub use wait::*;

#[async_trait::async_trait]
pub trait Component {
    type Output;

    async fn handle(&self) -> Self::Output;
}
