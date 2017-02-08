use super::*;

#[test]
fn write_address() {
    let vram = VramBase::default();
    assert_eq!(0, vram.address.get());
    assert_eq!(0, vram.address.get());

    vram.write_address(0x10);
    assert_eq!(0x1000, vram.address.get());

    vram.write_address(0x11);
    assert_eq!(0x1011, vram.address.get());

    vram.write_address(0x12);
    assert_eq!(0x1211, vram.address.get());

    vram.write_address(0x13);
    assert_eq!(0x1213, vram.address.get());
}

#[test]
fn clear_latch() {
    let vram = VramBase::default();
    assert_eq!(0x0, vram.address.get());

    vram.write_address(0x10);
    assert_eq!(0x1000, vram.address.get());

    assert_eq!(LatchState::WriteLowByte, vram.latch_state.get());
    vram.clear_latch();
    assert_eq!(LatchState::WriteHighByte, vram.latch_state.get());
}

#[test]
fn internal_memory_mapping_read() {
    let mut vram = VramBase::default();
    vram.pattern_tables = [1; 0x2000];
    vram.name_tables = [2; 0x1000];
    vram.palette = [3; 0x20];

    for _ in 0..0x2000 {
        assert_eq!(1, vram.read_data_increment_address().unwrap())
    }

    for _ in 0x2000..0x3f00 {
        assert_eq!(2, vram.read_data_increment_address().unwrap())
    }

    for _ in 0x3f00..0x4000 {
        assert_eq!(3, vram.read_data_increment_address().unwrap())
    }
}

#[test]
fn internal_memory_mapping_write() {
    let mut vram = VramBase::default();

    for _ in 0..0x2000 {
        vram.write_data_increment_address(1).unwrap()
    }

    for _ in 0x2000..0x3f00 {
        vram.write_data_increment_address(2).unwrap()
    }

    for _ in 0x3f00..0x4000 {
        vram.write_data_increment_address(3).unwrap()
    }

    assert_eq!(true, vram.pattern_tables.into_iter().all(|val| *val == 1));
    assert_eq!(true, vram.name_tables.into_iter().all(|val| *val == 2));
    assert_eq!(true, vram.palette.into_iter().all(|val| *val == 3));
}
