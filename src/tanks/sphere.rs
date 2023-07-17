use std::f32::consts::PI;
use svg::node::element::*;
use svg::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn spherical_tank(height: f32, fluid: f32) -> String {
    let c1: f32 = 50.0;
    let c2 = 50.0;
    let r = 50.0;

    let hm = height / 1000.0;
    let fm = fluid / 1000.0;

    let radius = hm / 2.0;
    let volume = 1.3334 * PI * (radius * radius * radius);

    let filled = PI * ((fm * fm * (3.0 * radius - fm)) / 3.0);

    let percent = (filled / volume) * 100.0;

    let l = percent;
    let h = (r - l as f32).abs();

    let start_x1 = c1;
    let start_y1 = if l <= 50.0 { c2 + h } else { c2 - h }; //

    let to_right = r + ((r * r) - (h * h) as f32).sqrt();
    let to_left = (r - ((r * r) - (h * h) as f32).sqrt()).abs();

    let line_x1 = to_right;
    let line_y1 = if l <= 50.0 { c2 + h } else { c2 - h }; //

    let end_x1 = to_left;
    let end_y1 = if l <= 50.0 { c2 + h } else { c2 - h }; //

    let circle_one = Circle::new()
        .set("cx", c1)
        .set("cy", c2)
        .set("r", r)
        .set("fill", "grey")
        .set("stroke", "none");
    let circle_two = Circle::new()
        .set("cx", c1)
        .set("cy", c1)
        .set("r", r)
        .set(
            "fill",
            if l > 90.0 {
                "red"
            } else if l > 50.0 {
                "orange"
            } else {
                "green"
            },
        )
        .set("stroke", "none");

    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(if l <= 50.0 { circle_one } else { circle_two })
        .add(
            Path::new()
                .set(
                    "d",
                    format!(
                        "M {},{} L {},{} A {},{} 0 0,{} {},{} Z",
                        start_x1,
                        start_y1,
                        line_x1,
                        line_y1,
                        c1,
                        c2,
                        if l <= 50.0 { 1 } else { 0 },
                        end_x1,
                        end_y1
                    ),
                )
                .set("fill", if l <= 50.0 { "green" } else { "grey" })
                .set("stroke", "none"),
        );

    return document.to_string();
}