use crate::processor::EventProcessor;
use crate::provider::{EventProvider, FsEvent};

pub(crate) struct FsEventDispatcher<'a, R: EventProvider, P: EventProcessor> {
    provider: &'a R,
    processor: &'a P,
}

impl<'a, R: EventProvider, P: EventProcessor> FsEventDispatcher<'a, R, P> {
    pub(crate) fn new(provider: &'a R, processor: &'a P) -> Self {
        FsEventDispatcher {
            provider,
            processor,
        }
    }

    pub(crate) fn handle(&self) {
        loop {
            match self.provider.next() {
                Ok(FsEvent::NewFile(p)) => self.processor.process(p),
                Ok(FsEvent::Other) => println!("different event"),
                Ok(FsEvent::Stop) => break,
                Err(e) => eprint!("watch error: {:?}", e),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{EventProcessor, FsEventDispatcher};
    use std::cell::RefCell;
    use std::path::Path;
    use std::sync::mpsc::channel;

    #[test]
    fn test() {
        let (sender, receiver) = channel();
        FsEventDispatcher::new(receiver, &EventProcessorSpy::new()).handle();
    }

    struct EventProcessorSpy {
        executed: RefCell<bool>,
    }

    impl EventProcessorSpy {
        fn new() -> Self {
            Self {
                executed: RefCell::new(false),
            }
        }

        fn executed(self) -> bool {
            self.executed.into_inner()
        }
    }

    impl EventProcessor for EventProcessorSpy {
        fn process<P: AsRef<Path>>(&self, path: P) {
            *self.executed.borrow_mut() = true;
        }
    }
}
