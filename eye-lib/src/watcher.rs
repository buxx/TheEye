use crate::{
    change::Change,
    context::Context,
    runner::Runner,
    runner::{self},
};
use std::{sync::mpsc::Sender, thread};
use thiserror::Error;

pub type Message = Result<Change, Error>;

/// For given runner, execute it at regular interval (according to given [context](Watcher::context))
/// and send through given [context](Watcher::sender) when watched metric change.
pub struct Watcher<T: Runner> {
    /// Context permitting to watcher to know witch metric to watch, at witch interval, etc
    context: Context,
    /// Command to execute with given context
    command: T,
    /// Channel sender where send changes
    sender: Sender<Message>,
    /// Previously watched value
    value: Option<String>,
}

impl<T: Runner> Watcher<T> {
    pub fn new(context: Context, sender: Sender<Message>, command: T) -> Self {
        Self {
            context,
            command,
            sender,
            value: None,
        }
    }

    /// Execute the watch behavior. This function is blocking.
    pub fn run(&mut self) {
        loop {
            // Execute runner and deal with result
            match self.lookup() {
                // If it is a success and there is a change of value
                Ok(Some(change)) => {
                    // Send it over channel. If there is a send error (means channel is closed,
                    // so the application is closing) or only one execution must be executed,
                    // break the loop to end up the function.
                    if self.sender.send(Ok(change)).is_err() || self.context.one_iteration {
                        break;
                    };
                }
                // If command execution failed
                Err(error) => {
                    // Send error over the channel
                    // We allow to not use the self.sender.send() result because
                    // a closed channel means a closing application.
                    #[allow(unused_must_use)]
                    {
                        self.sender.send(Err(error));
                    }
                    return;
                }
                // If it is a success and there is NOT a change, do nothing
                Ok(None) => {}
            }

            thread::sleep(self.context.interval);
        }
    }

    /// Execute the given command ans return, if success, a possible change.
    fn lookup(&mut self) -> Result<Option<Change>, Error> {
        let value = self
            .command
            // .execute is here accessible because is part of "T: Runner"
            .execute(&self.context.process_name, &self.context.metric)?;

        // If value change
        // Note : we use references here to not own "value" and
        // be able to use it after
        if self.value.as_ref() != Some(&value) {
            // To store the new value, we must clone "value" : Without cloning it,
            // we can't give it to "Change" structure which is returned.
            self.value = Some(value.clone());
            return Ok(Some(Change(value)));
        }

        // If value is not different, return a success (Ok)
        // but with the None variant
        return Ok(None);
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error during command execution : {0}")]
    CommandExecutionError(#[from] runner::Error),
}
