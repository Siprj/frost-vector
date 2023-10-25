use std::sync::Mutex;
use std::vec::Vec;

struct Statistics {
    header: Vec<String>,
    row: Vec<String>,
}

static STATISTICS: Mutex<Statistics> = Mutex::new(Statistics::new());

impl Statistics {
    pub const fn new() -> Statistics {
        Statistics {
            header: Vec::new(),
            row: Vec::new(),
        }
    }
    pub fn publish_row(&mut self) {
        for v in self.row.iter_mut() {
            print!("{}, ", v);
            *v = "".to_string();
        }
        println!("");
    }

    pub fn report_value(&mut self, name: String, value: String) {
        match self.header.iter().position(|e| *e == name) {
            Some(column_index) => {
                self.row[column_index] = value;
            }
            None => {
                self.header.push(name);
                self.row.push(value);
            }
        }
    }

    fn print_header(&self) {
        for v in self.header.iter() {
            print!("{}, ", v);
        }
        println!("");
    }
}

pub fn publish_row() {
    STATISTICS.lock().unwrap().publish_row();
}

pub fn report_value(name: &str, value: String) {
    STATISTICS
        .lock()
        .unwrap()
        .report_value(name.to_string(), value.clone());
}

pub fn print_header() {
    STATISTICS.lock().unwrap().print_header();
}
