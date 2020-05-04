use crate::processor::EventProcessor;
use crate::provider::{EventProvider, FsEvent};
use anyhow::Result;

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

    pub(crate) fn handle(&self) -> Result<()> {
        loop {
            match self.provider.next() {
                Ok(FsEvent::NewFile(p)) => self.processor.process(p)?,
                Ok(FsEvent::Other) => println!("different event"),
                Ok(FsEvent::Stop) => break,
                Err(e) => eprint!("watch error: {:?}", e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{EventProcessor, FsEventDispatcher};
    use crate::provider::{EventProvider, FsEvent};
    use anyhow::Result;
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::sync::mpsc::RecvError;

    #[test]
    fn test_processor_executed_when_new_file_appeared() {
        // given
        let processor_spy = EventProcessorSpy::new();
        let events = vec![FsEvent::NewFile(PathBuf::from(r"/test")), FsEvent::Stop];
        let provider_stub = EventProviderStub::new(&events);
        // when
        FsEventDispatcher::new(&provider_stub, &processor_spy)
            .handle()
            .unwrap();
        // then
        assert_eq!(processor_spy.executed(), true);
    }

    #[test]
    fn test_processor_not_executed_when_no_new_file_event_occured() {
        // given
        let processor_spy = EventProcessorSpy::new();
        let stubbed_events = vec![FsEvent::Stop];
        let provider_stub = EventProviderStub::new(&stubbed_events);
        // when
        FsEventDispatcher::new(&provider_stub, &processor_spy)
            .handle()
            .unwrap();
        // then
        assert_eq!(processor_spy.executed(), false);
    }

    #[test]
    fn test_processor_not_executed_when_other_event_occured() {
        // given
        let processor_spy = EventProcessorSpy::new();
        let stubbed_events = vec![FsEvent::Other, FsEvent::Stop];
        let provider_stub = EventProviderStub::new(&stubbed_events);
        // when
        FsEventDispatcher::new(&provider_stub, &processor_spy)
            .handle()
            .unwrap();
        // then
        assert_eq!(processor_spy.executed(), false);
    }

    struct EventProviderStub {
        events: Vec<FsEvent>,
        current_event: RefCell<usize>,
    }

    impl EventProviderStub {
        fn new(events: &[FsEvent]) -> Self {
            if events.is_empty() {
                panic!("events should have at least stop event present");
            }
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
        fn process<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
            *self.executed.borrow_mut() = true;
            Ok(())
        }
    }
}
