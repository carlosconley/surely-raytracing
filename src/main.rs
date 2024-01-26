mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod render;
mod interval;
mod utils;
mod material;

// type aliasing
//use std::rc::Rc;
use std::sync::Arc;
use material::{Lambertian, Metal, Dielectric};
use sphere::Sphere;
use hittable::HittableList;
use render::{Camera, render};
use vec3::Point3;
use color::Color;



fn main() {
    // World

    let mut world = HittableList {
        objects: vec![]
    };

    let ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Arc::new(Lambertian::new(
        Color::new(0.1, 0.2, 0.5)
    ));
    let left = Arc::new(Dielectric::new(
        1.5
    ));
    let right = Arc::new(Metal::new(
        Color::new(0.8, 0.6, 0.2),
        0.
    ));

    world.objects.push(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center.clone()

    )));

    world.objects.push(Arc::new(Sphere::new(
        Point3::new(0., -1000.5, -1.),
        1000.,
        ground.clone()
    )));
    world.objects.push(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        left.clone()
    )));

    world.objects.push(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        left.clone()
    )));

    world.objects.push(Arc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right.clone()
    )));

    /*
    // random sphere generation
    for _ in 0..100 {
        world.objects.push(Arc::new(Sphere::new(
            vec3::random_vec3_range(-10., 10.) - Vec3::new(0., 0., 10.5),
            utils::random_range(0.1, 1.0)
        )))
    }*/

    // Camera
    let world = Arc::new(world);
    let cam = Arc::new(Camera::new(16. / 9., 1000, 400, 50, 90.));

    render(cam.clone(), world);

}
