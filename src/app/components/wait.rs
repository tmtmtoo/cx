use crate::io::*;

pub struct WaitSec {
    pub sec: f64,
    pub sleeper: std::sync::Arc<dyn Sleep + Send + Sync>,
}

#[async_trait::async_trait]
impl super::Component for WaitSec {
    type Output = ();

    async fn handle(&self) -> Self::Output {
        self.sleeper.sleep_sec(self.sec).await;
    }
}
