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

    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(
        Color::new(0.1, 0.2, 0.5)
    ));
    let left = Rc::new(Dielectric::new(
        1.5
    ));
    let right = Rc::new(Metal::new(
        Color::new(0.8, 0.6, 0.2),
        0.
    ));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center.clone()

    )));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., -1000.5, -1.),
        1000.,
        ground.clone()
    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        left.clone()
    )));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        left.clone()
    )));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right.clone()
    )));

    // Camera
    let cam = Camera::new(16. / 9., 800, 100, 50, 90.);

    render(&cam, &world);

}
