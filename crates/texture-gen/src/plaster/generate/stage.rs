/// One discrete stage in the plaster texture pipeline.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlasterGenerationStage {
    /// Broad tileable tone and fine grain height are ready.
    Tone,
    /// Soft stain masks are ready.
    Stains,
    /// Tiny pit masks are ready.
    Pits,
    /// Hairline crack masks are ready.
    Cracks,
    /// Final height buffer is composed.
    Height,
    /// Albedo texture is ready.
    Albedo,
    /// Normal texture is ready.
    Normal,
    /// ORM texture is ready.
    Orm,
}

impl PlasterGenerationStage {
    /// Human-readable stage label for UI and logs.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tone => "Tone",
            Self::Stains => "Stains",
            Self::Pits => "Pits",
            Self::Cracks => "Cracks",
            Self::Height => "Height",
            Self::Albedo => "Albedo",
            Self::Normal => "Normal",
            Self::Orm => "ORM",
        }
    }

    /// Discrete progress fraction for this stage.
    #[must_use]
    pub const fn fraction(self) -> f32 {
        match self {
            Self::Tone => 1.0 / 8.0,
            Self::Stains => 2.0 / 8.0,
            Self::Pits => 3.0 / 8.0,
            Self::Cracks => 4.0 / 8.0,
            Self::Height => 5.0 / 8.0,
            Self::Albedo => 6.0 / 8.0,
            Self::Normal => 7.0 / 8.0,
            Self::Orm => 1.0,
        }
    }
}
