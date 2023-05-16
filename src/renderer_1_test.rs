use crate::renderer_1;

pub async fn run() {
    let mut circles = Vec::new();
    // for _ in 0..10 {
    for _ in 0..100 {
        circles.push((
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 800_f32,
            rand::random::<f32>() * 100_f32,
        ));
    }

    renderer_1::run(
        |_| false,
        move |draw| {
            // println!("run handler!");
            circles.iter().for_each(|c| draw.circle(c.0, c.1, c.2));
        },
    )
    .await
}
