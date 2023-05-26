mod path_following_player;
mod path_remembering_player;

pub use path_following_player::*;
pub use path_remembering_player::*;

use crate::point::Point2D;

const START: Point2D = Point2D::from_cartesian(0, 0);

fn distance_from_start(position: &Point2D) -> usize {
    position.manhattan_distance_to(START)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_following_player_distance_from_start_test() {
        let position = Point2D::from_cartesian(5, -10);

        assert_eq!(15, distance_from_start(&position));
    }
}
