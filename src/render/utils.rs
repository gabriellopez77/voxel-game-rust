pub fn draw_indexed(primitive: gl::types::GLenum, trianles_count: i32) {
    unsafe {
        gl::DrawElements(primitive, trianles_count, gl::UNSIGNED_INT, std::ptr::null());
    }
}