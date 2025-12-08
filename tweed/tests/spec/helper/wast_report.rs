use std::fmt;
use std::{self};

#[derive(Debug, PartialEq)]
pub enum WastResult {
    AssertResult { name: String, line: usize },
}
#[derive(Default, Debug, PartialEq)]
pub struct WastReport {
    pub file: String,
    pub success: Vec<WastResult>,
    pub failure: Vec<WastResult>,
}

impl fmt::Display for WastReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let green = "\x1b[32m";
        let red = "\x1b[31m";
        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";

        writeln!(
            f,
            "\n{}Test Report for File:{} {}\n",
            yellow, reset, self.file
        )?;

        writeln!(
            f,
            "{}Tests Successful:{} {}/{}\n",
            green,
            reset,
            self.success.len(),
            self.success.len() + self.failure.len()
        )?;

        if !self.success.is_empty() {
            writeln!(f, "{}Successful Tests:{}\n", green, reset)?;
            for result in &self.success {
                match result {
                    WastResult::AssertResult { name, line } => {
                        writeln!(f, "- {}{}:{} {}", green, name, reset, line)?;
                    }
                };
            }
        }

        if !self.failure.is_empty() {
            writeln!(f, "{}Failures:{}\n", red, reset)?;
            for result in &self.failure {
                match result {
                    WastResult::AssertResult { name, line } => {
                        writeln!(f, "- {}{}:{} {}", red, name, reset, line)?;
                    }
                };
            }
        }

        Ok(())
    }
}
