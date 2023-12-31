use frost_vector::{
    render_common::RendererRunner, renderer_1_test, renderer_test_triange::RendererTestTriangle,
};
use log::{debug, info};
//use frost_vector::renderer_2_test;
use frost_vector::constants::ELEMENT_COUNT;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();

    debug!("Generating shapes");
    let mut circles: Vec<(f32, f32, f32, f32)> = Vec::new();
    for _ in 0..ELEMENT_COUNT {
        circles.push((
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 50_f32,
            1_f32 + rand::random::<f32>() * 10_f32,
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
    //    statistics::save_as_json("./statistics/renderer_1.json");
    //    statistics::into_csv_files("./statistics/renderer_1/");
    //    statistics::restart_statistics();

    pollster::block_on(renderer_1_test::run(event_loop, &circles));
    //    statistics::save_as_json("./statistics/renderer_2.json");
    //    statistics::into_csv_files("./statistics/renderer_2/");
    //    statistics::restart_statistics();
    //pollster::block_on(test_run2(event_loop));
}

async fn test_run2(mut event_loop: EventLoop<()>) {
    let mut renderrer_runner =
        RendererRunner::new(vec![Box::new(RendererTestTriangle {})], &mut event_loop).await;
    renderrer_runner.run(event_loop);
}
