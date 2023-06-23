use std::vec::Vec;
use std::sync::OnceLock;

pub struct StatisticsInternal {
    header: Vec<String>,
    row: Vec<String>
}

static mut STATISTICS: OnceLock<StatisticsInternal> = OnceLock::new();

fn ensure_statistics() -> &'static StatisticsInternal {
    STATISTICS.get_or_init(|| StatisticsInternal::new())
}


impl StatisticsInternal {
    pub const fn new() -> StatisticsInternal {
        StatisticsInternal {
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
            },
            None => {
                self.header.push(name);
                self.row.push(value);
            }
        }
    }
}

pub struct Statistics {}

impl Statistics {
    pub fn publish_row() {
        ensure_statistics().publish_row();
    }

    pub fn report_value(name: String, value: String) {
        ensure_statistics().report_value(name, value);
    }
}
