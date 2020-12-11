use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    utils::random_f64,
    vec3::{dot, reflect, refract, Colour, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Colour,
}
impl Lambertian {
    pub fn from(colour: Colour) -> Self {
        Self { albedo: colour }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Colour::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::from(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}
impl Metal {
    pub fn from(a: Colour, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        *scattered = Ray::from(rec.p, reflected + Vec3::random_unit_vector() * self.fuzz);
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f64,
}
impl Dielectric {
    pub fn from(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 * r0 + (1. - r0 * r0) * (1. - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Colour::from(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = dot(-unit_direction, rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > random_f64(0., 1.)
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::from(rec.p, direction);
        true
    }
}
