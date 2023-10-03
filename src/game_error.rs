use std::fmt::{Debug, Display, Formatter};

pub struct GameError {}

impl Debug for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for GameError {

}

impl From<String> for GameError {
    fn from(value: String) -> Self {
        return GameError{};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_error_instantiation () {
        let a = GameError{};
    }

    #[test]
    fn instantiate_from_str () {
        let a = GameError::from(String::from("test game error"));
    }
}