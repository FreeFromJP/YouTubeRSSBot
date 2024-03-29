mod publish;
pub use publish::AsyncNotePublisher;
pub use publish::NotePublisher;
use sonic_rs::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    nostr: Nostr,
}

#[derive(Debug, Deserialize)]
struct Nostr {
    relays: Vec<String>,
}
