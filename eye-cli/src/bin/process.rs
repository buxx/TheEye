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

fn main() {
    // Opt is build from command line arguments.
    // The .into() call (through `Into<context::Context>` trait implementation) build a
    // Context object.
    let context: Context = Opt::from_args().into();
    // Prepare a channel to establish communication with the `watcher::Watcher`
    let (sender, receiver): (Sender<watcher::Message>, Receiver<watcher::Message>) = channel();

    // `watcher::Watcher::run` method is blocking, so start it in a thread
    // Two things important here :
    //  - `sender` is the sender channel part which the watcher will use to send values
    //  - `Ps` is a unit structure which match with watcher expected generic type
    thread::spawn(move || Watcher::new(context, sender, Ps).run());

    // Block main program on channel reading while there is no channel receive error.
    // Error means channel is closed and so, application is closing.
    while let Ok(change) = receiver.recv() {
        match change {
            Ok(change) => println!("{}", change),
            Err(error) => {
                eprintln!("{:#}", error);
                std::process::exit(1);
            }
        }
    }

    // How the program can stop when `one_iteration` is provided ?
    //  1. `Watcher::run` function finish
    //  2. The thread containing `Watcher::run` finish too
    //  3. The `sender` value, previously owned by the thread
    //     (note the `move` on the thread closure) is destroyed.
    //  4. As the `sender` object as been destroyed, `receiver.recv()`
    //     receive an error because channel has been closed.
    //  5. While is interrupted because receive something else than a Ok(T)
    //  6. Program end
}
