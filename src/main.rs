mod color;
mod vec3;
mod ray;
mod hittable;
mod object;
mod render;
mod interval;
mod utils;
mod material;

// type aliasing
use material::{Lambertian, Metal, Dielectric};
use object::Sphere;
use hittable::HittableList;
use render::init_pixels;
use render::{Camera, render_par, render};
use utils::random_double;
use utils::random_range;
use vec3::random_vec3;
use vec3::random_vec3_range;
use vec3::{Point3, Vec3};
use color::Color;


fn main() {
    // World

    let mut world = HittableList {
        objects: vec![]
    };

    /*let ground = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));

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

    // let binding for testing
    let center_sphere = Object::Sphere(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center
    ));

    world.objects.push(center_sphere);

    world.objects.push(Object::Sphere(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        ground
    )));

    world.objects.push(Object::Sphere(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        left.clone()
    )));

    world.objects.push(Object::Sphere(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        left
    )));


    world.objects.push(Object::Sphere(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right
    )));*/

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.objects.push(Sphere::new(Point3::new(0., -2000., 0.), 2000., ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = random_double();   
            let center = Point3::new(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());

            if (center - Point3::new(4., 0.2, 0.)).length_squared() > (0.9 * 0.9) {
                if choose_mat < 0.8 {
                    let albedo: Color = random_vec3() * random_vec3();
                    let sphere_material = Lambertian::new(albedo);
                    world.objects.push(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.objects.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let ir = random_range(1.2, 1.6); 
                    let sphere_material = Dielectric::new(ir);
                    world.objects.push(Sphere::new(center, 0.2, sphere_material));
                }
            }

        }
    }

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    
    world.objects.push(Sphere::new(Point3::new(0., 1., 0.), 1.0, material1));
    world.objects.push(Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2));
    world.objects.push(Sphere::new(Point3::new(4., 1., 0.), 1.0, material3));
    // Camera
    let cam = Camera::new(16. / 9., 400, 100, 40, 20., Point3::new(13., 2., 3.), Point3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 0., 10.);

    let mut pixels = init_pixels(&cam);
    render_par(&cam, &world, &mut pixels);

}
