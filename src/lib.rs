pub fn point_to_buffer(p: (usize, usize), cols: usize, rows: usize) -> Option<usize> {
    let (x, y): (usize, usize) = p;
    if x >= cols || y >= rows {
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
    Stone,
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
                buffer.push(Box::new(Empty::new((x, y))));
            }
        }

        World { cols, rows, buffer }
    }

    pub fn update(&mut self) {
        for i in self.buffer.iter().rev().filter(|i| match i.id() {
            Id::Sand => true,
            _ => false,
        }) {
            if let Some(p) = i.update(&self) {
                if let Some(b1) = point_to_buffer(p, self.cols, self.rows) {
                    if let Some(b2) = point_to_buffer(i.get_pos(), self.cols, self.rows) {
                        self.buffer.swap(b1, b2);
                    }
                }
            }
        }
    }

    pub fn buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|item| item.get_color()).collect()
    }

    fn create_pixel(id: Id, pos: (usize, usize)) -> Box<dyn Pixel> {
        match id {
            Id::Empty => Box::new(Empty::new(pos)),
            Id::Sand => Box::new(Sand::new(pos)),
            Id::Stone => Box::new(Stone::new(pos)),
        }
    }

    pub fn change_pixel(&mut self, p: usize, id: Id) {
        self.buffer[p] = Self::create_pixel(id, buffer_to_point(p, self.cols));
    }

    pub fn id_lize(&self) {
        let mut counter = 1;
        let mut test = 0;
        println!("--------");
        for i in &self.buffer {
            if counter == i.get_pos().1 {
                counter += 1;
                println!("");
            }
            match i.id() {
                Id::Empty => print!("E"),
                Id::Sand => print!("S"),
                Id::Stone => print!("T"),
            }
        }
        println!("");
    }
}

pub trait Pixel {
    fn update(&self, world: &World) -> Option<(usize, usize)>;
    fn id(&self) -> Id;
    fn set_pos(self, pos: (usize, usize)) -> (usize, usize);
    fn get_pos(&self) -> (usize, usize);
    fn get_color(&self) -> u32;
}

pub struct Empty {
    pos: (usize, usize),
    color: u32,
}

impl Empty {
    pub fn new(pos: (usize, usize)) -> Empty {
        Empty {
            pos,
            color: 0x00000000,
        }
    }
}

impl Pixel for Empty {
    fn update(&self, world: &World) -> Option<(usize, usize)> {
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
    fn get_color(&self) -> u32 {
        self.color
    }
}

pub struct Sand {
    pos: (usize, usize),
    color: u32,
}

impl Sand {
    pub fn new(pos: (usize, usize)) -> Sand {
        Sand {
            pos,
            color: 0x00ffc433,
        }
    }
}

impl Pixel for Sand {
    fn update(&self, world: &World) -> Option<(usize, usize)> {
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
    fn get_color(&self) -> u32 {
        self.color
    }
}

pub struct Stone {
    pos: (usize, usize),
    color: u32,
}

impl Stone {
    pub fn new(pos: (usize, usize)) -> Stone {
        Stone {
            pos,
            color: 0x00888888,
        }
    }
}

impl Pixel for Stone {
    fn update(&self, world: &World) -> Option<(usize, usize)> {
        Some((12, 12))
    }
    fn id(&self) -> Id {
        Id::Stone
    }
    fn set_pos(mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
    fn get_color(&self) -> u32 {
        self.color
    }
}
