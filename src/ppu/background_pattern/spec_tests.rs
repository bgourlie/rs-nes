use super::*;

#[test]
fn quadrant_palette_index() {
    assert_eq!(0, AttributeQuadrant::TopLeft.palette(0b11111100));
    assert_eq!(1, AttributeQuadrant::TopLeft.palette(0b11111101));
    assert_eq!(2, AttributeQuadrant::TopLeft.palette(0b11111110));
    assert_eq!(3, AttributeQuadrant::TopLeft.palette(0b11111111));

    assert_eq!(0, AttributeQuadrant::TopRight.palette(0b11110011));
    assert_eq!(1, AttributeQuadrant::TopRight.palette(0b11110111));
    assert_eq!(2, AttributeQuadrant::TopRight.palette(0b11111011));
    assert_eq!(3, AttributeQuadrant::TopRight.palette(0b11111111));

    assert_eq!(0, AttributeQuadrant::BottomLeft.palette(0b11001111));
    assert_eq!(1, AttributeQuadrant::BottomLeft.palette(0b11011111));
    assert_eq!(2, AttributeQuadrant::BottomLeft.palette(0b11101111));
    assert_eq!(3, AttributeQuadrant::BottomLeft.palette(0b11111111));

    assert_eq!(0, AttributeQuadrant::BottomRight.palette(0b00111111));
    assert_eq!(1, AttributeQuadrant::BottomRight.palette(0b01111111));
    assert_eq!(2, AttributeQuadrant::BottomRight.palette(0b10111111));
    assert_eq!(3, AttributeQuadrant::BottomRight.palette(0b11111111));
}

#[test]
fn attribute_quadrant() {
    let pixel = BackgroundPattern::new(0, 0, 0);
    assert_eq!(AttributeQuadrant::TopLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(15, 0, 0);
    assert_eq!(AttributeQuadrant::TopLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(0, 15, 0);
    assert_eq!(AttributeQuadrant::TopLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(16, 0, 0);
    assert_eq!(AttributeQuadrant::TopRight, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(31, 0, 0);
    assert_eq!(AttributeQuadrant::TopRight, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(0, 16, 0);
    assert_eq!(AttributeQuadrant::BottomLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(0, 31, 0);
    assert_eq!(AttributeQuadrant::BottomLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(16, 16, 0);
    assert_eq!(AttributeQuadrant::BottomRight, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(31, 31, 0);
    assert_eq!(AttributeQuadrant::BottomRight, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(0, 223, 0);
    assert_eq!(AttributeQuadrant::BottomLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(15, 223, 0);
    assert_eq!(AttributeQuadrant::BottomLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(15, 224, 0);
    assert_eq!(AttributeQuadrant::TopLeft, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(16, 224, 0);
    assert_eq!(AttributeQuadrant::TopRight, pixel.attribute_quadrant());

    let pixel = BackgroundPattern::new(16, 240, 0);
    assert_eq!(AttributeQuadrant::BottomRight, pixel.attribute_quadrant());
}

#[test]
fn attribute_table_offset() {
    assert_eq!(0, BackgroundPattern::attribute_table_offset(0, 0));
    assert_eq!(0, BackgroundPattern::attribute_table_offset(31, 0));
    assert_eq!(0, BackgroundPattern::attribute_table_offset(0, 31));
    assert_eq!(0, BackgroundPattern::attribute_table_offset(31, 31));

    assert_eq!(1, BackgroundPattern::attribute_table_offset(32, 0));
    assert_eq!(1, BackgroundPattern::attribute_table_offset(63, 0));
    assert_eq!(1, BackgroundPattern::attribute_table_offset(32, 31));
    assert_eq!(1, BackgroundPattern::attribute_table_offset(63, 31));

    assert_eq!(9, BackgroundPattern::attribute_table_offset(32, 32));
    assert_eq!(9, BackgroundPattern::attribute_table_offset(63, 32));
    assert_eq!(9, BackgroundPattern::attribute_table_offset(32, 63));
    assert_eq!(9, BackgroundPattern::attribute_table_offset(63, 63));
}
