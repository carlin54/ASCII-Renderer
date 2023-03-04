use std::borrow::Borrow;
use image::{DynamicImage, imageops, GenericImageView, ImageBuffer, Rgba, RgbImage, SubImage, RgbaImage, Pixel, GenericImage};
use rusttype::{Point, Font, GlyphId, GlyphIter, Scale, Glyph, Rect, PositionedGlyph};
use std::cmp;
use std::fmt::format;
use std::ops::Index;

fn filter_diff(kernel: &SubImage<&mut DynamicImage>, character: &RgbaImage) -> f64 {

    let (kw, kh) = kernel.dimensions();
    let (iw, ih) = character.dimensions();

    let width = std::cmp::min(kw, iw);
    let height = std::cmp::min(kh, ih);

    let mut s = 0.0;

    for x in 0..width {
        for y in 0..height {
            // TODO: extract this as a lambda
            s += 255.0 - ((kernel.get_pixel(x, y).channels()[0] as f64) -
                (character.get_pixel(x, y).channels()[0] as f64)).abs();

            s += 255.0 - ((kernel.get_pixel(x, y).channels()[1] as f64) -
                (character.get_pixel(x, y).channels()[1] as f64)).abs();

            s += 255.0 - ((kernel.get_pixel(x, y).channels()[2] as f64) -
                (character.get_pixel(x, y).channels()[2] as f64)).abs();
        }
    }

    return s / ((width * height) as f64)
}

fn filter_convolve(kernel: &SubImage<&mut DynamicImage>, character: &RgbaImage) -> f64 {

    let (kw, kh) = kernel.dimensions();
    let (iw, ih) = character.dimensions();

    let width = std::cmp::min(kw, iw);
    let height = std::cmp::min(kh, ih);

    let mut s = 0.0;

    for x in 0..width {
        for y in 0..height {

            s += (kernel.get_pixel(x, y).channels()[0] as f64) *
                (character.get_pixel(x, y).channels()[0] as f64);

            s += (kernel.get_pixel(x, y).channels()[1] as f64) *
                (character.get_pixel(x, y).channels()[1] as f64);

            s += (kernel.get_pixel(x, y).channels()[2] as f64) *
                (character.get_pixel(x, y).channels()[2] as f64);

        }
    }

    return s / ((width * height) as f64)
}


fn optimized_filter(kernel: &SubImage<&mut DynamicImage>, character: &RgbaImage) -> (u32, u32) {
    let (kw, kh) = kernel.dimensions();
    let (iw, ih) = character.dimensions();
    return (3, 3);
}

fn match_character(kernel: &SubImage<&mut DynamicImage>, characters: &Vec<RgbaImage>) -> usize {

    let results: Vec<f64> = Vec::new();
    let mut max_idx = 0;
    let mut max_value : f64 = -1.0;
    for (idx, character) in characters.iter().enumerate() {
        let value = filter_diff(kernel, character);

        if (value > max_value) {
            max_value = value;
            max_idx = idx;
        }

    }

    println!("max index: {}, max value: {}", max_idx, max_value);

    return max_idx;
}

fn paint_background(img: &mut image::RgbaImage, colour: &image::Rgba<u8>) {
    for x in 0..img.width() {
        for y in 0..img.height() {
           img.put_pixel(x, y, *colour) ;
        }
    }
}

fn paint_character(img: &mut image::RgbaImage, colour: &image::Rgba<u8>, glyph: &PositionedGlyph) {
    let (r, g, b, a) = colour.channels4();

    let o = |x, y, v| {

        img.blend_pixel(
            // Offset the position by the glyph bounding box
            x as u32,
            y as u32,

            // Turn the coverage into an alpha value
            Rgba([r, g, b, (v * (a as f32)) as u8]),
        )
    };

    glyph.draw(o);
}

fn main() {
    let font_data = include_bytes!("../fonts/Arial-Monospaced.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    let image_name = "clouds.png";
    let colour = (150, 0, 0);

    // Desired font pixel height
    let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    let scale = Scale {
        x: 16.0,
        y: 16.0
    };

    let point = Point {
        x: 0.0,
        y: 0.0
    };

    let v_metrics = font.v_metrics(scale);

    println!("Rendering glyphs from font.");
    println!("scale.x: {}, scale.y: {}", scale.x, scale.y);
    println!("point.x: {}, point.y: {}", point.x, point.y);

    let mut glyphs = Vec::new();
    let mut glyph_height: u32 = 0;
    let mut glyph_width: u32 = 0;

    println!("glyph_width: {}, glyph_height: {}", glyph_width, glyph_height);
    let mut rendered_glyphs: Vec<RgbaImage> = Vec::new();

    let default_rect = Rect {
        min: Point { x: 0.0, y: 0.0 },
        max: Point { x: 0.0, y: 0.0 },
    };

    let mut glyph_colours: Vec<Rgba<u8>> = Vec::new();
    /*
    glyph_colours.push(Rgba([255,0,0,255]));
    glyph_colours.push(Rgba([0,255,0,255]));
    glyph_colours.push(Rgba([0,0,255,255]));
     */

    glyph_colours.push(Rgba([255,255,255,255]));
    //glyph_colours.push(Rgba([0,0,0,255]));
    //glyph_colours.push(Rgba([0,0,0,255]));

    let mut glyph_background: Vec<Rgba<u8>> = Vec::new();
    glyph_background.push(Rgba([0,0,0,255]));
    glyph_background.push(Rgba([255,0,0,255]));
    glyph_background.push(Rgba([0,255,0,255]));
    glyph_background.push(Rgba([0,0,255,255]));
    //glyph_background.push(Rgba([255,255,255,255]));
    //glyph_background.push(Rgba([250,255,255,255]));
    //glyph_background.push(Rgba([0,0,0,255]));

    let start = 0;
    let end = font.glyph_count() as u16;
    let mut counter = 0;
    for i in 0u16..font.glyph_count().try_into().unwrap() {

        if i < start || i > end {
            continue;
        }

        let glyphId = GlyphId(i);

        let scaled_glyph = font.glyph(glyphId).scaled(scale);
        let height = scaled_glyph.exact_bounding_box().unwrap_or(default_rect).height().ceil() as u32 + 1;
        let width = scaled_glyph.exact_bounding_box().unwrap_or(default_rect).width().ceil() as u32 + 1;

        if height == 0 || width == 0 {
            continue;
        }


        let positioned_glyph = scaled_glyph.positioned(point);

        println!("width: {}, height: {}", width, height);
        glyph_width = std::cmp::max(glyph_width, width);
        glyph_height = std::cmp::max(glyph_height, height);


        for background_colour in &glyph_background {
            let mut background = DynamicImage::new_rgba8(width, height).to_rgba8();

            paint_background(&mut background, background_colour);
            for glyph_colour in &glyph_colours {
                //if let Some(bb) = glyph.pixel_bounding_box() {
                let mut glyph = background.clone();
                paint_character(&mut glyph, glyph_colour, &positioned_glyph);
                // Could add rotation as well
                glyph.save(format!("./images/glyphs/{}.png", counter)).expect("TODO: panic message");
                counter += 1;

                glyph_width = glyph_width.max(glyph.width());
                glyph_height = glyph_height.max(glyph.height());

                rendered_glyphs.push(glyph);

            }
        }

        glyphs.push(positioned_glyph);

    }


    println!("glyph_width: {}, glyph_height: {}", glyph_width, glyph_height);
    //let mut rendered_glyphs : Vec<RgbaImage> = Vec::new();

    println!("Rendering glyphs into bitmaps.");
    for glyph in glyphs {
        let mut image = DynamicImage::new_rgba8(glyph_width, glyph_height).to_rgba8();

        if let Some(bb) = glyph.pixel_bounding_box() {
            let mut image = DynamicImage::new_rgba8(glyph_width, glyph_height).to_rgba8();
            let mut image = DynamicImage::new_rgba8(glyph_width, glyph_height).to_rgba8();

            if let Some(bb) = glyph.pixel_bounding_box() {
                let o = |x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x as u32,
                        y as u32,

                        // Turn the coverage into an alpha value
                        Rgba([255, 255, 255, (v * 255.0) as u8]),
                    )
                };

                glyph.draw(o);
                //let path = format!("./export/{}.png", counter);
                //image.save(path).expect("TODO: panic message");

                rendered_glyphs.push(image);
            }
        }
    }
    println!("Converting the image rendering image to grayscale.");
    let mut img = image::open("./images/input/".to_owned() + image_name).unwrap();
    let (width, height) = img.dimensions();
    let (kx, ky) = (glyph_width, glyph_height);
    let (stride_x, stride_y) = (glyph_width as usize, glyph_height as usize);

    let mut img_gray = img.grayscale();
    let mut out: RgbaImage = ImageBuffer::new(width * 2, height * 2);

    // 1. Filter Size
    for x in (0..width - kx).step_by(stride_x) {
        for y in (0..height - ky).step_by(stride_y) {

            let subimg = imageops::crop(&mut img, x, y, kx, ky);
            let index = match_character(&subimg, &rendered_glyphs);
            let character = &rendered_glyphs[index];
            imageops::overlay(&mut out, character, x as i64, y as i64);
        }
    }

    out.save("./images/output/".to_owned() + image_name).expect("TODO: panic message");
}