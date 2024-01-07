use super::{components::*, *};
use crate::io::*;

enum State<E, S> {
    ExecuteCommand(E),
    Sleep(S),
}

pub struct SuperviseApp<E, S> {
    state: State<E, S>,
    count: Option<usize>,
}

#[async_trait::async_trait]
impl<E, S> StateMachine for SuperviseApp<E, S>
where
    E: Component<Output = anyhow::Result<Exit>> + Into<S> + Send + Sync,
    S: Component<Output = ()> + Into<E> + Send + Sync,
{
    type Output = ();

    async fn handle(self) -> Transition<Self, Self::Output> {
        match self.state {
            State::ExecuteCommand(component) => match self.count {
                Some(0) => Transition::Done(()),
                _ => {
                    let _ = component.handle().await;
                    match self.count {
                        Some(1) => Transition::Done(()),
                        _ => Transition::Next(SuperviseApp {
                            state: State::Sleep(component.into()),
                            count: self.count.map(|c| c - 1),
                        }),
                    }
                }
            },
            State::Sleep(component) => {
                component.handle().await;

                Transition::Next(SuperviseApp {
                    state: State::ExecuteCommand(component.into()),
                    ..self
                })
            }
        }
    }
}

#[derive(new)]
pub struct SharedParams<'a, C> {
    command: &'a str,
    interval: f64,
    executor: &'a (dyn PipedCmdExecute + Send + Sync),
    sleeper: &'a (dyn Sleep + Send + Sync),
    inner: C,
}

#[async_trait::async_trait]
impl<T: 'static, C: Component<Output = T> + Send + Sync, 'a> Component for SharedParams<'a, C> {
    type Output = T;

    async fn handle(&self) -> Self::Output {
        self.inner.handle().await
    }
}

impl<'a> From<SharedParams<'a, PrintableCmdNotFound<'a, CmdExecutor<'a>>>>
    for SharedParams<'a, WaitSec<'a>>
{
    fn from(state: SharedParams<'a, PrintableCmdNotFound<'a, CmdExecutor<'a>>>) -> Self {
        Self {
            inner: WaitSec {
                sec: state.interval,
                sleeper: state.sleeper,
            },
            command: state.command,
            interval: state.interval,
            executor: state.executor,
            sleeper: state.sleeper,
        }
    }
}

impl<'a> From<SharedParams<'a, WaitSec<'a>>>
    for SharedParams<'a, PrintableCmdNotFound<'a, CmdExecutor<'a>>>
{
    fn from(state: SharedParams<'a, WaitSec<'a>>) -> Self {
        Self {
            inner: PrintableCmdNotFound {
                command: state.command,
                inner: CmdExecutor {
                    command: state.command,
                    executor: state.executor,
                },
            },
            command: state.command,
            interval: state.interval,
            executor: state.executor,
            sleeper: state.sleeper,
        }
    }
}

impl<'a>
    SuperviseApp<
        SharedParams<'a, PrintableCmdNotFound<'a, CmdExecutor<'a>>>,
        SharedParams<'a, WaitSec<'a>>,
    >
{
    pub fn new(
        command: &'a str,
        count: Option<usize>,
        interval: f64,
        executor: &'a (dyn PipedCmdExecute + Send + Sync),
        sleeper: &'a (dyn Sleep + Send + Sync),
    ) -> Self {
        Self {
            state: State::ExecuteCommand(SharedParams::new(
                command,
                interval,
                executor,
                sleeper,
                PrintableCmdNotFound::new(command, CmdExecutor::new(command, executor)),
            )),
            count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestE;

    #[async_trait::async_trait]
    impl Component for TestE {
        type Output = anyhow::Result<Exit>;
        async fn handle(&self) -> Self::Output {
            Ok(Exit::new(0))
        }
    }

    struct TestS;

    #[async_trait::async_trait]
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
            TestE
        }
    }

    #[lite_async_test::async_test]
    async fn exec_cmd_to_sleep_without_limit() {
        let app = SuperviseApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE),
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

    #[lite_async_test::async_test]
    async fn exec_cmd_to_sleep_with_limit() {
        let app = SuperviseApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE),
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
    }

    #[lite_async_test::async_test]
    async fn exec_cmd_to_done() {
        let app = SuperviseApp::<TestE, TestS> {
            state: State::ExecuteCommand(TestE),
            count: Some(1),
        };

        assert!(match app.handle().await {
            Transition::Done(_) => true,
            _ => false,
        });
    }

    #[lite_async_test::async_test]
    async fn sleep_to_exec() {
        let app = SuperviseApp::<TestE, TestS> {
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
