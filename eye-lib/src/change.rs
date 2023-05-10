use std::fmt::Display;

/// Process metric change container
#[derive(Debug)]
pub struct Change(pub String);

impl Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
