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
mod rt_image;
mod perlin;


use std::sync::Arc;

// type aliasing
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use object::{Quad, Sphere, Sun};
use hittable::HittableList;
use render::{init_pixels, render_par, Camera};
use texture::{CheckerTexture, ImageTexture, NoiseTexture };
use utils::{random_double, random_range};
use vec3::{Point3, Vec3, random_vec3, random_vec3_range};
use color::Color;




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


    let checker = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

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

    let cam = Camera::new(16. / 9., 500, 400, 50, 20., Point3::new(13., 2., 3.), Point3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 0.6, 10., Color::new(0.7, 0.8, 1.));


    let mut pixels = init_pixels(&cam);

    let world = world.create_bvh();

    crate::render::render_par(&cam, &world, &mut pixels, &vec![]);
}

fn two_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_color(0.3, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Sphere::new(Point3::new(0., -10., 0.), 10., Lambertian::from_texture(checker.clone())));

    world.add(Sphere::new(Point3::new(0., 10., 0.), 10., Lambertian::from_texture(checker)));

    let cam = Camera::new(16. / 9., 400, 100, 50, 20., Point3::new(13., 2., 3.), Point3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 0., 0., Color::new(0.7, 0.8, 1.));

    let mut pixels = init_pixels(&cam);

    render_par(&cam, &world, &mut pixels, &vec![]);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Lambertian::from_texture(earth_texture);
    let globe = Sphere::new(Point3::new_zero(), 2., earth_surface);


    // got stuck on a stupid "bug" reorienting to match the picture...
    let cam = Camera::new(16. / 9., 1000, 1000, 50, 20., Point3::new(13., 3., 2.), Point3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 0., 0., Color::new(0.7, 0.8, 1.));

    let mut pixels = init_pixels(&cam);

    let world = HittableList::from_object(globe);

    render_par(&cam, &world, &mut pixels, &vec![]);
}

fn two_perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.));

    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Lambertian::from_texture(pertext.clone())
    ));

    world.add(Sphere::new(
        Point3::new(0., 2., 0.), 2., Lambertian::from_texture(pertext)
    ));

    let cam = Camera::new(16. / 9., 400, 100, 50, 20., Point3::new(13., 2., 3.), Point3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 0., 0., Color::new(0.6, 0.7, 1.));

    let mut pixels = init_pixels(&cam);

    render_par(&cam, &world, &mut pixels, &vec![]);
}

fn quads() {
    let mut world = HittableList::new();

    let left_red = Lambertian::new(Color::new(1., 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.));
    let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    world.add(Quad::new(Point3::new(-3., -2., 5.), Vec3::new(0., 0., -4.), Vec3::new(0., 4., 0.), left_red));
    world.add(Quad::new(Point3::new(-2., -2., 0.), Vec3::new(4., 0., 0.), Vec3::new(0., 4., 0.), back_green));
    world.add(Quad::new(Point3::new(3., -2., 1.), Vec3::new(0., 0., 4.), Vec3::new(0., 4., 0.), right_blue));
    world.add(Quad::new(Point3::new(-2., 3., 1.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., 4.), upper_orange));
    world.add(Quad::new(Point3::new(-2., -3., 5.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., -4.), lower_teal));


    let cam = Camera::new(
        1.0,
        400,
        100,
        50,
        80.,
        Point3::new(0., 0., 9.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        0.,
        0.,
        Color::new(0.6, 0.7, 1.)
    );


    let mut pixels = init_pixels(&cam);

    render_par(&cam, &world, &mut pixels, &vec![]);

}

fn simple_light() {
    let mut world = HittableList::new();

    let pertex = Arc::new(NoiseTexture::new(4.));
    world.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., Lambertian::from_texture(pertex.clone())));
    world.add(Sphere::new(Point3::new(0., 2., 0.), 2., Lambertian::from_texture(pertex)));

    let difflight = DiffuseLight::new(Color::new(4., 4., 4.));
    world.add(Quad::new(Point3::new(3., 1., -2.), Vec3::new(2., 0., 0.), Vec3::new(0., 2., 0.), difflight.clone()));
    world.add(Sphere::new(Point3::new(0., 7., 0.), 2., difflight));

    let cam = Camera::new(
        16. / 9.,
        400,
        400,
        50,
        20.,
        Point3::new(26., 3., 6.),
        Point3::new(0., 2., 0.),
        Vec3::new(0., 1., 0.),
        0.,
        0.,
        Color::new_zero()
    );


    let mut pixels = init_pixels(&cam);

    render_par(&cam, &world, &mut pixels, &vec![]);
}

fn main() {
    let scene = 6;
    match scene {
        -1 => scene_three_spheres(),
        -2 => scene_sun_spheres(),
        1 => scene_random_balls(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        _ => (),
    }
}
