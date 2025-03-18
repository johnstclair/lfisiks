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
    Water,
}

impl Id {
    pub fn density(id: Id) -> u32 {
        match id {
            Id::Water => 2,
            Id::Empty => 0,
            Id::Sand => 10,
            Id::Stone => 20,
        }
    }
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
        println!("---------------------");
        for i in self.buffer.iter_mut() {
            i.set_updated(false);
        }

        for i in (0..self.buffer.len()).rev() {
            match self.buffer[i].id() {
                Id::Sand => (),
                Id::Water => (),
                _ => continue,
            }
            if self.buffer[i].get_updated() {
                continue;
            }
            if let Some(p) = &self.buffer[i].update(&*self) {
                if let Some(b1) = point_to_buffer(*p, self.cols, self.rows) {
                    self.buffer[b1].set_pos(buffer_to_point(i, self.cols));
                    self.buffer[i].set_pos(*p);
                    self.buffer[i].set_updated(true);
                    self.buffer.swap(b1, i);
                }
            }
        }
    }

    pub fn buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|item| item.get_color()).collect()
    }

    pub fn get_id_of(&self, p: (usize, usize)) -> Option<Id> {
        match point_to_buffer(p, self.cols, self.rows) {
            Some(i) => Some(self.buffer[i].id()),
            None => None,
        }
    }

    fn create_pixel(id: Id, pos: (usize, usize)) -> Box<dyn Pixel> {
        match id {
            Id::Empty => Box::new(Empty::new(pos)),
            Id::Sand => Box::new(Sand::new(pos)),
            Id::Stone => Box::new(Stone::new(pos)),
            Id::Water => Box::new(Water::new(pos)),
        }
    }

    pub fn change_pixel(&mut self, p: usize, id: Id) {
        self.buffer[p] = Self::create_pixel(id, buffer_to_point(p, self.cols));
    }

    pub fn id_lize(&self) {
        let mut counter = 1;
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
                Id::Water => print!("W"),
            }
        }
        println!("");
    }
}

pub trait Pixel {
    fn update(&self, world: &World) -> Option<(usize, usize)>;
    fn id(&self) -> Id;
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize);
    fn get_pos(&self) -> (usize, usize);
    fn set_updated(&mut self, update: bool);
    fn get_updated(&self) -> bool;
    fn get_color(&self) -> u32;
}

pub struct Empty {
    pos: (usize, usize),
    color: u32,
    updated: bool,
}

impl Empty {
    pub fn new(pos: (usize, usize)) -> Empty {
        Empty {
            pos,
            color: 0x00000000,
            updated: false,
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
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
    fn set_updated(&mut self, update: bool) {
        self.updated = update;
    }
    fn get_updated(&self) -> bool {
        self.updated
    }
    fn get_color(&self) -> u32 {
        self.color
    }
}

pub struct Sand {
    pos: (usize, usize),
    color: u32,
    updated: bool,
}

impl Sand {
    pub fn new(pos: (usize, usize)) -> Sand {
        Sand {
            pos,
            color: 0x00ffc433,
            updated: false,
        }
    }
}

impl Pixel for Sand {
    fn update(&self, world: &World) -> Option<(usize, usize)> {
        if let Some(id) = world.get_id_of((self.pos.0, self.pos.1 + 1)) {
            if Id::density(id) < Id::density(Id::Sand) {
                return Some((self.pos.0, self.pos.1 + 1));
            }
        }
        if let Some(Id::Empty) =
            world.get_id_of(((self.pos.0 as i32 - 1).max(0) as usize, self.pos.1))
        {
            if let Some(Id::Empty) =
                world.get_id_of(((self.pos.0 as i32 - 1).max(0) as usize, self.pos.1 + 1))
            {
                return Some((self.pos.0 - 1, self.pos.1 + 1));
            }
        }
        if let Some(Id::Empty) = world.get_id_of((self.pos.0 + 1, self.pos.1)) {
            if let Some(Id::Empty) = world.get_id_of((self.pos.0 + 1, self.pos.1 + 1)) {
                return Some((self.pos.0 + 1, self.pos.1 + 1));
            }
        }
        None
    }
    fn id(&self) -> Id {
        Id::Sand
    }
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
    fn set_updated(&mut self, update: bool) {
        self.updated = update;
    }
    fn get_updated(&self) -> bool {
        self.updated
    }
    fn get_color(&self) -> u32 {
        self.color
    }
}

pub struct Water {
    pos: (usize, usize),
    color: u32,
    updated: bool,
}

impl Water {
    pub fn new(pos: (usize, usize)) -> Water {
        Water {
            pos,
            color: 0x00408aed,
            updated: false,
        }
    }
}

impl Pixel for Water {
    fn update(&self, world: &World) -> Option<(usize, usize)> {
        if let Some(Id::Empty) = world.get_id_of((self.pos.0, self.pos.1 + 1)) {
            return Some((self.pos.0, self.pos.1 + 1));
        }
        if let Some(Id::Empty) =
            world.get_id_of(((self.pos.0 as i32 - 1).max(0) as usize, self.pos.1))
        {
            if let Some(Id::Empty) =
                world.get_id_of(((self.pos.0 as i32 - 1).max(0) as usize, self.pos.1 + 1))
            {
                return Some((self.pos.0 - 1, self.pos.1 + 1));
            }
            let mut offset = 2;
            while let Some(Id::Empty) =
                world.get_id_of(((self.pos.0 as i32 - offset).max(0) as usize, self.pos.1))
            {
                if let Some(Id::Empty) =
                    world.get_id_of(((self.pos.0 as i32 - offset).max(0) as usize, self.pos.1 + 1))
                {
                    return Some((self.pos.0 - 1, self.pos.1));
                }
                offset += 1;
            }
        }
        if let Some(Id::Empty) = world.get_id_of((self.pos.0 + 1, self.pos.1)) {
            if let Some(Id::Empty) = world.get_id_of((self.pos.0 + 1, self.pos.1 + 1)) {
                return Some((self.pos.0 + 1, self.pos.1 + 1));
            }
            let mut offset = 2;
            while let Some(Id::Empty) =
                world.get_id_of(((self.pos.0 as i32 + offset).max(0) as usize, self.pos.1))
            {
                if let Some(Id::Empty) =
                    world.get_id_of(((self.pos.0 as i32 + offset).max(0) as usize, self.pos.1 + 1))
                {
                    return Some((self.pos.0 + 1, self.pos.1));
                }
                offset += 1;
            }
        }
        None
    }
    fn id(&self) -> Id {
        Id::Water
    }
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
    fn set_updated(&mut self, update: bool) {
        self.updated = update;
    }
    fn get_updated(&self) -> bool {
        self.updated
    }
    fn get_color(&self) -> u32 {
        self.color
    }
}

pub struct Stone {
    pos: (usize, usize),
    color: u32,
    updated: bool,
}

impl Stone {
    pub fn new(pos: (usize, usize)) -> Stone {
        Stone {
            pos,
            color: 0x00888888,
            updated: false,
        }
    }
}

impl Pixel for Stone {
    fn update(&self, world: &World) -> Option<(usize, usize)> {
        None
    }
    fn id(&self) -> Id {
        Id::Stone
    }
    fn set_pos(&mut self, pos: (usize, usize)) -> (usize, usize) {
        self.pos = pos;
        pos
    }
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
    fn set_updated(&mut self, update: bool) {
        self.updated = update;
    }
    fn get_updated(&self) -> bool {
        self.updated
    }
    fn get_color(&self) -> u32 {
        self.color
    }
}
