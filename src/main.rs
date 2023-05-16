use frost_vector::renderer_1_test;

fn main() {
    pollster::block_on(renderer_1_test::run());
    // pollster::block_on(run());
}
