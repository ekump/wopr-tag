use crate::SharedFieldOfPlay;
use std::{thread, time};
use log::debug;

// This is very rudimentary but gets the job done for now
pub fn render_field(shared_field_of_play: SharedFieldOfPlay, refresh_interval: u64) {
    loop {
        {
            let field_of_play = shared_field_of_play.try_lock();
            match field_of_play {
                Ok(field_of_play) => {
                    println!("/// TURN");
                    field_of_play.field.iter().for_each(|y_axis| {
                        let mut y_axis_as_string = String::new();

                        y_axis.iter().for_each(|x_axis_element| {
                            let x_axis_element = match x_axis_element {
                                Some(player_index) => {
                                        if player_index == &field_of_play.last_known_it_id {
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
                },
                Err(_) => {
                    debug!("Can't get lock");
                }
            }
        } 
        let sleep_between_turn_dur = time::Duration::from_millis(refresh_interval);
        thread::sleep(sleep_between_turn_dur);
    }
}
