use crate::math::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};

// http://en.wikipedia.org/wiki/Wavefront_.obj_file
pub struct Model {
    pub vertices: Vec<Vec3<f32>>,
    pub normals: Vec<Vec3<f32>>,
    pub faces: Vec<Vec<usize>>,
}

/// Takes a line of the format `v -0.000581696 -0.734665 -0.623267` and returns a Result<Vec3> from the numbers
fn get_vertices(line: &str) -> Result<Vec3<f32>, String> {
    let result: Result<Vec<_>, _> = line
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse())
        .collect();
    match result {
        Ok(arr) => Ok(Vec3::from_slice(arr.as_slice())),
        Err(_) => Err(String::from("Couldn't parse 3 numbers form line")),
    }
}

// Takes a line of the format `f x/x/x y/y/y z/z/z` and returns a Vec<usize> containing the first
// number in each set - 1 (which is each vertex index for that face, adjusting for the 1-based
// indexing of the wavefront .obj format)
// f 1193/1240/1193 1180/1227/1180 1179/1226/1179
// f 开头表示由顶点、uv 纹理坐标、法向量索引确定的表面，如 5/2/1 表示 v 开头的第 5 个顶点、
// 这个点对应 vt 贴图的第 2 个坐标、这个点对应 vn 开头的第 1 个法向量；
fn get_face_indicies(line: &str) -> Result<Vec<usize>, String> {
    line.split_ascii_whitespace()
        .take(3)
        .map(|x| {
            if let Some(x) = x.split('/').next() {
                match x.parse::<usize>() {
                    // Note that in obj files indexes start from 1
                    Ok(x) => Ok(x - 1),
                    Err(_) => Err(String::from("Failed to parse face vertex number")),
                }
            } else {
                Err(String::from("Missing face vertex number"))
            }
        })
        .collect()
}

impl Model {
    pub fn from(filename: &str) -> Result<Self, String> {
        let f = File::open(filename);
        let f = match f {
            Err(_) => return Err(format!("Couldn't open object {filename}.")),
            Ok(f) => f,
        };
        let reader = BufReader::new(f);
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();
        for line in reader.lines().flatten() {
            if let Some(end) = line.strip_prefix("v ") {
                if let Ok(vertex) = get_vertices(end) {
                    vertices.push(vertex);
                }
            } else if let Some(end) = line.strip_prefix("vn ") {
                if let Ok(vertex) = get_vertices(end) {
                    normals.push(vertex);
                }
            } else if let Some(end) = line.strip_prefix("f ") {
                if let Ok(vertex) = get_face_indicies(end) {
                    faces.push(vertex);
                }
            }
        }
        Ok(Model { vertices, normals, faces })
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn vertex(&self, i: usize) -> &Vec3<f32> {
        &self.vertices[i]
    }

    pub fn normal(&self, i: usize) -> &Vec3<f32> {
        &self.normals[i]
    }

    pub fn face(&self, i: usize) -> &Vec<usize> {
        &self.faces[i]
    }
}
