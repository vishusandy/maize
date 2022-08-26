/// use the midpoint circle algorithm to draw a circle
// https://www.geeksforgeeks.org/mid-point-circle-drawing-algorithm/
fn midpoint_circle_algorithm(image: &mut image::RgbaImage, r: i32, color: image::Rgba<u8>) {
    let mut x = r;
    let mut y = 0;
    let mut p = 1 - r;
    while x > y {
        y += 1;
        if p <= 0 {
            p += 2 * y + 1;
        } else {
            x -= 1;
            p += 2 * y - 2 * x + 1;
        }
        if x < y {
            break;
        }
        image.put_pixel(x as u32, y as u32, color);
    }
}
