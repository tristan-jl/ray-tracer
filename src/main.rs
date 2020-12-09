mod colour;
mod objects;
mod ray;
mod utils;
mod vec3;
use objects::{hittable, hittable_list, sphere};
use std::rc::Rc;

fn ray_colour(r: &ray::Ray, world: &impl hittable::Hittable) -> vec3::Colour {
    let mut rec = hittable::HitRecord::new();

    if world.hit(r, 0., utils::INFINITY, &mut rec) {
        return (rec.normal + vec3::Colour::from(1., 1., 1.)) * 0.5;
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    vec3::Colour::from(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Colour::from(0.5, 0.7, 1.0) * t
}

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = (256 as f64 / ASPECT_RATIO) as i32;

    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(sphere::Sphere::from(
        vec3::Point3::from(0., 0., -1.),
        0.5,
    )));
    world.add(Rc::new(sphere::Sphere::from(
        vec3::Point3::from(0., -100.5, -1.),
        100.,
    )));

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::from(0., 0., 0.);
    let horizontal = vec3::Vec3::from(viewport_width, 0., 0.);
    let vertical = vec3::Vec3::from(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - vec3::Vec3::from(0., 0., focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = ray::Ray::from(
                &origin,
                &(lower_left_corner + horizontal * u + vertical * v - origin),
            );
            let pixel_colour = ray_colour(&r, &world);

            colour::write_colour(pixel_colour);
        }
    }
    eprintln!("\nDone.\n")
}
