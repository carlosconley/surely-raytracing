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
use material::{Lambertian, Metal, Dielectric, Material};
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

    let ground = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Material::Lambertian(Lambertian::new(
        Color::new(0.1, 0.2, 0.5)
    ));
    let left = Material::Dielectric(Dielectric::new(
        1.5
    ));
    let right = Material::Metal(Metal::new(
        Color::new(0.8, 0.6, 0.2),
        0.
    ));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center

    )));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., -1000.5, -1.),
        1000.,
        ground
    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        left
    )));

    world.objects.push(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right
    )));

    // Camera
    let cam = Camera::new(16. / 9., 600, 1000, 50, 90.);

    render(&cam, &world);

}
