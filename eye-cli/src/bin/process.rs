use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use eye_lib::{
    context::Context,
    metric::Metric,
    runner::Ps,
    watcher::{self, Watcher},
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Process name to watch, like `/usr/sbin/acpid`
    #[structopt(name = "process_name")]
    process_name: String,

    /// Which metric to watch
    #[structopt(name = "metric")]
    metric: Metric,

    /// Interval between each os processes information read. In milliseconds.
    #[structopt(short = "i", long = "interval", default_value = "100")]
    interval: u64,

    /// Stop after first iteration
    #[structopt(short, long)]
    one_iteration: bool,
}

impl Into<Context> for Opt {
    fn into(self) -> Context {
        Context::new(
            self.process_name.clone(),
            self.metric.clone(),
            Duration::from_millis(self.interval),
            self.one_iteration,
        )
    }
}

fn main() -> Result<(), watcher::Error> {
    let context: Context = Opt::from_args().into();
    let (sender, receiver): (Sender<watcher::Message>, Receiver<watcher::Message>) = channel();

    thread::spawn(move || Watcher::new(context, sender, Ps).run());
    while let Ok(change) = receiver.recv() {
        match change {
            Ok(change) => println!("{}", change),
            Err(error) => {
                eprintln!("{:#}", error);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
