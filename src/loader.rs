use std::convert;
use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::{ self, BufRead };
use std::num;
use std::path::Path;

use crate::vec::{ Point3 };

fn read_lines(filepath: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub enum LoaderError {
    Vert(String),
    Face(String),
    Io(io::Error),
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError)
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            LoaderError::Vert(msg) =>
                write!(f, "Could not read vertex from file: {}", msg),
            LoaderError::Face(msg) =>
                write!(f, "Could not read face from file: {}", msg),
            LoaderError::Io(err) =>
                write!(f, "Could not load file: {}", err),
            LoaderError::ParseFloat(err) =>
                write!(f, "Could not parse float in file: {}", err),
            LoaderError::ParseInt(err) =>
                write!(f, "Could not parse int in file: {}", err)
        }
    }
}

impl Error for LoaderError {}

impl convert::From<io::Error> for LoaderError {
    fn from(err: io::Error) -> Self {
        LoaderError::Io(err)
    }
}

impl convert::From<num::ParseFloatError> for LoaderError {
    fn from(err: num::ParseFloatError) -> Self {
        LoaderError::ParseFloat(err)
    }
}

impl convert::From<num::ParseIntError> for LoaderError {
    fn from(err: num::ParseIntError) -> Self {
        LoaderError::ParseInt(err)
    }
}

pub enum Polygon {
    Tri
}

pub trait Model {}

pub struct Obj {
    pub vertices: Vec<Point3>,
    pub indices: Vec<(Polygon, Vec<usize>)>
}

impl Model for Obj {}

pub struct Loader;

impl Loader {
    pub fn load_obj(filepath: &Path) -> Result<Obj, LoaderError> {
        let lines = read_lines(filepath)?;

        let mut vertices: Vec<Point3> = vec![];
        let mut indices: Vec<(Polygon, Vec<usize>)> = vec![];

        for line in lines {
            let line = line?;
            let mut data: Vec<_> = line.split(" ").collect();
            data.retain(|s| !s.is_empty());
            if data.len() <= 1 { continue; }

            match data[0] {
                "v" => {
                    if data.len() < 4 {
                        return Err(
                            LoaderError::Vert(format!("Received fewer than three coordinates"))
                        );
                    }
                    let v_x: f64 = data[1].parse()?;
                    let v_y: f64 = data[2].parse()?;
                    let v_z: f64 = data[3].parse()?;
                    vertices.push(Point3::new(v_x, v_y, v_z));
                },
                "f" => {
                    if data.len() < 4 {
                        return Err(LoaderError::Face(format!("Received fewer than three indices")));
                    }
                    let indices_strs: Vec<_> = data[1..].iter().collect();
                    match indices_strs.len() {
                        3 => {
                            let i_1: i64 = indices_strs[0].parse()?;
                            let i_2: i64 = indices_strs[1].parse()?;
                            let i_3: i64 = indices_strs[2].parse()?;

                            let u_1: usize =
                                if i_1 < 0 { vertices.len() + 1 - ((-i_1) as usize) }
                                else { i_1 as usize - 1 };
                            let u_2: usize =
                                if i_2 < 0 { vertices.len() + 1 - ((-i_2) as usize) }
                                else { i_2 as usize - 1 };
                            let u_3: usize =
                                if i_3 < 0 { vertices.len() + 1 - ((-i_3) as usize) }
                                else { i_3 as usize - 1 };

                            if u_1 >= vertices.len() || u_2 >= vertices.len() || u_3 >= vertices.len() {
                                return Err(
                                    LoaderError::Face(format!(
                                        "One of {}, {}, {} refers to vertex outside of current range",
                                        u_1, u_2, u_3
                                    )
                                ));
                            }

                            let face_indices = vec![u_1, u_3, u_2];
                            indices.push((Polygon::Tri, face_indices));
                        },
                        4 => {
                            todo!()
                        },
                        _ => return Err(LoaderError::Face(format!("Too many indices")))
                    }
                },
                _ => continue
            }
        }

        Ok(Obj { vertices, indices })
    }
}
