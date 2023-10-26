use std::time::{Duration, Instant};

use winit::event_loop::EventLoop;

use crate::renderer_1;
use crate::statistics;

pub async fn run(event_loop: &mut EventLoop<()>, circles: &Vec<(f32, f32, f32, f32)>, rectangles: &Vec<(f32, f32, f32, f32, f32)>) {
    let monotonic_time = Instant::now();
    let mut previous_end_time = Duration::from_secs(0);
    let local_circles = circles.clone();
    let local_rectangles = rectangles.clone();
    renderer_1::run(
        event_loop,
        |_| false,
        move |draw| {
            let frame_start = monotonic_time.elapsed();
            statistics::report_value_with_name(
                "run_lambda_wait_time_before_start",
                (frame_start - previous_end_time).as_secs_f64(),
            );
            local_circles
                .iter()
                .for_each(|c| draw.circle(c.0, c.1, c.2, c.3));
            let circles_end = monotonic_time.elapsed();
            statistics::report_value_with_name(
                "drawing_circles",
                (circles_end - frame_start).as_secs_f64(),
            );
            local_rectangles
                .iter()
                .for_each(|r| draw.rectangle(r.0, r.1, r.2, r.3, r.4));
            let current_end_time = monotonic_time.elapsed();
            statistics::report_value_with_name(
                "drawing_rectangles",
                (current_end_time - circles_end).as_secs_f64(),
            );
            statistics::report_value_with_name(
                "run_lambda",
                (current_end_time - frame_start).as_secs_f64(),
            );
            statistics::report_value_with_name(
                "whole_frame_time",
                (current_end_time - previous_end_time)
                    .as_secs_f64(),
            );
            statistics::report_value_with_name(
                "whole_fps",
                1.0 / (current_end_time - previous_end_time).as_secs_f64(),
            );
            previous_end_time = current_end_time;
        },
    )
    .await
}
