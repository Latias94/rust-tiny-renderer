use crate::math::Vec2;
use crate::tga::{ColorSpace, Image, GREEN, RED, RGBA, WHITE};
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

    /// A good method of drawing a triangle must have the following features:
    /// * It should be (surprise!) simple and fast.
    /// * It should be symmetrical: the picture should not depend on the order of vertices passed to the drawing function.
    /// * If two triangles have two common vertices, there should be no holes between them because of rasterization rounding.
    /// * We could add more requirements, but let’s do with these ones. Traditionally a line sweeping is used:
    /// 1. Sort vertices of the triangle by their y-coordinates;
    /// 2. Rasterize simultaneously the left and the right sides of the triangle;
    /// 3. Draw a horizontal line segment between the left and the right boundary points.
    /// 根据三个顶点的 y 坐标判定是否有两个相等，有则判断是平底还是平顶三角形，直接画找到 y 值在中间的点，划分出上下两个三角形，画两个
    /// 另一种画三角形是遍历 bounding box 的点，判断点在不在三角形内。
    /// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    pub fn triangle(&mut self, t0: Vec2<isize>, t1: Vec2<isize>, t2: Vec2<isize>, color: T) {
        let mut t0 = t0;
        let mut t1 = t1;
        let mut t2 = t2;
        if t0.y > t1.y {
            swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            swap(&mut t1, &mut t2);
        }

        let total_height = t2.y - t0.y;

        for i in 0..total_height {
            let second_half = i > t1.y - t0.y || t1.y == t0.y;
            let segment_height = if second_half {
                t2.y - t1.y
            } else {
                t1.y - t0.y
            };
            let alpha: f64 = i as f64 / total_height as f64;
            let minus_height = if second_half { t1.y - t0.y } else { 0 };
            let beta: f64 = (i - minus_height) as f64 / segment_height as f64; // be careful: with above conditions no division by zero here

            // 以t1.y水平为分界线，划分两个三角形后，算出两个边当前y值的点，分别为 a, b
            let mut a = t0 + (t2 - t0) * alpha;
            let mut b = if second_half {
                t1 + (t2 - t1) * beta
            } else {
                t0 + (t1 - t0) * beta
            };
            // 再排序一下 防止 gap
            if a.x > b.x {
                swap(&mut a, &mut b);
            }
            // 因为外面先从 y 遍历，因此横向 x 可能会相差大于 1 个像素，因此要横向填充好
            for x in a.x..=b.x {
                self.image.set(x as usize, (t0.y + i) as usize, color).ok(); // attention, due to int casts t0.y+i != a.y
            }
        }
    }
}

impl Rasterizer<RGBA> {
    pub fn triangle_test_1(&mut self, t0: Vec2<isize>, t1: Vec2<isize>, t2: Vec2<isize>) {
        let mut t0 = t0;
        let mut t1 = t1;
        let mut t2 = t2;
        if t0.y > t1.y {
            swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            swap(&mut t1, &mut t2);
        }
        self.line(t0, t1, GREEN);
        self.line(t1, t2, GREEN);
        self.line(t2, t0, RED);
    }

    pub fn triangle_test_2(&mut self, t0: Vec2<isize>, t1: Vec2<isize>, t2: Vec2<isize>) {
        let mut t0 = t0;
        let mut t1 = t1;
        let mut t2 = t2;
        if t0.y > t1.y {
            swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            swap(&mut t1, &mut t2);
        }

        let total_height = t2.y - t0.y;
        let segment_height = t1.y - t0.y + 1;

        // 下半个三角形
        for y in t0.y..=t1.y {
            let alpha = (y - t0.y) as f64 / total_height as f64;
            let beta = (y - t0.y) as f64 / segment_height as f64; // be careful with divisions by zero
            let alpha_t = (t2 - t0) * alpha;
            let beta_t = (t1 - t0) * beta;
            // 以t1.y水平为分界线，划分两个三角形后，算出两个边当前y值的点，分别为 p0, p1
            let p0 = t0 + alpha_t;
            let p1 = t0 + beta_t;
            self.image.set(p0.x as usize, y as usize, RED).ok();
            self.image.set(p1.x as usize, y as usize, GREEN).ok();
        }
    }

    pub fn triangle_test_3(&mut self, t0: Vec2<isize>, t1: Vec2<isize>, t2: Vec2<isize>) {
        let mut t0 = t0;
        let mut t1 = t1;
        let mut t2 = t2;
        if t0.y > t1.y {
            swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            swap(&mut t1, &mut t2);
        }

        let total_height = (t2.y - t0.y) as f64;
        let segment_height_t1_t0 = (t1.y - t0.y + 1) as f64;
        let segment_height_t2_t1 = (t2.y - t1.y + 1) as f64;

        // 下半个三角形
        for y in t0.y..=t1.y {
            let alpha = (y - t0.y) as f64 / total_height;
            let beta = (y - t0.y) as f64 / segment_height_t1_t0; // be careful with divisions by zero

            // 以t1.y水平为分界线，划分两个三角形后，算出两个边当前y值的点，分别为 a, b
            let mut a = t0 + (t2 - t0) * alpha;
            let mut b = t0 + (t1 - t0) * beta;
            // 再排序一下 防止 gap
            if a.x > b.x {
                swap(&mut a, &mut b);
            }
            // 因为外面先从 y 遍历，因此横向 x 可能会相差大于 1 个像素，因此要横向填充好
            for x in a.x..=b.x {
                self.image.set(x as usize, y as usize, RED).ok(); // attention, due to int casts t0.y+i != a.y
            }
        }
        // 上半个三角形
        for y in t1.y..=t2.y {
            let alpha = (y - t0.y) as f64 / total_height;
            let beta = (y - t1.y) as f64 / segment_height_t2_t1; // be careful with divisions by zero
            let mut a = t0 + (t2 - t0) * alpha;
            let mut b = t1 + (t2 - t1) * beta;
            // 再排序一下 防止 gap
            if a.x > b.x {
                swap(&mut a, &mut b);
            }
            for x in a.x..=b.x {
                self.image.set(x as usize, y as usize, WHITE).ok(); // attention, due to int casts t0.y+i != a.y
            }
        }
    }
}
