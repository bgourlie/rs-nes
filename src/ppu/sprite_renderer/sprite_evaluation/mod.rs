#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

use ppu::SpriteSize;

#[derive(Default)]
pub struct SpriteEvaluation {
    scanline: u8,
    sprites_found: u8,
    n: u8,
    m: u8,
    state: State,
    // TODO: Is it safe to assume sprite size *never* changes during evaluation?
    sprite_size: SpriteSize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SpriteEvaluationAction {
    None,
    SetSpriteOverflowFlag,
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    FirstRead,
    FirstWrite(u8),
    SecondRead,
    SecondWrite(u8),
    ThirdRead,
    ThirdWrite(u8),
    FourthRead,
    FourthWrite(u8),
    SpriteOverflowEvaluationRead,
    SpriteOverflowEvaluationWrite(u8),
    Done,
}

impl Default for State {
    fn default() -> Self {
        State::FirstRead
    }
}

impl SpriteEvaluation {
    pub fn new(scanline: u8, sprite_size: SpriteSize) -> Self {
        SpriteEvaluation {
            scanline: scanline,
            sprites_found: 0,
            n: 0,
            m: 0,
            state: State::default(),
            sprite_size: sprite_size,
        }
    }

    pub fn sprites_found(&self) -> u8 {
        self.sprites_found
    }

    pub fn tick(&mut self, primary_oam: &[u8]) -> SpriteEvaluationAction {
        let primary_oam_addr = (self.n as usize) * 4;
        match self.state {
            State::FirstRead => {
                self.state = State::FirstWrite(primary_oam[primary_oam_addr]);
                SpriteEvaluationAction::None
            }
            State::FirstWrite(y) => {
                if self.is_sprite_on_scanline(y) {
                    println!("sprite @ y = {} hits scanline {}", y, self.scanline);
                    self.sprites_found += 1;
                    self.state = State::SecondRead
                } else {
                    self.n += 1;
                    if self.n >= 64 {
                        self.state = State::Done
                    } else {
                        self.state = State::FirstRead;
                    }
                }
                SpriteEvaluationAction::None
            }
            State::SecondRead => {
                self.state = State::SecondWrite(primary_oam[primary_oam_addr + 1]);
                SpriteEvaluationAction::None
            }
            State::SecondWrite(_) => {
                self.state = State::ThirdRead;
                SpriteEvaluationAction::None
            }
            State::ThirdRead => {
                self.state = State::ThirdWrite(primary_oam[primary_oam_addr + 2]);
                SpriteEvaluationAction::None
            }
            State::ThirdWrite(_) => {
                self.state = State::FourthRead;
                SpriteEvaluationAction::None
            }
            State::FourthRead => {
                self.state = State::FourthWrite(primary_oam[primary_oam_addr + 3]);
                SpriteEvaluationAction::None
            }
            State::FourthWrite(_) => {
                self.n += 1;
                if self.n >= 64 {
                    self.state = State::Done
                } else if self.sprites_found < 8 {
                    self.state = State::FirstRead;
                } else {
                    self.state = State::SpriteOverflowEvaluationRead
                }
                SpriteEvaluationAction::None
            }
            State::SpriteOverflowEvaluationRead => {
                let y = primary_oam[primary_oam_addr + self.m as usize];
                self.state = State::SpriteOverflowEvaluationWrite(y);
                SpriteEvaluationAction::None
            }
            State::SpriteOverflowEvaluationWrite(y) => {
                if self.m == 0 && self.is_sprite_on_scanline(y) {
                    // There are some additional reads after setting sprite over flow flag but I
                    // don't think they matter
                    self.state = State::Done;
                    SpriteEvaluationAction::SetSpriteOverflowFlag
                } else {
                    self.n += 1;
                    if self.n >= 64 {
                        self.state = State::Done;
                    } else {
                        self.state = State::SpriteOverflowEvaluationRead
                    }
                    SpriteEvaluationAction::None
                }
            }

            // Reads and writes happen here, but not emulating them for now since they're
            // probably inconsequential
            State::Done => SpriteEvaluationAction::None,
        }
    }

    fn is_sprite_on_scanline(&self, y: u8) -> bool {
        match self.sprite_size {
            SpriteSize::X8 => y <= self.scanline && self.scanline - y < 8,
            SpriteSize::X16 => y <= self.scanline && self.scanline - y < 16,
        }
    }
}
