use notify::DebouncedEvent;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, RecvError};

pub(crate) trait EventProvider {
    fn next(&self) -> Result<Event, RecvError>;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Event {
    // for testing purposes
    #[allow(dead_code)]
    Stop,
    NewFile(PathBuf),
    Other,
}

pub(crate) struct DebouncedEventAdapter {
    receiver: Receiver<DebouncedEvent>,
}

impl DebouncedEventAdapter {
    pub(crate) fn new(receiver: Receiver<DebouncedEvent>) -> Self {
        DebouncedEventAdapter { receiver }
    }
}

impl EventProvider for DebouncedEventAdapter {
    fn next(&self) -> Result<Event, RecvError> {
        match self.receiver.recv() {
            Ok(DebouncedEvent::Create(p)) => Ok(Event::NewFile(p)),
            Ok(_) => Ok(Event::Other),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::provider::{DebouncedEventAdapter, Event, EventProvider};
    use notify::DebouncedEvent;
    use std::path::PathBuf;
    use std::sync::mpsc::channel;

    #[test]
    fn test_create_event_is_correctly_adapted_to_new_file_event() {
        // given
        let (sender, receiver) = channel();
        let provider = DebouncedEventAdapter::new(receiver);

        // when
        sender
            .send(DebouncedEvent::Create(PathBuf::from("/test")))
            .unwrap();

        // then
        assert_eq!(
            provider.next().unwrap(),
            Event::NewFile(PathBuf::from("/test")),
        );
    }

    #[test]
    fn test_any_other_event_is_converted_to_other_event() {
        // given
        let (sender, receiver) = channel();
        let provider = DebouncedEventAdapter::new(receiver);

        // when
        sender
            .send(DebouncedEvent::Chmod(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::Write(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::Remove(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::Rename(
                PathBuf::from("/test"),
                PathBuf::from("/test1"),
            ))
            .unwrap();
        sender
            .send(DebouncedEvent::NoticeWrite(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::NoticeRemove(PathBuf::from("/test")))
            .unwrap();
        sender.send(DebouncedEvent::Rescan).unwrap();

        // then
        assert_eq!(Event::Other, provider.next().unwrap());
        assert_eq!(Event::Other, provider.next().unwrap());
        assert_eq!(Event::Other, provider.next().unwrap());
        assert_eq!(Event::Other, provider.next().unwrap());
        assert_eq!(Event::Other, provider.next().unwrap());
        assert_eq!(Event::Other, provider.next().unwrap());
        assert_eq!(Event::Other, provider.next().unwrap());
    }
}
