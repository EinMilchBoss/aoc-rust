mod path_following_player;
mod path_remembering_player;

pub use path_following_player::*;
pub use path_remembering_player::*;

use crate::point::Point2D;

const START: Point2D = Point2D::from_cartesian(0, 0);
