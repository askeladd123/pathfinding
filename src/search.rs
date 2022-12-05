use std::collections::HashMap;
use crate::grid::*;

// TODO: skal path være relativ eller global?

pub fn cost_euclidian(pos: (u16, u16), goal: (u16, u16))->u16{
    let x = pos.0.abs_diff(goal.0) as f32;
    let y = pos.1.abs_diff(goal.1) as f32;
    f32::sqrt(x*x + y*y) as u16
}

pub fn cost_manhatten(pos: (u16, u16), goal: (u16, u16))->u16{
    pos.0.abs_diff(goal.0) + pos.1.abs_diff(goal.1)
}

struct QueueItem{pos: (u16, u16), cost: u16}

#[derive(Debug)]
pub enum VisualResult{
    Found{path: Vec::<(u16, u16)>},
    NotFound{queued: Vec::<(u16, u16)>, head: (u16, u16)},
    Impossible,
}

pub fn ants_visual(grid: & Grid, steps: u32, ants: u16) -> VisualResult{
    todo!()
}

pub fn bfs_visual(grid: & Grid, steps: u32) -> VisualResult{
    todo!()
}

pub fn astar_visual(
    grid: & Grid,
    steps: u32,
    cost: fn(cos: (u16, u16), goal: (u16, u16)) -> u16)
    -> VisualResult {
    
    let (mut start, mut goal) = (None, None);
    for (i, tile) in grid.iter().enumerate(){
        if matches!(tile, Tile::Start){
            start = Some(grid.index_to_pos(i));
        }
        if matches!(tile, Tile::Goal){
            goal = Some(grid.index_to_pos(i));
        }
    }
    let (start, goal) = (
        start.expect("couldn't find starting tile"),
        goal.expect("couldn't find goal tile")
    );
    
    let mut curr = start;
    let mut queue= vec![QueueItem{pos: curr, cost: u16::MAX}];
    let mut parents = HashMap::<(u16, u16), (u16, u16)>::new();
    
    for i in 1..steps{
        if curr == goal{
            // TODO: return value er feil
            return VisualResult::Found {path: Vec::new()};
        }
        
        queue.append(
            &mut grid.empty_bros(curr)
                .into_iter()
                .filter(|pos|parents.get(pos).is_none())
                .map(|x|QueueItem{pos: x, cost: cost(x, goal)})
                .collect()
        );
        queue.sort_unstable_by(|a, b|b.cost.cmp(&a.cost));
        
        let last = curr;
        curr = match queue.pop() {
            None => return VisualResult::Impossible,
            Some(t) => t.pos
        };
        parents.insert(curr, last);
    }
    VisualResult::NotFound {
        queued: queue.into_iter().map(|x|x.pos).collect(),
        head: curr
    }
}

#[test]
fn astar(){
    let mut grid = Grid::new((6, 4), (200., 100.));
    grid.set_tile((0, 0), Tile::Start);
    grid.set_tile((4, 2), Tile::Goal);
    assert!(matches!(
        astar_visual(& grid, 8, cost_manhatten),
        VisualResult::Found{path:_}
        )
    );
    
    assert!(matches!(
        astar_visual(& grid, 2, cost_manhatten),
        VisualResult::NotFound {queued:_, head:_}
        )
    );
    
    grid.set_tile((2, 0), Tile::Wall);
    grid.set_tile((2, 1), Tile::Wall);
    grid.set_tile((2, 2), Tile::Wall);
    grid.set_tile((2, 3), Tile::Wall);
    
    assert!(matches!(
        astar_visual(& grid, 32, cost_manhatten),
        VisualResult::Impossible
        )
    );
}

#[test]
fn lol(){
    let mut grid = Grid::new((6, 4), (200., 100.));
    grid.set_tile((0, 0), Tile::Start);
    grid.set_tile((4, 2), Tile::Goal);
    
    let r = astar_visual(& grid, 3, cost_manhatten);
    dbg!(&r);
}