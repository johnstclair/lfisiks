pub fn point_to_buffer(x: f32, y: f32, rows: usize, cols: usize) -> Option<usize> {
    if x <= 0.0 || y <= 0.0 || x >= rows as f32 || y >= cols as f32 {
        return None;
    }
    let (x, y): (usize, usize) = (x.floor() as usize, y.floor() as usize);
    Some(y * rows + x)
}

pub fn buffer_to_point(p: usize, rows: usize, cols: usize) -> (usize, usize) {
    (p % cols, p / rows)
}

pub enum Id {
    Empty,
    Sand,
    Water,
}

pub struct World {
    rows: usize,
    cols: usize,
    buffer: Vec<Box<dyn Pixel>>,
}

impl World {
    pub fn new(rows: usize, cols: usize) -> World {
        let mut buffer: Vec<Box<dyn Pixel>> = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                buffer.push(Box::new(Empty::new((r, c))))
            }
        }

        World { rows, cols, buffer }
    }

    pub fn id_lize(&self) {
        for i in &self.buffer {
            match i.id() {
                Id::Empty => println!("empty {:?}", i.get_pos()),
                _ => println!("not emptry"),
            }
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
        self.buffer[p] = Self::create_pixel(id, buffer_to_point(p, self.rows, self.cols));
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
