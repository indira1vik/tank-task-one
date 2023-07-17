use std::f32::consts::PI;
use svg::node::element::*;
use svg::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn cylinder_tank(height: f32, width: f32, fluid: f32) -> String {
    let hm = height / 1000.0;
    let wm = width / 1000.0;
    let fm = fluid / 1000.0;

    let scx1 = 30.0;
    let scy1 = 15.0;

    let wr = 40.0;
    let hr = 70.0;

    let radius = wm / 2.0;
    let volume = PI * (radius * radius) * hm;
    let filled = PI * (radius * radius) * fm;

    let percent = (filled / volume) * 100.0;

    let l = (percent / 100.0) * 70.0;

    let y_pt = 85.0 - l;
    let x_pt1 = 30.0;

    let rect = Rectangle::new()
        .set("x", scx1)
        .set("y", scy1)
        .set("width", wr)
        .set("height", hr)
        .set("fill", "grey")
        .set("stroke", "none")
        .set("rx", 5)
        .set("ry", 5);
    let pathlvl1 = Path::new()
        .set(
            "d",
            format!(
                "M {} {} h {} a 5 5 0 0 1 5 5 v {} a 5 5 0 0 1 -5 5 h {} a 5 5 0 0 1 -5 -5 v {} a 5 5 0 0 1 5 -5 z",
                x_pt1+5.0,
                y_pt,
                wr-10.0,
                l - 10.0,
                -wr + 10.0,
                -l + 10.0
            ),
        )
        .set(
            "fill",
            if l > 60.0 {
                "red"
            } else if l > 35.0 {
                "orange"
            } else {
                "green"
            },
        )
        .set("stroke", "none");
    let pathlvl = Path::new()
        .set(
            "d",
            format!(
                "M {} {} h {} v {} a 5 5 0 0 1 -5 5 h {} a 5 5 0 0 1 -5 -5 v {} z",
                x_pt1,
                y_pt,
                wr,
                l - 5.0,
                -wr + 10.0,
                -l + 5.0
            ),
        )
        .set(
            "fill",
            if l > 60.0 {
                "red"
            } else if l > 35.0 {
                "orange"
            } else {
                "green"
            },
        )
        .set("stroke", "none");
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(rect)
        .add(if l > 65.0 { pathlvl1 } else { pathlvl });

    return document.to_string();
}
