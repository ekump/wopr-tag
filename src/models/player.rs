use super::action::Action;
use super::direction::Direction;
use super::field_of_play::FieldOfPlay;
use crate::SharedFieldOfPlay;
use log::{debug, error, info};
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub is_it: bool,
    pub name: String,
    x_coordinate: usize,
    y_coordinate: usize,
    pub risk_tolerance: f64,
    field_of_play: SharedFieldOfPlay,
    speed: u64 // denotes how long we sleep between taking actions
}

impl Player {
    pub fn new(index: usize, is_it: bool, field_of_play: SharedFieldOfPlay) -> Self {
        // Since we are calling init_pos right away it's likely safe to init the positions to 0
        // instead of using Option<usize>.
        let mut rng = rand::thread_rng();
        let name = format!("p{}", index);
        let mut risk_tolerance: f64 = rng.gen();
        risk_tolerance = risk_tolerance * 100.0;
        let rand_sleep = rng.gen_range(0..100);
        let speed = 200 - rand_sleep;
        let mut player = Player {
            name,
            id: index,
            is_it,
            x_coordinate: 0,
            y_coordinate: 0,
            risk_tolerance,
            field_of_play,
            speed
        };

        player.init_position();

        player
    }

    // Looks at the field of play and returns a starting position. It sets the position for the
    // player but the [FieldOfPlay] must be updated with the returned coordinates.
    fn init_position(&mut self) {
        let mut field_of_play = self.field_of_play.lock().expect("TODO");
        let y_len = field_of_play.field.len();
        let x_len = field_of_play.field[0].len();
        let mut rng = thread_rng();
        let mut found_pos = false;
        let id = self.id;
        while !found_pos {
            let rand_x = rng.gen_range(0..x_len);
            let rand_y = rng.gen_range(0..y_len);
            debug!("checking if x: {}, y: {} is available", rand_x, rand_y);
            found_pos = field_of_play.field[rand_y][rand_x].is_none();
            if found_pos {
                field_of_play.field[rand_y][rand_x] = Some(id);
                self.x_coordinate = rand_x;
                self.y_coordinate = rand_y;
                if self.is_it {
                    field_of_play.set_last_known_it_location(rand_x, rand_y);
                }
            }
        }
    }

    pub async fn simulate(&mut self) {
        loop {
            debug!("Player: {} is taking action", self.name);
            tokio::time::sleep(tokio::time::Duration::from_millis(self.speed)).await;
            // TODO: Fix this
            {
                let mut field_of_play = self.field_of_play.lock().expect("TODO");
                let (old_x, old_y) = self.get_location();

                if field_of_play.last_known_it_id == self.id && !self.is_it {
                    self.is_it = true;
                }

                if self.is_it {
                    if let Some(newly_tagged_id) = self.get_taggable_player(&field_of_play) {
                        self.is_it = false;
                        field_of_play.last_known_it_id = newly_tagged_id;
                        field_of_play.prev_it_id = self.id;
                        info!("{} is now it", newly_tagged_id);
                    }
                }

                // TODO: temp
                let action = self.take_move_action(&field_of_play);
                self.x_coordinate = action.x_coordinate.expect("TODO");
                self.y_coordinate = action.y_coordinate.expect("TODO");
                field_of_play.field[old_y][old_x] = None;
                field_of_play.field[self.y_coordinate][self.x_coordinate] = Some(self.id);
            }
        }
    }

    fn get_taggable_player(&self, field_of_play: &FieldOfPlay) -> Option<usize> {
        let adjacent_players = field_of_play.get_adjacent_player_indices(self.x_coordinate, self.y_coordinate);
        let taggable_players = adjacent_players.into_iter().find(|ap| *ap != field_of_play.last_known_it_id);
        debug!("Taggable player indices adjacent to {:?}: {:?}", self, taggable_players);

        taggable_players
    }

    fn take_move_action(&self, field_of_play: &FieldOfPlay) -> Action {
        let (x, y) = self.move_to_empty_position(field_of_play);
        // self.set_location(x, y);

        Action::new_move(x, y)
    }

    fn set_location(&mut self, x: usize, y: usize) {
        self.x_coordinate = x;
        self.y_coordinate = y;
    }

    pub fn get_location(&self) -> (usize, usize) {
        (self.x_coordinate, self.y_coordinate)
    }

    fn move_to_empty_position(&self, field_of_play: &FieldOfPlay) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let mut found_location = false;
        let mut retries_remaining = 1000;
        let mut x_coordinate = self.x_coordinate;
        let mut y_coordinate = self.y_coordinate;

        while !found_location {
            let rand_direction = rng.gen_range(0..8);
            match rand_direction {
                0 => {
                    // WEST
                    if field_of_play.is_position_valid_and_empty(Direction::West, self.x_coordinate, self.y_coordinate)
                    {
                        x_coordinate = self.x_coordinate - 1;
                        found_location = true;
                    }
                }
                1 => {
                    // EAST
                    if field_of_play.is_position_valid_and_empty(Direction::East, self.x_coordinate, self.y_coordinate)
                    {
                        x_coordinate = self.x_coordinate + 1;
                        found_location = true;
                    }
                }
                2 => {
                    // NORTH
                    if field_of_play.is_position_valid_and_empty(Direction::North, self.x_coordinate, self.y_coordinate)
                    {
                        y_coordinate = self.y_coordinate - 1;
                        found_location = true;
                    }
                }
                3 => {
                    // SOUTH
                    if field_of_play.is_position_valid_and_empty(Direction::South, self.x_coordinate, self.y_coordinate)
                    {
                        y_coordinate = self.y_coordinate + 1;
                        found_location = true;
                    }
                }
                4 => {
                    // NORTHWEST
                    if field_of_play.is_position_valid_and_empty(
                        Direction::NorthWest,
                        self.x_coordinate,
                        self.y_coordinate
                    ) {
                        y_coordinate = self.y_coordinate - 1;
                        x_coordinate = self.x_coordinate - 1;
                        found_location = true;
                    }
                }
                5 => {
                    // NORTHEAST
                    if field_of_play.is_position_valid_and_empty(
                        Direction::NorthEast,
                        self.x_coordinate,
                        self.y_coordinate
                    ) {
                        y_coordinate = self.y_coordinate - 1;
                        x_coordinate = self.x_coordinate + 1;
                        found_location = true;
                    }
                }
                6 => {
                    // SOUTHWEST
                    if field_of_play.is_position_valid_and_empty(
                        Direction::SouthWest,
                        self.x_coordinate,
                        self.y_coordinate
                    ) {
                        y_coordinate = self.y_coordinate + 1;
                        x_coordinate = self.x_coordinate - 1;
                        found_location = true;
                    }
                }
                7 => {
                    //SOUTHEAST
                    if field_of_play.is_position_valid_and_empty(
                        Direction::SouthEast,
                        self.x_coordinate,
                        self.y_coordinate
                    ) {
                        y_coordinate = self.y_coordinate + 1;
                        x_coordinate = self.x_coordinate + 1;
                        found_location = true;
                    }
                }
                _ => unreachable!()
            };

            // It is better to move than get stuck because we do not want to get closer to the it
            // player. So we only try to find a new position that isn't closer to the it player if
            // we have at least 100 retries remaining.
            if self.is_new_coordinates_too_close_to_it_player(x_coordinate, y_coordinate, &field_of_play)
                && retries_remaining > 100
            {
                found_location = false;
            }

            // It is possible that players will get bunched up at the edge of the field and the
            // current player has nowhere to go. If this is the case then skip this player and let
            // the next one try to move.
            retries_remaining += -1;
            if retries_remaining <= 0 {
                error!("Player: {} is stuck", self.name);
                found_location = true;
            }
        }

        (x_coordinate, y_coordinate)
    }

    // distance between two sets of points is d=sqrt(x2-x1)^2 + (y2-y1)^2. If the x and y input are
    // closer to the it player than the player's current coordinates we return false. Using floats
    // for calc instead of and arb precision type, like BigDecimal means we may have slightly less
    // reliable results, but an argument can be made that this makes the agents more human-like.
    fn is_new_coordinates_too_close_to_it_player(&self, x: usize, y: usize, field_of_play: &FieldOfPlay) -> bool {
        let it_coordinates = field_of_play.get_last_known_it_location();
        if let Some((it_x, it_y)) = it_coordinates {
            let new_distance = (((it_x - x).pow(2) + (it_y - y).pow(2)) as f64).sqrt();
            let current_distance =
                (((it_x - self.x_coordinate).pow(2) + (it_y - self.y_coordinate).pow(2)) as f64).sqrt();
            if new_distance < current_distance {
                return self.is_new_distance_outside_risk_tolerance(new_distance, current_distance);
            }
        }

        false
    }

    // players with greater risk tolerance take more risks. They're willing to move closer to the
    // it player.
    fn is_new_distance_outside_risk_tolerance(&self, new_distance: f64, current_distance: f64) -> bool {
        let pct_chg = ((new_distance - current_distance).abs() / current_distance) * 100.0;
        if pct_chg > self.risk_tolerance {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
use super::action::ActionType;

#[test]
// Creating a new player involves randomly placing the player on the field of play. If we wanted to
// make this test faster / more deterministic and realisitc we could wrap the calls to the rand
// crate in a function that we could mock using a library like mocktopus.
fn player_init_test() {
    // populate an almost full field and ensure the new player is in the only empty spot
    let mut field_of_play = FieldOfPlay::new(3, 3);
    field_of_play.field[0][0] = Some(1);
    field_of_play.field[0][1] = Some(2);
    field_of_play.field[0][2] = Some(3);
    field_of_play.field[1][0] = Some(4);
    field_of_play.field[1][2] = Some(5);
    field_of_play.field[2][0] = Some(6);
    field_of_play.field[2][1] = Some(7);
    field_of_play.field[2][2] = Some(8);

    let new_player = Player::new(9, true, &mut field_of_play);

    assert_eq!(new_player.name, "p9".to_owned());
    assert_eq!(new_player.is_it, true);
    assert_eq!(new_player.get_location(), (1, 1));
}

#[test]
fn player_take_action_test() {
    // Test that I move to the only valid and empty location
    let mut field_of_play = FieldOfPlay::new(3, 3);
    field_of_play.field[0][0] = Some(1);
    field_of_play.field[0][1] = Some(2);
    field_of_play.field[0][2] = Some(3);
    // We temporary fill this position because we don't want the new user to init into it.
    field_of_play.field[1][0] = Some(100);
    field_of_play.field[1][1] = None;
    field_of_play.field[1][2] = Some(4);
    field_of_play.field[2][0] = Some(5);
    field_of_play.field[2][1] = Some(6);
    field_of_play.field[2][2] = Some(7);

    let mut player = Player::new(9, false, &mut field_of_play);
    assert_eq!(player.get_location(), (1, 1));
    // now we set this position to empty so player will move to it.
    field_of_play.field[1][0] = None;
    let actions = player.take_action(&field_of_play, 0);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, ActionType::Move);
    assert_eq!(actions[0].x_coordinate, Some(0));
    assert_eq!(actions[0].y_coordinate, Some(1));

    // The player is at (0,1). Now remove all but two of the other players
    // of which only one is taggable.
    field_of_play.field[0][0] = Some(1);
    field_of_play.field[0][1] = Some(2);
    field_of_play.field[0][2] = None;
    field_of_play.field[1][0] = Some(9);
    field_of_play.field[1][1] = None;
    field_of_play.field[1][2] = None;
    field_of_play.field[2][0] = None;
    field_of_play.field[2][1] = None;
    field_of_play.field[2][2] = None;
    player.is_it = true;
    let actions = player.take_action(&field_of_play, 1);

    assert!(!player.is_it);
    assert_eq!(actions.len(), 2);

    assert_eq!(actions[0].action, ActionType::Tag);
    assert_eq!(actions[0].new_it_index, Some(2));
}

#[test]
fn player_get_risk_tolerance_test() {
    let mut field_of_play = FieldOfPlay::new(3, 3);
    let mut player = Player::new(9, false, &mut field_of_play);

    player.risk_tolerance = 17.0;

    assert_eq!(player.get_risk_tolerance(), 17.0);
}

#[test]
fn player_get_location_test() {
    let mut field_of_play = FieldOfPlay::new(3, 3);
    let mut player = Player::new(9, false, &mut field_of_play);

    player.x_coordinate = 1;
    player.y_coordinate = 2;

    assert_eq!(player.get_location(), (1, 2));
}

#[test]
// Normally, you would not test private functions like this. But, for the purposes of this exercise
// we will for now due to time constraints
fn is_new_coordinates_too_close_test() {
    let mut field_of_play = FieldOfPlay::new(3, 3);
    let mut player = Player::new(1, false, &mut field_of_play);
    let mut it_player = Player::new(2, true, &mut field_of_play);
    player.risk_tolerance = 10.0;
    it_player.x_coordinate = 2;
    it_player.y_coordinate = 2;
    field_of_play.set_last_known_it_location(it_player.x_coordinate, it_player.y_coordinate);

    // test moving away from it
    player.x_coordinate = 2;
    player.y_coordinate = 1;

    assert_eq!(
        field_of_play.get_last_known_it_location(),
        Some((it_player.x_coordinate, it_player.y_coordinate))
    );
    assert!(!player.is_new_coordinates_too_close_to_it_player(0, 0, &field_of_play));

    // test moving closer but within tolerance
    player.x_coordinate = 0;
    player.y_coordinate = 0;
    assert!(player.is_new_coordinates_too_close_to_it_player(0, 1, &field_of_play));

    // test moving closer but outside tolerance
    player.x_coordinate = 0;
    player.y_coordinate = 0;
    player.risk_tolerance = 25.0;
    assert!(!player.is_new_coordinates_too_close_to_it_player(0, 1, &field_of_play));
}
