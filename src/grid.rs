use rand::Rng;

#[derive(Copy, Clone)]
pub enum Tile{ Empty, Wall, Goal, Start, Queued, Head, Path }

#[derive(Clone)]
pub struct Grid{
    tiles: Vec::<Tile>,
    dim_tiles: (u16, u16),
    dim_total: (f32, f32),
    changed_tiles: Vec::<usize>,
    gap_tile_ratio: f32,
}

// TODO: tiles blør fra høyre til venstre kant

impl Grid{
    pub fn new(dimension_tiles: (u16, u16), dimension_grid: (f32, f32)) ->Self{
        
        let tiles = vec![Tile::Empty; dimension_tiles.0 as usize * dimension_tiles.1 as usize];
        Self{
            changed_tiles: tiles.iter().enumerate().map(|x|x.0).collect(),
            tiles,
            dim_tiles: dimension_tiles,
            dim_total: dimension_grid,
            gap_tile_ratio: 0.2
        }
    }
    
    pub fn new_random(dimension_tiles: (u16, u16), dimension_grid: (f32, f32))->Self{
        let mut new = Self::new(dimension_tiles, dimension_grid);
        new.shuffle();
        new
    }
    
    pub fn shuffle(&mut self) {
        
        use rand::prelude::*;
        
        for tile in self.tiles.iter_mut(){
            *tile = Tile::Empty;
        }
        
        let mut rng = thread_rng();
    
        let len = self.tiles.len();
        self.tiles[rng.gen_range(0..len)] = Tile::Start;
        self.tiles[rng.gen_range(0..len)] = Tile::Goal;
    }
    
    pub fn shuffle_visual(&mut self) -> Vec<usize>{
        
        use rand::prelude::*;
    
        let mut rng = thread_rng();
    
        const SEED_R:f32 = 0.005;
        const BRANCH_LEN:u16 = 24;
        let seeds = (SEED_R * self.tiles.len() as f32) as u16;
        
        self.set_all(Tile::Empty);
        
        // så frø
        let mut tiles: Vec<usize> = (0..seeds).map(|_|rng.gen_range(0..self.tiles.len()))
            .collect();
        
        self.set_tiles(tiles.clone(), Tile::Wall);
        
        // gro greiner
        for tile in tiles.clone().into_iter(){
            
            let mut curr = tile;
            self.set_tile(curr, Tile::Wall);
    
            for _ in 0..BRANCH_LEN{
                curr = self.pos_to_index(
                    *self.empty_bros(curr).choose(&mut rng).unwrap()
                );
                tiles.push(curr);
                self.set_tile(curr, Tile::Wall);
            }
        }
    
        // fyll hull og kanter
        for (pos, tile) in self.tiles.iter().enumerate() {
            if matches!(tile, Tile::Empty) &&
                rng.gen_bool(match self.empty_bros(pos).len() {
                0 => 0.90,
                1 => 0.75,
                2 => 0.50,
                3 => 0.25,
                _ => 0.00,
            }) {
                tiles.push(pos);
            }
        }

        let start = loop{
            let i = rng.gen_range(0..self.tiles.len());
            if matches!(self.tiles[i], Tile::Empty){
                break i;
            }
        };
        let goal = loop{
            let i = rng.gen_range(0..self.tiles.len());
            if matches!(self.tiles[i], Tile::Empty){
                break i;
            }
        };
        self.set_all(Tile::Empty);
        self.set_tile(start, Tile::Start);
        self.set_tile(goal, Tile::Goal);
        
        tiles.reverse();
        tiles
    }
    
    pub fn empty_bros(& self, pos: impl ToPos) -> Vec::<(u16, u16)>{
        
        let pos = pos.to_pos(self.dim_tiles.0);
        let mut v = vec![
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
        ];
        
        if 0 < pos.0 {
            v.push((pos.0 - 1, pos.1));
        }
        if 0 < pos.1 {
            v.push((pos.0, pos.1 - 1));
        }
        
        v.into_iter().filter(
            |pos|!matches!(self.get_tile(*pos).unwrap_or(Tile::Wall), Tile::Wall)
        ).collect()
    }
    
    pub fn draw(& self, ctx: & web_sys::CanvasRenderingContext2d){
        
        // TODO: forenkle matten her
        let t = self.dim_total.0 / self.dim_tiles.0 as f32;
        let w = t * (1. - self.gap_tile_ratio);
        let g = w * self.gap_tile_ratio;
        
        for (i, tile) in self.changed_tiles.iter()
                             .map(|i|(*i, self.tiles[*i])){
            
            match tile {
                Tile::Empty => ctx.set_fill_style(&"black".into()),
                Tile::Wall => ctx.set_fill_style(&"grey".into()),
                Tile::Goal => ctx.set_fill_style(&"lime".into()),
                Tile::Start => ctx.set_fill_style(&"olive".into()),
                Tile::Queued => ctx.set_fill_style(&"teal".into()),
                Tile::Head => ctx.set_fill_style(&"silver".into()),
                Tile::Path => ctx.set_fill_style(&"blue".into()),
            }
            
            let (x, y) = i.to_pos(self.dim_tiles.0);
            ctx.fill_rect(
                x as f64 * t as f64,
                y as f64 * t as f64,
                w as f64,
                w as f64,
            );
        }
    }
    
    pub fn index_to_pos(& self, index: usize) -> (u16, u16){
        index.to_pos(self.dim_tiles.0)
    }
    
    pub fn pos_to_index(& self, pos: (u16, u16)) -> usize{
        pos.to_index(self.dim_tiles.0)
    }
    
    pub fn iter(& self) -> std::slice::Iter<'_, Tile> {
        self.tiles.iter()
    }
    
    pub fn get_tile(& self, index: impl ToIndex) -> Option<Tile>{
    
        let index = index.to_index(self.dim_tiles.0);
    
        self.tiles.get(index).map(|x|*x)
    }
    
    pub fn set_tile(&mut self, index: impl ToIndex, new_tile: Tile) {
        
        let index = index.to_index(self.dim_tiles.0);
        match self.tiles.get_mut(index){
            None => panic!("index too large or small"),
            Some(t) => {
                *t = new_tile;
                self.changed_tiles.push(index);
            }
        }
    }
    
    pub fn set_all(&mut self, new_tile: Tile){
        for tile in self.tiles.iter_mut(){
            *tile = new_tile;
        }
        
        self.changed_tiles.append(&mut (0..self.tiles.len()).collect())
    }
    
    pub fn set_tiles(&mut self, mut indices: Vec::<usize>, new_tile: Tile) {
        
        for i in indices.iter(){
            self.tiles[*i] = new_tile;
        }
        
        self.changed_tiles.append(&mut indices);
    }
}


pub trait ToPos{
    fn to_pos(& self, width: u16)->(u16, u16);
}

pub trait ToIndex{
    fn to_index(& self, width: u16)->usize;
}

impl ToPos for (u16, u16){
    fn to_pos(&self, width: u16) -> (u16, u16) {
        *self
    }
}

impl ToPos for usize {
    fn to_pos(&self, width: u16) -> (u16, u16) {
        (*self as u16 % width, *self as u16 / width)
    }
}

impl ToIndex for usize{
    fn to_index(&self, width: u16) -> usize {
        *self
    }
}

impl ToIndex for (u16, u16){
    fn to_index(&self, width: u16) -> usize {
        self.1 as usize * width as usize + self.0 as usize
    }
}

#[test]
fn index_to_pos(){
    assert_eq!(0.to_pos(6), (0, 0));
    assert_eq!(5.to_pos(6), (5, 0));
    assert_eq!(6.to_pos(6), (0, 1));
    assert_eq!(13.to_pos(6), (1, 2));
    assert_eq!(23.to_pos(6), (5, 3));
}

#[test]
fn pos_to_index(){
    assert_eq!((0, 0).to_index(6), 0);
    assert_eq!((5, 0).to_index(6), 5);
    assert_eq!((0, 1).to_index(6), 6);
    assert_eq!((1, 2).to_index(6), 13);
    assert_eq!((5, 3).to_index(6), 23);
}