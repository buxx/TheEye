use std::process;
use std::{io, string::FromUtf8Error};
use thiserror::Error;

use crate::metric::Metric;

pub trait Runner {
    fn execute(&self, process_name: &str, metric: &Metric) -> Result<String, Error>;
}

pub struct Ps;

impl Runner for Ps {
    fn execute(&self, process_name: &str, metric: &Metric) -> Result<String, Error> {
        let raw_output =
            String::from_utf8(process::Command::new("ps").arg("aux").output()?.stdout)?;

        // for line in raw_output.lines() {
        //     println!("{}", line);
        //     println!("{:?}", line.split(" ").collect::<Vec<&str>>());
        //     let a = 1;
        // }

        if let Some(line) = raw_output
            .lines()
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
        {
            let metric_column = match metric {
                Metric::Cpu => 2,
                Metric::ResidentMemory => 5,
            };
            match line
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .filter(|v| v.len() > 0)
                .collect::<Vec<&&str>>()
                .get(metric_column)
            {
                Some(value) => return Ok(value.to_string()),
                None => return Err(Error::CommandOutputFormatError),
            }
        }

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
