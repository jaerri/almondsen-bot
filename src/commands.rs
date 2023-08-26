// Remember to publish command module for every command

pub mod hello;
pub mod ball;
pub mod ping;
pub mod addlogin;

pub type CommandResult<T = ()> = Result<T, anyhow::Error>;