use crate::tile::{CardinalDir, CornerDir, TJunctionDir, WallKind, WallOpening, WallShape, WallTile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExteriorFaceClass {
    Straight,
    Corner,
    TJunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallFaceDir {
    NegX,
    PosX,
    NegZ,
    PosZ,
    PosY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallCutout {
    Door,
    Window,
}

pub fn exterior_face_dirs(wall: WallTile) -> Vec<WallFaceDir> {
    if wall.kind != WallKind::Exterior {
        return Vec::new();
    }

    match wall.shape {
        WallShape::Straight(CardinalDir::Left) => vec![WallFaceDir::NegX],
        WallShape::Straight(CardinalDir::Right) => vec![WallFaceDir::PosX],
        WallShape::Straight(CardinalDir::Bottom) => vec![WallFaceDir::NegZ],
        WallShape::Straight(CardinalDir::Top) => vec![WallFaceDir::PosZ],
        WallShape::Corner(CornerDir::BottomLeft) => vec![WallFaceDir::NegX, WallFaceDir::NegZ],
        WallShape::Corner(CornerDir::BottomRight) => vec![WallFaceDir::PosX, WallFaceDir::NegZ],
        WallShape::Corner(CornerDir::TopLeft) => vec![WallFaceDir::NegX, WallFaceDir::PosZ],
        WallShape::Corner(CornerDir::TopRight) => vec![WallFaceDir::PosX, WallFaceDir::PosZ],
        WallShape::TJunction(TJunctionDir::Left) => vec![WallFaceDir::NegX],
        WallShape::TJunction(TJunctionDir::Right) => vec![WallFaceDir::PosX],
        WallShape::TJunction(TJunctionDir::Bottom) => vec![WallFaceDir::NegZ],
        WallShape::TJunction(TJunctionDir::Top) => vec![WallFaceDir::PosZ],
        WallShape::Cross => Vec::new(),
    }
}

pub fn exterior_face_class(wall: WallTile) -> ExteriorFaceClass {
    match wall.shape {
        WallShape::Corner(_) => ExteriorFaceClass::Corner,
        WallShape::TJunction(_) => ExteriorFaceClass::TJunction,
        WallShape::Straight(_) | WallShape::Cross => ExteriorFaceClass::Straight,
    }
}

pub fn opening_cutout(opening: Option<WallOpening>) -> Option<WallCutout> {
    match opening {
        Some(WallOpening::Door { .. } | WallOpening::Doorway) => Some(WallCutout::Door),
        Some(WallOpening::Window { .. }) => Some(WallCutout::Window),
        None => None,
    }
}
