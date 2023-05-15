use std::process;
use std::{io, string::FromUtf8Error};
use thiserror::Error;

use crate::metric::Metric;

pub trait Runner {
    fn execute(&self, process_name: &str, metric: &Metric) -> Result<String, Error>;
}

/// Ps is an unit struct without data. It represent a executable
/// command which produce data for given metric
pub struct Ps;

impl Runner for Ps {
    fn execute(&self, process_name: &str, metric: &Metric) -> Result<String, Error> {
        // Get the `ps aux` command output (as bytes)
        // Note the `?` : if fail, return a `io::Result`
        // which is automatically converted to `Error` (see `#[from] io::Error`)
        let bytes_output = process::Command::new("ps").arg("aux").output()?.stdout;
        // Convert bytes output as String
        // Note the `?` : if fail, return a `string::FromUtf8Error`
        // which is automatically converted to `Error` (see `#[from] string::FromUtf8Error`)
        let raw_output = String::from_utf8(bytes_output)?;

        // Search wanted line (where process name match)
        if let Some(line) = find_process_line(&raw_output, &process_name) {
            // Get the wanted metric column value
            match find_metric(&line, &metric) {
                // And return it if found
                Some(value) => return Ok(value.to_string()),
                // If column not found, return an error about command output format
                // which seems to be incorrect or badly read
                None => return Err(Error::CommandOutputFormatError),
            }
        }

        // If wanted line was not found, so process has not been found
        return Err(Error::ProcessNotFound);
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error during command execution")]
    CommandExecutionError(#[from] io::Error),
    #[error("Error during command execution output utf8 decode")]
    CommandOutputUtf8DecodeError(#[from] FromUtf8Error),
    #[error("No process found for given name")]
    ProcessNotFound,
    #[error("Command output format error")]
    CommandOutputFormatError,
}

impl From<u8> for Error {
    fn from(value: u8) -> Self {
        Error::ProcessNotFound
    }
}

fn foo() {
    let e = 5;
    let error: Error = e.into();
}

fn find_process_line(raw: &str, process_name: &str) -> Option<String> {
    raw.lines()
        .filter(|l| {
            l.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .filter(|v| v.len() > 0)
                .collect::<Vec<&&str>>()
                .get(10)
                == Some(&&process_name)
        })
        .collect::<Vec<&str>>()
        .first()
        .and_then(|v| Some(v.to_string()))
}

fn find_metric(line: &str, metric: &Metric) -> Option<String> {
    line.split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|v| v.len() > 0)
        .collect::<Vec<&&str>>()
        .get(metric.column())
        .and_then(|v| Some(v.to_string()))
}
