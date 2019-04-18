/// Preset filters you can apply to images.
extern crate image;

use image::{GenericImage, DynamicImage, GenericImageView};

/// Add an aquamarine-tinted hue to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::oceanic(img);
/// ```
pub fn oceanic(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_two_channels(img, 1, 9, 2, 173);
    return filtered_img;
}

/// Custom filter with an aquamarine tint.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::islands(img);
/// ```
pub fn islands(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_two_channels(img, 1, 24, 2, 95);
    return filtered_img;
}

/// Add a green/blue mixed hue to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::marine(img);
/// ```
pub fn marine(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_two_channels(img, 1, 14, 2, 119);
    return filtered_img;
}

/// Dark green hue, with tones of blue.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::seagreen(img);
/// ```
pub fn seagreen(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_two_channels(img, 1, 68, 2, 62);
    return filtered_img;
}

/// Royal blue tint
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::flagblue(img);
/// ```
pub fn flagblue(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_blue_channel(img, 131);
    return filtered_img;
}

/// Custom filter with a blue/turquoise tint.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::diamante(img);
/// ```
pub fn diamante(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_two_channels(img, 1, 82, 2, 87);
    return filtered_img;
}

/// Blue-inspired tint.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::liquid(img);
/// ```
pub fn liquid(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::inc_two_channels(img, 1, 10, 2, 75);
    return filtered_img;
}

/// Solarization on the Blue channel.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::neue(img);
/// ```
pub fn neue(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 255 as i32 - px.data[2] as i32 > 0 {
                px.data[2] = 255 - px.data[2];
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Solarization on the Red and Green channels.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::lix(img);
/// ```
pub fn lix(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            
            px.data[0] = 255 - px.data[0];
            px.data[1] = 255 - px.data[1];
            
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Solarization on the Red and Blue channels.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::ryo(img);
/// ```
pub fn ryo(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 255 as i32 - px.data[2] as i32 > 0 {
                px.data[0] = 255 - px.data[0];
                px.data[2] = 255 - px.data[2];
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Fallout-style radio effect.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::radio(img);
/// ```
pub fn radio(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome::monochroma(img, 5, 40, 20);
    return filtered_img;
}

/// Slight-blue tinted historical effect.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::twenties(img);
/// ```
pub fn twenties(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome::monochroma(img, 18, 12, 20);
    return filtered_img;
}

/// Rose-tinted filter.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::rosetint(img);
/// ```
pub fn rosetint(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome::monochroma(img, 80, 20, 31);
    return filtered_img;
}

/// Purple-infused filter.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::mauve(img);
/// ```
pub fn mauve(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome::monochroma(img, 90, 40, 80);
    return filtered_img;
}

/// Blue monochrome effect.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::bluechrome(img);
/// ```
pub fn bluechrome(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome::monochroma(img, 20, 30, 60);
    return filtered_img;
}

/// Vintage filter with a red tint.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::vintage(img);
/// ```
pub fn vintage(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::tint(img, 120, 70, 13);
    return filtered_img;
}

/// Increase the blue channel, with moderate increases in the Red and Green channels.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::perfume(img);
/// ```
pub fn perfume(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::tint(img, 80, 40, 120);
    return filtered_img;
}

/// Custom filter with an increase in the Blue channel's values.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// photon::filters::serenity(img);
/// ```
pub fn serenity(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::tint(img, 10, 40, 90);
    return filtered_img;
}