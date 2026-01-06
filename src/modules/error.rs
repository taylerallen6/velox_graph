use std::io;
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VeloxGraphError {
    #[error("Failed to create/load database file: {0}")]
    FileFail(String),
    #[error("io Error")]
    IoError(#[from] io::Error),
    #[error("toml deserialize Error")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("toml serialize Error")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("postcard Error")]
    PostcardError(#[from] postcard::Error),
    #[error("Node ID overflow: value too large for usize")]
    Overflow(#[from] TryFromIntError),

    #[error("database: Empty_slots vector is empty")]
    EmptySlotsVectorIsEmpty,
    #[error("database: Slot {0} is not allocated. No data here. This means that slot >= nodes_vector.len(). You cannot access, update, or remove a slot that is not allocated.")]
    SlotNotAllocated(usize),
    #[error("database: Slot {0} is not used. No data here. Slot should already be in empty_slots vector. You cannot access, update, or remove a slot that is not in use.")]
    SlotNotUsed(usize),
    #[error("database: Connection {0} is not set. No data here.")]
    ConnectionNotSet(usize),

    #[error("unknown database error")]
    Unknown,
}
