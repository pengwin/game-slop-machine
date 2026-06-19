import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

# Replace Z coordinates in generate_shelf_mesh
code = code.replace(
"""    // Back panel
    append_colored_box(&mut mesh, [0.0, actual_h/2.0, hd - pt/2.0], [w - 2.0*pt, actual_h, pt], config.wood_color);

    // Cabinet base
    let base_h = actual_h * 0.35;
    append_colored_box(&mut mesh, [0.0, base_h/2.0, shelf_z], [w - 2.0*pt, base_h, shelf_d], config.cabinet_color);

    // Cabinet doors detailing
    let door_w = (w - 2.0*pt) / 2.0 - 0.02;
    let door_h = base_h - 0.04;
    let door_z = -hd + 0.01;""",
"""    // Back panel (now at -Z)
    append_colored_box(&mut mesh, [0.0, actual_h/2.0, -hd + pt/2.0], [w - 2.0*pt, actual_h, pt], config.wood_color);

    // Cabinet base
    let base_h = actual_h * 0.35;
    append_colored_box(&mut mesh, [0.0, base_h/2.0, shelf_z], [w - 2.0*pt, base_h, shelf_d], config.cabinet_color);

    // Cabinet doors detailing
    let door_w = (w - 2.0*pt) / 2.0 - 0.02;
    let door_h = base_h - 0.04;
    let door_z = hd - 0.01; // now at front (+Z)""")

code = code.replace("let shelf_z = -hd + shelf_d / 2.0;", "let shelf_z = hd - shelf_d / 2.0;")
code = code.replace("v[2] += shelf_z;", "v[2] += shelf_z; // flipped")

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
