use super::{point::Point, state::GameState, player_direction::PlayerDirection};



pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
}

impl GameContext{ 
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3,1), Point(2,1), Point(1,1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3,3),
        }
    }

    pub fn next_tick(&mut self) {

        if let GameState::Paused = self.state {
            return;
        }

        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };
            
            self.player_position.pop();
            self.player_position.reverse();
            self.player_position.push(next_head_position);
            self.player_position.reverse();
    }

    pub fn move_up(&mut self) {
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        self.player_direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        self.player_direction = PlayerDirection::Left;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing
        }
        
    }
    
}