use super::game_error::GameError;
use super::game_context::GameContext;

pub type GameStepResult = Result<(), GameError>;
pub type GameStepper = fn(GameContext) -> GameStepResult;


pub struct Game {
    update: GameStepper,
    draw: GameStepper,
    context: GameContext
}


impl Game {
    pub fn new (update: GameStepper, draw: GameStepper) -> Self {
        return Game {
            update,
            draw,
            context: Default::default()
        }
    }

    pub fn run (self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_step_result_instantiation () {
        let a: GameStepResult = Ok(());
        let game_error = GameError::from(String::from("test game error"));
        let b: GameStepResult = Err(game_error);
    }
}