use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

use data::TicketDraft;
use store::TicketStore;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        let Ok(cmd) = receiver.recv() else {
            continue;
        };
        match cmd {
            Command::Insert(draft) => {
                store.add_ticket(draft);
            }
        }
    }
}
