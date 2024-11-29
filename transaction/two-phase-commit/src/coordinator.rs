#![allow(dead_code, unused_variables, unused_mut, unused_assignments, unused_imports)]

use crate::coordinator::CoordinatorStatus::{Failure, Rollback, Success};
use crate::participant::Participant;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CoordinatorStatus {
    Ready,
    Failure,
    Rollback,
    WaitPrepared,
    ReadyToCommit,
    WaitCommitted,
    Success,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParticipantStatus {
    Ready(u8),
    Failure(u8),
    Prepared(u8),
    Commited(u8),
    Rollback(u8),
}

#[derive(Debug)]
pub struct Coordinator<'a> {
    pub participants: Vec<Rc<RefCell<Participant<'a>>>>,
    pub status: CoordinatorStatus,
    sender: UnboundedSender<ParticipantStatus>, // participants use sender send message to me
    receiver: UnboundedReceiver<ParticipantStatus>, // consume message from sender
    timestamp: u128,
}

impl<'a> Coordinator<'a> {
    pub fn new() -> Self {
        let mut participants: Vec<Rc<RefCell<Participant>>> = vec![];
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Coordinator {
            status: CoordinatorStatus::Ready,
            timestamp: Util::now(),
            participants,
            sender,
            receiver,
        }
    }

    pub fn start_transaction(&mut self) {
        self.status = CoordinatorStatus::WaitPrepared;
        for participant in &mut self.participants {
            participant.borrow_mut().prepare()
        }
    }

    pub fn async_send(&self, msg: ParticipantStatus) {
        match self.sender.send(msg) {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    pub fn transaction_end(&self) -> bool {
        match self.status {
            Failure => true,
            Success => true,
            Rollback => true,
            _ => false,
        }
    }

    pub fn recv(&mut self) {
        if !Util::timeout(self.timestamp) {
            let x = self.receiver.try_recv();
            match x {
                Ok(status) => self.handle_recv(status),
                Err(e) => self.handle_recv_error(e),
            }
        } else {
            self.handle_timeout();
        }
    }

    fn handle_timeout(&mut self) {
        match &self.status {
            CoordinatorStatus::WaitPrepared => {
                self.handle_timeout_prepared();
                self.status = Failure;
            }
            CoordinatorStatus::WaitCommitted => {
                self.handle_timeout_committed();
                self.status = Rollback;
            }
            _ => {
                panic!("unexpected coordinator status: {:?}", self.status);
            }
        }
    }

    fn handle_timeout_prepared(&mut self) {
        for participant in &mut self.participants {
            let _ = participant.borrow_mut().fail();
        }
    }

    fn handle_timeout_committed(&mut self) {
        for participant in &mut self.participants {
            let mut participant = participant.borrow_mut();
            let _ = participant.rollback();
        }
    }

    fn handle_recv(&mut self, status: ParticipantStatus) {
        match status {
            ParticipantStatus::Ready(_) => panic!("unexpected participant status: {:?}", status),
            ParticipantStatus::Failure(_) => self.handle_participant_failure(),
            ParticipantStatus::Prepared(_) => self.commit_all(),
            ParticipantStatus::Commited(_) => self.try_success(),
            ParticipantStatus::Rollback(_) => self.rollback(),
        }
    }

    fn handle_recv_error(&self, e: TryRecvError) {
        match e {
            TryRecvError::Empty => {}
            TryRecvError::Disconnected => { panic!("{:?}", e) }
        }
    }

    fn handle_participant_failure(&mut self) {
        for participant in &mut self.participants {
            let _ = participant.borrow_mut().fail();
        }
    }

    fn rollback(&mut self) {
        for participant in &mut self.participants {
            let _ = participant.borrow().sender.send(ParticipantStatus::Rollback(participant.borrow().id));
        }
    }

    fn try_success(&mut self) {

        for participant in &mut self.participants {
            if participant.borrow().status != ParticipantStatus::Commited(participant.borrow().id) {
                return;
            }
        }
        self.status = Success;
    }

    fn commit_all(&mut self) {
        for participant in &mut self.participants {
            if participant.borrow().status != ParticipantStatus::Prepared(participant.borrow().id) {
                return;
            }
        }

        self.status = CoordinatorStatus::WaitCommitted;
        for participant in &mut self.participants {
            participant.borrow_mut().commit();
        }
    }

    fn rollback_all(&mut self) {}
}

impl<'a> Drop for Coordinator<'a> {
    fn drop(&mut self) {
        // TODO It is necessary to check the status of each participant and set correct status for each participant.
    }
}


// Network used to simulate a network error when communicate between coordinator and participant
pub(crate) struct Network {
    distribution: rand::distributions::Bernoulli,
}

impl Network {
    pub(crate) fn network_error() -> bool {
        use rand::distributions::Distribution;
        let mut rng = rand::thread_rng();
        let distribution = rand::distributions::Bernoulli::new(0.2).unwrap();
        distribution.sample(&mut rng)
    }
}

pub(crate) struct Util {}

impl Util {
    const MILLISECONDS_FIVE_SEC: u128 = 1000;

    fn now() -> u128 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_millis()
    }

    fn timeout(timestamp: u128) -> bool {
        Self::now() - timestamp > Self::MILLISECONDS_FIVE_SEC
    }
}
#[cfg(test)]
mod tests {
    use crate::coordinator::{Coordinator, CoordinatorStatus, Participant, ParticipantStatus};
    use std::cell::{RefCell, RefMut};
    use std::rc::Rc;
    use std::thread;

    #[test]
    pub fn batch_random_test() {
        let mut handles = vec![];

        for _ in 0..1000 {
            let handle = thread::spawn(|| {
                random_test();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn random_test() {
        let coordinator = Rc::new(RefCell::new(Coordinator::new()));
        for id in 0..10 {
            let participant = Rc::new(RefCell::new(Participant::new(id)));
            participant.borrow_mut().coordinator = Some(Rc::downgrade(&coordinator));
            coordinator.borrow_mut().participants.push(participant.clone());
        }
        coordinator.borrow_mut().start_transaction();
        while !coordinator.borrow().transaction_end() {
            for x in &coordinator.borrow().participants {
                x.borrow_mut().participant_recv()
            }
            coordinator.borrow_mut().recv();
        }

        loop {
            let mut end = true;
            for x in &coordinator.borrow().participants {
                x.borrow_mut().participant_recv();
                if !x.borrow().transaction_end() {
                    end = false;
                }
            }
            if end {
                break;
            }
        }

        assert_result(coordinator)
    }

    fn assert_result(coordinator: Rc<RefCell<Coordinator>>) {
        let coordinator = coordinator.borrow_mut();
        match coordinator.status {
            CoordinatorStatus::Failure => assert_fail(coordinator),
            CoordinatorStatus::Success => assert_success(coordinator),
            CoordinatorStatus::Rollback => assert_rollback(coordinator),
            _ => {
                panic!("expected failure status");
            }
        }
    }

    fn assert_success(coordinator: RefMut<Coordinator>) {
        assert!(matches!(coordinator.status, CoordinatorStatus::Success));
        for participant in &coordinator.participants {
            let id = participant.borrow().id;
            assert_eq!(participant.borrow().status, ParticipantStatus::Commited(id));
            assert!(participant.borrow().data)
        }
    }

    fn assert_fail(coordinator: RefMut<Coordinator>) {
        assert!(matches!(coordinator.status, CoordinatorStatus::Failure));
        for participant in &coordinator.participants {
            let id = participant.borrow().id;
            assert_eq!(participant.borrow().status, ParticipantStatus::Failure(id));
            assert!(!participant.borrow().data)
        }
    }

    fn assert_rollback(coordinator: RefMut<Coordinator>) {
        assert!(matches!(coordinator.status, CoordinatorStatus::Rollback));
        for participant in &coordinator.participants {
            let id = participant.borrow().id;
            assert_eq!(participant.borrow().status, ParticipantStatus::Rollback(id));
            assert!(!participant.borrow().data)
        }
    }
}
