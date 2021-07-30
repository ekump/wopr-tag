use log::{info};
mod models;
mod renderer;
use models::field_of_play::FieldOfPlay;
use models::player::Player;
use std::sync::{Arc, Mutex};

pub type SharedFieldOfPlay = Arc<Mutex<FieldOfPlay>>;

// If this were a real project we would test the actual simulation somehow. But that would eat up
// quite a bit of time.
#[tokio::main]
pub async fn init(
    num_players: usize,
    x_axis_len: usize,
    y_axis_len: usize,
    wait_between_turn_ms: u64,
    show_field: bool,
    _num_turns: usize
) {
    info!(
        "Initalizing game with num players: {}, x-axis size: {}, y-axis size: {}",
        num_players, x_axis_len, y_axis_len
    );

    let field_of_play: FieldOfPlay = FieldOfPlay::new(x_axis_len as usize, y_axis_len as usize);

    let shared_field_of_play = Arc::new(Mutex::new(field_of_play));
    (0..num_players).for_each(|player_num| {
        let is_it = player_num == 0;
        let mut player = Player::new(player_num, is_it, shared_field_of_play.clone());
        tokio::spawn(async move {
            player.simulate().await;
        });
    });

    if show_field {
        renderer::render_field(shared_field_of_play.clone(), wait_between_turn_ms);
    }
}
