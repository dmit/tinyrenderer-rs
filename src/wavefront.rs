use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error as IOErr, ErrorKind},
    path::Path,
};

pub struct Obj {
    pub vertices: Vec<Vert>,
    pub faces: Vec<Face>,
}
impl Obj {
    pub fn from_file(path: &Path) -> io::Result<Obj> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for l in reader.lines() {
            let line = l?;
            if let Some(vert) = line.strip_prefix("v ") {
                vertices.push(Vert::parse(vert)?);
            }

            if let Some(face) = line.strip_prefix("f ") {
                faces.push(Face::parse(face)?);
            }
        }

        let obj = Obj { vertices, faces };
        Ok(obj)
    }
}

#[derive(Clone, Copy)]
pub struct Vert {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vert {
    fn parse(s: &str) -> io::Result<Vert> {
        let mut coords = s.split_whitespace();

        let x_str =
            coords.next().ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read x"))?;
        let x = x_str.parse::<f32>().map_err(|err| {
            IOErr::new(ErrorKind::InvalidData, format!("invalid format for x: {err}"))
        })?;

        let y_str =
            coords.next().ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read y"))?;
        let y = y_str.parse::<f32>().map_err(|err| {
            IOErr::new(ErrorKind::InvalidData, format!("invalid format for y: {err}"))
        })?;

        let z_str =
            coords.next().ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read z"))?;
        let z = z_str.parse::<f32>().map_err(|err| {
            IOErr::new(ErrorKind::InvalidData, format!("invalid format for z: {err}"))
        })?;

        let v = Vert { x, y, z };
        Ok(v)
    }
}

pub struct Face {
    pub vertices: (usize, usize, usize),
}
impl Face {
    fn parse(s: &str) -> io::Result<Face> {
        let mut coords = s.split_whitespace();

        let x_str = coords
            .next()
            .ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read x group"))?;
        let mut x_group = x_str.split('/');
        let x_v = x_group
            .next()
            .ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read x vertex id"))?
            .parse::<usize>()
            .map_err(|err| {
                IOErr::new(ErrorKind::InvalidData, format!("invalid format for x vertex id: {err}"))
            })?;

        let y_str = coords
            .next()
            .ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read y group"))?;
        let mut y_group = y_str.split('/');
        let y_v = y_group
            .next()
            .ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read y vertex id"))?
            .parse::<usize>()
            .map_err(|err| {
                IOErr::new(ErrorKind::InvalidData, format!("invalid format for y vertex id: {err}"))
            })?;

        let z_str = coords
            .next()
            .ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read z group"))?;
        let mut z_group = z_str.split('/');
        let z_v = z_group
            .next()
            .ok_or_else(|| IOErr::new(ErrorKind::InvalidData, "failed to read z vertex id"))?
            .parse::<usize>()
            .map_err(|err| {
                IOErr::new(ErrorKind::InvalidData, format!("invalid format for z vertex id: {err}"))
            })?;

        let f = Face { vertices: (x_v - 1, y_v - 1, z_v - 1) };
        Ok(f)
    }
}

pub struct Line(pub Vert, pub Vert);

pub struct Tri(pub Vert, pub Vert, pub Vert);
impl Tri {
    pub fn lines(&self) -> Vec<Line> {
        vec![Line(self.0, self.1), Line(self.1, self.2), Line(self.2, self.0)]
    }
}
