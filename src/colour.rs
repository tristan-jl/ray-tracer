use crate::vec3::Colour;

pub fn write_colour(pixel_colour: Colour) {
    let x = (255.999 * pixel_colour.x()) as i64;
    let y = (255.999 * pixel_colour.y()) as i64;
    let z = (255.999 * pixel_colour.z()) as i64;

    println!("{} {} {}", x, y, z)
}
