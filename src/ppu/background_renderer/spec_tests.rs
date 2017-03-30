use super::*;

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
