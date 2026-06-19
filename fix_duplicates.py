with open('crates/building-gen/src/furniture/catalog.rs', 'r') as f:
    lines = f.readlines()

# Duplicates to remove:
# 1. Lines 484 to 680 (inclusive): duplicated append_colored_box, append_colored_beveled_box, append_colored_rotated_box
# 2. Lines 1103 to 1125 (inclusive): duplicated ShelfConfig

# Convert 1-indexed to 0-indexed and delete ranges in reverse order
del lines[1102:1125]
del lines[483:680]

with open('crates/building-gen/src/furniture/catalog.rs', 'w') as f:
    f.writelines(lines)
