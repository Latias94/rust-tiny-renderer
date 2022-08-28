use crate::math::Vec2;
use crate::tga::{ColorSpace, Image};
use std::mem::swap;
use std::path::Path;

pub struct Rasterizer<T: ColorSpace + Copy> {
    pub image: Image<T>,
}

impl<T: ColorSpace + Copy> Rasterizer<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image: Image::new(width, height),
        }
    }

    pub fn clear(&mut self, color: T) {
        self.image.clear(color);
    }

    pub fn write_to_file<P: AsRef<Path>>(&mut self, path: P) {
        self.image.write_to_file(path, true, true).unwrap();
    }

    /// Both the multiplication/division and the use of floating-point numbers can be avoided
    /// by using a specific version of Bresenham’s algorithm.
    /// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn line(&mut self, p0: Vec2<isize>, p1: Vec2<isize>, color: T) {
        let mut x0 = p0.x;
        let mut y0 = p0.y;
        let mut x1 = p1.x;
        let mut y1 = p1.y;
        // Is it steeper than 45°? If so, we transpose the line
        // (https://en.wikipedia.org/wiki/Transpose). This
        // essentially guarantees we are drawing a line less
        // steep than 45°.
        let steep = (x0 - x1).abs() < (y0 - y1).abs();
        if steep {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }
        // If our line is running right-to-left, flip the points
        // so we start on the left.
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let dx_mul_2 = dx * 2;
        // The error variable gives us the distance to the best straight line from
        // our current (x, y) pixel. Each time error is greater than one pixel,
        // we increase (or decrease) y by one, and decrease the error by one as well.
        // 斜率 slope=dy/dx，原本计算 1*slope<0.5 => 2*slope<1 => 2*dy<dx => 2*dy-dx<0
        // 由于上面的置换，斜率 0<slope<1 相邻横坐标对于的纵坐标最多加 1
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut x = x0;
        let mut y = y0;

        while x <= x1 {
            if steep {
                // Remember the transpose? This is where we undo it,
                // by swapping our y and x coordinates again
                // self.image.set_unchecked(y as usize, x as usize, color);
                self.image.set(y as usize, x as usize, color).ok();
            } else {
                // self.image.set_unchecked(x as usize, y as usize, color);
                self.image.set(x as usize, y as usize, color).ok();
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx_mul_2;
            }
            x += 1;
        }
    }

    pub fn triangle(&mut self, t0: Vec2<isize>, t1: Vec2<isize>, t2: Vec2<isize>, color: T) {
        self.line(t0, t1, color);
        self.line(t1, t2, color);
        self.line(t2, t0, color);
    }
}
