use super::action::Action;
use super::direction::Direction;
use super::field_of_play::FieldOfPlay;
use log::{debug, error, info};
use rand::{thread_rng, Rng};

#[derive(Clone, Default, Debug)]
pub struct Player {
    pub is_it: bool,
    pub name: String,
    x_coordinate: usize,
    y_coordinate: usize
}

impl Player {
    pub fn new(index: usize, is_it: bool, field_of_play: &mut FieldOfPlay) -> Self {
        // Since we are calling init_pos right away it's likely safe to init the positions to 0
        // instead of using Option<usize>.
        let name = format!("p{}", index);
        let mut player = Player {
            name,
            is_it,
            x_coordinate: 0,
            y_coordinate: 0
        };

        player.init_position(field_of_play, index);

        player
    }

    // Looks at the field of play and returns a starting position. It sets the position for the
    // player but the [FieldOfPlay] must be updated with the returned coordinates.
    fn init_position(&mut self, field_of_play: &mut FieldOfPlay, index: usize) {
        let y_len = field_of_play.field.len();
        let x_len = field_of_play.field[0].len();
        let mut rng = thread_rng();
        let mut found_pos = false;

        while !found_pos {
            let rand_x = rng.gen_range(0..x_len);
            let rand_y = rng.gen_range(0..y_len);
            info!("checking if x: {}, y: {} is available", rand_x, rand_y);
            found_pos = field_of_play.field[rand_y][rand_x].is_none();
            if found_pos {
                self.set_location(rand_x, rand_y);
                field_of_play.field[rand_y][rand_x] = Some(index);
            }
        }
    }

    /// Looks at the field of play and takes at least one action. If the player is not it, it will
    /// attempt to move. If the player is it, it will attempt to tag any nearby players and also
    /// move. The tag action will only occur before the move action. If a player is it, does not
    /// tag anyone, and moves to a new position where they are able to tag another player they must
    /// wait until their next turn.
    pub fn take_action(&mut self, field_of_play: &FieldOfPlay, last_it_index: usize) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        if self.is_it {
            if let Some(newly_tagged_index) = self.get_taggable_player(field_of_play, last_it_index) {
                self.is_it = false;
                actions.push(Action::new_tag(newly_tagged_index));
            }
        }
        actions.push(self.take_move_action(field_of_play));

        actions
    }

    fn get_taggable_player(&self, field_of_play: &FieldOfPlay, last_it_index: usize) -> Option<usize> {
        let adjacent_players = field_of_play.get_adjacent_player_indices(self.x_coordinate, self.y_coordinate);
        let taggable_players = adjacent_players.into_iter().find(|ap| *ap != last_it_index);
        debug!("Taggable player indices adjacent to {:?}: {:?}", self, taggable_players);

        taggable_players
    }

    fn take_move_action(&mut self, field_of_play: &FieldOfPlay) -> Action {
        let (x, y) = self.move_to_empty_position(field_of_play);
        self.set_location(x, y);

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
