use std::borrow::Borrow;
use image::{DynamicImage, imageops, GenericImageView, ImageBuffer, Rgba, RgbImage, SubImage, RgbaImage, Pixel, GenericImage};
use rusttype::{Point, Font, GlyphId, GlyphIter, Scale, Glyph, Rect, PositionedGlyph};
use std::cmp;
use std::fmt::format;
use std::ops::Index;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::time::Instant;


#[wasm_bindgen]
pub struct AsciiImageTransformer {
    status_callback: Option<Box<dyn Fn(f64)>>,
    status: bool,
}

#[wasm_bindgen]
impl AsciiImageTransformer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AsciiImageTransformer {
        AsciiImageTransformer {
            status_callback: None,
            status: false,
        }
    }

    pub fn set_status_callback(&mut self, callback: js_sys::Function) {
        let callback = Box::new(move |progress: f64| {
            callback.call1(&JsValue::NULL, &JsValue::from(progress)).unwrap();
        });

        self.status_callback = Some(callback);
    }

    pub fn start_processing(&mut self) {
        self.status = true;

        let num_threads = 24;
        ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();

        let font_data = include_bytes!(font_path);
        let font = Font::try_from_bytes(font_data).unwrap();
        let image_name = image_path;
        let mut img = image::open("./images/input/".to_owned() + image_name).unwrap();

        let colour = (150, 0, 0);

        // Desired font pixel height
        let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
        let pixel_height = height.ceil() as usize;

        let scale = Scale {
            x: 18.0,
            y: 18.0
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

        // Colour pallet method
        let mut glyph_colours: Vec<Rgba<u8>> = Vec::new();
        let mut glyph_background: Vec<Rgba<u8>> = Vec::new();
        let black = Rgba([0,0,0,255]);
        let white = Rgba([255,255,255,255]);

        colour_scheme_gray_scale(&mut glyph_background, 3);
        glyph_background.push(white);
        glyph_colours.push(black);
        //colour_scheme_gradations(&mut glyph_background, 3);
        //colour_scheme_gradations(&mut glyph_colours, 4);
        colour_scheme_gray_scale(&mut glyph_colours, 7);
        //colour_scheme_plotlogic(&mut glyph_colours);
        //glyph_colours.push(black);

        let start= 36; // = 36;
        let end = font.glyph_count() as u16; // = 36+25; //

        let glyph_background_size = "glyph-min";
        let glyph_background_size = "glyph-max";

        let mut max_height = 0;
        let mut max_width = 0;
        let interception = true;

        // Get the max width, max height
        for i in 0u16..font.glyph_count().try_into().unwrap() {
            if i < start || i > end {
                continue;
            }

            let glyphId = GlyphId(i);

            let scaled_glyph = font.glyph(glyphId).scaled(scale);
            let height = scaled_glyph.exact_bounding_box().unwrap_or(default_rect).height().ceil() as u32 + 1;
            let width = scaled_glyph.exact_bounding_box().unwrap_or(default_rect).width().ceil() as u32 + 1;

            max_width = std::cmp::max(width, max_width);
            max_height = std::cmp::max(height, max_height);
        }

        // Get
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

                let mut background;
                if glyph_background_size == "glyph-min" {
                    background = DynamicImage::new_rgba8(width, height).to_rgba8();
                } else if glyph_background_size == "glyph-max" {
                    background = DynamicImage::new_rgba8(max_width, max_height).to_rgba8();
                } else {
                    background = DynamicImage::new_rgba8(width, height).to_rgba8();
                }

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
        let (width, height) = img.dimensions();
        let (kx, ky) = (glyph_width, glyph_height);
        let (stride_x, stride_y) = (glyph_width as usize, glyph_height as usize);

        let mut img_gray = img.grayscale();
        let mut img_cp = img.clone();
        let mut out: RgbaImage = ImageBuffer::new(width, height);

        // 1. Filter Size

        let total_kernel_ops = ((width - kx) / stride_x as u32) * ((height - ky) / stride_y as u32);
        let mut data = vec![];
        for x in (0..width - kx).step_by(stride_x) {
            for y in (0..height - ky).step_by(stride_y) {
                let subimg = img.view(x, y, kx, ky);
                let item = (subimg, x, y);
                data.push(item);
            }
        }

        let results = Arc::new(Mutex::new(Vec::with_capacity(data.len())));
        let start_time = Instant::now(); // Record the start time before the loop starts

        data.par_iter().for_each(|item| {
            let subimg = &item.0;
            let x = &item.1;
            let y = &item.2;

            let index = match_character(subimg, &rendered_glyphs);
            let result = (index, x, y);
            let mut results_guard = results.lock().unwrap();
            results_guard.push(result);
            print_eta(results_guard.len() as u32, total_kernel_ops, start_time);
        });

        let results_guard = results.lock().unwrap();
        for result in results_guard.iter() {
            let index = result.0;
            let x = result.1;
            let y = result.2;

            let character = &rendered_glyphs[index];
            imageops::overlay(&mut out, character, *x as i64, *y as i64);
        }

        out.save("./images/output/".to_owned() + image_name).expect("TODO: panic message");

    }

    pub fn stop_processing(&mut self) {
        self.status = false;
    }



}


fn print_opentype_layout(name: &str, table: &ttf_parser::opentype_layout::LayoutTable) {
    println!("OpenType {}:", name);
    println!("  Scripts:");
    for script in table.scripts {
        println!("    {}", script.tag);

        if script.languages.is_empty() {
            println!("      No languages");
            continue;
        }

        println!("      Languages:");
        for lang in script.languages {
            println!("        {}", lang.tag);
        }
    }

    let mut features: Vec<_> = table.features.into_iter().map(|f| f.tag).collect();
    features.dedup();
    println!("  Features:");
    for feature in features {
        println!("    {}", feature);
    }
}


#[wasm_bindgen]
pub fn parse_image(
    font: Uint8Array,
    image: Uint8Array,
    background: Uint8Array,
    foreground: Uint8Array,
    status_callback: &js_sys::Function,

) {
    /*
           1. Preprocess the image
               - Greyscale


           2. Generate the characters
               - Generate backgrounds colours
               - Generate glyph colours - Histogram of colour
               - Scale glyphs
               - Render glyphs

           3. Map the characters
               - Single threaded
               - Multi threaded

           TODO:
               - Colour picking tool
               - Exporting for .bashrc
               - Estimate the compute time
               - Generate output/input
               - GPU, CUDA or WebAssembly
               - ANN (tff, scale, backgrounds, colours)
                   - Train a large network, just do drop out on the glyphs that dont exist
        */
    let num_threads = 24;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let font_data = include_bytes!(font_path);
    let font = Font::try_from_bytes(font_data).unwrap();
    let image_name = image_path;
    let mut img = image::open("./images/input/".to_owned() + image_name).unwrap();

    let colour = (150, 0, 0);

    // Desired font pixel height
    let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    let scale = Scale {
        x: 18.0,
        y: 18.0
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

    // Colour pallet method
    let mut glyph_colours: Vec<Rgba<u8>> = Vec::new();
    let mut glyph_background: Vec<Rgba<u8>> = Vec::new();
    let black = Rgba([0,0,0,255]);
    let white = Rgba([255,255,255,255]);

    colour_scheme_gray_scale(&mut glyph_background, 3);
    glyph_background.push(white);
    glyph_colours.push(black);
    //colour_scheme_gradations(&mut glyph_background, 3);
    //colour_scheme_gradations(&mut glyph_colours, 4);
    colour_scheme_gray_scale(&mut glyph_colours, 7);
    //colour_scheme_plotlogic(&mut glyph_colours);
    //glyph_colours.push(black);

    let start= 36; // = 36;
    let end = font.glyph_count() as u16; // = 36+25; //

    let glyph_background_size = "glyph-min";
    let glyph_background_size = "glyph-max";

    let mut max_height = 0;
    let mut max_width = 0;
    let interception = true;

    // Get the max width, max height
    for i in 0u16..font.glyph_count().try_into().unwrap() {
        if i < start || i > end {
            continue;
        }

        let glyphId = GlyphId(i);

        let scaled_glyph = font.glyph(glyphId).scaled(scale);
        let height = scaled_glyph.exact_bounding_box().unwrap_or(default_rect).height().ceil() as u32 + 1;
        let width = scaled_glyph.exact_bounding_box().unwrap_or(default_rect).width().ceil() as u32 + 1;

        max_width = std::cmp::max(width, max_width);
        max_height = std::cmp::max(height, max_height);
    }

    // Get
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

            let mut background;
            if glyph_background_size == "glyph-min" {
                background = DynamicImage::new_rgba8(width, height).to_rgba8();
            } else if glyph_background_size == "glyph-max" {
                background = DynamicImage::new_rgba8(max_width, max_height).to_rgba8();
            } else {
                background = DynamicImage::new_rgba8(width, height).to_rgba8();
            }

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
    let (width, height) = img.dimensions();
    let (kx, ky) = (glyph_width, glyph_height);
    let (stride_x, stride_y) = (glyph_width as usize, glyph_height as usize);

    let mut img_gray = img.grayscale();
    let mut img_cp = img.clone();
    let mut out: RgbaImage = ImageBuffer::new(width, height);

    // 1. Filter Size


    /*
    counter = 0;
    let max_counter = ((width - kx)/stride_x as u32) * ((height - ky)/stride_y as u32);

    for x in (0..width - kx).step_by(stride_x) {
        for y in (0..height - ky).step_by(stride_y) {
            println!("{} of {}", counter, max_counter);
            counter += 1;
            let subimg = imageops::crop(&mut img, x, y, kx, ky);
            let index = match_character(&subimg, &rendered_glyphs);
            let character = &rendered_glyphs[index];
            imageops::overlay(&mut out, character, x as i64, y as i64);
        }
    }
    */

    let total_kernel_ops = ((width - kx) / stride_x as u32) * ((height - ky) / stride_y as u32);
    let mut data = vec![];
    for x in (0..width - kx).step_by(stride_x) {
        for y in (0..height - ky).step_by(stride_y) {
            let subimg = img.view(x, y, kx, ky);
            let item = (subimg, x, y);
            data.push(item);
        }
    }

    let results = Arc::new(Mutex::new(Vec::with_capacity(data.len())));
    let start_time = Instant::now(); // Record the start time before the loop starts

    data.par_iter().for_each(|item| {
        let subimg = &item.0;
        let x = &item.1;
        let y = &item.2;

        let index = match_character(subimg, &rendered_glyphs);
        let result = (index, x, y);
        let mut results_guard = results.lock().unwrap();
        results_guard.push(result);
        print_eta(results_guard.len() as u32, total_kernel_ops, start_time);
    });

    let results_guard = results.lock().unwrap();
    for result in results_guard.iter() {
        let index = result.0;
        let x = result.1;
        let y = result.2;

        let character = &rendered_glyphs[index];
        imageops::overlay(&mut out, character, *x as i64, *y as i64);
    }

    out.save("./images/output/".to_owned() + image_name).expect("TODO: panic message");
}
