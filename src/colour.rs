use crate::{utils::clamp, vec3::Colour};

pub fn write_colour(pixel_colour: Colour, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * pixel_colour.x()).sqrt();
    let g = (scale * pixel_colour.y()).sqrt();
    let b = (scale * pixel_colour.z()).sqrt();

    println!(
        "{} {} {}",
        (256. * clamp(r, 0.0, 0.999)) as i32,
        (256. * clamp(g, 0.0, 0.999)) as i32,
        (256. * clamp(b, 0.0, 0.999)) as i32
    )
}
