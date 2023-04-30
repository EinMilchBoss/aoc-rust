mod fixed_lumination_light_grid;
mod variable_lumination_light_grid;

pub use fixed_lumination_light_grid::FixedLuminationLightGrid;
pub use variable_lumination_light_grid::VariableLuminationLightGrid;

pub const GRID_DIMENSION_SIZE: usize = 1_000;
pub const GRID_SIZE: usize = GRID_DIMENSION_SIZE * GRID_DIMENSION_SIZE;
