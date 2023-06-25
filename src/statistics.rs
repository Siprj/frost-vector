use std::cell::UnsafeCell;
use std::vec::Vec;


struct Statistics {
    header: Vec<String>,
    row: Vec<String>
}

unsafe impl Sync for Statistics {}

struct Bla<T> {
    value: UnsafeCell<T>
}

impl<T> Bla<T> {
    #[inline(always)]
    const fn new(data: T) -> Bla<T>{
        Bla {
            value: UnsafeCell::new(data)
        }
    }

    #[inline(always)]
    pub fn get(&self) -> *mut T {
        self.value.get()
    }
}

unsafe impl<T: Sync> Sync for Bla<T> {}

static STATISTICS: Bla<Statistics> = Bla::new(Statistics::new());

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
            },
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

fn ensure_statistic<'a>() -> &'a mut Statistics {
    unsafe {&mut *STATISTICS.get()}
}

pub fn publish_row() {
    ensure_statistic().publish_row();
}

pub fn report_value(name: &str, value: String) {
    ensure_statistic().report_value(name.to_string(), value.clone());
}

pub fn print_header() {
    ensure_statistic().print_header();
}
