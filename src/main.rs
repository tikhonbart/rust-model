extern crate kiss3d;
extern crate nalgebra as na;

use std::borrow::{Borrow, BorrowMut};
use std::path::Path;
use std::sync::mpsc::Sender;
use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::ncollide3d::interpolation::{RigidMotion, RigidMotionComposition};
use kiss3d::post_processing::SobelEdgeHighlight;
use kiss3d::scene::{Object, SceneNode};
use kiss3d::window::{Canvas, Window};
use na::{Isometry3, Point3, Quaternion, Rotation3, SimdValue, Translation3, Unit, UnitComplex, UnitQuaternion, Vector3};


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
    //let mut link4 = window.add_obj(link4_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));
    //let mut link5 = window.add_obj(link5_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));

    let mut top_group :SceneNode = window.add_group();
    top_group.add_obj(link4_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));
    top_group.add_obj(link5_path, mtl_path, Vector3::new(0.1, 0.1, 0.1));

    //позиционирование
    link1.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    link2.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    link3.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    //link4.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    //link5.append_translation(&Translation3::new(0.0, 9.3, 0.0));
    top_group.append_translation(&Translation3::new(0.0, 9.3, 0.0));

    let mut objct = 0;

    while window.render_with_camera(&mut arc_ball) {

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(kiss3d::event::Key::A, Action::Press, _) => {

                    link1.set_color(1.0,1.0,1.0);
                    link2.set_color(1.0,1.0,1.0);
                    link3.set_color(1.0,1.0,1.0);
                    top_group.set_color(1.0,0.0,0.0);
                    objct = 1;

                }
                WindowEvent::Key(kiss3d::event::Key::S, Action::Press, _) => {
                    link1.set_color(1.0,1.0,1.0);
                    link2.set_color(1.0,1.0,1.0);
                    link3.set_color(1.0,0.0,0.0);
                    top_group.set_color(1.0,1.0,1.0);
                    objct = 2;
                }
                WindowEvent::Key(kiss3d::event::Key::D, Action::Press, _) => {
                    link1.set_color(1.0,1.0,1.0);
                    link2.set_color(1.0,0.0,0.0);
                    link3.set_color(1.0,1.0,1.0);
                    top_group.set_color(1.0,1.0,1.0);
                    objct = 3;
                }
                WindowEvent::Key(kiss3d::event::Key::F, Action::Press, _) => {
                    link1.set_color(1.0,0.0,0.0);
                    link2.set_color(1.0,0.0,0.0);
                    link3.set_color(1.0,0.0,0.0);
                    top_group.set_color(1.0,0.0,0.0);
                    objct = 4;
                }
                WindowEvent::Key(kiss3d::event::Key::C, Action::Press, _) => {
                    let axisangle = Vector3::z() * 0.0;
                    let translation = Vector3::new(0.0, 9.3, 0.0);
                    let iso = Isometry3::new(translation, axisangle);
                    link1.set_local_transformation(iso);
                    link2.set_local_transformation(iso);
                    link3.set_local_transformation(iso);
                    top_group.set_local_transformation(iso);
                }
                WindowEvent::Key(kiss3d::event::Key::Z, Action::Press, _) => {
                    match &mut objct {
                        1 => {
                            &mut top_group.prepend_to_local_transformation(&Isometry3::new(
                                Vector3::new(-0.805, 0.005, 0.0),
                                Vector3::z()*-0.014));
                        },
                        2 => {
                            &mut link3.prepend_to_local_transformation(&Isometry3::new(
                                Vector3::new(-0.525, 0.004, 0.0),
                                Vector3::z()*-0.014));

                            &mut top_group.append_transformation(&Isometry3::new(
                                Vector3::new(-0.656, 0.005, 0.0),
                                Vector3::z()*-0.014));
                        },
                        3 => {

                            &mut link2.prepend_to_local_transformation(&Isometry3::new(
                                Vector3::new(-0.21, 0.0, 0.0),
                                Vector3::z()*-0.014));
                            &mut link3.append_transformation(&Isometry3::new(
                                Vector3::new(-0.34, 0.003, 0.0),
                                Vector3::z()*-0.014));
                            &mut top_group.append_transformation(&Isometry3::new(
                                Vector3::new(-0.34, 0.003, 0.0),
                                Vector3::z()*-0.014));
                        },
                        4 => {
                            &mut link1.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*-0.014));
                            &mut link2.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*-0.014));
                            &mut link3.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*-0.014));
                            &mut top_group.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*-0.014));
                        }
                        _ => println!("none")
                    }

                }
                WindowEvent::Key(kiss3d::event::Key::X, Action::Press, _) => {
                    match &mut objct {
                        1 => {
                            &mut top_group.prepend_to_local_transformation(&Isometry3::new(
                                Vector3::new(0.805, 0.005, 0.0),
                                Vector3::z()*0.014));

                        },
                        2 => {
                            &mut link3.prepend_to_local_transformation(&Isometry3::new(
                                Vector3::new(0.525, 0.004, 0.0),
                                Vector3::z()*0.014));

                            &mut top_group.append_transformation(&Isometry3::new(
                                Vector3::new(0.625, 0.0, 0.0),
                                Vector3::z()*0.014));
                            //&mut top_group.prepend_to_local_rotation(&cord1);

                        },
                        3 => {

                            &mut link2.prepend_to_local_transformation(&Isometry3::new(
                                Vector3::new(0.21, 0.0, 0.0),
                                Vector3::z()*0.014));
                            &mut link3.append_transformation(&Isometry3::new(
                                Vector3::new(0.34, -0.003, 0.0),
                                Vector3::z()*0.014));
                            &mut top_group.append_transformation(&Isometry3::new(
                                Vector3::new(0.34, -0.003, 0.0),
                                Vector3::z()*0.014));
                        },
                        4 => {
                            &mut link1.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*0.014));
                            &mut link2.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*0.014));
                            &mut link3.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*0.014));
                            &mut top_group.append_transformation(&Isometry3::new(
                                Vector3::new(0.0, 0.0, 0.0),
                                Vector3::y()*0.014));
                        }
                        _ => println!("none")
                    }

                }
                _ => {}
            }
        }
    }
}