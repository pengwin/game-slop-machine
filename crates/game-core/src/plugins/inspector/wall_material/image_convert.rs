use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{Extent3d, TextureDataOrder, TextureDimension, TextureFormat},
};
use texture_gen::{GeneratedMipTexture, TextureColorSpace};

/// Converts a `GeneratedMipTexture` into a Bevy `Image`.
#[must_use]
pub fn bevy_image(texture: GeneratedMipTexture) -> Image {
    let format = match texture.color_space {
        TextureColorSpace::Srgb => TextureFormat::Rgba8UnormSrgb,
        TextureColorSpace::Linear => TextureFormat::Rgba8Unorm,
    };
    let mut image = Image::new_uninit(
        Extent3d {
            width: texture.width,
            height: texture.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        format,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.data = Some(texture.data);
    image.data_order = TextureDataOrder::MipMajor;
    image.texture_descriptor.mip_level_count = texture.mip_level_count;
    image.sampler = ImageSampler::Descriptor(repeating_linear_sampler());
    image
}

/// Creates a repeating linear sampler for tiled textures.
#[must_use]
pub fn repeating_linear_sampler() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    }
}
