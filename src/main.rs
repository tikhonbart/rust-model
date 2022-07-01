extern crate kiss3d;
extern crate nalgebra as na;

use std::path::Path;
use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::post_processing::SobelEdgeHighlight;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Isometry3, Point3, Quaternion, Rotation3, Translation3, Unit, UnitQuaternion, Vector3};


fn main() {
    let mut window = Window::new("model");
    window.set_background_color(0.0, 0.0, 0.3);
    window.set_light(Light::StickToCamera);
    //положение камеры
    let eye = Point3::new(60.0, 120.0, 60.0);
    let at = Point3::new(0.0, 50.0, 0.0);
    let mut arc_ball = ArcBall::new(eye, at);

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



    /*let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, 20.0));
    let rot = UnitQuaternion::from_axis_angle(&axis, 0.1);
    link5.prepend_to_local_rotation(&rot);*/




    //вращение
    /*let mut rotat = Isometry3::rotation(Vector3::new(0.0,0.0,0.0));
    link5.set_local_transformation(rotat);*/

    //подсветка
    /*let mut sobel = SobelEdgeHighlight::new(4.0);
    while !window.should_close() {
        //window.render_with_effect(&mut sobel);
    }*/



    while window.render_with_camera(&mut arc_ball) {


        let mut axi = Vector3::z()*-0.014;
        let mut trans = Vector3::new(-0.805, 0.005, 0.0);


        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    if button == kiss3d::event::Key::A{

                        let mut vec = Isometry3::new(trans, axi);
                        link5.prepend_to_local_transformation(&vec);
                        link4.prepend_to_local_transformation(&vec);

                        /*if axi.le(){

                        }*/

                        println!("You pressed the button: {:?}", button);
                    }
                    //event.inhibited = true // override the default keyboard handler
                    if button == kiss3d::event::Key::S{
                        let axisangle = Vector3::z() * 0.0;
                        let translation = Vector3::new(0.0, 9.3, 0.0);
                        let iso = Isometry3::new(translation, axisangle);
                        link5.set_local_transformation(iso);
                        link4.set_local_transformation(iso);


                        println!("You pressed the button: {:?}", button);
                    }
                }
                _ => {}
            }
        }
    }
}
