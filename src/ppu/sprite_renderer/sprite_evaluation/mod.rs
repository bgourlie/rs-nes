#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

#[derive(Default)]
pub struct SpriteEvaluation {
    scanline: u8,
    sprites_found: u8,
    n: u8,
    m: u8,
    state: State,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SpriteEvaluationAction {
    None,
    SetSpriteOverflowFlag,
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    ReadOamY,
    WriteSecondaryOamY(u8),
    ReadOamTile,
    WriteSecondaryOamTile(u8),
    ReadOamAttributes,
    WriteSecondaryOamAttributes(u8),
    ReadOamX,
    WriteSecondaryOamX(u8),
    SpriteOverflowEvaluationRead(u8),
    SpriteOverflowEvaluationWrite(u8, u8),
    Done,
}

impl Default for State {
    fn default() -> Self {
        State::ReadOamY
    }
}

impl SpriteEvaluation {
    pub fn new(scanline: u8) -> Self {
        SpriteEvaluation {
            scanline: scanline,
            sprites_found: 0,
            n: 0,
            m: 0,
            state: State::default(),
        }
    }

    pub fn sprites_found(&self) -> u8 {
        self.sprites_found
    }

    pub fn tick(&mut self, primary_oam: &[u8]) -> SpriteEvaluationAction {
        let primary_oam_addr = (self.n as usize) * 4;
        match self.state {
            State::ReadOamY => {
                self.state = State::WriteSecondaryOamY(primary_oam[primary_oam_addr]);
                SpriteEvaluationAction::None
            }
            State::WriteSecondaryOamY(y) => {
                if self.is_sprite_on_scanline(y) {
                    println!("checking if sprite with y = {} is on scanline {}",
                             y,
                             self.scanline);
                    self.sprites_found += 1;
                    self.state = State::ReadOamTile
                } else {
                    self.n += 1;
                    if self.n >= 64 {
                        self.state = State::Done
                    } else {
                        self.state = State::ReadOamY;
                    }
                }
                SpriteEvaluationAction::None
            }
            State::ReadOamTile => {
                self.state = State::WriteSecondaryOamTile(primary_oam[primary_oam_addr + 1]);
                SpriteEvaluationAction::None
            }
            State::WriteSecondaryOamTile(_) => {
                self.state = State::ReadOamAttributes;
                SpriteEvaluationAction::None
            }
            State::ReadOamAttributes => {
                self.state = State::WriteSecondaryOamAttributes(primary_oam[primary_oam_addr + 2]);
                SpriteEvaluationAction::None
            }
            State::WriteSecondaryOamAttributes(_) => {
                self.state = State::ReadOamX;
                SpriteEvaluationAction::None
            }
            State::ReadOamX => {
                self.state = State::WriteSecondaryOamX(primary_oam[primary_oam_addr + 3]);
                SpriteEvaluationAction::None
            }
            State::WriteSecondaryOamX(_) => {
                // secondary_oam[(self.sprites_found as usize) * 4 + 3] = x;
                self.n += 1;
                if self.n >= 64 {
                    self.state = State::Done
                } else if self.sprites_found < 8 {
                    self.state = State::ReadOamY
                } else {
                    self.state = State::SpriteOverflowEvaluationRead(0)
                }
                SpriteEvaluationAction::None
            }
            State::SpriteOverflowEvaluationRead(m) => {
                self.state = State::SpriteOverflowEvaluationWrite(m,
                                                                  primary_oam[primary_oam_addr +
                                                                  m as usize]);
                SpriteEvaluationAction::None
            }
            State::SpriteOverflowEvaluationWrite(m, y) => {
                if m == 0 && self.is_sprite_on_scanline(y) {
                    // There are some additional reads after setting sprite over flow flag but I
                    // don't think they matter
                    self.state = State::Done;
                    SpriteEvaluationAction::SetSpriteOverflowFlag
                } else {
                    self.n += 1;
                    if self.n >= 64 {
                        self.state = State::Done;
                    } else {
                        self.state = State::SpriteOverflowEvaluationRead(m + 1)
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
        y >= self.scanline && y - self.scanline < 8
    }
}
