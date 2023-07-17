use cube::cubical_tank;
use cuboidal::cuboidal_tank;
use serde::{Deserialize, Serialize};
use sphere::spherical_tank;
use cylinder::cylinder_tank;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen;

#[path ="./tanks/sphere.rs"]
mod sphere;
#[path ="./tanks/cylinder.rs"]
mod cylinder;
#[path ="./tanks/cube.rs"]
mod cube;
#[path ="./tanks/cuboidal.rs"]
mod cuboidal;

#[derive(Serialize, Deserialize)]
struct Tank {
    id: String,
    str: String,
    lng: f32,
    wdt: f32,
    hgt: f32,
    lvl: f32,
}


#[wasm_bindgen]
pub fn cylinderical_tank(input_tank: JsValue) -> Result<String, JsValue> {
    let tank_one: Tank = serde_wasm_bindgen::from_value(input_tank)?;
    let svg_data = cylinder_tank(tank_one.hgt, tank_one.wdt, tank_one.lvl);
    Ok(svg_data)
}

#[wasm_bindgen]
pub fn sphere_tank(input_tank: JsValue) -> Result<String, JsValue> {
    let tank_one: Tank = serde_wasm_bindgen::from_value(input_tank)?;
    let svg_data = spherical_tank(tank_one.hgt, tank_one.lvl);
    Ok(svg_data)
}

#[wasm_bindgen]
pub fn cuboid_tank(input_tank: JsValue) -> Result<String, JsValue> {
    let tank_one: Tank = serde_wasm_bindgen::from_value(input_tank)?;
    let svg_data = cuboidal_tank(tank_one.hgt, tank_one.wdt, tank_one.lng, tank_one.lvl);
    Ok(svg_data)
}

#[wasm_bindgen]
pub fn cube_tank(input_tank: JsValue) -> Result<String, JsValue> {
    let tank_one: Tank = serde_wasm_bindgen::from_value(input_tank)?;
    let svg_data = cubical_tank(tank_one.hgt, tank_one.lvl);
    Ok(svg_data)
}