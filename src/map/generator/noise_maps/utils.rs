use crate::map::chunk::Chunk;

pub const VERTICAL_SCALE: f32 = 100.0;

pub trait Noise2D<T> {
    fn get(&self, x: i32, z: i32) -> T;
}

pub trait Noise3D<T> {
    fn get(&self, x: i32, y: u8, z: i32) -> T;

    fn get_zoom(&self) -> f64;
    fn get_chunk_pos(&self) -> (f64, f64);

    fn get_pos(&self, x: i32, y: u8, z: i32) -> (f64, f64, f64) {
        let (ch_x, ch_z) = self.get_chunk_pos();
        let fx = (x as f64) * self.get_zoom() / Chunk::LENGTH as f64 + ch_x;
        let fy = (y as f64) * self.get_zoom() / 2.0;
        let fz = (z as f64) * self.get_zoom() / Chunk::WIDTH as f64 + ch_z;

        (fx, fy, fz)
    }
}
