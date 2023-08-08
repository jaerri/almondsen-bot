// Remember to publish command module for every command

pub mod hello;
pub mod ball;
pub mod ping;

pub type CommandResult<T = ()> = Result<T, serenity::Error>;