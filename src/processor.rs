use std::path::Path;
use std::process::{Command, Stdio};

pub(crate) trait EventProcessor {
    fn process<P: AsRef<Path>>(&self, path: P);
}

pub(crate) struct NewBookEventProcessor;

impl NewBookEventProcessor {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl EventProcessor for NewBookEventProcessor {
    fn process<P: AsRef<Path>>(&self, path: P) {
        println!("new file created: {:?}", path.as_ref());
        Command::new("bt-obex")
            .stdout(Stdio::inherit())
            .arg("-p")
            .arg("64:A2:F9:E9:AE:C3")
            .arg(path.as_ref())
            .output()
            .expect("failed sending file via bt-obex");
    }
}
