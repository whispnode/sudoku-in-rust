#[derive(Copy, Clone)]
pub struct Cell {
    pub value: u8,
    pub is_fixed: bool,
    pub position: (u8, u8),
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: 0,
            is_fixed: false,
            position: (9, 9),
        }
    }
}
