use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::slice;

pub trait ColorSpace {
    fn new() -> Self;
    const BYTE_PER_PIXEL: u8;
}

#[derive(Copy, Clone)]
pub struct Grayscale {
    pub i: u8,
}

#[derive(Copy, Clone)]
pub struct RGB {
    // BGR
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

#[derive(Copy, Clone)]
pub struct RGBA {
    // BGRA
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

pub const WHITE: RGBA = RGBA {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const BLACK: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub const RED: RGBA = RGBA {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

impl ColorSpace for Grayscale {
    fn new() -> Self {
        Grayscale { i: 0 }
    }

    const BYTE_PER_PIXEL: u8 = 1;
}

impl ColorSpace for RGB {
    fn new() -> Self {
        RGB { r: 0, g: 0, b: 0 }
    }
    const BYTE_PER_PIXEL: u8 = 3;
}

impl ColorSpace for RGBA {
    fn new() -> Self {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    const BYTE_PER_PIXEL: u8 = 4;
}

pub struct Image<T: ColorSpace> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

const DEVELOPER_AREA_REF: [u8; 4] = [0, 0, 0, 0];
const EXTENSION_AREA_REF: [u8; 4] = [0, 0, 0, 0];
const FOOTER: &[u8; 18] = b"TRUEVISION-XFILE.\0";

unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    let data_ptr: *const u8 = s as *const T as *const u8;
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = &s[0] as *const T as *const u8;
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}

#[repr(C, packed)]
#[derive(Default)]
struct Header {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    c_map_start: u16,
    c_map_length: u16,
    c_map_depth: u8,
    x_offset: u16,
    y_offset: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,
    image_descriptor: u8,
}

impl<T: ColorSpace + Copy> Image<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![T::new(); (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: T) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(String::from("Coordinates out of bounds for image"));
        }
        self.set_unchecked(x, y, c);
        Ok(())
    }

    pub fn set_unchecked(&mut self, x: usize, y: usize, c: T) {
        self.data[x + y * self.width] = c;
    }

    pub fn clear(&mut self, c: T) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_unchecked(x, y, c);
            }
        }
    }

    /// rle: run-length encoding
    #[allow(dead_code)]
    fn write_rle_data(&self, _out: &mut dyn Write) -> io::Result<()> {
        todo!()
    }

    /// rle: run-length encoding
    pub fn write_to_file(&self, filename: &str, vflip: bool, rle: bool) -> io::Result<()> {
        let h = Header {
            width: self.width as u16,
            height: self.height as u16,
            pixel_depth: T::BYTE_PER_PIXEL << 3, // 8 bits per byte
            image_type: if T::BYTE_PER_PIXEL == Grayscale::BYTE_PER_PIXEL {
                match rle {
                    true => 11, // Compressed, black and white images.
                    false => 3, // Uncompressed, black and white images.
                }
            } else {
                match rle {
                    true => 10, // Runlength encoded RGB images.
                    false => 2, // Uncompressed, RGB images.
                }
            },
            image_descriptor: if vflip { 0x00 } else { 0x20 },
            ..Header::default()
        };

        let mut f = File::create(filename)?;
        unsafe {
            f.write_all(struct_to_u8_slice(&h))?;
            if !rle {
                println!("writing non run-length encoding");
                // f.write_all(self.data_vec().as_slice())
                //     .expect("Error dumping data to TGA file.");
                f.write_all(slice_to_u8_slice(&self.data[..]))?
            } else {
                println!("writing run-length encoding");
                self.write_rle_data(&mut f)
                    .expect("Error dumping RLE data to TGA file");
            }
            f.write_all(&DEVELOPER_AREA_REF)
                .expect("Error writing developer area ref to TGA file");
            f.write_all(&EXTENSION_AREA_REF)
                .expect("Error writing extension area ref to TGA file");
            f.write_all(FOOTER)
                .expect("Error writing footer to TGA file");
        }
        Ok(())
    }
}
