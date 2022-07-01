extern crate kiss3d;
extern crate nalgebra as na;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Translation3, UnitQuaternion, Vector3};
use std::path::Path;
use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::scene::Object;

fn main() {
    let mut window = Window::new("model");
    window.set_background_color(0.0, 0.0, 0.3);
    window.set_light(Light::StickToCamera);

    //ссылки на 3д объекты
    let base_path = Path::new("media/base.obj");
    let link1_path = Path::new("media/new Link1.obj");
    let link2_path = Path::new("media/new Link2.obj");
    let link3_path = Path::new("media/new Link3.obj");
    let link4_path = Path::new("media/new Link4.obj");
    let link5_path = Path::new("media/new Link5.obj");
    let mtl_path = Path::new("media/");

    //создание 3д объектов
    let mut base = window.add_obj(base_path, mtl_path, Vector3::new(2.54, 2.54, 2.54));
    let mut link1 = window.add_obj(link1_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));
    let mut link2 = window.add_obj(link2_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));
    let mut link3 = window.add_obj(link3_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));
    let mut link4 = window.add_obj(link4_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));
    let mut link5 = window.add_obj(link5_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));

    //позиционирование
    link1.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    link2.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    link3.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    link4.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    link5.append_translation(&Translation3::new(0.0, 9.3, 0.0));


    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut first_person = FirstPerson::new(eye, at);
    let mut arc_ball = ArcBall::new(eye, at);
    let mut use_arc_ball = true;

    while !window.should_close() {
        // update the current camera.
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(key, Action::Release, _) => {
                    if key == Key::Numpad1 {
                        use_arc_ball = true
                    } else if key == Key::Numpad2 {
                        use_arc_ball = false
                    }
                }
                _ => {}
            }
        }
        window.draw_line(
            &Point3::origin(),
            &Point3::new(1.0, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 0.0),
        );
        window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 1.0, 0.0),
            &Point3::new(0.0, 1.0, 0.0),
        );
        window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 0.0, 1.0),
            &Point3::new(0.0, 0.0, 1.0),
        );

        if use_arc_ball {
            window.render_with_camera(&mut arc_ball);
        } else {
            window.render_with_camera(&mut first_person);
        }
    }

}