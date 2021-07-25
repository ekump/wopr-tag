use super::direction::Direction;

type Field = Vec<Vec<Option<usize>>>;

// It shouldn't be possible to ever see this message since we should be correctly checking bounds.
// We could probably get away with just using uwnrap.
static GENERIC_VEC_ACCESS_PANIC_ERR_MSG: &str = "Invalid field index.";

#[derive(Debug)]
pub struct FieldOfPlay {
    pub field: Field,
    last_known_it_coordinates: Option<(usize, usize)>
}

#[derive(Default)]
struct PositionDetails {
    is_valid: bool,
    occupant: Option<usize>
}

impl PositionDetails {
    pub fn can_move_to(&self) -> bool {
        self.is_valid && self.occupant.is_none()
    }
}

impl FieldOfPlay {
    pub fn new(x_len: usize, y_len: usize) -> Self {
        let mut field_of_play = Vec::new();
        (0..y_len).for_each(|_| {
            let x_vec: Vec<Option<usize>> = (0..x_len).map(|_| None).collect();
            field_of_play.push(x_vec);
        });

        FieldOfPlay {
            field: field_of_play,
            last_known_it_coordinates: None
        }
    }

    /// Returns a vec of player indices that are adjacent to the input coordinates. This can be
    /// used by a player who is it to find players to tag. It is up to the caller to determine if
    /// the players are taggable and if there are multiple players, which one to tag it.
    pub fn get_adjacent_player_indices(&self, it_x: usize, it_y: usize) -> Vec<usize> {
        let mut player_indicies = Vec::new();

        // NORTH
        if let Some(player_index) = self.is_position_valid_and_get_occupant_north(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // SOUTH
        if let Some(player_index) = self.is_position_valid_and_get_occupant_south(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // EAST
        if let Some(player_index) = self.is_position_valid_and_get_occupant_east(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // WEST
        if let Some(player_index) = self.is_position_valid_and_get_occupant_west(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // NORTHWEST
        if let Some(player_index) = self.is_position_valid_and_get_occupant_north_west(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // NORTHEAST
        if let Some(player_index) = self.is_position_valid_and_get_occupant_north_east(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // SOUTHWEST
        if let Some(player_index) = self.is_position_valid_and_get_occupant_south_west(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }
        // SOUTHEAST
        if let Some(player_index) = self.is_position_valid_and_get_occupant_south_east(it_x, it_y).occupant {
            player_indicies.push(player_index);
        }

        player_indicies
    }

    /// Returns a bool that is true if the adjacent position for the provided coordinates in the
    /// provided direction is empty. The coordinates should be the current coordinates of a player,
    /// not the desired ones.
    pub fn is_position_valid_and_empty(&self, direction: Direction, x: usize, y: usize) -> bool {
        match direction {
            Direction::East => self.is_position_valid_and_get_occupant_east(x, y).can_move_to(),
            Direction::North => self.is_position_valid_and_get_occupant_north(x, y).can_move_to(),
            Direction::NorthEast => self.is_position_valid_and_get_occupant_north_east(x, y).can_move_to(),
            Direction::NorthWest => self.is_position_valid_and_get_occupant_north_west(x, y).can_move_to(),
            Direction::South => self.is_position_valid_and_get_occupant_south(x, y).can_move_to(),
            Direction::SouthEast => self.is_position_valid_and_get_occupant_south_east(x, y).can_move_to(),
            Direction::SouthWest => self.is_position_valid_and_get_occupant_south_west(x, y).can_move_to(),
            Direction::West => self.is_position_valid_and_get_occupant_west(x, y).can_move_to()
        }
    }

    pub fn set_last_known_it_location(&mut self, x: usize, y: usize) {
        self.last_known_it_coordinates = Some((x, y));
    }

    pub fn get_last_known_it_location(&self) -> Option<(usize, usize)> {
        self.last_known_it_coordinates
    }

    fn is_position_valid_and_get_occupant_south(&self, x: usize, y: usize) -> PositionDetails {
        if y < self.field.len() - 1 {
            let y_axis = self
                .field
                .get((y + 1) as usize)
                .expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_north(&self, x: usize, y: usize) -> PositionDetails {
        if y > 0 {
            let y_axis = self
                .field
                .get((y - 1) as usize)
                .expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_east(&self, x: usize, y: usize) -> PositionDetails {
        let y_axis = self.field.get(y).expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
        if let Some(pos_x) = y_axis.get(x + 1) {
            return PositionDetails {
                is_valid: true,
                occupant: *pos_x
            };
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_west(&self, x: usize, y: usize) -> PositionDetails {
        if x > 0 {
            let y_axis = self.field.get(y).expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x - 1) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_south_east(&self, x: usize, y: usize) -> PositionDetails {
        if y < self.field.len() - 1 {
            let y_axis = self.field.get(y + 1).expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x + 1) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_south_west(&self, x: usize, y: usize) -> PositionDetails {
        if y < self.field.len() - 1 && x > 0 {
            let y_axis = self.field.get(y + 1).expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x - 1) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_north_west(&self, x: usize, y: usize) -> PositionDetails {
        if y > 0 && x > 0 {
            let y_axis = self.field.get(y - 1).expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x - 1) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }

    fn is_position_valid_and_get_occupant_north_east(&self, x: usize, y: usize) -> PositionDetails {
        if y > 0 {
            let y_axis = self.field.get(y - 1).expect(GENERIC_VEC_ACCESS_PANIC_ERR_MSG);
            if let Some(pos_x) = y_axis.get(x + 1) {
                return PositionDetails {
                    is_valid: true,
                    occupant: *pos_x
                };
            }
        }

        PositionDetails::default()
    }
}
#[test]
fn position_can_move_to_test() {
    let invalid_pos = PositionDetails {
        is_valid: false,
        occupant: None
    };
    let valid_but_occupied_pos = PositionDetails {
        is_valid: false,
        occupant: Some(1)
    };
    let valid_and_empty_pos = PositionDetails {
        is_valid: true,
        occupant: None
    };

    assert!(valid_and_empty_pos.can_move_to());
    assert!(!valid_but_occupied_pos.can_move_to());
    assert!(!invalid_pos.can_move_to());
}

#[test]
fn field_of_play_new_test() {
    let field_of_play = FieldOfPlay::new(3, 3);

    assert_eq!(field_of_play.field.len(), 3);

    field_of_play.field.iter().for_each(|y_row| {
        assert_eq!(y_row.len(), 3);
        assert!(y_row.iter().all(|x| x.is_none()));
    });
}

#[test]
fn field_get_adjacent_player_indices_test() {
    let mut field_of_play = FieldOfPlay::new(3, 3);

    field_of_play.field[0][0] = Some(1);
    field_of_play.field[0][1] = Some(2);
    field_of_play.field[0][2] = Some(3);
    field_of_play.field[1][0] = Some(4);
    field_of_play.field[1][2] = Some(5);
    field_of_play.field[2][0] = Some(6);
    field_of_play.field[2][1] = Some(7);
    field_of_play.field[2][2] = Some(8);

    let adj_players = field_of_play.get_adjacent_player_indices(1, 1);
    assert_eq!(adj_players.len(), 8);
}

#[test]
fn field_is_position_valid_and_empty_test() {
    let mut field_of_play = FieldOfPlay::new(3, 3);

    // First test that all positions are full and so all calls should return false
    field_of_play.field[0][0] = Some(1);
    field_of_play.field[0][1] = Some(2);
    field_of_play.field[0][2] = Some(3);
    field_of_play.field[1][0] = Some(4);
    field_of_play.field[1][2] = Some(5);
    field_of_play.field[2][0] = Some(6);
    field_of_play.field[2][1] = Some(7);
    field_of_play.field[2][2] = Some(8);

    assert!(!field_of_play.is_position_valid_and_empty(Direction::East, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::West, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::North, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::South, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::NorthEast, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::NorthWest, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::SouthEast, 1, 1));
    assert!(!field_of_play.is_position_valid_and_empty(Direction::SouthWest, 1, 1));

    // Now test empty positions
    field_of_play.field[0][0] = None;
    field_of_play.field[0][1] = None;
    field_of_play.field[0][2] = None;
    field_of_play.field[1][0] = None;
    field_of_play.field[1][2] = None;
    field_of_play.field[2][0] = None;
    field_of_play.field[2][1] = None;
    field_of_play.field[2][2] = None;

    assert!(field_of_play.is_position_valid_and_empty(Direction::East, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::West, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::North, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::South, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::NorthEast, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::NorthWest, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::SouthEast, 1, 1));
    assert!(field_of_play.is_position_valid_and_empty(Direction::SouthWest, 1, 1));
}

#[test]
fn field_last_known_it_position_test() {
    let mut field_of_play = FieldOfPlay::new(3, 3);
    assert_eq!(field_of_play.get_last_known_it_location(), None);
    field_of_play.set_last_known_it_location(3, 4);
    assert_eq!(field_of_play.get_last_known_it_location(), Some((3, 4)));
}
