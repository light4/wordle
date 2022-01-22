use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Not enough letters")]
    NotEnoughLetters,
    #[error("Not in word list")]
    NotInWordList,
}
