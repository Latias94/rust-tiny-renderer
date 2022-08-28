use tinyrenderer::math::{Vec2, Vec3};
use tinyrenderer::model::Model;
use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::{RGB, RGBA};

#[test]
fn test_wire_render() {
    let width = 800;
    let height = 800;
    let mut rasterizer = Rasterizer::new(width, height);

    let model = Model::from("model/african_head.obj").unwrap();
    const WHITE: RGB = RGB {
        r: 255,
        g: 255,
        b: 255,
    };
    for i in 0..model.num_faces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vertex(face[j]);
            let v1 = model.vertex(face[(j + 1) % 3]); // 三角形内的下一个顶点

            // 只要 xy 不要 z
            let x0 = ((v0.x + 1.) * (width as f32) / 2.) as isize;
            let y0 = ((v0.y + 1.) * (height as f32) / 2.) as isize;
            let x1 = ((v1.x + 1.) * (width as f32) / 2.) as isize;
            let y1 = ((v1.y + 1.) * (height as f32) / 2.) as isize;
            rasterizer.line(Vec2::new(x0, y0), Vec2::new(x1, y1), WHITE)
        }
    }

    rasterizer.write_to_file("test.tga");
}

#[test]
fn test_flat_shading_random_color_render() {
    let width = 800;
    let height = 800;
    let mut rasterizer = Rasterizer::new(width, height);

    let model = Model::from("model/african_head.obj").unwrap();
    for i in 0..model.num_faces() {
        let face = model.face(i);
        let mut screen_coords = vec![Vec2::default(); 3];
        for j in 0..3 {
            let world_corrds = model.vertex(face[j]);
            let x = ((world_corrds.x + 1.) * (width as f32) / 2.) as isize;
            let y = ((world_corrds.y + 1.) * (height as f32) / 2.) as isize;
            screen_coords[j] = Vec2::new(x, y);
        }
        rasterizer.triangle(
            screen_coords[0],
            screen_coords[1],
            screen_coords[2],
            RGB::random_color(),
        )
    }

    rasterizer.write_to_file("test.tga");
}

#[test]
fn test_calculate_flat_shading_render() {
    let width = 800;
    let height = 800;
    let mut rasterizer = Rasterizer::new(width, height);
    let light_dir = Vec3::new(0f32, 0f32, -1f32);

    let model = Model::from("model/african_head.obj").unwrap();
    for i in 0..model.num_faces() {
        let face = model.face(i);
        let mut screen_coords = vec![Vec2::default(); 3];
        let mut world_corrds = vec![Vec3::default(); 3];
        for j in 0..3 {
            let vertex = *model.vertex(face[j]);
            let x = ((vertex.x + 1.) * (width as f32) / 2.) as isize;
            let y = ((vertex.y + 1.) * (height as f32) / 2.) as isize;
            screen_coords[j] = Vec2::new(x, y);
            world_corrds[j] = vertex;
        }
        let vector0 = world_corrds[2] - world_corrds[0];
        let vector1 = world_corrds[1] - world_corrds[0];
        let mut normal = vector0.cross_product(vector1);
        normal = normal.normalize();
        let intensity = normal * light_dir;
        // 点乘结果为负，说明光在三角形面的背面，因此剔除不渲染这些三角形，这就是 Back-face culling
        if intensity > 0_f32 {
            let gray_scale = (intensity * 255_f32) as u8;
            rasterizer.triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                RGBA {
                    r: gray_scale,
                    g: gray_scale,
                    b: gray_scale,
                    a: 255,
                },
            )
        }
    }

    rasterizer.write_to_file("test.tga");
}

#[test]
fn test_flat_shading_render() {
    let width = 800;
    let height = 800;
    let mut rasterizer = Rasterizer::new(width, height);
    let light_dir = Vec3::new(0f32, 0f32, -1f32);

    let model = Model::from("model/african_head.obj").unwrap();
    for i in 0..model.num_faces() {
        let face = model.face(i);
        let mut screen_coords = vec![Vec2::default(); 3];
        let mut world_corrds = vec![Vec3::default(); 3];
        let mut world_normals = vec![Vec3::default(); 3];
        for j in 0..3 {
            let vertex = *model.vertex(face[j]);
            let normal = *model.normal(face[j]);
            let x = ((vertex.x + 1.) * (width as f32) / 2.) as isize;
            let y = ((vertex.y + 1.) * (height as f32) / 2.) as isize;
            screen_coords[j] = Vec2::new(x, y);
            world_corrds[j] = vertex;
            world_normals[j] = normal;
        }
        let mut normal = (world_normals[0] + world_normals[1] + world_normals[2]) / 3f32;
        // use model's normal
        normal = normal.normalize();
        let intensity = normal * light_dir;
        // 如果像之前那样判断 > 0 好像看到的是后脑勺，这里和下面 gray_scale 都翻转一下，可能我哪里写错了
        if intensity < 0_f32 {
            let gray_scale = -(intensity * 255_f32) as u8;
            rasterizer.triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                RGBA {
                    r: gray_scale,
                    g: gray_scale,
                    b: gray_scale,
                    a: 255,
                },
            )
        }
    }

    rasterizer.write_to_file("test.tga");
}
