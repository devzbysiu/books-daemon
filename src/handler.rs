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
