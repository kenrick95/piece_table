#[derive(Debug)]
pub struct PieceTableEntry {
    start: u32,
    length: u32,
    is_read_only: bool, // which buffer: read-only or buffer-only?
}

pub struct PieceTable {
    data: Vec<PieceTableEntry>,
    read_only_buffer: String,
    append_only_buffer: String,
}

impl PieceTable {
    pub fn init(&self) {}

    fn check_position_validity(&self, position: u32) {
        let max_position = self.data.len() as u32;
        if position < 0 || position > max_position {
            panic!("Position out of bound")
        }
    }

    pub fn add(&self, new_string: &str, position: u32) {
        self.check_position_validity(position);
    }
    pub fn delete(&self, position: u32, length: u32) {
        self.check_position_validity(position);
    }
    pub fn char_at(&self, position: u32) {
        self.check_position_validity(position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn init() {
        let piece_table = PieceTable {
            data: Vec::new(),
            read_only_buffer: String::new(),
            append_only_buffer: String::new(),
        };
        piece_table.add("a", 3);
    }
}
