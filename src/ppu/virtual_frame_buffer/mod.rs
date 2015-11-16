enum WriteData {
}

struct VirtualFrameBuffer {
    buffer: [u8; 341 * 261],
    mid_frame_writes: Vec<WriteData>,
}
