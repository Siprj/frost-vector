use frost_vector::renderer_1_test;
use frost_vector::renderer_2_test;
use winit::event_loop::EventLoop;

const ELEMENT_COUNT: usize = 200_000;
//const ELEMENT_COUNT: usize = 1;

fn main() {
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

    let mut event_loop = EventLoop::new();
    pollster::block_on(renderer_1_test::run(&mut event_loop, &circles, &rectangles));
    pollster::block_on(renderer_2_test::run(&mut event_loop, &circles, &rectangles));
}
