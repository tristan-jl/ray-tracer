mod camera;
mod colour;
mod material;
mod objects;
mod ray;
mod utils;
mod vec3;

use camera::Camera;
use material::material::{Dielectric, Lambertian, Material, Metal};
use objects::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    sphere::Sphere,
};
use ray::Ray;
use std::io::prelude::*;
use std::{fs::File, rc::Rc};
use utils::{random_f64, INFINITY};
use vec3::{unit_vector, Colour, Point3, Vec3};

fn ray_colour(r: Ray, world: &impl Hittable, depth: i32) -> vec3::Colour {
    if depth <= 0 {
        return Colour::new();
    }

    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Colour::new();

        if rec
            .mat_ptr
            .scatter(r, rec.clone(), &mut attenuation, &mut scattered)
        {
            return attenuation * ray_colour(scattered, world, depth - 1);
        }

        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return ray_colour(Ray::from(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Colour::from(1.0, 1.0, 1.0) * (1.0 - t) + Colour::from(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::from(Lambertian::from(Colour::from(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::from(
        Point3::from(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64(0., 1.);
            let centre = Point3::from(
                a as f64 + 0.9 * random_f64(0., 1.),
                0.2,
                b as f64 + 0.9 * random_f64(0., 1.),
            );

            if (centre - Point3::from(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Colour::random(0., 1.) * Colour::random(0., 1.);
                    sphere_material = Rc::new(Lambertian::from(albedo));
                    world.add(Rc::new(Sphere::from(centre, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random(0.5, 1.);
                    let fuzz = random_f64(0., 0.5);
                    sphere_material = Rc::new(Metal::from(albedo, fuzz));
                    world.add(Rc::new(Sphere::from(centre, 0.2, sphere_material)));
                } else {
                    sphere_material = Rc::new(Dielectric::from(1.5));
                    world.add(Rc::new(Sphere::from(centre, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::from(1.5));
    world.add(Rc::new(Sphere::from(
        Point3::from(0., 1., 0.),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::from(Colour::from(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::from(
        Point3::from(-4., 1., 0.),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::from(Colour::from(0.7, 0.6, 0.5), 0.));
    world.add(Rc::new(Sphere::from(
        Point3::from(4., 1., 0.),
        1.0,
        material3,
    )));

    world
}

fn main() {
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_WIDTH: i32 = 120;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 3;
    const MAX_DEPTH: i32 = 50;

    let world = random_scene();
    let camera = Camera::from(
        Point3::from(13., 2., 3.),
        Point3::from(0., 0., 0.),
        Point3::from(0., 1., 0.),
        20.,
        ASPECT_RATIO,
        0.1,
        10.,
    );

    let mut file = File::create("result/result.ppm").expect("Unable to create file");
    write!(file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("Unable to write to file");

    fn calculate_pixel_colour(camera: &Camera, world: &impl Hittable, i: i32, j: i32) -> Colour {
        let u = (i as f64 + random_f64(0., 1.)) / (IMAGE_WIDTH - 1) as f64;
        let v = (j as f64 + random_f64(0., 1.)) / (IMAGE_HEIGHT - 1) as f64;
        let r = camera.get_ray(u, v);

        ray_colour(r, world, MAX_DEPTH)
    }

    for row_num in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rRows remaining: {}/{} ", IMAGE_HEIGHT - row_num, IMAGE_HEIGHT);
        for pixel_num in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                pixel_colour += calculate_pixel_colour(&camera, &world, pixel_num, row_num);
            }

            let (r, g, b) = colour::rescale_colour(pixel_colour, SAMPLES_PER_PIXEL);
            write!(file, "{} {} {}\n", r, g, b).expect("Unable to write to file");
        }
    }
    eprintln!("\nDone.\n")
}
