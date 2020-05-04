use anyhow::Result;
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub(crate) trait EventProcessor {
    fn process<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

pub(crate) struct NewBookEventProcessor;

impl NewBookEventProcessor {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl EventProcessor for NewBookEventProcessor {
    fn process<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        println!("new file created: {:?}", path.as_ref());
        println!("sending new file via bluetooth");
        send_file(path)?;
        Ok(())
    }
}

fn send_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let output = Command::new("bt-obex")
        .stdout(Stdio::inherit())
        .arg("-p")
        .arg("64:A2:F9:E9:AE:C3")
        .arg(path.as_ref())
        .output()?;
    check_status_code(output);
    Ok(())
}

fn check_status_code(output: Output) {
    if output.status.success() {
        println!("command finished with status code: {}", status_code(output));
    } else {
        eprint!(
            "failed to send file via bluetooth, status code: {}",
            status_code(output)
        );
    }
}

fn status_code(output: Output) -> i32 {
    output.status.code().expect("failed to get status code")
}
