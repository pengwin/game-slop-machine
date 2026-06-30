/// One discrete stage in the concrete texture pipeline.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ConcreteGenerationStage {
    /// Broad tileable tone and fine grain height are ready.
    Tone,
    /// Aggregate masks are ready.
    Aggregate,
    /// Void and pore masks are ready.
    Voids,
    /// Large exposed aggregate stones are ready.
    ExposedAggregate,
    /// Soft stain masks are ready.
    Stains,
    /// Formwork board marks are ready.
    Formwork,
    /// Hairline crack masks are ready.
    Cracks,
    /// Efflorescence masks are ready.
    Efflorescence,
    /// Final height buffer is composed.
    Height,
    /// Albedo texture is ready.
    Albedo,
    /// Normal texture is ready.
    Normal,
    /// ORM texture is ready.
    Orm,
}

impl ConcreteGenerationStage {
    /// Human-readable stage label for UI and logs.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tone => "Tone",
            Self::Aggregate => "Aggregate",
            Self::Voids => "Voids",
            Self::ExposedAggregate => "Exposed agg",
            Self::Stains => "Stains",
            Self::Formwork => "Formwork",
            Self::Cracks => "Cracks",
            Self::Efflorescence => "Efflorescence",
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
            Self::Tone => 1.0 / 12.0,
            Self::Aggregate => 2.0 / 12.0,
            Self::Voids => 3.0 / 12.0,
            Self::ExposedAggregate => 4.0 / 12.0,
            Self::Stains => 5.0 / 12.0,
            Self::Formwork => 6.0 / 12.0,
            Self::Cracks => 7.0 / 12.0,
            Self::Efflorescence => 8.0 / 12.0,
            Self::Height => 9.0 / 12.0,
            Self::Albedo => 10.0 / 12.0,
            Self::Normal => 11.0 / 12.0,
            Self::Orm => 1.0,
        }
    }
}
