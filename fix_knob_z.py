import re

with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    code = f.read()

code = code.replace("door_z - 0.015", "door_z + 0.015")

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.write(code)
