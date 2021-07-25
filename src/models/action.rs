#[derive(Debug)]
pub struct Action {
    pub action: ActionType,
    pub x_coordinate: Option<usize>,
    pub y_coordinate: Option<usize>,
    pub new_it_index: Option<usize>
}

impl Action {
    pub fn new_move(x: usize, y: usize) -> Self {
        Self {
            action: ActionType::Move,
            x_coordinate: Some(x),
            y_coordinate: Some(y),
            new_it_index: None
        }
    }

    pub fn new_tag(index: usize) -> Self {
        Self {
            action: ActionType::Tag,
            x_coordinate: None,
            y_coordinate: None,
            new_it_index: Some(index)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ActionType {
    Move,
    Tag
}

#[test]
fn action_new_tag_test() {
    let new_tag = Action::new_tag(1);

    assert_eq!(new_tag.action, ActionType::Tag);
    assert_eq!(new_tag.new_it_index, Some(1));
    assert_eq!(new_tag.x_coordinate, None);
    assert_eq!(new_tag.y_coordinate, None);
}

#[test]
fn action_new_move_test() {
    let new_tag = Action::new_move(1, 2);

    assert_eq!(new_tag.action, ActionType::Move);
    assert_eq!(new_tag.new_it_index, None);
    assert_eq!(new_tag.x_coordinate, Some(1));
    assert_eq!(new_tag.y_coordinate, Some(2));
}
