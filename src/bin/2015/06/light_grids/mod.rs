mod fixed_lumination;
mod variable_lumination;

pub use fixed_lumination::FixedLuminationLightGrid;
pub use variable_lumination::VariableLuminationLightGrid;

pub const GRID_DIMENSION_SIZE: usize = 1_000;
pub const GRID_SIZE: usize = GRID_DIMENSION_SIZE * GRID_DIMENSION_SIZE;
