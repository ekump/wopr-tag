use log::{debug, info};
pub fn init(num_players: usize, x_axis_len: usize, y_axis_len: usize, wait_between_turn_ms: u64, show_field: bool) {
    info!(
        "Initalizing game with num players: {}, x-axis size: {}, y-axis size: {}",
        num_players, x_axis_len, y_axis_len
    );
}
