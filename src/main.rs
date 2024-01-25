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
use material::{Lambertian, Metal};
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

    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let right = Rc::new(Metal::new(
        Color::new(0.8, 0.6, 0.2)
    ));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center.clone()

    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        ground.clone()
    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        left.clone()
    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right.clone()
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
    let cam = Camera::new(16. / 9., 1280, 400, 25);

    render(&cam, &world);

}
