mod components;
mod retry;
mod supervise;

pub use retry::*;
pub use supervise::*;

pub enum Transition<N, D> {
    Next(N),
    Done(D),
}

#[async_trait::async_trait]
pub trait StateMachine: Sized {
    type Output;

    async fn handle(self) -> Transition<Self, Self::Output>;
}

pub async fn run<S: StateMachine>(mut machine: S) -> S::Output {
    loop {
        match machine.handle().await {
            Transition::Next(next) => machine = next,
            Transition::Done(done) => break done,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_trait::async_trait]
    impl StateMachine for u32 {
        type Output = u32;

        async fn handle(self) -> Transition<Self, Self::Output> {
            if self <= 4 {
                Transition::Next(self + 1)
            } else {
                Transition::Done(self)
            }
        }
    }

    #[lite_async_test::async_test]
    async fn run_must_be_done() {
        let actual = run(0).await;
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
