use log::{debug, info};
mod models;
mod renderer;
use models::action::ActionType;
use models::field_of_play::FieldOfPlay;
use models::player::Player;
use models::stats::Stats;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use futures::future::join_all;
use futures::poll;

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
    num_turns: usize
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

    renderer::render_field(shared_field_of_play.clone());
}

// fn simulate(
//     mut field_of_play_cache: FieldOfPlay,
//     mut players: Vec<Player>,
//     wait_between_turn_ms: u64,
//     show_field: bool,
//     num_turns: usize
// ) {
//     let sleep_between_turn_dur = time::Duration::from_millis(wait_between_turn_ms);
//     let mut last_it_index = 0;
//     let mut turn_num = 0;
//     let mut stats = Stats::new(
//         &players,
//         num_turns,
//         field_of_play_cache.field[0].len(),
//         field_of_play_cache.field.len()
//     );
//     while turn_num < num_turns {
//         turn_num += 1;
//         players_take_action(&mut field_of_play_cache, &mut players, &mut last_it_index, &mut stats);
//
//         if show_field {
//             renderer::render_field(&field_of_play_cache, &players, turn_num);
//         }
//
//         thread::sleep(sleep_between_turn_dur);
//     }
//     stats.output_stats_about_players();
// }
//
// fn players_take_action(
//     field_of_play_cache: &mut FieldOfPlay,
//     players: &mut Vec<Player>,
//     last_it_index: &mut usize,
//     stats: &mut Stats
// ) {
//     let generic_action_panic_msg = "Invalid action param";
//
//     for player_index in 0..players.len() {
//         let player = players
//             .get_mut(player_index)
//             .expect("Invalid player index when attempting to take action.");
//         let player_name = player.name.to_owned();
//         stats.record_start_player_details(&player);
//         let (old_x, old_y) = player.get_location();
//         // We only set the last known it location here, instead of also when a new player is tagged
//         // to simulate a non-zero reaction time from the other players with regards to knowing who
//         // is it.
//         field_of_play_cache.set_last_known_it_location(old_x, old_y);
//         let actions = player.take_action(&field_of_play_cache, *last_it_index);
//         debug!(
//             "player at index: {} is acting. old_x: {}, old_y: {}, actions: {:?}",
//             player_index, old_x, old_y, actions
//         );
//         actions.into_iter().for_each(|action| match action.action {
//             ActionType::Move => {
//                 field_of_play_cache.field[action.y_coordinate.expect(generic_action_panic_msg)]
//                     [action.x_coordinate.expect(generic_action_panic_msg)] = Some(player_index);
//                 field_of_play_cache.field[old_y][old_x] = None;
//             }
//             ActionType::Tag => {
//                 let new_tagged_player = players
//                     .get_mut(action.new_it_index.expect(generic_action_panic_msg))
//                     .expect("Invalid player index when attempting to tag player");
//                 new_tagged_player.is_it = true;
//                 stats.record_new_it_details(new_tagged_player.name.to_owned());
//                 *last_it_index = player_index;
//                 info!("{} has tagged {}", player_name, new_tagged_player.name);
//             }
//         });
//     }
// }
