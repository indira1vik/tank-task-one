use svg::node::element::*;
use svg::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn cubical_tank(height: f32, fluid: f32) -> String {
    let hm = height / 1000.0;
    let fm = fluid / 1000.0;

    let scx1 = 30.0;
    let scy1 = 30.0;

    let wr = 40.0;
    let hr = 40.0;

    let volume = hm * hm * hm;
    let filled = hm * hm * fm;

    let percent = (filled / volume) * 100.0;

    let l = (percent / 100.0) * 40.0;

    let y_pt = 70.0 - l;
    let x_pt1 = 30.0;

    let rect = Rectangle::new()
        .set("x", scx1)
        .set("y", scy1)
        .set("width", wr)
        .set("height", hr)
        .set("fill", "grey")
        .set("stroke", "none")
        .set("rx", 1)
        .set("ry", 1);
    let pathlvl1 = Path::new()
        .set(
            "d",
            format!(
                "M {} {} h {} a 1 1 0 0 1 1 1 v {} a 1 1 0 0 1 -1 1 h {} a 1 1 0 0 1 -1 -1 v {} a 1 1 0 0 1 1 -1 z",
                x_pt1+1.0,
                y_pt,
                wr-2.0,
                l - 2.0,
                -wr + 2.0,
                -l + 2.0
            ),
        )
        .set(
            "fill",
            if l > 35.0 {
                "red"
            } else if l > 20.0 {
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
                "M {} {} h {} v {} a 1 1 0 0 1 -1 1 h {} a 1 1 0 0 1 -1 -1 v {} z",
                x_pt1,
                y_pt,
                wr,
                l - 1.0,
                -wr + 2.0,
                -l + 1.0
            ),
        )
        .set(
            "fill",
            if l > 35.0 {
                "red"
            } else if l > 20.0 {
                "orange"
            } else {
                "green"
            },
        )
        .set("stroke", "none");
    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(rect)
        .add(if l > 35.0 { pathlvl1 } else { pathlvl });

    return document.to_string();
}
