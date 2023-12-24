use std::collections::{HashMap, VecDeque};
use std::convert::AsRef;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::vec::Vec;

#[derive(Debug, serde::Serialize)]
struct Entry {
    frame: usize,
    value: f64,
}

struct DataStream {
    stream: VecDeque<Entry>,
}

#[derive(Copy, Debug, Clone)]
pub struct DataStreamId {
    id: usize,
}

static STATISTICS_FRAME: AtomicUsize = AtomicUsize::new(0); // current frame

struct Statistics {
    id_by_name: HashMap<String, DataStreamId>,
    data_streams: Vec<DataStream>,
}

impl Statistics {
    fn new() -> Self {
        Statistics {
            id_by_name: HashMap::new(),
            data_streams: Vec::new(),
        }
    }

    fn get_data_stream_id(&mut self, name: &str) -> DataStreamId {
        match self.id_by_name.get(name) {
            Some(index) => *index,
            None => {
                let id = self.data_streams.len();
                self.id_by_name.insert(name.into(), DataStreamId { id });
                self.data_streams.push(DataStream {
                    stream: VecDeque::new(),
                });
                DataStreamId { id }
            }
        }
    }

    fn report_value_with_name(&mut self, name: &str, value: f64) {
        let stream_id = self.get_data_stream_id(name);
        self.report_value(stream_id, value);
    }

    fn report_value(&mut self, stream_id: DataStreamId, value: f64) {
        let frame = STATISTICS_FRAME.load(Ordering::Relaxed);
        self.data_streams[stream_id.id]
            .stream
            .push_back(Entry { frame, value });
    }

    fn restart(&mut self) {
        STATISTICS_FRAME.store(0, Ordering::Relaxed);
        self.data_streams.clear();
        self.id_by_name.clear();
    }
}

static STATISTICS: OnceLock<Mutex<Statistics>> = OnceLock::new(); // current frame

fn get_initialized_statistics<'a>() -> &'a Mutex<Statistics> {
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

pub fn into_csv_files<P: AsRef<Path>>(path: P) {
    fs::create_dir_all(path.as_ref()).unwrap();

    let statistics = get_initialized_statistics().lock().unwrap();
    for (name, index) in statistics.id_by_name.iter() {
        let mut csv_path_name: PathBuf = path.as_ref().into();
        csv_path_name.push(name.replace(' ', "_"));
        csv_path_name.set_extension("csv");

        println!("csv_path_name: {}", csv_path_name.display());

        let mut csv_file = csv::Writer::from_path(csv_path_name).unwrap();
        for entry in &statistics.data_streams[index.id].stream {
            csv_file.serialize(entry).unwrap();
        }
        csv_file.flush().unwrap();
    }
}

#[derive(Debug, serde::Serialize)]
struct StatisticsJson<'a> {
    statistics: &'a HashMap<&'a String, &'a VecDeque<Entry>>,
}

pub fn save_as_json<P: AsRef<Path>>(path: P) {
    // TODO: Think about error handling... unwrap everywhere is not cool...
    let statistics = get_initialized_statistics().lock().unwrap();
    let statistics_fmap = statistics
        .id_by_name
        .iter()
        .map(|(name, index)| (name, &statistics.data_streams[index.id].stream))
        .collect();

    let json_string = serde_json::to_string(&StatisticsJson {
        statistics: &statistics_fmap,
    })
    .unwrap();
    fs::create_dir_all(path.as_ref().parent().unwrap()).unwrap();
    let mut json_file = fs::File::create(path).unwrap();
    json_file.write_all(json_string.as_bytes()).unwrap();
}
