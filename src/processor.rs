use std::path::Path;

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
    }
}

