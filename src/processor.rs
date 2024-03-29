use crate::settings::Settings;
use anyhow::Result;
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub(crate) trait EventProcessor {
    fn process(&self, path: &dyn AsRef<Path>) -> Result<()>;
}

pub(crate) struct NewBookEventProcessor<'a> {
    settings: &'a Settings,
}

impl<'a> NewBookEventProcessor<'a> {
    pub(crate) fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }

    fn send_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let output = Command::new("bt-obex")
            .stdout(Stdio::inherit())
            .arg("-p")
            .arg(self.settings.device_mac())
            .arg(path.as_ref())
            .output()?;
        println!(
            "command finished with a status code: {}",
            status_code(&output)
        );
        Ok(())
    }
}

impl<'a> EventProcessor for NewBookEventProcessor<'a> {
    fn process(&self, path: &dyn AsRef<Path>) -> Result<()> {
        println!("new file created: {:?}", path.as_ref());
        println!("sending new file via bluetooth");
        self.send_file(path)?;
        Ok(())
    }
}

fn status_code(output: &Output) -> i32 {
    output.status.code().expect("failed to get status code")
}
