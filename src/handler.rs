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
    use crate::provider::{EventProvider, FsEvent};
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::sync::mpsc::{channel, Receiver, RecvError};

    #[test]
    fn test() {
        // given
        let processor_spy = EventProcessorSpy::new();
        let events = vec![FsEvent::NewFile(PathBuf::from(r"/test")), FsEvent::Stop];
        // when
        FsEventDispatcher::new(&EventProviderStub::new(events), &processor_spy).handle();
        // then
        assert_eq!(processor_spy.executed(), true);
    }

    struct EventProviderStub {
        events: Vec<FsEvent>,
        current_event: RefCell<usize>,
    }

    impl EventProviderStub {
        fn new(events: Vec<FsEvent>) -> Self {
            EventProviderStub {
                events: events.to_vec(),
                current_event: RefCell::new(0),
            }
        }
    }

    impl EventProvider for EventProviderStub {
        fn next(&self) -> Result<FsEvent, RecvError> {
            let idx = self.current_event.clone().into_inner();
            let res = self.events[idx].clone();
            *self.current_event.borrow_mut() += 1;
            Ok(res)
        }
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
