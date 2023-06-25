use std::time::{Duration, Instant};

use crate::renderer_1;
use crate::statistics;

const ELEMENT_COUNT: usize = 200_000;

pub async fn run() {

    let mut circles: Vec<(f32, f32, f32, f32)> = Vec::new();
    for _ in 0..ELEMENT_COUNT {
        circles.push((
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 5_f32,
            1_f32 + rand::random::<f32>() * 2_f32,
        ));
    }

    let mut rectangles = Vec::new();
    for _ in 0..ELEMENT_COUNT {
        rectangles.push((
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 10_f32,
            rand::random::<f32>() * 20_f32,
            1_f32 + rand::random::<f32>() * 2_f32,
        ));
    }



    let monotonic_time = Instant::now();
    let mut previous_end_time = Duration::from_secs(0);
    renderer_1::run(
        |_| false,
        move |draw| {

            let frame_start = monotonic_time.elapsed();
            statistics::report_value("run_lambda_wait_time_before_start", (frame_start - previous_end_time).as_secs_f64().to_string());
            circles.iter().for_each(|c| draw.circle(c.0, c.1, c.2, c.3));
            let circles_end = monotonic_time.elapsed();
            statistics::report_value("drawing_circles", (circles_end - frame_start).as_secs_f64().to_string());
            rectangles
                .iter()
                .for_each(|r| draw.rectangle(r.0, r.1, r.2, r.3, r.4));
            let current_end_time = monotonic_time.elapsed();
            statistics::report_value("drawing_rectangles", (current_end_time - circles_end).as_secs_f64().to_string());
            statistics::report_value("run_lambda", (current_end_time - frame_start).as_secs_f64().to_string());
            statistics::report_value("whole_frame_time", (current_end_time - previous_end_time).as_secs_f64().to_string());
            statistics::report_value("whole_fps", (1.0/(current_end_time - previous_end_time).as_secs_f64()).to_string());
            previous_end_time = current_end_time;

        },
    )
    .await
}
