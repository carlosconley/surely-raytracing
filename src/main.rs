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
use std::rc::Rc;
use material::{Lambertian, PerfectLambertian};
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

    let teal = Rc::new(Lambertian::new(Color::new(0.3, 0.7, 0.0)));
    let magenta = Rc::new(Lambertian::new(Color::new(0.9, 0.2, 0.1)));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        teal.clone()

    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        magenta.clone()
    )));

    /*
    // random sphere generation
    for _ in 0..100 {
        world.objects.push(Rc::new(Sphere::new(
            vec3::random_vec3_range(-10., 10.) - Vec3::new(0., 0., 10.5),
            utils::random_range(0.1, 1.0)
        )))
    }*/

    // Camera
    let cam = Camera::new(16. / 9., 400, 100, 25);

    render(&cam, &world);

}
