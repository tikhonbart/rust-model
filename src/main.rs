extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Translation3, UnitQuaternion, Vector3};
use std::f32;
use std::path::Path;

fn main() {
    let mut window = Window::new("model");
    window.set_background_color(0.0, 0.0, 0.3);

    let obj_path = Path::new("media/base.obj");
    let mtl_path = Path::new("media/");
    let mut c = window.add_obj(obj_path, mtl_path, Vector3::new(1.0, 1.0, 1.0));
    c.append_translation(&Translation3::new(0.0, -0.05, -0.2));



    window.set_light(Light::StickToCamera);

    let rot_teapot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);


    while window.render() {
        c.prepend_to_local_rotation(&rot_teapot);

    }
}
