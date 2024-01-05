use super::{components::*, *};
use crate::exec2::*;
use crate::prelude::*;

pub enum RetryResult {
    Success,
    Failure,
}

enum State<E, S> {
    ExecuteCommand(E),
    Sleep(S),
}

pub struct RetryApp<E, S> {
    state: State<E, S>,
    count: Option<usize>,
}

#[async_trait]
impl<E, S> StateMachine for RetryApp<E, S>
where
    E: Component<Output = Result<Exit>> + Into<S> + Send + Sync,
    S: Component<Output = ()> + Into<E> + Send + Sync,
{
    type Output = RetryResult;

    async fn handle(self) -> Transition<Self, Self::Output> {
        match self.state {
            State::ExecuteCommand(component) => match self.count {
                Some(0) => Transition::Done(RetryResult::Failure),
                _ => match (component.handle().await, self.count) {
                    (anyhow::Result::Ok(exit), _) if *exit.code() == 0 => {
                        Transition::Done(RetryResult::Success)
                    }
                    (_, Some(1)) => Transition::Done(RetryResult::Failure),
                    (_, _) => Transition::Next(RetryApp {
                        state: State::Sleep(component.into()),
                        count: self.count.map(|c| c - 1),
                    }),
                },
            },
            State::Sleep(component) => {
                component.handle().await;

                Transition::Next(RetryApp {
                    state: State::ExecuteCommand(component.into()),
                    ..self
                })
            }
        }
    }
}

#[derive(new)]
pub struct SharedParams<C> {
    command: String,
    interval: f64,
    executor: Arc<dyn PipedCmdExecutor>,
    inner: C,
}

#[async_trait]
impl<T: 'static, C: Component<Output = T> + Send + Sync> Component for SharedParams<C> {
    type Output = T;

    async fn handle(&self) -> Self::Output {
        self.inner.handle().await
    }
}

impl From<SharedParams<PrintableCmdNotFound<CmdExecutor>>> for SharedParams<WaitSec> {
    fn from(state: SharedParams<PrintableCmdNotFound<CmdExecutor>>) -> Self {
        Self {
            inner: WaitSec {
                sec: state.interval,
            },
            command: state.command,
            interval: state.interval,
            executor: state.executor,
        }
    }
}

impl From<SharedParams<WaitSec>> for SharedParams<PrintableCmdNotFound<CmdExecutor>> {
    fn from(state: SharedParams<WaitSec>) -> Self {
        Self {
            inner: PrintableCmdNotFound {
                command: state.command.to_owned(),
                inner: CmdExecutor {
                    command: state.command.to_owned(),
                    executor: state.executor.clone(),
                },
            },
            command: state.command,
            interval: state.interval,
            executor: state.executor,
        }
    }
}

impl RetryApp<SharedParams<PrintableCmdNotFound<CmdExecutor>>, SharedParams<WaitSec>> {
    pub fn new(command: String, count: Option<usize>, interval: f64) -> Self {
        let executor = Arc::new(tokio_impl::TokioPipedCmdExecutor);

        Self {
            state: State::ExecuteCommand(SharedParams::new(
                command.to_owned(),
                interval,
                executor.clone(),
                PrintableCmdNotFound::new(command.to_owned(), CmdExecutor::new(command, executor)),
            )),
            count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestE {
        output: Box<dyn Fn() -> Result<Exit> + Send + Sync>,
    }

    #[async_trait]
    impl Component for TestE {
        type Output = Result<Exit>;
        async fn handle(&self) -> Self::Output {
            (*self.output)()
        }
    }

    struct TestS;

    #[async_trait]
    impl Component for TestS {
        type Output = ();
        async fn handle(&self) -> Self::Output {
            ()
        }
    }

    impl From<TestE> for TestS {
        fn from(_: TestE) -> Self {
            TestS
        }
    }

    impl From<TestS> for TestE {
        fn from(_: TestS) -> Self {
            TestE {
                output: Box::new(|| Ok(Exit::new(1))),
            }
        }
    }

    #[tokio::test]
    async fn exec_cmd_to_done_with_success() {
        let app = RetryApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE {
                output: Box::new(|| Ok(Exit::new(0))),
            }),
            count: None,
        };

        assert!(matches!(
            app.handle().await,
            Transition::Done(RetryResult::Success)
        ));
    }

    #[tokio::test]
    async fn exec_cmd_to_sleep_without_limit() {
        let app = RetryApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE {
                output: Box::new(|| Ok(Exit::new(1))),
            }),
            count: None,
        };

        let next = app.handle().await;

        assert!(match &next {
            Transition::Next(a) => match a.state {
                State::Sleep(_) => true,
                _ => false,
            },
            _ => false,
        });
    }

    #[tokio::test]
    async fn exec_cmd_to_sleep_with_limit() {
        let app = RetryApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE {
                output: Box::new(|| Ok(Exit::new(1))),
            }),
            count: Some(2),
        };

        let next = app.handle().await;

        assert!(match &next {
            Transition::Next(a) => match a.state {
                State::Sleep(_) => true,
                _ => false,
            },
            _ => false,
        });

        assert_eq!(
            match next {
                Transition::Next(a) => a.count,
                _ => None,
            },
            Some(1)
        );
    }

    #[tokio::test]
    async fn exec_cmd_to_done_with_fail() {
        let app = RetryApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE {
                output: Box::new(|| Ok(Exit::new(1))),
            }),
            count: Some(1),
        };

        assert!(matches!(
            app.handle().await,
            Transition::Done(RetryResult::Failure)
        ));
    }

    #[tokio::test]
    async fn sleep_to_exec() {
        let app = RetryApp::<TestE, TestS> {
            state: State::Sleep(TestS),
            count: Some(1),
        };

        assert!(match app.handle().await {
            Transition::Next(a) => match a.state {
                State::ExecuteCommand(_) => true,
                _ => false,
            },
            _ => false,
        });
    }
}
