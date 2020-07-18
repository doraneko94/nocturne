use std::collections::HashMap;

use crate::code::*;
use crate::io::*;

const DIRS: [(i32, i32); 8] = [( -1, -1), ( -1,  0), ( -1,  1),
                               (  0, -1),            (  0,  1),
                               (  1, -1), (  1,  0), (  1,  1)];

pub enum Status {
    WhiteWins,
    BlackWins,
    Draw,
    OnGoing,
}

pub struct Game {
    pub codes: Codes,
    pub history: HashMap<Codes, usize>,
}

impl Game {
    pub fn new() -> Self {
        let codes = Codes::from_slices(
            &[ 0,  1,  2,  3,  4],
            &[ 0,  0,  0,  0,  0],
            &[25, 26, 27, 28, 29],
            &[ 0,  0,  0,  0,  0],
        );
        let history = HashMap::new();
        Game { codes, history }
    }

    pub fn init(&mut self) {
        self.codes = Codes::from_slices(
            &[ 0,  1,  2,  3,  4],
            &[ 0,  0,  0,  0,  0],
            &[25, 26, 27, 28, 29],
            &[ 0,  0,  0,  0,  0],
        );
        self.history = HashMap::new();
    }

    pub fn view(&self) {
        let v = self.codes.to_vec();
        for i in 0..6 {
            println!("-------------------------------");
            println!("| {:3} | {:3} | {:3} | {:3} | {:3} |", v[5*i], v[5*i+1], v[5*i+2], v[5*i+3], v[5*i+4]);
        }
        println!("-------------------------------");
    }

    pub fn next(&mut self, dir: usize) -> Result<Status, ()> {
        match self.is_valid(self.codes, dir) {
            Ok((Status::OnGoing, codes)) => {
                self.move_unchecked(codes);
                Ok(Status::OnGoing)
            }
            Ok((s, _)) => Ok(s),
            Err(_) => Err(()),
        }
    }

    pub fn move_unchecked(&mut self, codes: Codes) {
        self.codes = codes;
    }

    pub fn is_valid(&self, mut codes: Codes, dir: usize) -> Result<(Status, Codes), ()> {
        let turn = codes.to_turn();
        let obj =  dir / 8;
        let (dy, dx) = DIRS[dir % 8];
        let (pos, step) = match turn {
            Color::White => (codes.w_pos, codes.w_step),
            Color::Black => (codes.b_pos, codes.b_step),
        };
        let p0 = pos.get_point(obj);
        let y0 = p0 / 5;
        let x0 = p0 % 5;
        let y1 = y0 + dy;
        let x1 = x0 + dx;
        if x1 < 0 || x1 > 4 {
            return Err(());
        }
        if y1 == -1 {
            match turn {
                Color::Black => { return Ok((Status::BlackWins, codes)); }
                Color::White => { return Err(()); }
            };
        }
        if y1 == 6 {
            match turn {
                Color::White => { return Ok((Status::WhiteWins, codes)); }
                Color::Black => { return Err(()); }
            };
        }
        let p1 = y1 * 5 + x1;
        let mut s0 = -1;
        let mut s1 = -1;
        for i in 0..5 {
            if codes.w_pos.get_point(i) == p0 { s0 += 1; }
            if codes.b_pos.get_point(i) == p0 { s0 += 1; }
            if codes.w_pos.get_point(i) == p1 { s1 += 1; }
            if codes.b_pos.get_point(i) == p1 { s1 += 1; }
        }
        if s0 > step.get_point(obj) || s1 > 1 { return Err(()) }
        match turn {
            Color::White => {
                codes.w_pos.set_point(p1, obj);
                codes.w_step.set_point(s1 + 1, obj);
                codes.sort(Color::White);
            }
            Color::Black => {
                codes.b_pos.set_point(p1, obj);
                codes.b_step.set_point(s1 + 1, obj);
                codes.sort(Color::Black);
            }
        };
        codes.next_turn();
        match self.history.get(&codes) {
            Some(&t) => { if t >= 2 { return Ok((Status::Draw, codes)); } }
            None => (),
        };
        Ok((Status::OnGoing, codes))
    }

    pub fn player_move(&mut self) -> Result<Status, ()> {
        self.view();
        match self.codes.to_turn() {
            Color::White => { println!("White, Move: "); }
            Color::Black => { println!("Black, Move: "); }
        }
        let v = readn::<usize>(" ");
        if v.len() < 2 { return Err(()); }
        let dir = v[0] * 8 + v[1];
        self.next(dir)
    }
}
