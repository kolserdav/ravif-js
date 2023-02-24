use imgref::ImgVec;
use load_image::{
    export::{imgref::ImgVecKind, rgb::ComponentMap},
    load_data,
};
use napi::{bindgen_prelude::*, Error as NapiError, JsBoolean, JsTypedArray};
use napi_derive::napi;
use ravif::{AlphaColorMode, ColorSpace, EncodedImage, Encoder, RGBA8};
use std::{
    convert::AsRef,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Error(NapiError);

impl<'a> AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Debug)]
#[napi(constructor)]
pub struct Test {
    pub test: bool,
}

#[napi]
pub fn say_hello(args: This<&Test>) {
    println!("{:?}", args);
}

#[napi(constructor)]
pub struct EncoderConfig {
    pub quality: f64,
    pub speed: u8,
    pub alpha_quality: f64,
    pub dirty_alpha: bool,
    pub threads: u32,
}

#[napi]
pub fn encode_image(config: This<&EncoderConfig>) -> Result<bool, Error> {
    let EncoderConfig {
        quality,
        speed,
        alpha_quality,
        dirty_alpha,
        threads,
    } = config;
    let enc = Encoder::new()
        .with_quality(*quality as f32)
        .with_speed(*speed)
        .with_alpha_quality(*alpha_quality as f32)
        .with_alpha_color_mode(if *dirty_alpha {
            AlphaColorMode::UnassociatedDirty
        } else {
            AlphaColorMode::UnassociatedClean
        })
        .with_num_threads(Some(*threads as usize).filter(|&n| n > 0));
    let img = load_rgba("./".as_bytes(), false).expect("Err 54");
    let EncodedImage {
        avif_file,
        color_byte_size,
        alpha_byte_size,
        ..
    } = enc.encode_rgba(img.as_ref()).expect("Err 23");
    Ok(false)
}

fn load_rgba(data: &[u8], premultiplied_alpha: bool) -> Result<ImgVec<RGBA8>, Error> {
    let img = load_data(data).unwrap().into_imgvec();
    let mut img = match img {
        ImgVecKind::RGB8(img) => {
            img.map_buf(|buf| buf.into_iter().map(|px| px.alpha(255)).collect())
        }
        ImgVecKind::RGBA8(img) => img,
        ImgVecKind::RGB16(img) => img.map_buf(|buf| {
            buf.into_iter()
                .map(|px| px.map(|c| (c >> 8) as u8).alpha(255))
                .collect()
        }),
        ImgVecKind::RGBA16(img) => img.map_buf(|buf| {
            buf.into_iter()
                .map(|px| px.map(|c| (c >> 8) as u8))
                .collect()
        }),
        ImgVecKind::GRAY8(img) => img.map_buf(|buf| {
            buf.into_iter()
                .map(|g| {
                    let c = g.0;
                    RGBA8::new(c, c, c, 255)
                })
                .collect()
        }),
        ImgVecKind::GRAY16(img) => img.map_buf(|buf| {
            buf.into_iter()
                .map(|g| {
                    let c = (g.0 >> 8) as u8;
                    RGBA8::new(c, c, c, 255)
                })
                .collect()
        }),
        ImgVecKind::GRAYA8(img) => img.map_buf(|buf| {
            buf.into_iter()
                .map(|g| {
                    let c = g.0;
                    RGBA8::new(c, c, c, g.1)
                })
                .collect()
        }),
        ImgVecKind::GRAYA16(img) => img.map_buf(|buf| {
            buf.into_iter()
                .map(|g| {
                    let c = (g.0 >> 8) as u8;
                    RGBA8::new(c, c, c, (g.1 >> 8) as u8)
                })
                .collect()
        }),
    };

    if premultiplied_alpha {
        img.pixels_mut().for_each(|px| {
            px.r = (u16::from(px.r) * u16::from(px.a) / 255) as u8;
            px.g = (u16::from(px.g) * u16::from(px.a) / 255) as u8;
            px.b = (u16::from(px.b) * u16::from(px.a) / 255) as u8;
        });
    }
    Ok(img)
}
