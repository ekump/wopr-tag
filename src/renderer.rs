use crate::models::player::Player;
use crate::FieldOfPlay;

// This is very rudimentary but gets the job done for now
pub fn render_field(field_of_play: &FieldOfPlay, players: &[Player], turn_num: usize) {
    println!("/// TURN {}", turn_num);
    field_of_play.field.iter().for_each(|y_axis| {
        let mut y_axis_as_string = String::new();
        y_axis.iter().for_each(|x_axis_element| {
            let x_axis_element = match x_axis_element {
                Some(player_index) => {
                    if players[*player_index].is_it {
                        "*"
                    } else {
                        "P"
                    }
                }
                None => "-"
            };

            let x_element_as_str = x_axis_element.to_owned();
            y_axis_as_string = format!("{}{}", y_axis_as_string, x_element_as_str);
        });
        println!("{}", y_axis_as_string);
    });
}
