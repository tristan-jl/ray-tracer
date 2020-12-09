use crate::{objects::hittable::*, ray::Ray, vec3};

pub struct Sphere {
    pub centre: vec3::Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn from(c: vec3::Point3, r: f64) -> Self {
        Self {
            centre: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.centre;
        let a = r.direction().squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }

        let root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
