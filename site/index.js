class Tank {
    constructor(id, str, hgt, wdt, lng) {
        this.id = id,
        this.str = str,
        this.hgt = hgt,
        this.wdt = wdt,
        this.lng = lng,
        this.lvl = 0.0
    }
    setLevel(lvl) {
        this.lvl = lvl;
    }
}

function checkTank(shape) {
    if (shape == "sphere") {
        return new Tank("t01-18-bng-cmp", "sphere", 3000.0, 3000.0, 3000.0);
    } else if (shape == "cylinder") {
        return new Tank("t99-02-chn-cmp", "cylinder", 3000.0, 2100.0, 2100.0);
    } else if (shape == "cuboidal") {
        return new Tank("t45-06-mys-cmp", "cuboidal", 3000.0, 2100.0, 2400.0);
    } else if (shape == "cube") {
        return new Tank("t34-14-hyd-cmp", "cube", 3000.0, 3000.0, 3000.0);
    }
    return null;
}

function updateLevel() {
    const lvl = parseFloat(document.getElementById("lvl").value);
    const mySelect = document.getElementById("mySelect");
    const selectedOption = mySelect.options[mySelect.selectedIndex].value;
    const tank_created = checkTank(selectedOption);
    if (lvl >= 0.0) {
        tank_created.setLevel(lvl);
    }
    return tank_created;
}

import("./node_modules/tank-task-one/tank_task_one.js").then((wasm) => {
    document.getElementById("updbtn").addEventListener('click', function () {
        document.getElementById("dispTank").style.display = "block";
        const tank_created = updateLevel();
        console.log(tank_created);
        var svgImg = "";
        if (tank_created.str == "sphere") {
            svgImg = wasm.sphere_tank(tank_created);
        } else if (tank_created.str == "cylinder") {
            svgImg = wasm.cylinderical_tank(tank_created);
        } else if (tank_created.str == "cuboidal") {
            svgImg = wasm.cuboid_tank(tank_created);
        } else if (tank_created.str == "cube") {
            svgImg = wasm.cube_tank(tank_created);
        } else {
            const error_handle = document.createElement('div');
            error_handle.innerText = "INVALID";
            document.getElementById("dispTank").appendChild = error_handle;
        }
        document.getElementById("svgimg").innerHTML = svgImg;
        document.getElementById("id").innerText = tank_created.id;
        document.getElementById("st").innerHTML = tank_created.str;
        document.getElementById("ht").innerHTML = tank_created.hgt;
        document.getElementById("wd").innerHTML = tank_created.wdt;
        document.getElementById("ln").innerHTML = tank_created.lng;
        document.getElementById("lv").innerHTML = tank_created.lvl;

    });

});