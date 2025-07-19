#![allow(dead_code)]

use std::cmp::Ordering;
use std::path::Path;

use ab_glyph::{Font, FontRef, PxScale};
use anyhow::{Context, Result, bail};
use data_url::DataUrl;
use image::ColorType::Rgba8;
use image::imageops::FilterType::Gaussian;
use image::{ImageBuffer, Rgb, RgbImage};
use log::{debug, error, warn};
use reqwest::blocking::Client;
use resvg::{tiny_skia, usvg};
use url::Url;
use web_app_manifest::resources::IconResource;
use web_app_manifest::types::{ImagePurpose, ImageSize, Url as ManifestUrl};

//////////////////////////////
// Public
//////////////////////////////

/// Remove all invalid filename characters and limit the length.
///
/// Name is capped at 60 characters is sanitized using the [`sanitize_filename`]
/// crate to prevent it from containing any invalid filenames characters. Dots
/// at the start are also removed to prevent the file from being hidden. In case
/// the sanitized name is an empty string, the new name is constructed from the ID.
#[cfg(any(platform_windows, platform_macos))]
pub fn sanitize_name<'a>(name: &'a str, id: &'a str) -> String {
    let mut sanitized: String = name.chars().take(60).collect();
    sanitized = sanitized.trim_start_matches([' ', '.']).into();
    sanitized = sanitize_filename::sanitize(sanitized);

    if sanitized.is_empty() { format!("Site {}", &id) } else { sanitized }
}

/// Normalize category name.
///
/// Category name is converted to lower-case and all word separators (`-`, `_`, ` `)
/// are removed. This allows easier matching with keys from the categories map.
#[cfg(any(
    platform_linux,
    platform_macos,
    platform_bsd,
    all(platform_windows, feature = "portable")
))]
#[inline]
pub fn normalize_category_name(category: &str) -> String {
    category.to_lowercase().replace(['-', '_', ' '], "")
}

/// Download the icon from the URL.
///
/// Icon can be downloaded from the network using the `reqwest` crate
/// or decoded from a data URL. Once downloaded, the function returns
/// the icon bytes and its content type.
pub fn download_icon(url: Url, client: &Client) -> Result<(Vec<u8>, String)> {
    // Download using `reqwest`
    if url.scheme() != "data" {
        let response = client.get(url).send()?;
        let r#type = match response.headers().get(reqwest::header::CONTENT_TYPE) {
            Some(r#type) => r#type.to_str()?.into(),
            None => "application/octet-stream".into(),
        };
        let bytes = response.bytes()?.to_vec();
        Ok((bytes, r#type))

    // Download using `data-url`
    } else {
        let url = DataUrl::process(url.as_str())?;
        let r#type = url.mime_type().to_string();
        let (bytes, _) = url.decode_to_vec()?;
        Ok((bytes, r#type))
    }
}

/// Generate an icon from a letter.
pub fn generate_icon(letter: char, size: &ImageSize) -> Result<RgbImage> {
    // Icon must have a fixed size
    let size = match size {
        ImageSize::Fixed(a, b) => (a, b),
        _ => bail!("A fixed image size variant must be provided"),
    };

    // Load the font from OTF file
    let bytes = include_bytes!("../../assets/Metropolis-SemiBold.otf");
    let font = FontRef::try_from_slice(bytes).context("Failed to construct the font")?;

    // Get and scale the glyph
    let scale = PxScale::from(*size.1 as f32 / 1.6);
    let glyph = font.glyph_id(letter).with_scale(scale);

    // Store the background and foreground colors
    let background = Rgb([80, 80, 80]);
    let foreground = Rgb([255, 255, 255]);

    // Create a new RGBA image with a gray background
    let mut image: RgbImage = ImageBuffer::from_pixel(*size.0, *size.1, background);

    if let Some(outlined) = font.outline_glyph(glyph) {
        // Get the glyph width and height
        let bounds = outlined.px_bounds();
        let width = (bounds.max.x - bounds.min.x) as u32;
        let height = (bounds.max.y - bounds.min.y) as u32;

        // Check for glyph size overflows
        // This shouldn't happen, but just in case
        if width > *size.0 || height > *size.1 {
            bail!("Glyph is bigger than image");
        }

        // Calculate the offset so the glyph is in the middle
        let offset_x = (size.0 - width) / 2;
        let offset_y = (size.1 - height) / 2;

        // Draw the glyph into the image per-pixel by using the draw closure
        outlined.draw(|x, y, v| {
            // Convert the alpha value with the background
            let pixel = Rgb([
                ((1.0 - v) * background.0[0] as f32 + v * foreground.0[0] as f32) as u8,
                ((1.0 - v) * background.0[1] as f32 + v * foreground.0[1] as f32) as u8,
                ((1.0 - v) * background.0[2] as f32 + v * foreground.0[2] as f32) as u8,
            ]);

            // Put the glyph pixel into the image
            image.put_pixel(x + offset_x, y + offset_y, pixel)
        });
    }

    Ok(image)
}

/// Obtain and process the best available icon from the icon list.
///
/// Icon needs to be processed and converted to a correct format (determined from
/// the filename). In case anything fails, the next icons are tried. If no provided
/// icons are working, the icon is generated from the first letter of the name.
///
/// See [`normalize_icons`] and [`process_icon`] for more details.
///
/// # Parameters
///
/// - `icons`: A list of available icons for the web app or shortcut.
/// - `fallback`:  A web app or shortcut name. Used to generate a fallback icon.
/// - `size`: A target icon size. Must be a valid fixed (non-zero) size variant.
/// - `path`:  A path where the icon should be saved.
/// - `client`: An instance of a blocking HTTP client.
///
pub fn process_icons(
    icons: &[IconResource],
    fallback: &str,
    size: &ImageSize,
    path: &Path,
    client: &Client,
) -> Result<()> {
    for icon in normalize_icons(icons, size) {
        match process_icon(icon, size, path, client).context("Failed to process icon") {
            Ok(_) => return Ok(()),
            Err(error) => {
                error!("{error:?}");
                warn!("Falling back to the next available icon");
            }
        }
    }

    warn!("No compatible or working icon was found");
    warn!("Falling back to the generated icon from the name");
    let letter = fallback.chars().next().context("Failed to get the first letter")?;
    let icon = generate_icon(letter, size).context("Failed to generate icon")?;
    icon.save(path).context("Failed to save generated image")?;
    Ok(())
}

//////////////////////////////
// Internal
//////////////////////////////

/// Check if the icon is supported.
///
/// Supported icons must contain "any" purpose and must only have absolute URLs.
/// Other icons cannot / should not be parsed and need to be ignored.
fn is_icon_supported(icon: &&IconResource) -> bool {
    // Normal icons must contain "any" purpose
    if !icon.purpose.contains(&ImagePurpose::Any) {
        return false;
    }

    // Only icons with absolute URLs can be used
    matches!(&icon.src, ManifestUrl::Absolute(_))
}

/// Filter out all incompatible icons and sort them.
///
/// All icons are first filtered to remove unsupported icons, and then sorted
/// by their largest size. Icons larger than the target icon size are sorted
/// in the ascending order, and others are sorted in descending.
fn normalize_icons<'a>(icons: &'a [IconResource], size: &'a ImageSize) -> Vec<&'a IconResource> {
    let mut icons: Vec<&IconResource> = icons.iter().filter(is_icon_supported).collect();

    icons.sort_by(|icon1, icon2| {
        let size1 = icon1.sizes.iter().max();
        let size2 = icon2.sizes.iter().max();

        if size1.is_none() || size2.is_none() {
            return Ordering::Equal;
        };

        // Unwrap is safe, because sizes is checked above
        let size1 = size1.unwrap();
        let size2 = size2.unwrap();

        if size1 >= size && size2 >= size { size1.cmp(size2) } else { size1.cmp(size2).reverse() }
    });

    icons
}

/// Process the icon and stores it to a file.
///
/// Icon can be downloaded from the network or from a data URL using
/// the [`download_icon`] function. Icon is then resized to a specified
/// size and stored to a file. Both SVG and raster icons are supported.
///
/// # Parameters
///
/// - `icon`: An icon resource representing the icon. Must provide an absolute icon URL.
/// - `size`: A target icon size. Must be a valid fixed (non-zero) size variant.
/// - `path`: A path where the icon should be stored.
/// - `client`: An instance of a blocking HTTP client.
///
fn process_icon(icon: &IconResource, size: &ImageSize, path: &Path, client: &Client) -> Result<()> {
    let size = match size {
        ImageSize::Fixed(a, b) => (*a, *b),
        _ => bail!("A fixed image size variant must be provided"),
    };

    let url: Url = icon.src.clone().try_into().context("Failed to convert icon URL")?;
    debug!("Processing icon {url}");

    // Download icon and get its content type
    let (content, content_type) = download_icon(url, client).context("Failed to download icon")?;

    if content_type == "image/svg+xml" {
        // Parse and render SVG icons using `resvg` crate
        debug!("Processing as SVG icon");

        let mut pixmap = tiny_skia::Pixmap::new(size.0, size.1).context("Invalid target size")?;

        let mut opt = usvg::Options::default();

        opt.fontdb_mut().load_system_fonts();

        let resolver = Box::new(move |_: &str, _: &usvg::Options| None);
        opt.image_href_resolver.resolve_string = resolver;

        let tree = usvg::Tree::from_data(&content, &opt).context("Failed to parse SVG icon")?;

        let transform = tiny_skia::Transform::from_scale(
            size.0 as f32 / tree.size().width(),
            size.1 as f32 / tree.size().height(),
        );

        resvg::render(&tree, transform, &mut pixmap.as_mut());

        image::save_buffer(path, pixmap.data(), size.0, size.1, Rgba8)
            .context("Failed to save SVG icon")?;

        return Ok(());
    }

    // Parse raster icons using the `image` crate, resize them and store them to a file
    debug!("Processing as raster icon");
    let mut img = image::load_from_memory(&content).context("Failed to load raster icon")?;
    img = img.resize(size.0, size.1, Gaussian);
    img.save(path).context("Failed to save raster icon")?;

    Ok(())
}
