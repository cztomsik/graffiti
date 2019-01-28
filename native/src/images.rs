use webrender::api::{IdNamespace, ImageData, ImageDescriptor, ImageFormat, ImageKey};

pub type JsImageId = u32;

#[derive(Deserialize, Debug)]
pub struct ImgJsPayload {
    // used only to represent something from js
    pub size: u32,
    pub id: JsImageId,
}

pub type MyImageData = (ImageDescriptor, ImageData);

// this is just a mock image
pub fn make_checkerboard(width: u32, height: u32) -> MyImageData {
    let mut image_data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let lum = 255 * (((x & 8) == 0) ^ ((y & 8) == 0)) as u8;
            image_data.extend_from_slice(&[lum, lum, lum, 0xff]);
        }
    }
    (
        ImageDescriptor::new(width as i32, height as i32, ImageFormat::BGRA8, true, false),
        ImageData::new(image_data),
    )
}

pub fn convert_to_image_key(id: JsImageId) -> ImageKey {
    ImageKey(IdNamespace(0), id)
}
