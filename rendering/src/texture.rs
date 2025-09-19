use image::GenericImageView;
use anyhow::*;

pub(crate) struct Texture {
    #[allow(unused)]
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
}

impl Texture {
    pub(crate) fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8], 
        label: &str
    ) -> Result<Self> {
        // Turns the byte array into an image
        let img = image::load_from_memory(bytes)?;
        // Get texture from image
        Self::from_image(device, queue, &img, Some(label))
    }

    pub(crate) fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>
    ) -> Result<Self> {
        // Converts to image to an rgba8 format
        let rgba = img.to_rgba8();
        // Get dimensions of image
        let dimensions = img.dimensions();

        // Record the size of the texture
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            depth_or_array_layers: 1,
        };
        // Create a texture of the right size and format to hold the image
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                // 2D texture
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB, so we need to reflect that here
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                // This is the same as with the SurfaceConfig - it
                // specifies what texture formats can be used to
                // create TextureViews for this texture

                // The base texture format (Rgba8UnormSrgb in this case) is
                // always supported. Note that using a different
                // texture format is not supported on the WebGL2
                // backend
                view_formats: &[],
            }
        );

        // Copy the pixel data of the image to the texture
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The actual pixel data
            &rgba,
            // The layout of the texture
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );
        
        // Create a TextureView type from the Texture type which specifies how the 
        // data in the Texture should be read and used by the GPU (e.g. format, dimension, mipmap levels)
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Create a sampler to control how the Texture is sampled in the shader
        let sampler = device.create_sampler(
            // Address_mode_* describes what happens when the sampler gets a textue coordinate outside of the texture
            // Clamp to edge will return the colour of the nearest pixel on the edge of the texture
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                // What to do when the sample footprint is smaller than one texel
                // Linear does linear interpolation between two texels in each dimension while
                // Nearest returns the value closest to the texture coordinates
                mag_filter: wgpu::FilterMode::Nearest,
                // What to do when the sample footprint is larger than one texel
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Ok(Self { texture, view, sampler })
    }
}
