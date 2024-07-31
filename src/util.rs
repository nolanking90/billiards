pub fn f64to32(float64: [f64; 2]) -> [f32; 2] {
    let [f1, f2] = float64;
    [f1 as f32, f2 as f32]
}

pub fn coord_to_pixel(coord: [f32; 2]) -> [f32; 2] {
    let [f1, f2] = coord;
    [f1 * 250., f2 * 250.]
}
