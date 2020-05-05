use conrod_core;
use conrod_core::{widget, widget_ids};
use conrod_core::widget::Widget;
use conrod_core::color::Colorable;
use conrod_core::position::{Sizeable, Positionable, Position};

pub struct Game{
    pub cells: Vec<Vec<Cell>>,
    buffer: Vec<Vec<i32>>,
    pub current_time_fs: i32,
    pub is_time_pass : bool,
    pub map_length: i32,
    ids: Ids,
    
}

widget_ids!{
    struct Ids{
        cells[]
    }
}


pub enum State{
    ALIVE,
    DEAD(i32)
}

pub struct Cell{
    pub state: State,
    pub image_type: i32,
}
impl Cell{
    pub fn new()->Cell{
        Cell{
            state: State::DEAD(0),
            image_type:0
        }
    }
}
pub const CELL_LENGTH : f64 = 45_f64;

impl Game {
    pub fn new(map_length: i32, id_gen: widget::id::Generator) -> Game{
        let mut cells = Vec::new();
        let mut buffer = Vec::new();
        cells.resize_with(map_length as usize, ||{
            let mut p = Vec::new();
            p.resize_with(map_length as usize, ||Cell::new());
            p
        });
        buffer.resize_with(map_length as usize, ||{
            let mut p = Vec::new();
            p.resize_with(map_length as usize, ||0);
            p
        });
        Game{
            cells: cells,
            buffer: buffer,
            current_time_fs: 0,
            is_time_pass: true,
            map_length: map_length,
            ids: Ids::new(id_gen)
        }
    }

    pub fn update(&mut self){
        let map_length = self.map_length;
        for i in 0..map_length as usize{
            for j in 0..map_length as usize{
                match self.cells[i][j].state{
                    State::DEAD(x)=>{
                        if x > 0{
                            self.cells[i][j].state = State::DEAD(x-1);
                        }
                    },
                    _=>()
                }
            }
        }
        
        if self.is_time_pass {self.current_time_fs += 1}
        if self.current_time_fs % 10 != 0 {return;}

        self.buffer.clear();
        self.buffer.resize_with(map_length as usize, ||{
            let mut p = Vec::new();
            p.resize_with(map_length as usize, ||0);
            p
        });
        for i in 0..map_length{
            for j in 0..map_length{
                if let State::DEAD(_) =  self.cells[i as usize][j as usize].state {continue}
                for a in -1..2{
                    for b in -1..2{
                        if a == b && a == 0 {continue}
                        if 0 <= a + i && a + i < map_length
                            && 0 <= b+j && b + j < map_length {
                            self.buffer[(a+i) as usize][(b+j) as usize] += 1;
                        }
                    }
                }
            }
        }

        for i in 0..map_length as usize{
            for j in 0..map_length as usize{
                match self.cells[i][j].state{
                    State::ALIVE=>{
                        match self.buffer[i][j] {
                            0|1|4..=8 => {
                                self.cells[i][j].state = State::DEAD(20);
                                self.cells[i][j].image_type += 3;
                            },
                            _ => ()
                        }
                    },
                    State::DEAD(x)=>{
                        if  x > 0{
                            self.cells[i][j].state = State::DEAD(x-1);
                        }
                        if self.buffer[i][j] == 3 {
                            self.cells[i][j].state = State::ALIVE;
                            self.cells[i][j].image_type += 1;
                            self.cells[i][j].image_type %= 3;
                        }
                    }
                }
                // self.cells[i][j].state = State::ALIVE;
            }
        }

    }

    pub fn draw(&mut self, ui: &mut conrod_core::UiCell, camera_pos: (f64,f64), zoom: f64){
        self.ids.cells.resize((self.map_length * self.map_length) as usize, &mut ui.widget_id_generator());

        for i in 0..self.map_length as usize{
            for j in 0..self.map_length as usize{
                if let Cell{state: State::DEAD(0), ..} = self.cells[i][j]{
                    continue;
                }
                let mut size = 0.8;
                if let State::DEAD(x) = self.cells[i][j].state{
                    size *= x as f64 / 100.0;
                }
                // widget::Image::new(image_ids[self.cells[i][j].image_type as usize])
                //     .w_h(CELL_LENGTH * size * zoom, CELL_LENGTH * size * zoom)
                //     .x_position(Position::Absolute((CELL_LENGTH * (i as f64 + 0.5) - camera_pos.0) * zoom - ui.win_w / 2.0))
                //     .y_position(Position::Absolute(ui.win_h / 2.0 - (CELL_LENGTH * (j as f64 + 0.5) - camera_pos.1) * zoom))
                //     .set(self.ids.cells[i*(self.map_length as usize)+j], ui);
                // println!("{}", i);
                widget::Rectangle::fill([CELL_LENGTH * size * zoom, CELL_LENGTH * size * zoom])
                    .x_position(Position::Absolute((CELL_LENGTH * (i as f64 + 0.5) - camera_pos.0) * zoom - ui.win_w / 2.0))
                    .y_position(Position::Absolute(ui.win_h / 2.0 - (CELL_LENGTH * (j as f64 + 0.5) - camera_pos.1) * zoom))
                    .set(self.ids.cells[i*(self.map_length as usize)+j], ui);
            }
        }
    }
}