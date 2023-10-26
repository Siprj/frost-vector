use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::{Mutex, OnceLock};
use std::vec::Vec;
use serde::ser::{Serialize, Serializer, SerializeStruct};

struct Entry {
    frame: usize,
    value: f64
}

struct DataStream {
    stream: VecDeque<Entry>
}

#[derive(Copy, Debug, Clone)]
struct DataStreamId { id: usize}

static STATISTICS_FRAME: AtomicUsize = AtomicUsize::new(0); // current frame

struct Statistics {
    id_by_name: HashMap<String, DataStreamId>,
    data_streams: Vec<DataStream>
}

impl Statistics {
    fn new() -> Self {
        Statistics {id_by_name: HashMap::new(), data_streams: Vec::new()}
    }

    fn get_stream_from_name(&mut self, name: &str) -> &mut DataStream {
        match self.id_by_name.get(name) {
            Some(index) => &mut self.data_streams[index.id],
            None => {
                let id = self.data_streams.len();
                self.id_by_name.insert(name.into(), DataStreamId { id });
                self.data_streams.push(DataStream{ stream: VecDeque::new()});
                &mut self.data_streams[id]
            },
        }
    }

    fn get_data_stream_id(&mut self, name: &str) -> DataStreamId {
        match self.id_by_name.get(name) {
            Some(index) => *index,
            None => {
                let id = self.data_streams.len();
                self.id_by_name.insert(name.into(), DataStreamId { id });
                self.data_streams.push(DataStream{ stream: VecDeque::new()});
                DataStreamId{ id }
            },
        }
    }

    fn report_value_with_name(&mut self, name: &str, value: f64) {
        let stream_id = self.get_data_stream_id(name);
        self.report_value(stream_id, value);
    }

    fn report_value(&mut self, stream_id: DataStreamId, value: f64) {
        let frame = STATISTICS_FRAME.load(Ordering::Relaxed);
        self.data_streams[stream_id.id].stream.push_back(Entry{frame, value});
    }

    fn restart(&mut self) {
        STATISTICS_FRAME.store(0, Ordering::Relaxed);
        self.data_streams.clear();
        self.id_by_name.clear();
    }
}

static STATISTICS: OnceLock<Mutex<Statistics>> = OnceLock::new(); // current frame

pub fn get_initialized_statistics<'a>() -> &'a Mutex<Statistics> {
    STATISTICS.get_or_init(|| Mutex::new(Statistics::new()))
}

pub fn next_frame() {
    STATISTICS_FRAME.fetch_add(1, Ordering::Relaxed);
}

pub fn restart_statistics() {
    let mut statistics = get_initialized_statistics().lock().unwrap();
    statistics.restart();
}

pub fn get_data_stream_id(name: &str) -> DataStreamId {
    let mut statistics = get_initialized_statistics().lock().unwrap();
    statistics.get_data_stream_id(name)
}

pub fn report_value(stream_id: DataStreamId, value: f64) {
    let mut statistics = get_initialized_statistics().lock().unwrap();
    statistics.report_value(stream_id, value);
}

pub fn report_value_with_name(name: &str, value: f64) {
    let mut statistics = get_initialized_statistics().lock().unwrap();
    statistics.report_value_with_name(name, value);
}

pub fn into_csv(path: &Path) {
}

impl Serialize for Statistics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Statistics", 3)?;
        state.serialize_field("statistics", "asdf")?;
        state.end()
    }
}

pub fn into_json(path: &Path) {

}
