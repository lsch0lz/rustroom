use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

fn adjust_exposure(img: &DynamicImage, stops: f32) -> DynamicImage {
    let factor = 2f32.powf(stops); // e.g. stops = 1.0 doubles brightness
    let (w, h) = img.dimensions();
    let rgba = img.to_rgba8();

    let buf = ImageBuffer::from_fn(w, h, |x, y| {
        let Rgba([r, g, b, a]) = *rgba.get_pixel(x, y);
        let adjust = |c: u8| ((c as f32) * factor).clamp(0.0, 255.0) as u8;
        Rgba([adjust(r), adjust(g), adjust(b), a])
    });

    DynamicImage::ImageRgba8(buf)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("/Users/lukasscholz/Desktop/silt/front_spur_06_02.jpg")?.decode()?;
    let exposed = adjust_exposure(&img, 0.5); // +0.5 stops
    exposed.save("/Users/lukasscholz/Desktop/silt/front_spur_06_02.png")?;
    Ok(())
}