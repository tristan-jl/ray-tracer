use crate::{
    material::material::Material,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{dot, Point3},
};
use std::rc::Rc;

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn from(c: Point3, r: f64, m: Rc<dyn Material>) -> Self {
        Self {
            centre: c,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.centre;
        let a = r.direction().squared();
        let half_b = dot(oc, r.direction());
        let c = oc.squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
