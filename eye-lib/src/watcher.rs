use crate::{
    change::Change,
    context::Context,
    runner::Runner,
    runner::{self},
};
use std::{sync::mpsc::Sender, thread};
use thiserror::Error;

pub type Message = Result<Change, Error>;

pub struct Watcher<T: Runner> {
    context: Context,
    command: T,
    sender: Sender<Message>,
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

    pub fn run(&mut self) {
        loop {
            match self.execute() {
                Ok(Some(change)) => {
                    if self.sender.send(Ok(change)).is_err() || self.context.one_iteration {
                        break;
                    };
                }
                Err(error) => {
                    #[allow(unused_must_use)]
                    {
                        self.sender.send(Err(error));
                    }
                    return;
                }
                Ok(None) => {}
            }

            thread::sleep(self.context.interval);
        }
    }

    fn execute(&mut self) -> Result<Option<Change>, Error> {
        let value = self
            .command
            .execute(&self.context.process_name, &self.context.metric)?;

        if self.value != Some(value.clone()) {
            self.value = Some(value.clone());
            return Ok(Some(Change(value)));
        }

        return Ok(None);
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error during command execution : {0}")]
    CommandExecutionError(#[from] runner::Error),
}
