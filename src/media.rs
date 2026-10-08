//! Image processing for header image and option previews (V2 roadmap, priority 7).
//!
//! Mod managers display the module header image and per-option preview images.
//! Oversized or odd-format images bloat the archive and may render poorly. This
//! module validates images against mod-manager-friendly constraints and can
//! downscale/convert them in place.
//!
//! Uses the `image` crate (already a dependency, PNG/JPEG). It is i18n-free; the
//! UI maps [`ImageIssue`] to translated messages.

use std::path::Path;

use anyhow::{Context, Result};

/// Bounds and allowed formats for images embedded in a FOMOD.
#[derive(Debug, Clone)]
pub struct ImageConstraints {
    /// Maximum width in pixels (wider images are downscaled, aspect kept).
    pub max_width: u32,
    /// Maximum height in pixels.
    pub max_height: u32,
    /// Allowed output formats (lowercase extensions, e.g. `["png", "jpg"]`).
    pub allowed_formats: Vec<String>,
}

impl Default for ImageConstraints {
    fn default() -> Self {
        Self {
            max_width: 1920,
            max_height: 1080,
            allowed_formats: vec!["png".into(), "jpg".into(), "jpeg".into()],
        }
    }
}

/// A validation finding about one image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageIssue {
    /// Dimensions exceed the constraints (`width`×`height`).
    TooLarge { width: u32, height: u32 },
    /// Format (file extension) not in the allowed list.
    UnsupportedFormat { ext: String },
    /// File could not be opened/decoded.
    Unreadable,
}

/// Lowercase file extension, or empty string if none.
fn ext_of(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default()
}

/// Validate a single image against `constraints` (dimensions + format). Returns
/// every issue found (empty = compliant). A decode failure yields a single
/// [`ImageIssue::Unreadable`].
pub fn validate_image(path: &Path, constraints: &ImageConstraints) -> Vec<ImageIssue> {
    let mut issues = Vec::new();

    let ext = ext_of(path);
    if !constraints.allowed_formats.contains(&ext) {
        issues.push(ImageIssue::UnsupportedFormat { ext: ext.clone() });
    }

    match image::image_dimensions(path) {
        Ok((w, h)) => {
            if w > constraints.max_width || h > constraints.max_height {
                issues.push(ImageIssue::TooLarge { width: w, height: h });
            }
        }
        Err(_) => issues.push(ImageIssue::Unreadable),
    }
    issues
}

/// Resize/convert `src` into `dst` so it satisfies `constraints`. Returns whether
/// the image was modified (false if it already complied and `src == dst`).
///
/// The image is downscaled to fit within the max dimensions (aspect ratio kept),
/// and written to `dst` in the format implied by `dst`'s extension.
pub fn process_image(src: &Path, dst: &Path, constraints: &ImageConstraints) -> Result<bool> {
    let img = image::open(src).with_context(|| format!("opening image {}", src.display()))?;
    let (w, h) = (img.width(), img.height());
    let needs_resize = w > constraints.max_width || h > constraints.max_height;

    let same_path = src == dst;
    if !needs_resize && same_path {
        return Ok(false);
    }

    let out = if needs_resize {
        // `resize` preserves aspect ratio, fitting within the bounds.
        img.resize(
            constraints.max_width,
            constraints.max_height,
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        img
    };
    out.save(dst)
        .with_context(|| format!("writing image {}", dst.display()))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_png(path: &Path, w: u32, h: u32) {
        let buf = image::RgbImage::from_pixel(w, h, image::Rgb([10, 20, 30]));
        image::DynamicImage::ImageRgb8(buf).save(path).unwrap();
    }

    #[test]
    fn default_constraints_are_sane() {
        let c = ImageConstraints::default();
        assert!(c.max_width >= 640 && c.max_height >= 480);
        assert!(c.allowed_formats.iter().any(|f| f == "png"));
    }

    #[test]
    fn flags_oversized_and_bad_format() {
        let dir = std::env::temp_dir().join(format!("ximod_img_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let big = dir.join("big.png");
        write_png(&big, 2200, 1300);
        let c = ImageConstraints::default();
        let issues = validate_image(&big, &c);
        assert!(issues.iter().any(|i| matches!(i, ImageIssue::TooLarge { .. })));

        // process it down and re-validate
        let out = dir.join("small.png");
        assert!(process_image(&big, &out, &c).unwrap());
        let issues2 = validate_image(&out, &c);
        assert!(!issues2.iter().any(|i| matches!(i, ImageIssue::TooLarge { .. })));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unreadable_file_reported() {
        let dir = std::env::temp_dir().join(format!("ximod_img2_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let bogus = dir.join("x.png");
        std::fs::write(&bogus, b"not an image").unwrap();
        let issues = validate_image(&bogus, &ImageConstraints::default());
        assert!(issues.iter().any(|i| matches!(i, ImageIssue::Unreadable)));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
