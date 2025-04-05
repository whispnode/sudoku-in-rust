#[derive(Copy, Clone)]
pub struct Cell {
    pub value: u8,
    pub is_fixed: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: 0,
            is_fixed: false,
        }
    }
}
