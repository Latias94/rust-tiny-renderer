use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::path::Path;
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
        if x >= self.width {
            return Err(format!(
                "Coordinates out of bounds for image x >= width: {x} >= {}",
                self.width
            ));
        } else if y >= self.height {
            return Err(format!(
                "Coordinates out of bounds for image y >= height: {y} >= {}",
                self.height
            ));
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

    /// rle: run-length encoding 游程编码
    /// https://zh.wikipedia.org/wiki/%E6%B8%B8%E7%A8%8B%E7%BC%96%E7%A0%81
    /// 举例来说，一组资料串 "AAAABBBCCDEEEE"，由 4 个 A、3 个 B、2 个 C、1 个 D、4 个 E 组成，
    /// 经过变动长度编码法可将资料压缩为 4A3B2C1D4E（由 14 个单位转成 10 个单位）。
    /// 简言之，其优点在于将重复性高的资料量压缩成小单位；然而，其缺点在于——若该资料出现频率不高，可能导致压缩结果资料量比原始资料大，
    /// 例如：原始资料 "ABCDE"，压缩结果为 "1A1B1C1D1E"（由 5 个单位转成 10 个单位）。
    fn write_rle_data(&self, out: &mut dyn Write) -> io::Result<()> {
        const MAX_CHUNK_LENGTH: u8 = 128;
        let data = unsafe { slice_to_u8_slice(&self.data[..]) };
        let n_pixels = self.width * self.height;
        let mut current_pixel = 0;
        while current_pixel < n_pixels {
            let chunk_start = current_pixel * T::BYTE_PER_PIXEL as usize;
            let mut current_byte = chunk_start;
            let mut run_length: u8 = 1;
            let mut raw = true;
            while current_pixel + (run_length as usize) < n_pixels && run_length < MAX_CHUNK_LENGTH
            {
                let next_pixel = current_byte + (T::BYTE_PER_PIXEL as usize);
                let succ_eq = data[current_byte..next_pixel]
                    == data[next_pixel..next_pixel + (T::BYTE_PER_PIXEL as usize)];
                current_byte += T::BYTE_PER_PIXEL as usize;
                if run_length == 1 {
                    raw = !succ_eq;
                }
                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }
                if !raw && !succ_eq {
                    break;
                }
                run_length += 1;
            }
            current_pixel += run_length as usize;
            out.write_all(&[if raw {
                run_length - 1
            } else {
                run_length + 127
            }])?;
            out.write_all(
                &data[chunk_start
                    ..chunk_start
                        + (if raw {
                            run_length * T::BYTE_PER_PIXEL
                        } else {
                            T::BYTE_PER_PIXEL
                        }) as usize],
            )?;
        }
        Ok(())
    }

    /// rle: run-length encoding
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, vflip: bool, rle: bool) -> io::Result<()> {
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

        let mut f = File::create(path)?;
        unsafe {
            f.write_all(struct_to_u8_slice(&h))?;
            if !rle {
                println!("writing non run-length encoding");
                f.write_all(slice_to_u8_slice(&self.data[..]))
                    .expect("Error dumping data to TGA file.");
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

pub const GREEN: RGBA = RGBA {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

pub const BLUE: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
