use imgref::ImgVec;
use load_image::{export::imgref::ImgVecKind, load_data, Error as LoadImageError};
use napi::{bindgen_prelude::*, Error as NapiError, JsTypedArray};
use napi_derive::napi;
use ravif::{AlphaColorMode, ColorSpace, EncodedImage, Encoder, RGBA8};
use std::{
    convert::AsRef,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct Error(NapiError);

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        format!("{:?}", self).as_str()
    }
}

#[napi]
pub fn say_hello() {
    println!("Hello, world!");
}

#[napi(js_name = "Client")]
pub struct EncoderConfig {
    quality: f32,
    speed: u8,
    alpha_quality: f32,
    dirty_alpha: bool,
    threads: usize,
}

#[napi(js_name = "Keys")]
pub struct JsKeys {
    keys: Keys,
}

#[napi]
impl EncoderConfig {
    #[napi(constructor)]
    pub fn new(keys: &JsKeys) -> Self {
        Self {
            quality: Client::new(keys.deref()),
            speed: Client::new(keys.deref()),
            alpha_quality: Client::new(keys.deref()),
            dirty_alpha: Client::new(keys.deref()),
            threads: Client::new(keys.deref()),
        }
    }
}

#[napi]
pub fn encode_image(config: EncoderConfig) -> Result<bool, Error> {
    let EncoderConfig {
        quality,
        speed,
        alpha_quality,
        dirty_alpha,
        threads,
    } = config;
    let enc = Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .with_alpha_quality(alpha_quality)
        .with_alpha_color_mode(if dirty_alpha {
            AlphaColorMode::UnassociatedDirty
        } else {
            AlphaColorMode::UnassociatedClean
        })
        .with_num_threads(Some(threads).filter(|&n| n > 0));
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
