mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod render;
mod interval;
mod utils;

// type aliasing
use std::rc::Rc;
use sphere::Sphere;
use hittable::HittableList;
use render::{Camera, render};
use vec3::{Point3, Vec3};



fn main() {
    // World
    let mut world = HittableList {
        objects: vec![]
    };
    /* 
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5
    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.
    )));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(-1., -1., -1.),
        1.
    )));
    */
    for _ in 0..100 {
        world.objects.push(Rc::new(Sphere::new(
            vec3::random_vec3_range(-10., 10.) - Vec3::new(0., 0., 10.5),
            utils::random_double()
        )))
    }
    // Camera
    let cam = Camera::new(16. / 9., 400, 100);

    render(&cam, &world);

}
