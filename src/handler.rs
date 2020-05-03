use crate::processor::EventProcessor;
use notify::DebouncedEvent;
use std::sync::mpsc::Receiver;

pub(crate) struct FsEventHandler<'a, T: EventProcessor> {
    receiver: Receiver<DebouncedEvent>,
    processor: &'a T,
}

impl<'a, P: EventProcessor> FsEventHandler<'a, P> {
    pub(crate) fn new(receiver: Receiver<DebouncedEvent>, processor: &'a P) -> Self {
        Self {
            receiver,
            processor,
        }
    }

    pub(crate) fn handle(&self) {
        loop {
            match self.receiver.recv() {
                Ok(DebouncedEvent::Create(p)) => self.processor.process(p),
                Ok(_) => println!("different event"),
                Err(e) => eprint!("watch error: {:?}", e),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{EventProcessor, FsEventHandler};
    use std::cell::RefCell;
    use std::path::Path;
    use std::sync::mpsc::channel;

    #[test]
    fn test() {
        let (sender, receiver) = channel();
        FsEventHandler::new(receiver, &EventProcessorSpy::new()).handle();
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
