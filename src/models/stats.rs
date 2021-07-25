use super::player::Player;
use std::collections::HashMap;

static GENERIC_VEC_ACCESS_PANIC_ERR_MSG: &str = "Invalid player name key for stats.";

pub struct Stats {
    field_x_len: usize,
    field_y_len: usize,
    number_of_turns: usize,
    player_stats: HashMap<String, PlayerStats>
}

#[derive(Debug)]
struct PlayerStats {
    risk_tolerance: f64,
    rounds_started_as_it: usize,
    rounds_made_it: usize
}

impl Stats {
    pub fn new(players: &Vec<Player>, number_of_turns: usize, field_x_len: usize, field_y_len: usize) -> Self {
        let mut player_stats = HashMap::new();
        players.iter().for_each(|player| {
            player_stats.insert(
                player.name.to_owned(),
                PlayerStats {
                    risk_tolerance: player.get_risk_tolerance(),
                    rounds_started_as_it: 0,
                    rounds_made_it: 0
                }
            );
        });

        Stats {
            field_x_len,
            field_y_len,
            player_stats,
            number_of_turns
        }
    }

    pub fn record_start_player_details(&mut self, player: &Player) {
        let player_stats = self
            .player_stats
            .get_mut(&player.name)
            .expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
        if player.is_it {
            player_stats.rounds_started_as_it += 1;
        }
    }

    pub fn record_new_it_details(&mut self, name: String) {
        let player_stats = self
            .player_stats
            .get_mut(&name)
            .expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
        player_stats.rounds_made_it += 1;
    }

    pub fn output_stats_about_players(&self) {
        println!(
            "{} players played for {} turns on a field {} by {} large",
            self.player_stats.keys().len(),
            self.number_of_turns,
            self.field_x_len,
            self.field_y_len
        );
        self.player_stats.iter().for_each(|(name, stats_for_player)| {
            println!("{}: {:?}", name, stats_for_player);
        });
    }
}
