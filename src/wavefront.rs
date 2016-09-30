use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Error as IOErr, ErrorKind};
use std::path::Path;

pub struct Obj {
    pub vertices: Vec<Vert>,
    pub faces: Vec<Face>,
}
impl Obj {
    pub fn from_file(path: &Path) -> io::Result<Obj> {
        let file = try!(File::open(path));
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for l in reader.lines() {
            let line = try!(l);
            if line.starts_with("v ") {
                let vert = try!(Vert::parse(&line[2..]));
                vertices.push(vert);
            }

            if line.starts_with("f ") {
                let face = try!(Face::parse(&line[2..]));
                faces.push(face);
            }
        }

        let obj = Obj {
            vertices: vertices,
            faces: faces,
        };
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

        let x_str = try!(coords.next()
            .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read x")));
        let x = try!(x_str.parse::<f32>().map_err(|err| {
            IOErr::new(ErrorKind::InvalidData,
                       format!("invalid format for x: {}", err.description()))
        }));

        let y_str = try!(coords.next()
            .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read y")));
        let y = try!(y_str.parse::<f32>().map_err(|err| {
            IOErr::new(ErrorKind::InvalidData,
                       format!("invalid format for y: {}", err.description()))
        }));

        let z_str = try!(coords.next()
            .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read z")));
        let z = try!(z_str.parse::<f32>().map_err(|err| {
            IOErr::new(ErrorKind::InvalidData,
                       format!("invalid format for z: {}", err.description()))
        }));

        let v = Vert { x: x, y: y, z: z };
        Ok(v)
    }
}

pub struct Face {
    pub vertices: (usize, usize, usize),
}
impl Face {
    fn parse(s: &str) -> io::Result<Face> {
        let mut coords = s.split_whitespace();

        let x_str = try!(coords.next()
            .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read x group")));
        let mut x_group = x_str.split('/');
        let x_v = try!(try!(x_group.next()
                .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read x vertex id")))
            .parse::<usize>()
            .map_err(|err| {
                IOErr::new(ErrorKind::InvalidData,
                           format!("invalid format for x vertex id: {}", err.description()))
            }));

        let y_str = try!(coords.next()
            .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read y group")));
        let mut y_group = y_str.split('/');
        let y_v = try!(try!(y_group.next()
                .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read y vertex id")))
            .parse::<usize>()
            .map_err(|err| {
                IOErr::new(ErrorKind::InvalidData,
                           format!("invalid format for y vertex id: {}", err.description()))
            }));

        let z_str = try!(coords.next()
            .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read z group")));
        let mut z_group = z_str.split('/');
        let z_v = try!(try!(z_group.next()
                .ok_or(IOErr::new(ErrorKind::InvalidData, "failed to read z vertex id")))
            .parse::<usize>()
            .map_err(|err| {
                IOErr::new(ErrorKind::InvalidData,
                           format!("invalid format for z vertex id: {}", err.description()))
            }));

        let f = Face { vertices: (x_v - 1, y_v - 1, z_v - 1) };
        Ok(f)
    }
}

pub struct Line(pub Vert, pub Vert);

pub struct Tri(pub Vert, pub Vert, pub Vert);
impl Tri {
    pub fn lines(&self) -> Vec<Line> {
        vec!(
            Line(self.0, self.1),
            Line(self.1, self.2),
            Line(self.2, self.0),
        )
    }
}
