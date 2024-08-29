// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::io::Error;
use std::sync::mpsc::{Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = self.sender.send(Command::Insert {
            draft,
            response_channel: tx,
        });
        let result = rx.recv();
        match result {
            Ok(ticket_id) => Result::Ok(ticket_id),
            Err(_) => panic!("Ticket could not be created"),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = self.sender.send(Command::Get {
            id,
            response_channel: tx,
        });
        let result = rx.recv();
        match result {
            Ok(ticket) => Result::Ok(ticket),
            Err(_) => panic!("Ticket could not be created"),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
