use crate::gamestates::{game_state::GameState, player_data::PlayerData};

pub trait RewardFn {
    fn reset(&mut self, initial_state: &GameState, reward_stage: Option<usize>);
    fn pre_step(&mut self, _state: &GameState) {}
    fn get_reward(&mut self, player: &PlayerData, state: &GameState) -> f32;
    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState) -> f32;
}
