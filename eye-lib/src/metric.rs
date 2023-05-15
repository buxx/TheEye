use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum Metric {
    Cpu,
    ResidentMemory,
}
#[derive(Error, Debug)]
pub enum MetricError {
    #[error("No known metric for name '{0}'")]
    UnknownMetricName(String),
}

impl FromStr for Metric {
    type Err = MetricError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpu" => Ok(Metric::Cpu),
            "res" => Ok(Metric::ResidentMemory),
            _ => Err(MetricError::UnknownMetricName(s.to_string())),
        }
    }
}

impl Metric {
    pub fn column(&self) -> usize {
        match self {
            Metric::Cpu => 2,
            Metric::ResidentMemory => 5,
        }
    }
}
