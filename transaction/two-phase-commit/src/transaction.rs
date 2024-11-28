#![allow(dead_code, unused_variables, unused_mut, unused_assignments, unused_imports)]

use async_channel::{Receiver, Sender};
use std::sync::Arc;
// pub enum CoordinatorCommand {
//     Prepare,
//     Commit,
//     Rollback,
// }
//
// pub enum ParticipantCommand {
//     Prepared,
//     Committed,
//
// }

enum CoordinatorStatus {
    Ready,
    Failure,
    WaitPrepared,
    ReadyToCommit,
    WaitCommitted,
    Success,
}

#[derive(Debug)]
pub enum ParticipantStatus {
    Ready(u8),
    Failure(u8),
    Prepared(u8),
    Commited(u8),
    RollBack(u8),
}

pub struct Coordinator<'a> {
    pub participants: Vec<Arc<Participant<'a>>>,
    num_of_participants: u8,
    status: CoordinatorStatus,
    sender: &'a Sender<ParticipantStatus>, // participants use sender send message to me
    receiver: &'a Receiver<ParticipantStatus>, // consume message from sender
}

impl<'a> Coordinator<'a> {
    pub fn new(num_of_participants: u8, sender: &'a Sender<ParticipantStatus>, receiver: &'a Receiver<ParticipantStatus>) -> Self {
        let mut participants: Vec<Arc<Participant>> = vec![];
        for id in 0..num_of_participants {
            let participant = Participant::new(id, &sender);
            participants.push(Arc::new(participant));
        }
        Coordinator {
            status: CoordinatorStatus::Ready,
            num_of_participants,
            participants,
            sender,
            receiver,
        }
    }

    pub fn transaction(&mut self) -> bool {
        let prepared = self.prepare_all();
        if !prepared {
            self.undo_prepare_all();
            return false;
        }

        true
    }

    pub fn async_send(&self, msg: ParticipantStatus) {
        let _ = self.sender.send(msg);
    }

    pub fn transaction_end(&self) -> bool {
        match self.status {
            CoordinatorStatus::Failure => true,
            CoordinatorStatus::Success => true,
            _ => false,
        }
    }

    fn prepare_all(&mut self) -> bool {
        for participant in &mut self.participants {
            {
                // TODO delete comment
                if participant.id == 5 {
                    continue
                }
            }
            if !participant.prepare() {
                return false;
            }
        }
        true
    }

    fn undo_prepare_all(&mut self) {
        for participant in &mut self.participants {
            participant.undo_prepare()
        }
    }

    fn commit_all(&mut self) {}

    fn rollback_all(&mut self) {}
}

impl<'a> Drop for Coordinator<'a> {
    fn drop(&mut self) {
        // TODO It is necessary to check the status of each participant and set correct status for each participant.
        self.sender.close();
        self.receiver.close();
    }
}

#[derive(Debug)]
pub struct Participant<'a> {
    id: u8,
    // Indicates whether the current participant has completed the transaction.
    // It is set to false if any errors are encountered, otherwise true.
    data: bool,
    status: ParticipantStatus,
    coordinator: &'a Sender<ParticipantStatus>,
    sender: Sender<ParticipantStatus>, // coordinator use sender send message to me
    receiver: Receiver<ParticipantStatus>, // consume message from sender
}

impl<'a> Participant<'a> {
    fn new(id: u8, coordinator: &'a Sender<ParticipantStatus>) -> Self {
        let (sender, receiver) = async_channel::unbounded();
        Participant {
            id,
            sender,
            receiver,
            data: false,
            status: ParticipantStatus::Ready(id),
            coordinator,
        }
    }

    pub async fn recv(&self) {
        if let Ok(msg) = self.receiver.recv().await {
            match &msg {
                ParticipantStatus::Ready(r) => self.handle_ready(),
                ParticipantStatus::Failure(_) => self.handle_failure(),
                ParticipantStatus::Prepared(_) => self.handle_prepared(),
                ParticipantStatus::Commited(_) => self.handle_commited(),
                ParticipantStatus::RollBack(_) => self.handle_roll_back(),
            }
        }
    }

    fn handle_ready(&self) {
        panic!("error new status [ready] for : {}", self.id)
    }

    fn handle_failure(&self) {
        println!("failure : {}", self.id)
    }

    fn handle_prepared(&self) {
        println!("prepared : {}", self.id)
    }

    fn handle_commited(&self) {
        println!("committed : {}", self.id)
    }

    fn handle_roll_back(&self) {
        println!("roll back : {}", self.id)
    }

    fn prepare(&self) -> bool {
        let _ = self.sender.send_blocking(ParticipantStatus::Prepared(self.id));
        true
    }

    // This method don't produce errors, because we assume the coordinator is responsible for
    // retry when this method produce an error as if it doesn't produce an error.
    fn undo_prepare(&self) {}

    fn commit(&mut self) -> bool {
        self.data = true;
        self.status = ParticipantStatus::Commited(self.id);
        true
    }

    fn rollback(&mut self) -> bool {
        self.data = false;
        self.status = ParticipantStatus::RollBack(self.id);
        true
    }
}

impl<'a> Drop for Participant<'a> {
    fn drop(&mut self) {
        self.sender.close();
        self.receiver.close();
    }
}


// Network used to simulate a network error when communicate between coordinator and participant
struct Network {
    distribution: rand::distributions::Bernoulli,
}

impl Network {
    fn network_error() -> bool {
        use rand::distributions::Distribution;
        let mut rng = rand::thread_rng();
        let distribution = rand::distributions::Bernoulli::new(0.2).unwrap();
        distribution.sample(&mut rng)
    }
}

