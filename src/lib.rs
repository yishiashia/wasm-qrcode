mod utils;

use wasm_bindgen::prelude::*;

use base64::encode;

extern crate qrcode;
use qrcode::{QrCode, Version, EcLevel};
use qrcode::types::QrError;
use qrcode::render::svg;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn gen_qrcode<T>(data: T, width: u32, height: u32) -> Result<String, QrError>
where T: AsRef<[u8]> {
  QrCode::new(data.as_ref())
    .map(|code| code.render::<svg::Color>()
        .min_dimensions(width, height)
        .quiet_zone(false)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
        .replace(r#"<?xml version="1.0" standalone="yes"?>"#, "")
    )
}

#[wasm_bindgen]
pub fn qrcode(arg: &str, width: u32, height: u32) -> String {
  match gen_qrcode(arg, width, height) {
      Ok(v) => v,
      Err(e) => format!("{}", e),
  }
}

#[wasm_bindgen]
pub fn qrcode_base64(arg: &str, width: u32, height: u32) -> String {
  match gen_qrcode(arg, width, height) {
      Ok(v) => base64::encode(v),
      Err(e) => format!("{}", e),
  }
}