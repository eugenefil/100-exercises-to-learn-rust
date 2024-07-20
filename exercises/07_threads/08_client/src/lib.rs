use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{self, Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    server_channel: Sender<Command>,
}

impl TicketStoreClient {
    pub fn new(server_channel: Sender<Command>) -> TicketStoreClient {
        TicketStoreClient { server_channel }
    }

    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (sender, receiver) = mpsc::channel();
        let cmd = Command::Insert {
            draft,
            response_channel: sender,
        };
        self.server_channel.send(cmd).expect("server inaccessible");
        receiver.recv().expect("server inaccessible")
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (sender, receiver) = mpsc::channel();
        let cmd = Command::Get {
            id,
            response_channel: sender,
        };
        self.server_channel.send(cmd).expect("server inaccessible");
        receiver.recv().expect("server inaccessible")
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender)
}

// No longer public! This becomes an internal detail of the library now.
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
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
