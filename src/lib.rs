pub fn point_to_buffer(p: (usize, usize), cols: usize, rows: usize) -> Option<usize> {
    let (x, y): (usize, usize) = p;
    if x <= 0 || y <= 0 || x >= cols || y >= rows {
        return None;
    }
    Some(y * cols + x)
}

pub fn buffer_to_point(p: usize, cols: usize) -> (usize, usize) {
    (p % cols, p / cols)
}

pub enum Id {
    Empty,
    Sand,
    Water,
}

pub struct World {
    cols: usize,
    rows: usize,
    buffer: Vec<Box<dyn Pixel>>,
}

impl World {
    pub fn new(cols: usize, rows: usize) -> World {
        let mut buffer: Vec<Box<dyn Pixel>> = Vec::new();
        for y in 0..rows {
            for x in 0..cols {
                buffer.push(Box::new(Empty::new((x, y))))
            }
        }

        World { cols, rows, buffer }
    }

    pub fn id_lize(&self) {
        let mut counter = 0;
        for i in &self.buffer {
            match i.id() {
                Id::Empty => print!("empty {:?}", i.get_pos()),
                Id::Sand => print!("sand {:?}", i.get_pos()),
                _ => print!("not emptry or sandy"),
            }
            println!("{:?}", buffer_to_point(counter, self.cols),);
            assert_eq!(
                format!("{:?}", buffer_to_point(counter, self.cols)),
                format!("{:?}", i.get_pos())
            );
            counter += 1;
        }
    }

    fn create_pixel(id: Id, pos: (usize, usize)) -> Box<dyn Pixel> {
        match id {
            Id::Empty => Box::new(Empty::new(pos)),
            Id::Sand => Box::new(Sand::new(pos)),
            Id::Water => Box::new(Sand::new(pos)),
        }
    }

    pub fn change_pixel(&mut self, p: usize, id: Id) {
        self.buffer[p] = Self::create_pixel(id, buffer_to_point(p, self.rows));
    }
}

pub trait Pixel {
    fn update(&self) -> Option<(usize, usize)>;
    fn id(&self) -> Id;
    fn set_pos(self, pos: (usize, usize)) -> (usize, usize);
    fn get_pos(&self) -> (usize, usize);
}

pub struct Empty {
    pos: (usize, usize),
}

impl Empty {
    pub fn new(pos: (usize, usize)) -> Empty {
        Empty { pos }
    }
}

impl Pixel for Empty {
    fn update(&self) -> Option<(usize, usize)> {
        None
    }
    fn id(&self) -> Id {
        Id::Empty
    }
    fn set_pos(mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

pub struct Sand {
    pos: (usize, usize),
}

impl Sand {
    pub fn new(pos: (usize, usize)) -> Sand {
        Sand { pos }
    }
}

impl Pixel for Sand {
    fn update(&self) -> Option<(usize, usize)> {
        Some((12, 12))
    }
    fn id(&self) -> Id {
        Id::Sand
    }
    fn set_pos(mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
