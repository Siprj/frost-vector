use log::info;
use winit::event_loop::EventLoop;

use crate::render_common::RendererRunner;
use crate::renderer_1::Renderer1;

pub async fn run(mut event_loop: EventLoop<()>, circles: &Vec<(f32, f32, f32, f32)>, rectangles: &Vec<(f32, f32, f32, f32, f32)>) {
    info!("running the renderer_1 test");
    let mut renderer = Renderer1::new();
    info!("Renderer1 created");
    renderer.circles(&circles);
    renderer.rectangles(&rectangles);
    info!("shapes in the renderer");
    let mut renderrer_runner = RendererRunner::new(vec![Box::new(renderer)], &mut event_loop).await;
    info!("Renderer runner with Renderer1 ready to go");
    renderrer_runner.run(event_loop);
}
