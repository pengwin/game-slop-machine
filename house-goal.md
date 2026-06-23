# House Basic Goal TODO

Reference image: `house-basic-goal.jpg`
Current fixture: `fixtures/texture-plaster-wall.png`

## Visual Gaps

- [ ] Floor material
  - [ ] Add a tiled floor pattern with visible grout lines.
  - [ ] Add per-tile color variation, dirt, and worn patches.
  - [ ] Add floor normal and roughness variation so tiles are not a flat fill.
  - [ ] Ensure floor UVs are meter-scaled and align cleanly across the room.

- [ ] Wood materials
  - [ ] Make furniture, door, shelves, chairs, table, barrel, crate, and trim use readable wood grain.
  - [ ] Add darker edge/crevice shading to wooden objects.
  - [ ] Increase wood contrast so small props read from the isometric camera.
  - [ ] Give the door a plank/panel texture instead of a flat dark slab.

- [ ] Wall plaster
  - [ ] Add stronger aged plaster variation: stains, mottling, vertical dirt, and subtle patches.
  - [ ] Make exterior walls less uniformly smooth.
  - [ ] Add more edge dirt near the bottom and around corners/openings.
  - [ ] Keep the top wall cap light and dusty, close to the goal image.

- [ ] Foundation / concrete edge
  - [ ] Keep the top edge more pale/whitish.
  - [ ] Reduce the nearly black look of the side/base strip.
  - [ ] Add more concrete/stone variation on the visible side faces.
  - [ ] Consider bevels or edge highlights so the curb reads as a solid slab.

- [ ] Windows and door detail
  - [ ] Add window muntins/crossbars.
  - [ ] Improve glass with brighter highlights and blue-gray variation.
  - [ ] Make window frames more dimensional and wood-textured.
  - [ ] Add a visible door handle and panel/plank details.

- [ ] Furniture detail
  - [ ] Add wood grain to the bed frame, table, chairs, shelves, crate, and barrel.
  - [ ] Add darker contact/ambient shadows under furniture.
  - [ ] Make shelves and books slightly more detailed and less blocky.
  - [ ] Add simple texture variation to blanket, pillow, and small props.

- [ ] Lighting and color grading
  - [ ] Lower ambient flatness so material texture and shadows read better.
  - [ ] Increase soft directional shadow contrast.
  - [ ] Make the background/ground closer to the goal's muted gray-brown.
  - [ ] Avoid over-bright low-poly studio lighting on interior surfaces.

- [ ] Camera and composition
  - [ ] Keep the current isometric angle close to the goal image.
  - [ ] Verify the room fills the frame similarly to the goal.
  - [ ] Keep wall height and top cap visibility consistent with the goal.

## Suggested Implementation Order

1. [ ] Build the tiled floor material and apply it to the room floor.
2. [ ] Apply readable wood materials to furniture, door, and trim.
3. [ ] Strengthen aged plaster variation on walls.
4. [ ] Refine the concrete foundation edge brightness and side material.
5. [ ] Improve windows and door details.
6. [ ] Tune lighting, shadows, and background color.
