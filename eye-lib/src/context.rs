use std::time::Duration;

use crate::metric::Metric;

#[derive(Clone)]
pub struct Context {
    pub process_name: String,
    pub metric: Metric,
    pub interval: Duration,
    pub one_iteration: bool,
}

impl Context {
    pub fn new(
        process_name: String,
        metric: Metric,
        interval: Duration,
        one_iteration: bool,
    ) -> Self {
        Self {
            process_name,
            metric,
            interval,
            one_iteration,
        }
    }
}
