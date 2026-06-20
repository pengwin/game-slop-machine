# Game Slop Machine — Fixture Screenshot Generator
# Usage: just [recipe]
# Run `just` to see all available recipes.

# Generate all fixture screenshots
all: buildings districts furniture

# ── Building Fixtures ────────────────────────────────────────────────────

# Generate all building fixtures
buildings: procedural with-roof corridor four-doors four-windows two-room picture-room

procedural:
    cargo run -p game-headless -- fixtures/building-procedural.png procedural

with-roof:
    cargo run -p game-headless -- fixtures/building-with-roof.png with-roof

corridor:
    cargo run -p game-headless -- fixtures/building-corridor.png corridor

four-doors:
    cargo run -p game-headless -- fixtures/building-four-doors.png four-doors

four-windows:
    cargo run -p game-headless -- fixtures/building-four-windows.png four-windows

two-room:
    cargo run -p game-headless -- fixtures/building-two-room.png two-room

picture-room:
    cargo run -p game-headless -- fixtures/building-picture-room.png picture-room

# ── District Fixtures ────────────────────────────────────────────────────

# Generate all district fixtures
districts: district district-lots district-no-roof huge-trade-district huge-trade-district-lots huge-trade-district-no-roof

district:
    cargo run -p game-headless -- fixtures/district.png district

district-lots:
    cargo run -p game-headless -- fixtures/district-lots.png district-lots

district-no-roof:
    cargo run -p game-headless -- fixtures/district-no-roof.png district-no-roof

huge-trade-district:
    cargo run -p game-headless -- fixtures/district-huge.png huge-trade-district

huge-trade-district-lots:
    cargo run -p game-headless -- fixtures/district-huge-lots.png huge-trade-district-lots

huge-trade-district-no-roof:
    cargo run -p game-headless -- fixtures/district-huge-no-roof.png huge-trade-district-no-roof

# ── Furniture Fixtures ───────────────────────────────────────────────────

# Generate all furniture fixtures
furniture: furniture-all furniture-table furniture-chair furniture-bed furniture-stove furniture-counter furniture-desk furniture-barrel furniture-crate furniture-bench furniture-shelf

furniture-all:
    cargo run -p game-headless -- fixtures/furniture-all.png all-furniture

furniture-table:
    cargo run -p game-headless -- fixtures/furniture-table.png table

furniture-chair:
    cargo run -p game-headless -- fixtures/furniture-chair.png chair

furniture-bed:
    cargo run -p game-headless -- fixtures/furniture-bed.png bed

furniture-stove:
    cargo run -p game-headless -- fixtures/furniture-stove.png stove

furniture-counter:
    cargo run -p game-headless -- fixtures/furniture-counter.png counter

furniture-desk:
    cargo run -p game-headless -- fixtures/furniture-desk.png desk

furniture-barrel:
    cargo run -p game-headless -- fixtures/furniture-barrel.png barrel

furniture-crate:
    cargo run -p game-headless -- fixtures/furniture-crate.png crate

furniture-bench:
    cargo run -p game-headless -- fixtures/furniture-bench.png bench

furniture-shelf:
    cargo run -p game-headless -- fixtures/furniture-shelf.png shelf

# ── Utilities ────────────────────────────────────────────────────────────

# Clean generated fixture images
clean:
    rm -f fixtures/*.png

# List all fixture files
list:
    @ls -la fixtures/*.png 2>/dev/null || echo "No fixtures generated yet. Run: just all"
