#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Codes {
    pub w_pos: i32,
    pub w_step: i16,
    pub b_pos: i32,
    pub b_step: i16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

pub trait Code {
    fn get_point(&self, id: usize) -> i32;
    fn to_slice(&self) -> [i32; 5];
    fn set_point(&mut self, point: i32, id: usize);
    fn from_slice(&mut self, slice: &[i32; 5]) {
        for i in 0..5 {
            self.set_point(slice[i], i);
        }
    }
}

pub trait TurnCode {
    fn next_turn(&mut self);
    fn to_turn(&self) -> Color;
}

impl Code for i32 {
    fn get_point(&self, id: usize) -> i32 {
        (self >> id * 5) % 32
    }

    fn to_slice(&self) -> [i32; 5] {
        [
            self.get_point(0),
            self.get_point(1),
            self.get_point(2),
            self.get_point(3),
            self.get_point(4),
        ]
    }

    fn set_point(&mut self, point: i32, id: usize) {
        let pos_old = (*self >> id * 5) % 32;
        *self -= pos_old << id * 5;
        *self += point << id * 5;
    }
}

impl Code for i16 {
    fn get_point(&self, id: usize) -> i32 {
        ((self >> id * 2) % 4) as i32
    }

    fn to_slice(&self) -> [i32; 5] {
        [
            self.get_point(0),
            self.get_point(1),
            self.get_point(2),
            self.get_point(3),
            self.get_point(4),
        ]
    }

    fn set_point(&mut self, point: i32, id: usize) {
        let pos_old = (*self >> id * 2) % 4;
        *self -= pos_old << id * 2;
        *self += (point << id * 2) as i16;
    }
}

impl TurnCode for i16 {
    fn next_turn(&mut self) {
        match self.to_turn() {
            Color::White => { *self += 1 << 10; }
            Color::Black => { *self -= 1 << 10; }
        }
    }

    fn to_turn(&self) -> Color {
        match (*self >> 10) % 2 {
            0 => Color::White,
            1 => Color::Black,
            _ => panic!(),
        }
    }
}

impl Codes {
    pub fn from_int(w_pos: i32, w_step: i16, b_pos: i32, b_step: i16) -> Self {
        Codes { w_pos, w_step, b_pos, b_step }
    }

    pub fn from_slices(w_pos_slice: &[i32; 5], w_step_slice: &[i32; 5], b_pos_slice: &[i32; 5], b_step_slice: &[i32; 5]) -> Self {
        let mut w_pos = 0;
        let mut w_step = 0;
        let mut b_pos = 0;
        let mut b_step = 0;
        w_pos.from_slice(w_pos_slice);
        w_step.from_slice(w_step_slice);
        b_pos.from_slice(b_pos_slice);
        b_step.from_slice(b_step_slice);
        Codes { w_pos, w_step, b_pos, b_step }
    }

    pub fn to_slices(&self, color: Color) -> ([i32; 5], [i32; 5]) {
        match color {
            Color::White => (
                self.w_pos.to_slice(),
                self.w_step.to_slice(),
            ),
            Color::Black => (
                self.b_pos.to_slice(),
                self.b_step.to_slice(),
            ),
        }
    }

    pub fn sort(&mut self, color: Color) {
        let slices = self.to_slices(color);

        let mut idx = vec![0, 1, 2, 3, 4];
        idx.sort_by(|&i, &j| (slices.0[i], slices.1[i]).cmp(&(slices.0[j], slices.1[j])));
        let mut pos: i32 = 0;
        let mut step: i16 = 0;
        pos.from_slice(
            &[
                slices.0[idx[0]],
                slices.0[idx[1]],
                slices.0[idx[2]],
                slices.0[idx[3]],
                slices.0[idx[4]],
            ]
        );
        step.from_slice(
            &[
                slices.1[idx[0]],
                slices.1[idx[1]],
                slices.1[idx[2]],
                slices.1[idx[3]],
                slices.1[idx[4]],
            ]
        );

        match color {
            Color::White => {
                self.w_pos = pos;
                self.w_step = step;
            }
            Color::Black => {
                self.b_pos = pos;
                self.b_step = step;
            }
        }
    }

    pub fn to_vec(&self) -> Vec<i8> {
        let mut v = vec![0; 30];
        for (&pos, &step) in self.w_pos.to_slice().iter().zip(self.w_step.to_slice().iter()) {
            if pos >= 30 { continue; }
            v[pos as usize] += 2i8.pow(step as u32 * 2);
        }
        for (&pos, &step) in self.b_pos.to_slice().iter().zip(self.b_step.to_slice().iter()) {
            if pos >= 30 { continue; }
            v[pos as usize] -= 2i8.pow(step as u32 * 2);
        }
        v
    }

    pub fn next_turn(&mut self) {
        self.w_step.next_turn();
    }

    pub fn to_turn(&self) -> Color {
        self.w_step.to_turn()
    }
}
