use super::*;

#[test]
fn palette_index() {
    assert_eq!(0, Quadrant::TopLeft.palette_index(0b11111100));
    assert_eq!(1, Quadrant::TopLeft.palette_index(0b11111101));
    assert_eq!(2, Quadrant::TopLeft.palette_index(0b11111110));
    assert_eq!(3, Quadrant::TopLeft.palette_index(0b11111111));

    assert_eq!(0, Quadrant::TopRight.palette_index(0b11110011));
    assert_eq!(1, Quadrant::TopRight.palette_index(0b11110111));
    assert_eq!(2, Quadrant::TopRight.palette_index(0b11111011));
    assert_eq!(3, Quadrant::TopRight.palette_index(0b11111111));

    assert_eq!(0, Quadrant::BottomLeft.palette_index(0b11001111));
    assert_eq!(1, Quadrant::BottomLeft.palette_index(0b11011111));
    assert_eq!(2, Quadrant::BottomLeft.palette_index(0b11101111));
    assert_eq!(3, Quadrant::BottomLeft.palette_index(0b11111111));

    assert_eq!(0, Quadrant::BottomRight.palette_index(0b00111111));
    assert_eq!(1, Quadrant::BottomRight.palette_index(0b01111111));
    assert_eq!(2, Quadrant::BottomRight.palette_index(0b10111111));
    assert_eq!(3, Quadrant::BottomRight.palette_index(0b11111111));
}

#[test]
fn tile32_quad() {
    let pixel = Pixel(0, 0);
    assert_eq!(Quadrant::TopLeft, pixel.tile32_quadrant());

    let pixel = Pixel(15, 0);
    assert_eq!(Quadrant::TopLeft, pixel.tile32_quadrant());

    let pixel = Pixel(0, 15);
    assert_eq!(Quadrant::TopLeft, pixel.tile32_quadrant());

    let pixel = Pixel(16, 0);
    assert_eq!(Quadrant::TopRight, pixel.tile32_quadrant());

    let pixel = Pixel(31, 0);
    assert_eq!(Quadrant::TopRight, pixel.tile32_quadrant());

    let pixel = Pixel(0, 16);
    assert_eq!(Quadrant::BottomLeft, pixel.tile32_quadrant());

    let pixel = Pixel(0, 31);
    assert_eq!(Quadrant::BottomLeft, pixel.tile32_quadrant());

    let pixel = Pixel(16, 16);
    assert_eq!(Quadrant::BottomRight, pixel.tile32_quadrant());

    let pixel = Pixel(31, 31);
    assert_eq!(Quadrant::BottomRight, pixel.tile32_quadrant());

    let pixel = Pixel(0, 223);
    assert_eq!(Quadrant::BottomLeft, pixel.tile32_quadrant());

    let pixel = Pixel(15, 223);
    assert_eq!(Quadrant::BottomLeft, pixel.tile32_quadrant());

    let pixel = Pixel(15, 224);
    assert_eq!(Quadrant::TopLeft, pixel.tile32_quadrant());

    let pixel = Pixel(16, 224);
    assert_eq!(Quadrant::TopRight, pixel.tile32_quadrant());

    let pixel = Pixel(16, 240);
    assert_eq!(Quadrant::BottomRight, pixel.tile32_quadrant());
}
