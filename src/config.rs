use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(author = "")]
#[structopt(raw(setting = "AppSettings::AllowLeadingHyphen"))]
/// Command eXecutor
pub enum Config {
    /// Retry command execution until successful.
    #[structopt(author = "")]
    retry {
        /// maximum number of retries
        #[structopt(short, long)]
        max: Option<usize>,

        /// execution interval (sec)
        #[structopt(short, long, default_value = "0.1")]
        interval: f64,

        /// command and options
        #[structopt(name = "COMMAND")]
        command: Vec<String>,
    },
    /// Supervise command execution.
    #[structopt(author = "")]
    supervise {
        /// re-execution limit
        #[structopt(short, long)]
        limit: Option<usize>,

        /// execution interval (sec)
        #[structopt(short, long, default_value = "0.1")]
        interval: f64,

        /// command and options
        #[structopt(name = "COMMAND")]
        command: Vec<String>,
    },
}
