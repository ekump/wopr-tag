use log::{debug, info};
mod models;
mod renderer;
use models::action::ActionType;
use models::field_of_play::FieldOfPlay;
use models::player::Player;
use std::{thread, time};

// If this were a real project we would test the actual simulation somehow. But that would eat up
// quite a bit of time.
pub fn init(
    num_players: usize,
    x_axis_len: usize,
    y_axis_len: usize,
    wait_between_turn_ms: u64,
    show_field: bool,
    num_turns: usize
) {
    info!(
        "Initalizing game with num players: {}, x-axis size: {}, y-axis size: {}",
        num_players, x_axis_len, y_axis_len
    );

    let mut field_of_play: FieldOfPlay = FieldOfPlay::new(x_axis_len as usize, y_axis_len as usize);
    let mut players: Vec<Player> = Vec::new();

    (0..num_players).for_each(|player_num| {
        let is_it = player_num == 0;
        let player = Player::new(player_num, is_it, &mut field_of_play);

        players.push(player);
    });

    simulate(field_of_play, players, wait_between_turn_ms, show_field, num_turns);
}

fn simulate(
    mut field_of_play_cache: FieldOfPlay,
    mut players: Vec<Player>,
    wait_between_turn_ms: u64,
    show_field: bool,
    num_turns: usize
) {
    let sleep_between_turn_dur = time::Duration::from_millis(wait_between_turn_ms);
    let mut last_it_index = 0;
    let mut turn_num = 0;
    while turn_num < num_turns {
        turn_num += 1;
        players_take_action(&mut field_of_play_cache, &mut players, &mut last_it_index);

        if show_field {
            renderer::render_field(&field_of_play_cache, &players, turn_num);
        }

        thread::sleep(sleep_between_turn_dur);
    }
}

fn players_take_action(field_of_play_cache: &mut FieldOfPlay, players: &mut Vec<Player>, last_it_index: &mut usize) {
    let generic_action_panic_msg = "Invalid action param";

    for player_index in 0..players.len() {
        let player = players
            .get_mut(player_index)
            .expect("Invalid player index when attempting to take action.");
        let player_name = player.name.to_owned();
        let (old_x, old_y) = player.get_location();
        // We only set the last known it location here, instead of also when a new player is tagged
        // to simulate a non-zero reaction time from the other players with regards to knowing who
        // is it.
        field_of_play_cache.set_last_known_it_location(old_x, old_y);
        let actions = player.take_action(&field_of_play_cache, *last_it_index);
        debug!(
            "player at index: {} is acting. old_x: {}, old_y: {}, actions: {:?}",
            player_index, old_x, old_y, actions
        );
        actions.into_iter().for_each(|action| match action.action {
            ActionType::Move => {
                field_of_play_cache.field[action.y_coordinate.expect(generic_action_panic_msg)]
                    [action.x_coordinate.expect(generic_action_panic_msg)] = Some(player_index);
                field_of_play_cache.field[old_y][old_x] = None;
            }
            ActionType::Tag => {
                let new_tagged_player = players
                    .get_mut(action.new_it_index.expect(generic_action_panic_msg))
                    .expect("Invalid player index when attempting to tag player");
                new_tagged_player.is_it = true;
                *last_it_index = player_index;
                info!("{} has tagged {}", player_name, new_tagged_player.name);
            }
        });
    }
}
