// #![allow(dead_code)]
// #![allow(unused_variables)]

extern crate powierza_coefficient;
extern crate strsim;
extern crate wasm_bindgen;
// extern crate serde_json;
// extern crate hex;
// use regex::Regex;
use wasm_bindgen::prelude::*;
use powierza_coefficient::powierża_coefficient as pc;
use strsim::{
  // hamming,
  // levenshtein,
  // normalized_levenshtein,
  osa_distance,
  // damerau_levenshtein,
  // normalized_damerau_levenshtein,
  // jaro,
  // jaro_winkler,
  // sorensen_dice
};
// #[macro_use]
// extern crate serde_derive;


//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn powierza( a: &str, b: &str ) -> u32 {
  // return match pc( a, b ) {
  //   Ok( v ) => v,
  //   Err( error ) => {
  //     alert( &format!( "^895734^ error decoding hexadecimal: {}", error ) );
      // std::process::exit( 1 ); }, }; }
  let r = pc( a, b );
  if r.is_some() { return r.unwrap() }
  return 999999999; }

//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn osa( a: &str, b: &str ) -> usize {
  return osa_distance( a, b ); }
  // let r = pc( a, b );
  // if r.is_some() { return r.unwrap() }
  // return 999999999; }

//----------------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------------


//----------------------------------------------------------------------------------------------------------
/* ### NOTE these methods must be provided as global functions by the calling JS; we're doing this e.g. by
  setting `globalThis.alert = -> ...` in the accompanying demo. */
#[wasm_bindgen]
extern {
  pub fn info(  s: &str );
  pub fn alert( s: &str );
  pub fn help(  s: &str );
  pub fn urge(  s: &str ); }

// //----------------------------------------------------------------------------------------------------------
// use std::str::FromStr;
// use rustybuzz;
// use ttf_parser;
// // use svgtypes::WriteBuffer;
// use svgtypes::PathSegment;
// use serde_json::json;
// use textwrap;
// // use std::collections::HashMap;
// // #[macro_use]
// // extern crate lazy_static;


// //==========================================================================================================
// // HELPERS
// //----------------------------------------------------------------------------------------------------------
// fn rtodp( a: f64 ) -> String {
//   /* "Round To One Decimal Place", returns string with up to one decimal place, using 4/5 rounding. */
//   return format!( "{}", ( a * 10.0f64 ).round() / 10.0f64 ); }



// //==========================================================================================================
// // PERSISTENT STATE
// // //----------------------------------------------------------------------------------------------------------
// // // thx to https://stackoverflow.com/a/19608953/256361
// // //----------------------------------------------------------------------------------------------------------
// // lazy_static! {
// //   // thx to https://docs.rs/lazy_static/1.4.0/lazy_static/ and Rust error messages
// //   static ref CACHE: Vec<&'static CacheEntry<'static>> = {
// //     #![allow(unused_mut)]
// //     let mut m = Vec::new();
// //     // m.insert(0, "foo");
// //     m
// // }; }

// // thx to https://stackoverflow.com/a/27826181/256361
// // use lazy_static::lazy_static; // 1.4.0
// // use std::sync::Mutex;

// // lazy_static! {
// //     static ref CACHE: Mutex<Vec<CacheEntry<'static>>> = Mutex::new(vec![]);
// // }

// // fn do_a_call() { CACHE.lock().unwrap().push(1);}
// // fn main() {
// //     do_a_call();
// //     do_a_call();
// //     do_a_call();
// //     println!("called {}", CACHE.lock().unwrap().len());}

// // //----------------------------------------------------------------------------------------------------------
// // pub struct CacheEntry<'a> {
// //   rustybuzz_face:    rustybuzz::Face<'a>,
// //   ttfparser_face:   ttf_parser::Face<'a>,
// //   // scale: ...,
// // }

// static mut FONTBYTES_0: Vec<u8> = vec![];
// static mut FONTBYTES_1: Vec<u8> = vec![];
// static mut FONTBYTES_2: Vec<u8> = vec![];
// static mut FONTBYTES_3: Vec<u8> = vec![];
// static mut FONTBYTES_4: Vec<u8> = vec![];
// static mut FONTBYTES_5: Vec<u8> = vec![];
// static mut FONTBYTES_6: Vec<u8> = vec![];
// static mut FONTBYTES_7: Vec<u8> = vec![];
// static mut FONTBYTES_8: Vec<u8> = vec![];
// static mut FONTBYTES_9: Vec<u8> = vec![];
// static mut FONTBYTES_10: Vec<u8> = vec![];
// static mut FONTBYTES_11: Vec<u8> = vec![];
// static mut FONTBYTES_12: Vec<u8> = vec![];
// static mut FONTBYTES_13: Vec<u8> = vec![];
// static mut FONTBYTES_14: Vec<u8> = vec![];
// static mut FONTBYTES_15: Vec<u8> = vec![];


// // static mut FONT_BYTES: Vec<u8> = vec![];
// // //----------------------------------------------------------------------------------------------------------
// // #[wasm_bindgen]
// // pub fn set_font_bytes( font_bytes_hex: String ) {
// //   unsafe { FONT_BYTES = match hex::decode( font_bytes_hex ) {
// //     Ok( v ) => v,
// //     Err( error ) => {
// //       alert( &format!( "^895734^ error decoding hexadecimal: {}", error ) );
// //       std::process::exit( 1 ); }, }; }; }
// // //----------------------------------------------------------------------------------------------------------
// // #[wasm_bindgen]
// // pub fn has_font_bytes() -> bool { unsafe { !FONT_BYTES.is_empty() } }

// //----------------------------------------------------------------------------------------------------------
// // #![allow(unused_mut)]
// #[wasm_bindgen]
// pub fn register_font( font_idx: u16, font_bytes_hex: String ) {
//   // if font_idx > 4 {
//   //   alert( &format!( "^895455^ font_idx must be between 1 and 3, got {}", font_idx ) );
//   //   std::process::exit( 1 ); };
//   let face_idx    = 0;
//   let font_bytes  = match hex::decode( font_bytes_hex ) {
//     Ok( v ) => v,
//     Err( error ) => {
//       alert( &format!( "^895734^ error decoding hexadecimal: {}", error ) );
//       std::process::exit( 1 ); }, };
//   unsafe {
//     match font_idx {
//       0  => FONTBYTES_0  = font_bytes,
//       1  => FONTBYTES_1  = font_bytes,
//       2  => FONTBYTES_2  = font_bytes,
//       3  => FONTBYTES_3  = font_bytes,
//       4  => FONTBYTES_4  = font_bytes,
//       5  => FONTBYTES_5  = font_bytes,
//       6  => FONTBYTES_6  = font_bytes,
//       7  => FONTBYTES_7  = font_bytes,
//       8  => FONTBYTES_8  = font_bytes,
//       9  => FONTBYTES_9  = font_bytes,
//       10 => FONTBYTES_10 = font_bytes,
//       11 => FONTBYTES_11 = font_bytes,
//       12 => FONTBYTES_12 = font_bytes,
//       13 => FONTBYTES_13 = font_bytes,
//       14 => FONTBYTES_14 = font_bytes,
//       15 => FONTBYTES_15 = font_bytes,
//       16 ..= std::u16::MAX => {
//         alert( &format!( "^895433^ font_idx must be between 0 and 15, got {}", font_idx ) );
//         std::process::exit( 1 ); } } }; }

// //----------------------------------------------------------------------------------------------------------
// #[wasm_bindgen]
// pub fn font_register_is_free( font_idx: u16 ) -> bool {
//   unsafe {
//     match font_idx {
//       0  => ( FONTBYTES_0.is_empty() ),
//       1  => ( FONTBYTES_1.is_empty() ),
//       2  => ( FONTBYTES_2.is_empty() ),
//       3  => ( FONTBYTES_3.is_empty() ),
//       4  => ( FONTBYTES_4.is_empty() ),
//       5  => ( FONTBYTES_5.is_empty() ),
//       6  => ( FONTBYTES_6.is_empty() ),
//       7  => ( FONTBYTES_7.is_empty() ),
//       8  => ( FONTBYTES_8.is_empty() ),
//       9  => ( FONTBYTES_9.is_empty() ),
//       10 => ( FONTBYTES_10.is_empty() ),
//       11 => ( FONTBYTES_11.is_empty() ),
//       12 => ( FONTBYTES_12.is_empty() ),
//       13 => ( FONTBYTES_13.is_empty() ),
//       14 => ( FONTBYTES_14.is_empty() ),
//       15 => ( FONTBYTES_15.is_empty() ),
//       16 ..= std::u16::MAX => {
//         alert( &format!( "^895433^ font_idx must be between 0 and 15, got {}", font_idx ) );
//         std::process::exit( 1 ); } } } }

// //----------------------------------------------------------------------------------------------------------
// pub fn get_fontbytes( font_idx: u16 ) -> &'static Vec<u8> {
//   unsafe {
//     match font_idx {
//       0  => &FONTBYTES_0,
//       1  => &FONTBYTES_1,
//       2  => &FONTBYTES_2,
//       3  => &FONTBYTES_3,
//       4  => &FONTBYTES_4,
//       5  => &FONTBYTES_5,
//       6  => &FONTBYTES_6,
//       7  => &FONTBYTES_7,
//       8  => &FONTBYTES_8,
//       9  => &FONTBYTES_9,
//       10 => &FONTBYTES_10,
//       11 => &FONTBYTES_11,
//       12 => &FONTBYTES_12,
//       13 => &FONTBYTES_13,
//       14 => &FONTBYTES_14,
//       15 => &FONTBYTES_15,
//       16 ..= std::u16::MAX => {
//         alert( &format!( "^895433^ font_idx must be between 0 and 15, got {}", font_idx ) );
//         std::process::exit( 1 ); } } } }


// //==========================================================================================================
// // CONFIGURATION
// //----------------------------------------------------------------------------------------------------------
// #[derive(Serialize, Deserialize)]
// // acc to https://doc.rust-lang.org/std/fmt/trait.Debug.html
// #[derive(Debug)]
// pub struct CfgOpt {
//     pub font_idx:         Option<u16>,
//     pub text:             Option<String>,
//     // pub font_bytes_hex:   Option<String>,
//     pub format:           Option<String>, }

// #[derive(Debug)]
// pub struct Cfg {
//     pub font_idx:         u16,
//     pub text:             String,
//     pub cluster_level:    rustybuzz::BufferClusterLevel,
//     pub direction:        rustybuzz::Direction,
//     pub face_idx:       u32,
//     // pub font_bytes:       Vec<u8>,
//     pub format:           String,
//     pub variations:       Vec<rustybuzz::Variation>,
//     pub features:         Vec<rustybuzz::Feature>,
//     pub script:           Option<rustybuzz::Script>,
//     pub language:         rustybuzz::Language, }

// const FONT_SIZE: f64 = 1000.0;
// const PRECISION: f64 = 1.0;


// //==========================================================================================================
// //
// //----------------------------------------------------------------------------------------------------------
// fn parse_features(s: &str) -> Result<Vec<rustybuzz::Feature>, String> {
//     let mut features = Vec::new();
//     for f in s.split(',') {
//         features.push(rustybuzz::Feature::from_str(&f)?); }
//     Ok(features) }


// //==========================================================================================================
// //
// //----------------------------------------------------------------------------------------------------------
// // thx to https://hacks.mozilla.org/2019/11/multi-value-all-the-wasm/
// // #[wasm_bindgen]
// // pub fn shape_text( user_cfg: &JsValue ) -> String {
// //   //........................................................................................................
// //   let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
// //   let cfg = Cfg {
// //     text:           match cfg_opt.text      { None => String::from( "some text" ), Some( x ) => x, },
// //     // ### TAINT use enumeration
// //     format:         match cfg_opt.format    { None => String::from( "json" ),      Some( x ) => x, },
// //     font_idx:       match cfg_opt.font_idx  { None => 0,                           Some( x ) => x, },
// //     language:       rustybuzz::Language::from_str( "English" ).unwrap(),
// //     //......................................................................................................
// //     // script:         Some( rustybuzz::Script::new() ),
// //     script:         None,
// //     features:       vec![],
// //     variations:     vec![],
// //     direction:      rustybuzz::Direction::LeftToRight,
// //     // cluster_level:  rustybuzz::BufferClusterLevel::MonotoneGraphemes,
// //     // cluster_level: rustybuzz::BufferClusterLevel::MonotoneCharacters,
// //     cluster_level: rustybuzz::BufferClusterLevel::Characters,
// //     face_idx:   0, };
// //   //........................................................................................................
// //   // info( &format!( "^3344^ size_mm: {}, scale_factor: {}", cfg.size_mm, scale_factor ) );
// //   if font_register_is_free( cfg.font_idx ) {
// //     alert( &format!( "^rustybuzz-wasm/shape_text@39883^ no font registered for font_idx: {}", cfg.font_idx ) );
// //     panic!( "no font registered for font_idx" ); }
// //   //........................................................................................................
// //   let mut face = rustybuzz::Face::from_slice( get_fontbytes( cfg.font_idx ), cfg.face_idx ).unwrap();
// //   if !cfg.variations.is_empty() { face.set_variations( &cfg.variations ); }
// //   let mut buffer = rustybuzz::UnicodeBuffer::new();
// //   buffer.push_str( &cfg.text );
// //   buffer.set_direction( cfg.direction );
// //   //........................................................................................................
// //   // TAINT in rustybuzz 0.4, `face.units_per_em()` has become public so no need for `ttfp_face` ###
// //   let ttfp_face     = ttf_parser::Face::from_slice( get_fontbytes( cfg.font_idx ), cfg.face_idx ).unwrap();
// //   let units_per_em  = match ttfp_face.units_per_em() { None => 0 as u16, Some( x ) => x, };
// //   let scale         = FONT_SIZE / units_per_em as f64;
// //   //........................................................................................................
// //   buffer.set_language(cfg.language);
// //   if let Some(script) = cfg.script { buffer.set_script(script); }
// //   buffer.set_cluster_level(cfg.cluster_level);
// //   // if !cfg.utf8_clusters { buffer.reset_clusters(); }
// //   //........................................................................................................
// //   let glyph_buffer = rustybuzz::shape( &face, &cfg.features, buffer );
// //   // urge( &format!( "^5454^ text: {:#?}", cfg.text ) );
// //   // urge( &format!( "^5454^ glyph_buffer: {:#?}", glyph_buffer ) );
// //   //........................................................................................................
// //   if cfg.format == "json" { return glyfs_as_json( scale, &glyph_buffer, ); }
// //   else if cfg.format == "rusty" {
// //     let format_flags: rustybuzz::SerializeFlags =
// //       // rustybuzz::SerializeFlags::NO_GLYPH_NAMES |
// //       // rustybuzz::SerializeFlags::GLYPH_EXTENTS |
// //       rustybuzz::SerializeFlags::GLYPH_FLAGS;
// //     return glyph_buffer.serialize( &face, format_flags ); }
// //   // urge( &format!( "^33321^ {}", glyfs_as_short( &glyph_buffer, ) ) );
// //   return glyfs_as_short( &glyph_buffer, ); }


// //==========================================================================================================
// //
// //----------------------------------------------------------------------------------------------------------
// pub fn glyfs_as_json( scale: f64, glyph_buffer: &rustybuzz::GlyphBuffer, ) -> String {
//   _glyfs_as_json( scale, &glyph_buffer, ).unwrap_or_default() }

// //----------------------------------------------------------------------------------------------------------
// fn _glyfs_as_json( scale: f64, glyph_buffer: &rustybuzz::GlyphBuffer, ) -> Result<String, std::fmt::Error> {
//   use std::fmt::Write;
//   let mut s       = String::with_capacity(64);
//   let info        = glyph_buffer.glyph_infos();
//   let pos         = glyph_buffer.glyph_positions();
//   let mut x: f64  = 0.0;
//   let mut y: f64  = 0.0;
//   write!(&mut s, "[" )?;
//   for (info, pos) in info.iter().zip(pos) {
//     let dx    = ( pos.x_advance as f64 ) * scale;
//     let dy    = ( pos.y_advance as f64 ) * scale;
//     let nobr  = info.unsafe_to_break(); // see https://docs.rs/rustybuzz/0.4.0/rustybuzz/struct.GlyphInfo.html#method.unsafe_to_break
//     write!(&mut s, "{{" )?;
//     write!(&mut s, "\"gid\":{},",   info.glyph_id           )?;
//     write!(&mut s, "\"b\":{},",     info.cluster            )?; // *b*yte index
//     if nobr { write!(&mut s, "\"nobr\":true," )?; };
//     write!(&mut s, "\"x\":{},\"y\":{},",  rtodp( x  ), rtodp( y  ) )?;
//     write!(&mut s, "\"dx\":{},\"dy\":{}", rtodp( dx ), rtodp( dy ) )?;
//     x += dx;
//     y += dy;
//     //....................................................................................................
//     write!(&mut s, "}}" )?;
//     s.push(','); }
//   //........................................................................................................
//   if !s.is_empty() { s.pop(); } // Remove last `,`
//   write!(&mut s, "]" )?;
//   Ok(s) }


// //==========================================================================================================
// //
// //----------------------------------------------------------------------------------------------------------
// pub fn glyfs_as_short( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> String {
//   _glyfs_as_short( &glyph_buffer, ).unwrap_or_default() }

// //----------------------------------------------------------------------------------------------------------
// fn _glyfs_as_short( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> Result<String, std::fmt::Error> {
//   use std::fmt::Write;
//   let mut s = String::with_capacity(64);
//   let info  = glyph_buffer.glyph_infos();
//   let pos   = glyph_buffer.glyph_positions();
//   let mut x = 0;
//   let mut y = 0;
//   write!(&mut s, "|" )?;
//   for (info, pos) in info.iter().zip(pos) {
//     write!(&mut s, "{}:", info.glyph_id)?;
//     write!(&mut s, "{},{};", x, y )?;
//     write!(&mut s, "{},{}", pos.x_advance, pos.y_advance )?;
//     x += pos.x_advance;
//     y += pos.y_advance;
//     //....................................................................................................
//     write!(&mut s, "|" )?;
//     }
//   //........................................................................................................
//   Ok(s) }


// //==========================================================================================================
// //
// //----------------------------------------------------------------------------------------------------------
// // #[wasm_bindgen]
// // pub fn glyphs_to_path( glyph_ids: &JsValue ) -> Vec<String> {
// //   return String::from( "" ); }

// // #[wasm_bindgen]
// // pub fn glyph_to_svg_pathdata( js_font_idx: &JsValue, js_glyph_id: &JsValue ) -> String {
// //   // ### TAINT try to shorten
// //   let font_idx_u16: u16             = js_font_idx.into_serde().unwrap();
// //   let glyph_id_u16: u16             = js_glyph_id.into_serde().unwrap();
// //   let glyph_id: ttf_parser::GlyphId = ttf_parser::GlyphId( glyph_id_u16 );
// //   //........................................................................................................
// //   // ### TAINT use cache for face_idx, face
// //   // ### TAINT almost identical to `rustybuzz::Face`
// //   let face_idx      = 0;
// //   let face          = ttf_parser::Face::from_slice( get_fontbytes( font_idx_u16 ), face_idx).unwrap();
// //   let units_per_em  = match face.units_per_em() { None => FONT_SIZE as u16, Some( x ) => x, };
// //   let scale         = FONT_SIZE / units_per_em as f64;
// //   let mut path_buf  = svgtypes::Path::with_capacity(64);
// //   let mut builder   = Builder( &mut path_buf );
// //   let bbox          = face.outline_glyph( glyph_id, &mut builder );
// //   for seg in path_buf.iter_mut() { scale_segment( seg, scale ); };
// //   let bbox_svg      = rectangle_from_bbox( match bbox {
// //     None      => ttf_parser::Rect { x_min: 0, y_min: 0, x_max: 0, y_max: 0, },
// //     Some( x ) => x, },
// //     scale );
// //   let left_d_right_no_d_re = Regex::new( r"([0-9])\x20([^0-9])" ).unwrap();
// //   let left_no_d_right_d_re = Regex::new( r"([^0-9])\x20([0-9])" ).unwrap();
// //   let mut path_str  = path_buf.to_string();
// //   path_str          = left_d_right_no_d_re.replace_all( &path_str, "$1$2" ).to_string();
// //   path_str          = left_no_d_right_d_re.replace_all( &path_str, "$1$2" ).to_string();
// //   //........................................................................................................
// //   return json!({
// //     "pd": path_str,
// //     "br": bbox_svg,
// //   }).to_string();
// // }

// // #[wasm_bindgen]
// // pub fn get_font_metrics( js_font_idx: &JsValue ) -> String {
// //   let font_idx_u16: u16             = js_font_idx.into_serde().unwrap();
// //   //........................................................................................................
// //   // ### TAINT use cache for face_idx, face
// //   let face_idx      = 0;
// //   let face          = ttf_parser::Face::from_slice( get_fontbytes( font_idx_u16 ), face_idx ).unwrap();
// //   let units_per_em  = match face.units_per_em() { None => 0 as u16, Some( x ) => x, };
// //   let scale         = FONT_SIZE / units_per_em as f64;
// //   //........................................................................................................
// //   return json!({
// //     // "style": match face.style() { None => 0 as u16, Some( x ) => x, },
// //     "ascender":       ( -face.ascender()  as f64 * scale ).round() as i16,
// //     "descender":      ( -face.descender() as f64 * scale ).round() as i16,
// //     "x_height":       ( -( match face.x_height()       { None => 0 as i16, Some( x ) => x, } ) as f64 * scale ).round() as i16,
// //     "capital_height": ( -( match face.capital_height() { None => 0 as i16, Some( x ) => x, } ) as f64 * scale ).round() as i16,
// //     "units_per_em": units_per_em,
// //     "scale": scale,
// //     "angle":          (    match face.italic_angle()   { None => 0 as f32, Some( x ) => x, } ) as f32 * scale as f32,
// //   }).to_string();
// // }


// //----------------------------------------------------------------------------------------------------------
// fn rectangle_from_bbox( bbox: ttf_parser::Rect, scale: f64, ) -> String {
//   return format!( "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
//     scale_coordinate(  bbox.x_min    as f64, scale ),
//     scale_coordinate(  ( - bbox.y_min - bbox.height() )   as f64, scale ),
//     scale_coordinate(  bbox.width()  as f64, scale ),
//     scale_coordinate(  bbox.height() as f64, scale ), ) }

// //----------------------------------------------------------------------------------------------------------
// struct Builder<'a>(&'a mut svgtypes::Path);
//   /// see https://docs.rs/ttf-parser/0.11.0/ttf_parser/struct.FaceTables.html#method.outline_glyph

// impl ttf_parser::OutlineBuilder for Builder<'_> {
//   fn move_to(&mut self, x: f32, y: f32) {
//     self.0.push_move_to(x as f64, -y as f64); }

//   fn line_to(&mut self, x: f32, y: f32) {
//     self.0.push_line_to(x as f64, -y as f64); }

//   fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
//     self.0.push_quad_to(x1 as f64, -y1 as f64, x as f64, -y as f64); }

//   fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
//     self.0.push_curve_to(x1 as f64, -y1 as f64, x2 as f64, -y2 as f64, x as f64, -y as f64); }

//   fn close(&mut self) {
//     self.0.push_close_path(); }
//   }

// //----------------------------------------------------------------------------------------------------------
// fn scale_coordinate( a: f64, scale: f64 ) -> f64 { ( a  * scale * PRECISION ).round() / PRECISION }

// //----------------------------------------------------------------------------------------------------------
// fn scale_segment(d: &mut PathSegment, scale: f64 ) {
//   match *d {
//     PathSegment::MoveTo { ref mut x, ref mut y, .. } => {
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::LineTo { ref mut x, ref mut y, .. } => {
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::HorizontalLineTo { ref mut x, .. } => {
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::VerticalLineTo { ref mut y, .. } => {
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::CurveTo { ref mut x1, ref mut y1, ref mut x2, ref mut y2, ref mut x, ref mut y, .. } => {
//       *x1 = ( *x1 * scale * PRECISION ).round() / PRECISION;
//       *y1 = ( *y1 * scale * PRECISION ).round() / PRECISION;
//       *x2 = ( *x2 * scale * PRECISION ).round() / PRECISION;
//       *y2 = ( *y2 * scale * PRECISION ).round() / PRECISION;
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::SmoothCurveTo { ref mut x2, ref mut y2, ref mut x, ref mut y, .. } => {
//       *x2 = ( *x2 * scale * PRECISION ).round() / PRECISION;
//       *y2 = ( *y2 * scale * PRECISION ).round() / PRECISION;
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::Quadratic { ref mut x1, ref mut y1, ref mut x, ref mut y, .. } => {
//       *x1 = ( *x1 * scale * PRECISION ).round() / PRECISION;
//       *y1 = ( *y1 * scale * PRECISION ).round() / PRECISION;
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::SmoothQuadratic { ref mut x, ref mut y, .. } => {
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::EllipticalArc { ref mut x, ref mut y, .. } => {
//       *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
//       *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
//     PathSegment::ClosePath { .. } => {} }
//     }



// /*
// ############################################################################################################

// 88888888888                888        888       888                                    d8b
//     888                    888        888   o   888                                    Y8P
//     888                    888        888  d8b  888
//     888   .d88b.  888  888 888888     888 d888b 888 888d888  8888b.  88888b.  88888b.  888 88888b.   .d88b.
//     888  d8P  Y8b `Y8bd8P' 888        888d88888b888 888P"       "88b 888 "88b 888 "88b 888 888 "88b d88P"88b
//     888  88888888   X88K   888        88888P Y88888 888     .d888888 888  888 888  888 888 888  888 888  888
//     888  Y8b.     .d8""8b. Y88b.      8888P   Y8888 888     888  888 888 d88P 888 d88P 888 888  888 Y88b 888
//     888   "Y8888  888  888  "Y888     888P     Y888 888     "Y888888 88888P"  88888P"  888 888  888  "Y88888
//                                                                      888      888                        888
//                                                                      888      888                   Y8b d88P
//                                                                      888      888                    "Y88P"
// TEXT WRAPPING
// ############################################################################################################
// */

// //----------------------------------------------------------------------------------------------------------
// /// A piece of wrappable text, including any trailing whitespace.
// ///
// /// A `Slab` is an example of a [`Fragment`], so it has a width,
// /// trailing whitespace, and potentially a penalty_width item.
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Slab {
//   pub width:            usize,
//   pub whitespace_width: usize,
//   pub penalty_width:    usize, }

// //----------------------------------------------------------------------------------------------------------
// impl std::ops::Deref for Slab {
//   type Target = usize;

//   fn deref(&self) -> &Self::Target { &self.width } }

// // //----------------------------------------------------------------------------------------------------------
// // impl Slab {
// //   pub fn new( width: usize, whitespace_width: usize, penalty_width: usize,  ) -> Slab {
// //     Slab { width, whitespace_width, penalty_width, } } }

// //----------------------------------------------------------------------------------------------------------
// impl textwrap::core::Fragment for Slab {
//   #[inline] fn width(&self) -> usize { self.width }
//   #[inline] fn whitespace_width(&self) -> usize { self.whitespace_width }
//   #[inline] fn penalty_width(&self) -> usize { self.penalty_width } }

// //==========================================================================================================
// #[derive(Serialize, Deserialize)]
// pub struct ArrangementLine {
//   pub first_slab_idx: usize,
//   pub last_slab_idx:  usize,
//   pub line_length:    usize,
//   // pub
// }

// //----------------------------------------------------------------------------------------------------------
// #[derive(Serialize, Deserialize)]
// pub struct Arrangement {
//   // pub line_length:  usize,
//   pub lines: Vec<ArrangementLine>,
//   // pub
// }

// // //----------------------------------------------------------------------------------------------------------
// // impl ArrangementLine {
// //   pub fn new( first_slab_idx: usize, last_slab_idx: usize, line_length: usize, ) -> ArrangementLine {
// //     return ArrangementLine {
// //     last_slab_idx,
// //     first_slab_idx,
// //     line_length, } }
// // }

// //----------------------------------------------------------------------------------------------------------
// impl Arrangement {
//   pub fn new() -> Arrangement { Arrangement {
//     lines: vec![], } }
// }

// // impl Serialize for Arrangement {
// //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// //     where
// //         S: Serializer;
// // }
// //==========================================================================================================
// // ### TAINT use triplets `[m,w,p,]` (material width, whitespace width, penalty width) instead to make JSON significantly smaller ###
// // #[wasm_bindgen]
// // pub fn wrap_text_with_arbitrary_slabs( slabs_js: JsValue, line_width_js: JsValue ) -> String {
// //   let slabs:      Vec<Slab> = slabs_js.into_serde().unwrap();
// //   let line_width: usize     = line_width_js.into_serde().unwrap();
// //   let lines                 = textwrap::core::wrap_optimal_fit( &slabs, |_| line_width );
// //   // urge( &format!( "^827^ slabs: {:#?}", slabs ) );
// //   // help( &format!( "^827^ line_width: {:#?}", line_width ) );
// //   // help( &format!( "^827^ lines: {:#?}", lines ) );
// //   let mut slab_idx      = 0;
// //   let mut r             = Arrangement::new();
// //   for line in lines {
// //     // urge( &format!( "^1113398-2^ {:#?}", slab_idx ) );
// //     // urge( &format!( "^1113398-3^ {:#?}", line.len() ) );
// //     // urge( &format!( "^1113398-4^ {:#?}", slab_idx + line.len() - 1 ) );
// //     let arrangement_line =  ArrangementLine {
// //       first_slab_idx: slab_idx,
// //       last_slab_idx:  slab_idx + line.len() - 1, // ### TAINT may become negative ###
// //       line_length:    line.len(), };
// //     // urge( "^1113398-5^" );
// //     r.lines.push( arrangement_line );
// //     // urge( "^1113398-6^" );
// //     slab_idx += line.len();
// //     // urge( "^1113398-7^" );
// //     }
// //   // urge( "^1113398-8^" );
// //   return json!( r ).to_string(); }



// /*
// ############################################################################################################

// 888      d8b                       888888b.                             888      d8b
// 888      Y8P                       888  "88b                            888      Y8P
// 888                                888  .88P                            888
// 888      888 88888b.   .d88b.      8888888K.  888d888  .d88b.   8888b.  888  888 888 88888b.   .d88b.
// 888      888 888 "88b d8P  Y8b     888  "Y88b 888P"   d8P  Y8b     "88b 888 .88P 888 888 "88b d88P"88b
// 888      888 888  888 88888888     888    888 888     88888888 .d888888 888888K  888 888  888 888  888
// 888      888 888  888 Y8b.         888   d88P 888     Y8b.     888  888 888 "88b 888 888  888 Y88b 888
// 88888888 888 888  888  "Y8888      8888888P"  888      "Y8888  "Y888888 888  888 888 888  888  "Y88888
//                                                                                                    888
//                                                                                               Y8b d88P
//                                                                                                "Y88P"
// LINE BREAKING
// ############################################################################################################
// */

// //----------------------------------------------------------------------------------------------------------
// #[wasm_bindgen]
// pub fn find_line_break_positions( text: String ) -> String {
//   use unicode_linebreak; //::{linebreaks, BreakOpportunity::{Mandatory, Allowed}};
//   let mut r: Vec<usize> = vec![ 0, ];
//   for linebreak in unicode_linebreak::linebreaks( &text ) { r.push( linebreak.0, ); }
//   return json!( r ).to_string(); }


// /*
// ############################################################################################################

// 888b    888  .d8888b.  8888888b.
// 8888b   888 d88P  Y88b 888   Y88b
// 88888b  888 888    888 888    888
// 888Y88b 888 888        888   d88P .d8888b
// 888 Y88b888 888        8888888P"  88K
// 888  Y88888 888    888 888 T88b   "Y8888b.
// 888   Y8888 Y88b  d88P 888  T88b       X88
// 888    Y888  "Y8888P"  888   T88b  88888P'

// NCRs
// ############################################################################################################
// */

// //----------------------------------------------------------------------------------------------------------
// #[wasm_bindgen]
// pub fn decode_ncrs( text: String ) -> String {
//   // decode_html_entities                        Decode html entities in a given string.
//   // decode_script                               Decode text from the <script> element.
//   // decode_script_double_quoted_text            Decode text from a double quoted text in the <script> element.
//   // decode_script_quoted_text                   Decode text from a quoted text in the <script> element.
//   // decode_script_single_quoted_text            Decode text from a single quoted text in the <script> element.
//   // decode_style                                Decode text from the <style> element.
//   // decode_style_double_quoted_text             Decode text from a double quoted text in the <style> element.
//   // decode_style_quoted_text                    Decode text from a quoted text in the <style> element.
//   // decode_style_single_quoted_text             Decode text from a single quoted text in the <style> element.
//   // encode_double_quoted_attribute              Encode text used in a double-quoted attribute.
//   // encode_quoted_attribute                     Encode text used in a quoted attribute.
//   // encode_safe                                 Encode text to prevent special characters functioning.
//   // encode_script                               Encode text used in the <script> element.
//   // encode_script_double_quoted_text            Encode text used in a double quoted text in the <script> element.
//   // encode_script_quoted_text                   Encode text used in a quoted text in the <script> element.
//   // encode_script_single_quoted_text            Encode text used in a single quoted text in the <script> element.
//   // encode_single_quoted_attribute              Encode text used in a single-quoted attribute.
//   // encode_style                                Encode text used in the <style> element.
//   // encode_style_double_quoted_text             Encode text used in a double quoted text in the <style> element.
//   // encode_style_quoted_text                    Encode text used in a quoted text in the <style> element.
//   // encode_style_single_quoted_text             Encode text used in a single quoted text in the <style> element.
//   // encode_text                                 Encode text used as regular HTML text.
//   // encode_text_minimal                         Encode text used as regular HTML text.
//   // encode_unquoted_attribute                   Encode text used in an unquoted attribute. Except for alphanumeric characters, escape all characters which are less than 128.
//   use html_escape;
//   return html_escape::decode_html_entities( &text ).to_string(); }



// /*
// ############################################################################################################

// 888    888                   888                                 888   d8b
// 888    888                   888                                 888   Y8P
// 888    888                   888                                 888
// 8888888888 888  888 88888b.  88888b.   .d88b.  88888b.   8888b.  888888888  .d88b.  88888b.
// 888    888 888  888 888 "88b 888 "88b d8P  Y8b 888 "88b     "88b 888   888 d88""88b 888 "88b
// 888    888 888  888 888  888 888  888 88888888 888  888 .d888888 888   888 888  888 888  888
// 888    888 Y88b 888 888 d88P 888  888 Y8b.     888  888 888  888 Y88b. 888 Y88..88P 888  888
// 888    888  "Y88888 88888P"  888  888  "Y8888  888  888 "Y888888  "Y888888  "Y88P"  888  888
//                 888 888
//            Y8b d88P 888
//             "Y88P"  888

// Hyphenation
// ############################################################################################################
// */

// // Sadly none of the below works although code has been copy-pasted from
// // https://docs.rs/hyphenation/0.8.4/hyphenation/
// // https://lib.rs/crates/hyphenation

// // //----------------------------------------------------------------------------------------------------------
// // #[wasm_bindgen]
// // pub fn hyphenate( text: String ) -> String {
// //   // use hyphenation::*;
// //   // let en_us       = hyphenation::Standard::from_embedded( Language::EnglishUS );
// //   // let hyphenated  = en_us.hyphenate( &text );
// //   // let unmarked    = hyphenated.iter().segments();
// //   // let collected : Vec<&str> = unmarked.collect();

// //   extern crate hyphenation;
// //   use hyphenation::*;
// //   // use hyphenation::{Hyphenator, Standard, Language};
// //   // use hyphenation::Load;

// //   let path_to_dict = "target/debug/build//hyphenation-95b4852037d9f505/out/dictionaries/en-us.standard.bincode";
// //   let en_us = Standard::from_path(Language::EnglishUS, path_to_dict);
// //   let hyphenated  = en_us.hyphenate( &text );
// //   let unmarked    = hyphenated.into_iter().segments();
// //   let collected : Vec<&str> = unmarked.collect();

// //   // // Word breaks are represented as byte indices into the string.
// //   // let break_indices = &hyphenated.breaks;
// //   // assert_eq!(break_indices, &[2, 6, 7]);

// //   // // The segments of a hyphenated word can be iterated over, marked or unmarked.
// //   // let marked = hyphenated.iter();
// //   // let collected : Vec<String> = marked.collect();
// //   // assert_eq!(collected, vec!["hy-", "phen-", "a-", "tion"]);

// //   // let unmarked = hyphenated.iter().segments();
// //   // let collected : Vec<&str> = unmarked.collect();
// //   // assert_eq!(collected, vec!["hy", "phen", "a", "tion"]);

// //   return json!( collected ).to_string(); }

