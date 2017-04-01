use super::*;

#[test]
fn fill_shift_registers() {
    let mut renderer = BackgroundRenderer::default();

    renderer.pattern_low_latch = 0x0;
    renderer.pattern_high_latch = 0x0;
    renderer.attr_latch = 0x0;
    renderer.fill_shift_registers(0);
    assert_eq!(0x0, renderer.pattern_low_shift_register);
    assert_eq!(0x0, renderer.pattern_high_shift_register);
    assert_eq!(0x0, renderer.palette_low_bit_shift_register);
    assert_eq!(0x0, renderer.palette_high_bit_shift_register);

    renderer.pattern_low_latch = 0b10101010;
    renderer.pattern_high_latch = 0b01010101;
    renderer.attr_latch = 0xff;
    renderer.fill_shift_registers(0xff);
    assert_eq!(0b10101010, renderer.pattern_low_shift_register);
    assert_eq!(0b01010101, renderer.pattern_high_shift_register);
    assert_eq!(0xff, renderer.palette_low_bit_shift_register);
    assert_eq!(0xff, renderer.palette_high_bit_shift_register);

    renderer.pattern_low_latch = 0b01010101;
    renderer.pattern_high_latch = 0b10101010;
    renderer.attr_latch = 0xff;
    renderer.fill_shift_registers(0);
    assert_eq!(0xff, renderer.pattern_low_shift_register);
    assert_eq!(0xff, renderer.pattern_high_shift_register);
    assert_eq!(0xff, renderer.palette_low_bit_shift_register);
    assert_eq!(0xff, renderer.palette_high_bit_shift_register);
}

#[test]
fn pattern_shift_values() {
    // At a high level, this function should choose the correct two bits of the palette based on
    // the coarse x and y bits of v. It then returns a byte for each bit (low, high) with all
    // bits set or clear based on the attribute bit at that position.

    // Coarse X and Y bits are (0, 0), so top left palette should be chosen
    let result = BackgroundRenderer::palette_shift_bytes(0, 0b000000_11);
    assert_eq!((0xff, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(0, 0b111111_01);
    assert_eq!((0xff, 0), result);

    let result = BackgroundRenderer::palette_shift_bytes(0, 0b111111_10);
    assert_eq!((0, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(0, 0b111111_00);
    assert_eq!((0, 0), result);

    // Coarse X and Y bits are (1, 0), so top right palette should be chosen
    let result = BackgroundRenderer::palette_shift_bytes(1, 0b0000_11_00);
    assert_eq!((0xff, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(1, 0b1111_01_11);
    assert_eq!((0xff, 0), result);

    let result = BackgroundRenderer::palette_shift_bytes(1, 0b1111_10_11);
    assert_eq!((0, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(1, 0b1111_00_11);
    assert_eq!((0, 0), result);

    // Coarse X and Y bits are (0, 1), so bottom left palette should be chosen
    let result = BackgroundRenderer::palette_shift_bytes(0b100000, 0b00_11_0000);
    assert_eq!((0xff, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(0b100000, 0b11_01_1111);
    assert_eq!((0xff, 0), result);

    let result = BackgroundRenderer::palette_shift_bytes(0b100000, 0b11_10_1111);
    assert_eq!((0, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(0b100000, 0b11_00_1111);
    assert_eq!((0, 0), result);

    // Coarse X and Y bits are (1, 1), so bottom right palette should be chosen
    let result = BackgroundRenderer::palette_shift_bytes(0b100001, 0b_11_000000);
    assert_eq!((0xff, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(0b100001, 0b_01_111111);
    assert_eq!((0xff, 0), result);

    let result = BackgroundRenderer::palette_shift_bytes(0b100001, 0b_10_111111);
    assert_eq!((0, 0xff), result);

    let result = BackgroundRenderer::palette_shift_bytes(0b100001, 0b_00_111111);
    assert_eq!((0, 0), result);
}

#[test]
fn pixel_mux() {
    // Pixel mux truth table:
    // Palette bits make up the top two bits of the nibble, pattern bits make up the lower two bits
    // Also verifies proper masking of each word
    let result = BackgroundRenderer::pixel_mux(0xffff, 0xffff, 0xffff, 0xffff);
    assert_eq!(0b1111, result); // 15

    let result = BackgroundRenderer::pixel_mux(0xffff, 0xffff, 0xffff, 0x0000);
    assert_eq!(0b1110, result); // 14

    let result = BackgroundRenderer::pixel_mux(0xffff, 0xffff, 0x0000, 0xffff);
    assert_eq!(0b1101, result); // 13

    let result = BackgroundRenderer::pixel_mux(0xffff, 0xffff, 0x0000, 0x0000);
    assert_eq!(0b1100, result); // 12

    let result = BackgroundRenderer::pixel_mux(0xffff, 0x0000, 0xffff, 0xffff);
    assert_eq!(0b1011, result); // 11

    let result = BackgroundRenderer::pixel_mux(0xffff, 0x0000, 0xffff, 0x0000);
    assert_eq!(0b1010, result); // 10

    let result = BackgroundRenderer::pixel_mux(0xffff, 0x0000, 0x0000, 0xffff);
    assert_eq!(0b1001, result); // 9

    let result = BackgroundRenderer::pixel_mux(0xffff, 0x0000, 0x0000, 0x0000);
    assert_eq!(0b1000, result); // 8

    let result = BackgroundRenderer::pixel_mux(0x0000, 0xffff, 0xffff, 0xffff);
    assert_eq!(0b0111, result); // 7

    let result = BackgroundRenderer::pixel_mux(0x0000, 0xffff, 0xffff, 0x0000);
    assert_eq!(0b0110, result); // 6

    let result = BackgroundRenderer::pixel_mux(0x0000, 0xffff, 0x0000, 0xffff);
    assert_eq!(0b0101, result); // 5

    let result = BackgroundRenderer::pixel_mux(0x0000, 0xffff, 0x0000, 0x0000);
    assert_eq!(0b0100, result); // 4

    let result = BackgroundRenderer::pixel_mux(0x0000, 0x0000, 0xffff, 0xffff);
    assert_eq!(0b0011, result); // 3

    let result = BackgroundRenderer::pixel_mux(0x0000, 0x0000, 0xffff, 0x0000);
    assert_eq!(0b0010, result); // 2

    let result = BackgroundRenderer::pixel_mux(0x0000, 0x0000, 0x0000, 0xffff);
    assert_eq!(0b0001, result); // 1

    let result = BackgroundRenderer::pixel_mux(0x0000, 0x0000, 0x0000, 0x0000);
    assert_eq!(0b0000, result); // 0
}

