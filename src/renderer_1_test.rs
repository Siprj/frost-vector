use std::time::{Duration, Instant};

use crate::renderer_1;

pub async fn run() {
    let mut circles: Vec<(f32, f32, f32, f32)> = Vec::new();
    // for _ in 0..100 {
    //     circles.push((
    //         rand::random::<f32>() * 800_f32,
    //         rand::random::<f32>() * 800_f32,
    //         rand::random::<f32>() * 100_f32,
    //         1_f32 + rand::random::<f32>() * 5_f32,
    //     ));
    // }

    let mut rectangles = Vec::new();
    // for _ in 0..100 {
    //     rectangles.push((
    //         rand::random::<f32>() * 800_f32,
    //         rand::random::<f32>() * 800_f32,
    //         rand::random::<f32>() * 100_f32,
    //         rand::random::<f32>() * 400_f32,
    //         1_f32 + rand::random::<f32>() * 5_f32,
    //     ));
    // }
    rectangles.push((400_f32, 500_f32, 100_f32, 300_f32, 10_f32));

    let monotonic_time = Instant::now();
    println!("timestamp: {}", monotonic_time.elapsed().as_nanos());

    let mut previous_end_time = Duration::from_secs(0);
    renderer_1::run(
        |_| false,
        move |draw| {
            let frame_start = monotonic_time.elapsed();
            circles.iter().for_each(|c| draw.circle(c.0, c.1, c.2, c.3));
            rectangles
                .iter()
                .for_each(|r| draw.rectangle(r.0, r.1, r.2, r.3, r.4));
            let frame_end = monotonic_time.elapsed();
            // println!(
            //     "loop fsp: {:?}",
            //     1_f32 / (frame_end - previous_end_time).as_secs_f32()
            // );
            // previous_end_time = frame_end;
            // println!("timestamp: {:?}", frame_end - frame_start);
            // println!("end_frame_time: {:?}", frame_end);
            // println!("fps: {:?}", 1_f32 / (frame_end - frame_start).as_secs_f32());
        },
    )
    .await
}
