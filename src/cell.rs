#[derive(Copy, Clone, Default)]
pub struct Box {
    pub neighboors:u8,
    pub state: bool,
    pub x: i64,
    pub y: i64,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Box {
    pub fn new(neighboors:u8, state: bool,x: i64,y: i64,r: u8,g: u8,b: u8) -> Self {
        Self {
            neighboors,
            state,
            x,
            y,
            r,
            g,
            b
        }
    }

    pub fn count_neighboors(&mut self ,array:&Vec<Vec<Box>>, i:usize, j: usize, running: bool) -> u8 {
        if running == true {
            if i > 0 && i < 59 && j > 0 && j < 95 {
                // this implementation is bullshit, try something better
                if array[i - 1][j].state == true {
                    self.neighboors += 1;
                }
                if array[i + 1][j].state == true {
                    self.neighboors += 1;
                }
                if array[i][j + 1].state == true {
                    self.neighboors += 1;
                }
                if array[i][j - 1].state == true {
                    self.neighboors += 1;
                }
                if array[i - 1][j - 1].state == true {
                    self.neighboors += 1;
                }
                if array[i - 1][j + 1].state == true {
                    self.neighboors += 1;
                }
                if array[i + 1][j + 1].state == true {
                    self.neighboors += 1;
                }
                if array[i + 1][j - 1].state == true {
                    self.neighboors += 1;
                }
            }
        }
        return self.neighboors;
    }
}