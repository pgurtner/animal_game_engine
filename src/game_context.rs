pub struct GameContext {
}

impl Default for GameContext {
    fn default () -> Self {
        return GameContext {};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_instantiation () {
        let gctx: GameContext = Default::default();
    }
}