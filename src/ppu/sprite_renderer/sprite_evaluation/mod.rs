#[derive(Default)]
pub struct SpriteEvaluation {
    cur_cycle: u8, // 0-191
    scanline: u8,
    sprites_found: u8,
    n: u8,
    m: u8,
    state: State,
}

pub enum SpriteEvaluationAction {
    None,
    SetSpriteOverflowFlag,
}

enum State {
    ReadSpriteY,
    WriteSpriteY(u8),
    ReadTileIndex,
    WriteTileIndex(u8),
    ReadAttributes,
    WriteAttributes(u8),
    ReadX,
    WriteX(u8),
    SpriteOverflowEvaluationRead(u8),
    SpriteOverflowEvaluationWrite(u8, u8),
    Done,
}

impl Default for State {
    fn default() -> Self {
        State::ReadSpriteY
    }
}

impl SpriteEvaluation {
    pub fn new(scanline: u8) -> Self {
        SpriteEvaluation {
            scanline: scanline,
            cur_cycle: 0,
            sprites_found: 0,
            n: 0,
            m: 0,
            state: State::default(),
        }
    }

    pub fn tick(&mut self, primary_oam: &[u8], secondary_oam: &mut [u8]) -> SpriteEvaluationAction {
        let primary_oam_addr = (self.n as usize) * 4;
        match self.state {
            // Read Cycles
            State::ReadSpriteY => {
                self.state = State::WriteSpriteY(primary_oam[primary_oam_addr]);
                SpriteEvaluationAction::None
            }
            State::ReadTileIndex => {
                self.state = State::WriteTileIndex(primary_oam[primary_oam_addr + 1]);
                SpriteEvaluationAction::None
            }
            State::ReadAttributes => {
                self.state = State::WriteAttributes(primary_oam[primary_oam_addr + 2]);
                SpriteEvaluationAction::None
            }
            State::ReadX => {
                self.state = State::WriteX(primary_oam[primary_oam_addr + 3]);
                SpriteEvaluationAction::None
            }
            State::SpriteOverflowEvaluationRead(m) => {
                self.state = State::SpriteOverflowEvaluationWrite(m,
                                                                  primary_oam[primary_oam_addr +
                                                                  m as usize]);
                SpriteEvaluationAction::None
            }
            // Write Cycles
            State::WriteSpriteY(y) => {
                secondary_oam[(self.sprites_found as usize) * 4] = y;

                if self.is_sprite_on_scanline(y) {
                    self.state = State::ReadTileIndex
                } else {
                    self.n += 1;
                    if self.n >= 64 {
                        self.state = State::Done
                    } else {
                        self.state = State::ReadSpriteY;
                    }
                }
                SpriteEvaluationAction::None
            }

            State::WriteTileIndex(tile_index) => {
                secondary_oam[(self.sprites_found as usize) * 4 + 1] = tile_index;
                self.state = State::ReadAttributes;
                SpriteEvaluationAction::None
            }

            State::WriteAttributes(attributes) => {
                secondary_oam[(self.sprites_found as usize) * 4 + 2] = attributes;
                self.state = State::ReadX;
                SpriteEvaluationAction::None
            }

            State::WriteX(x) => {
                secondary_oam[(self.sprites_found as usize) * 4 + 3] = x;
                self.sprites_found += 1;
                self.n += 1;
                if self.n >= 64 {
                    self.state = State::Done
                } else if self.sprites_found < 8 {
                    self.state = State::ReadSpriteY
                } else {
                    self.state = State::SpriteOverflowEvaluationRead(0)
                }
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
        (y >= self.scanline) && (y + 8) < self.scanline
    }
}
