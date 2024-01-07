use crate::io::*;

pub struct WaitSec<'a> {
    pub sec: f64,
    pub sleeper: &'a (dyn Sleep + Send + Sync),
}

#[async_trait::async_trait]
impl<'a> super::Component for WaitSec<'a> {
    type Output = ();

    async fn handle(&self) -> Self::Output {
        self.sleeper.sleep_sec(self.sec).await;
    }
}
