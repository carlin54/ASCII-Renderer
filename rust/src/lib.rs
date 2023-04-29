use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use ttf_parser;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
struct FontInfo {
    family_names: String,
    post_script_name: String,
    units_per_em: String,
    ascender: String,
    descender: String,
    line_gap: String,
    global_bounding_box: String,
    number_of_glyphs: String,
    underline_metrics: String,
    x_height: String,
    weight: String,
    width: String,
    is_regular: String,
    is_italic: String,
    is_bold: String,
    is_oblique: String,
    strikeout_metrics: String,
    subscript_metrics: String,
    superscript_metrics: String,
    permissions: String,
    is_variable: String
}

fn load_font_info(font_data: &Vec<u8>) -> FontInfo {

    let face = match ttf_parser::Face::parse(&font_data, 0) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let mut family_names = Vec::new();
    for name in face.names() {
        if name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode() {
            if let Some(family_name) = name.to_string() {
                let language = name.language();
                family_names.push(format!(
                    "{} ({}, {})",
                    family_name,
                    language.primary_language(),
                    language.region()
                ));
            }
        }
    }

    #[cfg(feature = "opentype-layout")]
    {
        if let Some(ref table) = face.tables().gpos {
            print_opentype_layout("positioning", table);
        }

        if let Some(ref table) = face.tables().gsub {
            print_opentype_layout("substitution", table);
        }
    }

    #[cfg(feature = "variable-fonts")]
    {
        if face.is_variable() {
            println!("Variation axes:");
            for axis in face.variation_axes() {
                println!(
                    "  {} {}..{}, default {}",
                    axis.tag, axis.min_value, axis.max_value, axis.def_value
                );
            }
        }
    }

    let post_script_name = face
        .names()
        .into_iter()
        .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode())
        .and_then(|name| name.to_string());

    let font_info = FontInfo {
        family_names: format!("{:?}", family_names),
        post_script_name: format!("{:?}", post_script_name),
        units_per_em: format!("{:?}", face.units_per_em()),
        ascender: format!("{}", face.ascender()),
        descender: format!("{}", face.descender()),
        line_gap: format!("{}", face.line_gap()),
        global_bounding_box: format!("{:?}", face.global_bounding_box()),
        number_of_glyphs: format!("{}", face.number_of_glyphs()),
        underline_metrics: format!("{:?}", face.underline_metrics()),
        x_height: format!("{:?}", face.x_height()),
        weight: format!("{:?}", face.weight()),
        width: format!("{:?}", face.width()),
        is_regular: format!("{}", face.is_regular()),
        is_italic: format!("{}", face.is_italic()),
        is_bold: format!("{}", face.is_bold()),
        is_oblique: format!("{}", face.is_oblique()),
        strikeout_metrics: format!("{:?}", face.strikeout_metrics()),
        subscript_metrics: format!("{:?}", face.subscript_metrics()),
        superscript_metrics: format!("{:?}", face.superscript_metrics()),
        permissions: format!("{:?}", face.permissions().unwrap()),
        is_variable: format!("{:?}", face.is_variable())
    };

    return font_info;
}


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[wasm_bindgen]
pub fn font_info(file_contents: Uint8Array)  -> Result<JsValue, JsValue> {

    let font_data: Vec<u8> = file_contents.to_vec();

    //let now = std::time::Instant::now();
    let font_info = load_font_info(&font_data);
    //println!("Elapsed: {}us", now.elapsed().as_micros());

    return to_value(&font_info).map_err(JsValue::from);
}


