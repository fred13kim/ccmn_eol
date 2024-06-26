use std::{
    io::{BufRead, BufReader},
    time::{self, Duration},
};

use anyhow::Result;
use eol_shared::{TestResults, TEST_RESULT_START_MAGIC};

use crate::EolTest;
use tracing::debug;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid TestResults serialization: got \"{0}\"")]
    InvalidTestResultsString(String),
    #[error("Timed out waiting for TestResults")]
    TimedOut,
}

macro_rules! port_op {
    ($op: expr, $err: tt) => {
        $op.map_err(|e| Error::$err(e.to_string()))
    };
}

impl EolTest {
    pub fn get_test_result(&self) -> Result<TestResults> {
        let start = time::Instant::now();
        let mut reader = BufReader::new(self.tester.try_clone().unwrap());

        let mut got_first = false;

        while start.elapsed() < Duration::from_secs(15) {
            let mut line = String::new();

            reader.read_line(&mut line).ok();
            if line != "" {
                debug!("DUT: {line}");
            }
            if let Some(results) = line.strip_prefix(TEST_RESULT_START_MAGIC) {
                if !got_first {
                    // skip the first result
                    got_first = true;
                    continue;
                }
                // lol you can use port_op! for this
                let results = port_op!(serde_json::from_str(results), InvalidTestResultsString)?;
                return Ok(results);
            }
        }

        Err(Error::TimedOut.into())
    }
}
