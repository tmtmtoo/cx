#[derive(Debug, structopt::StructOpt, PartialEq)]
#[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
/// Command eXecutor
pub enum Config {
    /// Retry command execution until successful.
    retry {
        /// maximum number of retry counts
        #[structopt(short, long)]
        count: Option<usize>,

        /// execution interval (sec)
        #[structopt(short, long, default_value = "0.1")]
        interval: f64,

        /// command and options
        #[structopt(name = "COMMAND")]
        command: Vec<String>,
    },
    /// Supervise command execution.
    supervise {
        /// re-execution limit counts
        #[structopt(short, long)]
        count: Option<usize>,

        /// execution interval (sec)
        #[structopt(short, long, default_value = "0.1")]
        interval: f64,

        /// command and options
        #[structopt(name = "COMMAND")]
        command: Vec<String>,
    },
}
