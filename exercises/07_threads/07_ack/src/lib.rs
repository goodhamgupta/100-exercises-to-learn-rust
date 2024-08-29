use data::{Ticket, TicketDraft};
use std::sync::mpsc::{Receiver, Sender};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft: ticket_draft,
                sender,
            }) => {
                let ticket_id = store.add_ticket(ticket_draft);
                sender.send(ticket_id).unwrap();
            }
            Ok(Command::Get {
                id: ticket_id,
                sender,
            }) => {
                let ticket = store.get(ticket_id).cloned();
                sender.send(ticket).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
