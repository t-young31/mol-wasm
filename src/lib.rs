mod atom;
mod position;
mod f32;
mod molecule;
mod traits;
mod bond;

extern crate kiss3d;
extern crate web_sys;
extern crate nalgebra as na;

use std::vec;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::event::Modifiers;
use wasm_bindgen::prelude::*;

use kiss3d::light::Light;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use na::{Point3, UnitQuaternion};

use crate::molecule::Molecule;
use crate::position::Position;
use crate::traits::Square;

struct Scene {
    nodes: Vec<SceneNode>,
    camera: Box<ArcBall>,
}

impl Scene {
    pub fn com(&self) -> Point3<f32>{

        let mut com = na::Point3::origin();
        for node in self.nodes.iter(){
            com += node.data().local_translation().vector;
        }
        com /= self.nodes.len() as f32;
        com
    }
}

impl State for Scene {

    // Method called at each render loop
    fn step(&mut self, _window: &mut Window) {

        // Set the camera at the ~centre of mass
        let com = self.com();
        self.camera.as_mut().set_at(com);

        // console::log_1(&JsValue::from_str(format!("{}", self.tmp_n).as_str()));
    }

    fn cameras_and_effect_and_renderer(&mut self) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>) {
        (Some(&mut *self.camera), None, None, None)
    }
}


fn add_molecule(molecule: &Molecule, window: &mut Window, scene: &mut Scene) {

    const MIN_SPHERE_RADIUS: f32 = 0.3;
    const CYLINDER_WIDTH: f32 = 0.1;

    let mut com = Position::default();

    // Add atoms
    for atom in molecule.atoms.iter(){

        let r_i = &atom.position;
        let sphere_radius = MIN_SPHERE_RADIUS + 0.3 * atom.covalent_radius();
        let mut node = window.add_sphere(sphere_radius);
        node.append_translation(&r_i.to_translation());
        com += r_i;

        let color = atom.color();
        node.set_color(
            (color.0 as f32 / 255.0 + 0.25).min(1.0),
            (color.1 as f32 / 255.0 + 0.25).min(1.0),
            (color.2 as f32 / 255.0 + 0.25).min(1.0)
        );
        scene.nodes.push(node);
    }
    com /= molecule.atoms.len() as f32;

    // Add bonds
    for bond in molecule.bonds.iter(){
        let i = bond.i;
        let j = bond.j;
        let r_i = &molecule.atoms.get(i).unwrap().position;
        let r_j = &molecule.atoms.get(j).unwrap().position;

        let dist = ((r_i.x - r_j.x).sq() + (r_i.y - r_j.y).sq() + (r_i.z - r_j.z).sq()).sqrt();

        // Centred at (0, 0, 0) aligned with the y axis
        let mut node = window.add_cylinder(CYLINDER_WIDTH, dist);

        let r_ij = na::Vector3::new(r_j.x - r_i.x, r_j.y - r_i.y, r_j.z - r_i.z) / dist;
        let y_axis = na::Vector3::new(0.0, 1.0, 0.0);
        node.append_rotation(&UnitQuaternion::rotation_between(&y_axis, &r_ij).unwrap());

        node.append_translation(&r_i.midpoint(&r_j).to_translation());

        node.set_color(0.99, 0.99, 0.99);
        scene.nodes.push(node);
    }
}

#[wasm_bindgen]
pub fn render_molecule(xyz_lines: &str) -> Result<(), JsValue> {

    let mut window = Window::new("wam-rs");
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    let eye = Point3::new(3.0f32, 3.0, 3.0);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);
    camera.set_drag_modifiers(Some(Modifiers::Shift));

    let mut state = Scene { nodes: vec![], camera: Box::new(camera)};
    let molecule = Molecule::from_xyz_file(xyz_lines);
    add_molecule(&molecule, &mut window, &mut state);

    window.render_loop(state);
    Ok(())
}
