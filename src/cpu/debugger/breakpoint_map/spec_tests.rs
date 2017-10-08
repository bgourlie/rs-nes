use cpu::debugger::BreakpointMap;
use memory::ADDRESSABLE_MEMORY;
use rand::{thread_rng, Rng};

#[test]
fn set_breakpoint() {
    let mut rng = thread_rng();
    let mut addrs = [0_u16; ADDRESSABLE_MEMORY];
    let mut breakpoint_map = BreakpointMap::new();

    for i in 0..ADDRESSABLE_MEMORY {
        addrs[i] = i as u16;
    }

    rng.shuffle(&mut addrs);

    for (_, addr) in addrs.iter().enumerate() {
        let addr = *addr;
        assert_eq!(false, breakpoint_map.is_set(addr));
        breakpoint_map.toggle(addr);
        assert_eq!(true, breakpoint_map.is_set(addr));
    }

    for (_, addr) in addrs.iter().enumerate() {
        let addr = *addr;
        assert_eq!(true, breakpoint_map.is_set(addr));
        breakpoint_map.toggle(addr);
        assert_eq!(false, breakpoint_map.is_set(addr));
    }
}
