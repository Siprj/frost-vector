use frost_vector::renderer_1_test;
use log::debug;
//use frost_vector::renderer_2_test;
use winit::event_loop::EventLoop;
use frost_vector::constants::ELEMENT_COUNT;


fn main() {
    env_logger::init();

    debug!("Generating shapes");
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
            rand::random::<f32>() * 400_f32,
            rand::random::<f32>() * 400_f32,
            1_f32 + rand::random::<f32>() * 5_f32,
        ));
    }

    debug!("Creating event loop");
    let event_loop = EventLoop::new().expect("Event loop");

//    pollster::block_on(renderer_1_test::run(&mut event_loop, &circles, &rectangles));
//
//    statistics::save_as_json("./statistics/renderer_1.json");
//    statistics::into_csv_files("./statistics/renderer_1/");
//    statistics::restart_statistics();

    pollster::block_on(renderer_1_test::run(event_loop, &circles, &rectangles));

//    statistics::save_as_json("./statistics/renderer_2.json");
//    statistics::into_csv_files("./statistics/renderer_2/");
//    statistics::restart_statistics();
}
