pub struct WaitSec {
    pub sec: f64,
}

#[async_trait::async_trait]
impl crate::app::Component for WaitSec {
    type Output = ();

    async fn handle(&self) -> Self::Output {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(self.sec)).await
    }
}
