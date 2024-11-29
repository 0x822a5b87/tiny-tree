#![allow(dead_code, unused_variables, unused_mut, unused_assignments, unused_imports)]

use crate::coordinator::{Coordinator, ParticipantStatus};
use std::cell::RefCell;
use std::rc::Weak;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Debug)]
pub struct Participant<'a> {
    pub(crate) coordinator: Option<Weak<RefCell<Coordinator<'a>>>>,
    pub(crate) id: u8,
    // Indicates whether the current participant has completed the transaction.
    // It is set to false if any errors are encountered, otherwise true.
    pub(crate) data: bool,
    pub(crate) status: ParticipantStatus,
    pub(crate) sender: UnboundedSender<ParticipantStatus>, // coordinator use sender send message to me
    pub(crate) receiver: UnboundedReceiver<ParticipantStatus>, // consume message from sender
}


impl<'a> Participant<'a> {
    pub fn new(id: u8) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Participant {
            id,
            sender,
            receiver,
            data: false,
            status: ParticipantStatus::Ready(id),
            coordinator: None,
        }
    }

    pub fn participant_recv(&mut self) {
        let x = self.receiver.try_recv();
        match x {
            Ok(status) => self.handle_status(status),
            Err(e) => self.handle_recv_error(e)
        }
    }

    pub fn transaction_end(&self) -> bool {
        match self.status {
            ParticipantStatus::Failure(_) => true,
            ParticipantStatus::Commited(_) => true,
            ParticipantStatus::Rollback(_) => true,
            _ => false,
        }
    }

    fn handle_recv_error(&self, e: TryRecvError) {
        match e {
            TryRecvError::Empty => {}
            TryRecvError::Disconnected => { panic!("{:?}", e) }
        }
    }

    fn handle_status(&mut self, status: ParticipantStatus) {
        match status {
            ParticipantStatus::Ready(id) => self.handle_ready(),
            ParticipantStatus::Prepared(id) => self.handle_prepared(),
            ParticipantStatus::Commited(id) => self.handle_commited(),
            ParticipantStatus::Failure(id) => self.handle_failure(),
            ParticipantStatus::Rollback(id) => self.handle_roll_back(),
        }
    }

    fn handle_ready(&self) {
        panic!("error new status [ready] for : {}", self.id)
    }

    fn handle_failure(&mut self) {
        self.status = ParticipantStatus::Failure(self.id);
    }

    fn handle_prepared(&mut self) {
        if crate::coordinator::Network::network_error() {
            return;
        }
        self.status = ParticipantStatus::Prepared(self.id);
        if let Some(coordinator_weak) = &self.coordinator {
            if let Some(coordinator_rc) = coordinator_weak.upgrade() {
                let coordinator_ref = coordinator_rc.borrow();
                coordinator_ref.async_send(ParticipantStatus::Prepared(self.id));
            }
        }
    }

    fn handle_commited(&mut self) {
        if crate::coordinator::Network::network_error() {
            return;
        }
        self.data = true;
        self.status = ParticipantStatus::Commited(self.id);
        if let Some(coordinator_weak) = &self.coordinator {
            if let Some(coordinator_rc) = coordinator_weak.upgrade() {
                let coordinator_ref = coordinator_rc.borrow();
                coordinator_ref.async_send(ParticipantStatus::Commited(self.id));
            }
        }
    }

    fn handle_roll_back(&mut self) {
        self.data = false;
        self.status = ParticipantStatus::Rollback(self.id);
    }


    pub(crate) fn prepare(&self) {
        let _ = self.sender.send(ParticipantStatus::Prepared(self.id));
    }

    // This method don't produce errors, because we assume the coordinator is responsible for
    // retry when this method produce an error as if it doesn't produce an error.
    pub(crate) fn undo_prepare(&self) {}

    pub(crate) fn fail(&mut self) {
        let _ = self.sender.send(ParticipantStatus::Failure(self.id));
    }

    pub(crate) fn commit(&mut self) {
        let _ = self.sender.send(ParticipantStatus::Commited(self.id));
    }

    pub(crate) fn rollback(&mut self) {
        let _ = self.sender.send(ParticipantStatus::Rollback(self.id));
    }
}

impl<'a> Drop for Participant<'a> {
    fn drop(&mut self) {
        let _ = self.sender.closed();
        self.receiver.close();
    }
}
