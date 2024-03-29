#[derive(Debug, Copy, Clone)]
pub struct PieceTableEntry {
    start: u32,
    length: u32,
    is_read_only: bool, // which buffer: read-only or append-only buffer?
}
impl PartialEq for PieceTableEntry {
    fn eq(&self, other: &PieceTableEntry) -> bool {
        self.is_read_only == other.is_read_only && self.start == other.start
            && self.length == other.length
    }
}

pub struct PieceTable {
    data: Vec<PieceTableEntry>,
    read_only_buffer: String,
    append_only_buffer: String,
}

impl PieceTable {
    pub fn init(&mut self) {
        // No-op, for now
    }


    fn check_position_validity(&self, position: u32) {
        let mut max_position = 0;
        for elem in self.data.iter() {
            max_position += elem.length;
        }
        if position > max_position {
            panic!("Position out of bound")
        }
    }

    fn find_entry_at_position(&self, position: u32) -> Option<(PieceTableEntry, usize, u32)> {
        let mut cum = 0;
        for (index, elem) in self.data.iter().enumerate() {
            if cum + elem.length >= position {
                return Some((elem.clone(), index, cum));
            }
            cum += elem.length;
        }
        None
    }

    fn add_new_string_to_table(&mut self, new_string: &str, index: usize) {
        self.data.insert(index, PieceTableEntry {
            is_read_only: false,
            start: self.append_only_buffer.len() as u32,
            length: new_string.len() as u32,
        });

        self.append_only_buffer.push_str(new_string);
    }

    pub fn add(&mut self, new_string: &str, position: u32) {
        self.check_position_validity(position);

        let relevant_entry = self.find_entry_at_position(position);

        match relevant_entry {
            Some((ref buffer_entry, index, cum)) => {
                let real_entry_start = cum;
                let real_entry_end = cum + buffer_entry.length;
                // need to split relevant_entry into 2, and then insert "new_string" in between
                let first_piece_length = position - real_entry_start;
                let last_piece_length = real_entry_end - position;

                self.data.remove(index);

                let mut new_index = index;

                if first_piece_length > 0 {
                    self.data.insert(new_index, PieceTableEntry {
                        is_read_only: true,
                        start: buffer_entry.start,
                        length: first_piece_length,
                    });
                    new_index += 1;
                }

                self.add_new_string_to_table(new_string, new_index);
                new_index += 1;

                if last_piece_length > 0 {
                    self.data.insert(new_index, PieceTableEntry {
                        is_read_only: true,
                        start: buffer_entry.start + first_piece_length,
                        length: last_piece_length,
                    });
                }
            }
            None => {
                // Empty table, just add at back
                self.add_new_string_to_table(new_string, 0);
            }
        }
    }
    pub fn delete(&mut self, position: u32, length: u32) {
        self.check_position_validity(position);

        let relevant_entry = self.find_entry_at_position(position);

        match relevant_entry {
            Some((ref buffer_entry, index, cum)) => {
                let real_entry_start = cum;
                let real_entry_end = cum + buffer_entry.length;
                // Split relevant entry into two
                let first_piece_length = position - real_entry_start;

                // Removing the 2nd piece's start length
                let last_piece_start = buffer_entry.start + first_piece_length + length;
                let last_piece_length = real_entry_end - position- length;

                self.data.remove(index);

                let mut new_index = index;

                if first_piece_length > 0 {
                    self.data.insert(new_index, PieceTableEntry {
                        is_read_only: true,
                        start: buffer_entry.start,
                        length: first_piece_length,
                    });
                    new_index += 1;
                }

                if last_piece_length > 0 {
                    self.data.insert(new_index, PieceTableEntry {
                        is_read_only: true,
                        start: last_piece_start,
                        length: last_piece_length,
                    });
                }
            }
            None => {
                // Empty table, do nothing (or throw error)
            }
        }
    }
    pub fn char_at(&self, position: u32) -> char {
        self.check_position_validity(position);
        let relevant_entry = self.find_entry_at_position(position);
        match relevant_entry {
            Some((ref buffer_entry, _index, cum)) => {
                let pos = buffer_entry.start + position - cum;
                if buffer_entry.is_read_only {
                    return self.read_only_buffer.chars().nth(pos as usize).unwrap();
                } else {
                    return self.append_only_buffer.chars().nth(pos as usize).unwrap();
                }
            }
            None => {
                return '\0';
            }
        };
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
    fn add_string_at_empty() {
        let mut piece_table = PieceTable {
            data: Vec::new(),
            read_only_buffer: String::new(),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.add("haha", 0);
        assert!(piece_table.append_only_buffer == "haha");

        let expected_piece_table_data = vec![
            PieceTableEntry {
                is_read_only: false,
                start: 0,
                length: 4,
            },
        ];

        assert_eq!(piece_table.data, expected_piece_table_data);
    }


    #[test]
    fn add_string_at_start() {
        let mut piece_table = PieceTable {
            data: vec![
                PieceTableEntry {
                    is_read_only: true,
                    start: 0,
                    length: 5,
                },
            ],
            read_only_buffer: String::from("lorem"),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.add("hoho", 0);
        assert!(piece_table.append_only_buffer == "hoho");

        let expected_piece_table_data = vec![
            PieceTableEntry {
                is_read_only: false,
                start: 0,
                length: 4,
            },
            PieceTableEntry {
                is_read_only: true,
                start: 0,
                length: 5,
            },
        ];

        assert_eq!(piece_table.data, expected_piece_table_data);
    }


    #[test]
    fn add_string_at_end() {
        let mut piece_table = PieceTable {
            data: vec![
                PieceTableEntry {
                    is_read_only: true,
                    start: 0,
                    length: 5,
                },
            ],
            read_only_buffer: String::from("lorem"),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.add("hoho", 5);
        assert!(piece_table.append_only_buffer == "hoho");

        let expected_piece_table_data = vec![
            PieceTableEntry {
                is_read_only: true,
                start: 0,
                length: 5,
            },
            PieceTableEntry {
                is_read_only: false,
                start: 0,
                length: 4,
            },
        ];

        assert_eq!(piece_table.data, expected_piece_table_data);
    }



    #[test]
    fn add_string_at_middle() {
        let mut piece_table = PieceTable {
            data: vec![
                PieceTableEntry {
                    is_read_only: true,
                    start: 0,
                    length: 5,
                },
            ],
            read_only_buffer: String::from("lorem"),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.add("hoho", 2);
        assert!(piece_table.append_only_buffer == "hoho");

        let expected_piece_table_data = vec![
            PieceTableEntry {
                is_read_only: true,
                start: 0,
                length: 2,
            },
            PieceTableEntry {
                is_read_only: false,
                start: 0,
                length: 4,
            },
            PieceTableEntry {
                is_read_only: true,
                start: 2,
                length: 3,
            },
        ];

        assert_eq!(piece_table.data, expected_piece_table_data);
    }

    #[test]
    fn char_at_init_state() {
        let mut piece_table = PieceTable {
            data: vec![
                PieceTableEntry {
                    is_read_only: true,
                    start: 0,
                    length: 5,
                },
            ],
            read_only_buffer: String::from("lorem"),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        assert_eq!(piece_table.char_at(0), 'l');
        assert_eq!(piece_table.char_at(1), 'o');
        assert_eq!(piece_table.char_at(2), 'r');
        assert_eq!(piece_table.char_at(3), 'e');
        assert_eq!(piece_table.char_at(4), 'm');
    }

    #[test]
    fn delete_string_at_middle_on_read_only_buffer() {
        let mut piece_table = PieceTable {
            data: vec![
                PieceTableEntry {
                    is_read_only: true,
                    start: 0,
                    length: 10,
                },
            ],
            read_only_buffer: String::from("loremipsum"),
            append_only_buffer: String::new(),
        };
        piece_table.init();
        piece_table.delete(5, 2);
        assert!(piece_table.read_only_buffer == "loremipsum");

        let expected_piece_table_data = vec![
            PieceTableEntry {
                is_read_only: true,
                start: 0,
                length: 5,
            },
            PieceTableEntry {
                is_read_only: true,
                start: 7,
                length: 3,
            },
        ];

        assert_eq!(piece_table.data, expected_piece_table_data);
    }
}
