mod color;
mod vec3;
mod ray;
mod hittable;
mod object;
mod render;
mod interval;
mod utils;
mod material;
mod texture;


// type aliasing
use material::{Lambertian, Metal, Dielectric};
use object::{Sphere, Sun};
use hittable::HittableList;
use render::{Camera, init_pixels};
use texture::CheckerTexture;
use utils::{random_double, random_range};
use vec3::{Point3, Vec3, random_vec3, random_vec3_range};
use color::Color;


fn main() {
    scene_random_balls();
}

fn scene_sun_spheres() {
    let mut world = HittableList::new();

    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));

    let center = Lambertian::new(
        Color::new(0.1, 0.2, 0.5)
    );
    let left = Dielectric::new(
        1.5, Color::new(1., 1., 1.)
    );
    let right = Metal::new(
        Color::new(0.8, 0.6, 0.2),
        0.
    );

    // let binding for testing
    let center_sphere = Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center
    );

    world.add(center_sphere);

    world.add(Sphere::new(
        Point3::new(-1., 0., -1.25),
        0.5,
        left.clone()
    ));

    world.add(Sphere::new(
        Point3::new(-1., 0., -1.25),
        -0.4,
        left
    ));

    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        ground
    ));

    world.add(Sphere::new(
        Point3::new(1., 0., -0.75),
        0.5,
        right
    ));

    // High sample count required to get a not-too-grainy image because of non-light sampling rng
    let mut cam = Camera::new(16. / 9., 640, 1000, 50, 90., Point3::new(0., 0., 0.), Point3::new(0., 0., -1.), Vec3::new(0., 1., 0.), 0., 1., Color::new(0.02, 0.05, 0.1));

    // turn on super secret hidden auto_exposure so to adjust for wacky sun brightnesses
    cam.auto_exposure = true;

    let mut pixels = init_pixels(&cam);

//    eprintln!("Building BHV");

    //let world = world.create_bvh();

    // this takes about 1.5 minutes on my m2 with 8 cores
    // make sun super bright so that we accentuate shadows, showing off our nifty sun simulation!
    crate::render::render_par(&cam, &world, &mut pixels, &vec![Sun::new(Vec3::new(-1., 1., 1.), Color::new(1., 1., 1.) * 10., 2.)]);

}

fn scene_three_spheres() {
    let mut world = HittableList::new();

    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));

    let center = Lambertian::new(
        Color::new(0.1, 0.2, 0.5)
    );
    let left = Dielectric::new(
        1.5, Color::new(1.0, 0.9, 0.8)
    );
    let right = Metal::new(
        Color::new(0.8, 0.6, 0.2),
        0.
    );

    // let binding for testing
    let center_sphere = Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center
    );

    world.add(center_sphere);

    world.add(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        left.clone()
    ));

    world.add(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        left
    ));

    world.add(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        ground
    ));

    world.add(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right
    ));

    let cam = Camera::new(16. / 9., 800, 1000, 50, 90., Point3::new(0., 0., 0.), Point3::new(0., 0., -1.), Vec3::new(0., 1., 0.), 2., 1., Color::new(0.7, 0.8, 1.));

    let mut pixels = init_pixels(&cam);

    let world = world.create_bvh();

    crate::render::render_par(&cam, &world, &mut pixels, &vec![]);
}

fn scene_random_balls() {
    let mut world = HittableList::new();


    let checker = CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    let ground_material = Lambertian::from_texture(checker);

    world.add(Sphere::new(Point3::new(0., -2000., 0.), 2000., ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = random_double();
            let center = Point3::new(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());
            let center2 = center + Vec3::new(0., random_range(0., 0.5), 0.);

            if (center - Point3::new(4., 0.2, 0.)).length_squared() > (0.9 * 0.9) {
                if choose_mat < 0.8 {
                    let albedo: Color = random_vec3() * random_vec3();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));

                    // world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let ir = random_range(1.2, 1.6);
                    let sphere_material = Dielectric::new(ir, Color::new(1., 1., 1.));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }

        }
    }

    let material1 = Dielectric::new(1.5, Color::new(1., 1., 1.));
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(Point3::new(0., 1., 0.), 1.0, material1));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2));
    world.add(Sphere::new(Point3::new(4., 1., 0.), 1.0, material3));
    // Camera

    let cam = Camera::new(16. / 9., 400, 100, 50, 20., Point3::new(13., 2., 3.), Point3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 0.6, 10., Color::new(0.7, 0.8, 1.));


    let mut pixels = init_pixels(&cam);

    let world = world.create_bvh();

    crate::render::render_par(&cam, &world, &mut pixels, &vec![]);
}