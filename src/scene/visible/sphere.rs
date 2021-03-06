use crate::common::Intersection;
use crate::common::Ray;
use crate::common::Vec3;
use crate::common::VertexComp;
use crate::scene::visible::Visible;

#[derive(Debug)]
pub struct Sphere<T: VertexComp> {
    center: Vec3<T>,
    radius: T,
}

impl<T: VertexComp> Sphere<T> {
    fn new(center: Vec3<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }
}

impl<T: VertexComp> Visible<T> for Sphere<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        let oc = self.center.sub(&ray.origin);
        let tca = ray.direction.dot(&oc);
        let oc_d2 = oc.mag_sqrd();
        let inside_sphere = oc_d2 < self.radius;

        // Ray does not intersect sphere
        if tca < T::zero() && !inside_sphere {
            return None;
        }

        let thc2 = (self.radius * self.radius) - oc_d2 + (tca * tca);
        // Sphere is behind the ray. Ray does not intersect sphere
        if thc2 < T::zero() {
            return None;
        }

        let t;
        if inside_sphere {
            t = tca + thc2.sqrt();
        } else {
            t = tca - thc2.sqrt();
        }

        Some(Intersection {
            point: ray.origin.add(&ray.direction.mul(t)),
            normal: ray.origin.sub(&self.center).div(self.radius),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // tests that a ray along the x-axis intersects a sphere at the origin at the correct place
    fn intersect_origin_x_axis() {
        let ray = Ray::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));

        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

        let expected_point = Vec3::new(1.0, 0.0, 0.0);

        // Panic of intersect returns none. That's a bug
        let intersection_point = sphere.intersect(&ray).unwrap();

        assert_eq!(expected_point, intersection_point.point)
    }

    #[test]
    // tests a ray intersection that does not pass through the origin of the sphere
    fn intersect() {
        let ray = Ray::new(Vec3::new(10.0, 0.5, 0.0), Vec3::new(-1.0, 0.0, 0.0));

        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

        let expected_point = Vec3::new(0.8660254037844393, 0.5, 0.0);

        // Panic of intersect returns none. That's a bug
        let intersection_point = sphere.intersect(&ray).unwrap();

        assert_eq!(expected_point, intersection_point.point)
    }
}