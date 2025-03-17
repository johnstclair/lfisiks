pub fn point_to_buffer(mut x: f32, mut y: f32, rows: usize, cols: usize) -> Option<usize> {
    if x <= 0.0 || y <= 0.0 || x >= rows as f32 || y >= cols as f32 {
        return None;
    }
    let (x, y): (usize, usize) = (x.floor() as usize, y.floor() as usize);
    Some(y * rows + x)
}
