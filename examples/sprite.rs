extern crate crayon;
extern crate crayon_workflow;

extern crate cgmath;
extern crate rand;

use crayon::scene::camera::{Camera, Projection};
use crayon::scene::scene2d::Scene2d;
use cgmath as math;
use rand::{Rng, SeedableRng, XorShiftRng};

use crayon::graphics::Color;
use crayon::scene::{Sprite, Transform, Rect};
use crayon::resource;

mod prelude;

#[derive(Debug)]
struct SpriteParticle {
    lifetime: f32,
    velocity: math::Vector3<f32>,
    acceleration: math::Vector3<f32>,
    rotation_vel: math::Quaternion<f32>,
    color: Color,
    size: math::Vector2<f32>,
    handle: crayon::ecs::Entity,
}

fn main() {
    let mut scene: Option<Scene2d> = None;
    let mut atlas: Option<resource::AtlasPtr> = None;
    let mut particles = vec![];
    let mut cal = XorShiftRng::from_seed([0, 1, 2, 3]);

    prelude::compile();

    let mut settings = crayon::core::settings::Settings::default();
    settings.engine.max_fps = 60;
    settings.window.width = 640;
    settings.window.height = 480;

    crayon::Application::new_with(settings)
        .unwrap()
        .perform(|mut app| {
            scene = {
                let mut v = Scene2d::new(&mut app).unwrap();

                {
                    // Create and bind main camera of scene2d.
                    let c = Scene2d::camera(&mut v.world_mut());
                    v.set_main_camera(c);

                    {
                        let dimensions = app.window.dimensions().unwrap();
                        let mut camera = v.world_mut().fetch_mut::<Camera>(c).unwrap();
                        camera.set_aspect(dimensions.0 as f32 / dimensions.1 as f32);
                        camera.set_projection(Projection::Ortho(dimensions.1 as f32 * 0.5));
                    }

                    {
                        let mut arena = v.world_mut().arena::<Transform>().unwrap();
                        let mut position = Transform::world_position(&arena, c).unwrap();
                        position.z = 10f32;
                        Transform::set_world_position(&mut arena, c, position).unwrap();
                    }
                }

                for _ in 0..500 {
                    particles.push(None);
                }

                app.resources
                    .load_manifest("examples/compiled-resources/manifest")
                    .unwrap();

                atlas = Some(app.resources.load("atlas.json").unwrap());
                Some(v)

            };
        })
        .run(move |mut app| {
            if let Some(ref mut v) = scene {
                {
                    let mut world = &mut v.world_mut();
                    // Creates sprites randomly.
                    for i in &mut particles {
                        if i.is_none() {
                            let spr = {
                                let size = (cal.gen::<u32>() % 40) as f32 + 20.0;
                                SpriteParticle {
                                    lifetime: (cal.gen::<u32>() % 5) as f32,
                                    velocity: math::Vector3::new((cal.gen::<i32>() % 5) as f32 + 1f32,
                                                                 (cal.gen::<i32>() % 5) as f32 + 1f32,
                                                                 0f32),
                                    rotation_vel: math::Quaternion::from(math::Euler {
                                                                             x: math::Deg((cal.gen::<i32>() % 10) as f32),
                                                                             y: math::Deg((cal.gen::<i32>() % 10) as f32),
                                                                             z: math::Deg((cal.gen::<i32>() % 10) as f32),
                                                                         }),
                                    acceleration: math::Vector3::new((cal.gen::<i32>() % 5) as
                                                                     f32 + 1f32,
                                                                     (cal.gen::<i32>() % 5) as
                                                                     f32 + 1f32,
                                                                     0f32),
                                    color: [cal.gen::<u8>(), cal.gen::<u8>(), cal.gen::<u8>(), 255]
                                        .into(),
                                    size: math::Vector2::new(size, size),
                                    handle: Sprite::new(&mut world),
                                }
                            };

                            {
                                let mut sprite = world.fetch_mut::<Sprite>(spr.handle).unwrap();
                                sprite.set_color(&spr.color);

                                let mut rect = world.fetch_mut::<Rect>(spr.handle).unwrap();
                                rect.set_size(&spr.size);
                                rect.set_pivot(math::Vector2::new(0.5f32, 0.5f32));

                                if let Some(ref atlas) = atlas {
                                    let name = format!("y{:?}.png", cal.gen::<u32>() % 10);
                                    let frame = atlas
                                        .read()
                                        .unwrap()
                                        .frame(&mut app.resources, &name)
                                        .unwrap();

                                    sprite.set_texture_rect(frame.position, frame.size);
                                    sprite.set_texture(Some(frame.texture));

                                }
                            }

                            *i = Some(spr);
                            break;
                        }
                    }
                }

                let mut removes = vec![];

                {
                    let dt = app.engine.timestep_in_seconds();
                    let world = &mut v.world_mut();
                    let (_, mut arenas) = world.view_with_2::<Transform, Sprite>();
                    for (i, w) in particles.iter_mut().enumerate() {
                        if let Some(ref mut particle) = *w {
                            particle.velocity += particle.acceleration * dt;

                            {
                                let transform = arenas.0.get_mut(particle.handle).unwrap();
                                transform.translate(particle.velocity);
                                transform.rotate(particle.rotation_vel);
                            }

                            particle.lifetime -= dt;
                            if particle.lifetime < 0.0 {
                                removes.push(i);
                            }
                        }
                    }
                }

                {
                    let mut world = &mut v.world_mut();
                    for i in removes {
                        if let Some(ref v) = particles[i] {
                            world.free(v.handle);
                        }
                        particles[i] = None;
                    }
                }

                v.run_one_frame(&mut app).unwrap();
            }
            return true;
        });
}