// https://doc.rust-lang.org/stable/rustc/lints/listing/warn-by-default.html
#![deny(non_shorthand_field_patterns)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]
#![deny(path_statements)]
#![deny(renamed_and_removed_lints)]
#![deny(unconditional_recursion)]
#![deny(unknown_lints)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![deny(unused_assignments)]
#![deny(unused_comparisons)]
#![deny(unused_mut)]
#![deny(unused_parens)]
#![deny(unused_variables)]
#![deny(while_true)]
#![deny(unused_imports)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate derive_new;

mod app;
mod config;
mod io;

use app::*;
use config::*;

#[tokio::main]
async fn main() {
    use futures::{future::Either, FutureExt};
    use structopt::StructOpt;

    let config = Config::from_args();

    let state_machine = match config {
        Config::retry {
            command,
            count,
            interval,
        } => Either::Left(
            run(RetryApp::new(command.join(" "), count, interval)).map(|output| match output {
                RetryResult::Success => 0,
                RetryResult::Failure => 1,
            }),
        ),
        Config::supervise {
            command,
            count,
            interval,
        } => Either::Right(run(SuperviseApp::new(command.join(" "), count, interval)).map(|_| 0)),
    };

    std::process::exit(state_machine.await);
}
