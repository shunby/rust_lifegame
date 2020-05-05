use super::game::Game;
pub struct AnimationLoop<'a> {
    last_update_ms: i64,
    game: &'a Game
}

impl<'a> AnimationLoop<'a>{
    pub fn new(game: &'a Game) -> AnimationLoop<'a>{
        AnimationLoop{last_update_ms: 0, game: game}
    }
    pub fn update(&mut self){
        
    }
}