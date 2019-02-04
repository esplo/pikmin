use crate::error::Result;
use crate::recorder::ProgressRecorder;

/// A progress recorder on memory. This is basically for debugging purpose.
#[derive(Debug, Default)]
pub struct MemoryRecorder {
    value: String,
}

impl ProgressRecorder for MemoryRecorder {
    fn read(&self) -> Result<String> {
        Ok(self.value.clone())
    }

    fn out(&mut self, json: &str) -> Result<()> {
        self.value = String::from(json);
        Ok(())
    }
}
