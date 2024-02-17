use anyhow::*;
use image::GenericImageView;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    // This creates the depth stage for the depth texture
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    
    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> Self {
        // This needs to be the same size as our screen so that the surface is rendered correctly.
        // Not sure what would happen if you did do it that way. Probably that the edges of the
        // image would render weirdly
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            // We are rendering to this texture so we need the RENDER_ATTACHMENT flag here
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            // Technically we don't need a sampler from the depth texture but this function
            // requires it so we just add it in
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                // If we do end up wanting to render to our texture, we need to have the comparison
                // here because of some interactions of different texture functions
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self { texture, view, sampler }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // By setting this to 1, we're basically telling the gpu that this is an unused size.
            // This means that for 2d textures, we don't have a depth
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            // this is used for multisampling textures. Probably don't need to worry about this
            sample_count: 1,
            // This is setting the dimensionality of the texture. Suprisingly there's a 1D texture
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB, so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            // This is the same as with the SurfaceConfig. It
            // specifies what texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case) is
            // always supported.
            view_formats: &[],
        });
        
        // Cannot interact with the Texture type directly. It's just a tag type. Instead, you
        // interact with the data using the `queue` to line up some instructions that you can send
        // to the gpu

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The actual pixel data
            &rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // The old way to to make a buffer and copy from it. Will have it here but it's unlikely
        // that you'll need to use it. The above method is nicer because it uses one less buffer to
        // do its thing

        /*
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Temp Buffer"),
            contents: &diffuse_rgba,
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture_buffer_copy_encoder"),
        });

        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        queue.submit(std::iter::once(encoder.finish()));
        */

        // We have data but we need a way to read from it in the shader code
        //
        // - `TextureView` is a view into the texture (what does that mean?)
        // - `Sampler` is how data from the texture is read. The simplest that you can imagine is
        //   literally reading the data. But you can also imagine that you can have some algorithms
        //   that map the original data to something else.

        // We don't need anything special here so let's go with the defaults
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // This defines what happens if the sample coordinate is outside the range of the
            // texture.
            // - clamp to edge: returns the nearest pixel value on the edge of the texture
            // - repeat: just repeat the texture in a tiling format
            // - mirror repeat: similar but the texture is mirrored on the edges
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // This defines what happens when the sample granularity/increment is smaller than the
            // actual texture. Like if you have a texture that goes 1,2,3,4 but you sample at 0.5
            // increments. This usually happens when you're very close to the texture or very far
            // away from it.
            //
            // - Nearest: Cuts to the nearest value. Creates a pixeled effect on the mag filter
            // - Linear: Linearly interpolates between two values. Creates a blurry effect
            mag_filter: wgpu::FilterMode::Linear, // How to make the texture larger (magnify)
            min_filter: wgpu::FilterMode::Nearest, // How to make the texture smaller (minify)
            // hierarchical structure of different resolutions of the texture so that at different
            // rendering distances, you can use a different texture resolution to both resolve the
            // weird rendering artifacts and to improve performance. (mip stands for "multum in
            // parvo" i.e. much in a small space)
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
