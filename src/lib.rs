#[derive(Debug)]
pub struct PieceTableEntry {
    start: u32,
    length: u32,
    is_read_only: bool, // which buffer: read-only or buffer-only?
}

pub struct PieceTable {
    data: Vec<PieceTableEntry>, // TODO: seems not so correct as we need to traverse the whole entries for an operation?
    // TODO: sequence of entries in table is important! it determines the final result of read buffer + append buffer
    read_only_buffer: String,
    append_only_buffer: String,
}

impl PieceTable {
    pub fn init(&mut self) {
        self.data.push(PieceTableEntry {
            is_read_only: true,
            start: 0,
            length: self.read_only_buffer.len() as u32
        })
    }

    fn check_position_validity(&self, position: u32) {
        let max_position = self.data.len() as u32;
        if position < 0 || position > max_position {
            panic!("Position out of bound")
        }
    }

    pub fn add(&mut self, new_string: &str, position: u32) {
        self.check_position_validity(position);
        
        self.data.push(PieceTableEntry {
            is_read_only: false,
            start: self.append_only_buffer.len() as u32,
            length: new_string.len() as u32
        });

        self.append_only_buffer.push_str(new_string);

        // self.data.push(PieceTableEntry {
        //     is_read_only: true,
        //     start: ,
        //     length: 
        // })

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
        let mut piece_table = PieceTable {
            data: Vec::new(),
            read_only_buffer: String::new(),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.add("a", 3);
    }

    #[test]
    fn add_one_char_at_empty_buffer() {
        let mut piece_table = PieceTable {
            data: Vec::new(),
            read_only_buffer: String::new(),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.add("a", 0);
        assert!(piece_table.append_only_buffer == "a");
    }
}
