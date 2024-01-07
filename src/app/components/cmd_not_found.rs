#[derive(new)]
pub struct PrintableCmdNotFound<C> {
    pub command: String,
    pub inner: C,
}

#[async_trait::async_trait]
impl<T: 'static, C: super::Component<Output = anyhow::Result<T>> + Send + Sync> super::Component
    for PrintableCmdNotFound<C>
{
    type Output = anyhow::Result<T>;

    async fn handle(&self) -> Self::Output {
        let result = self.inner.handle().await;

        if let Err(_) = &result {
            if self.command.is_empty() {
                eprintln!("cx: no command entered")
            } else {
                eprintln!(
                    "cx: command not found '{}'",
                    self.command
                        .split(' ')
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap_or(&"")
                )
            }
        }

        result
    }
}
