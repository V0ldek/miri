use std::alloc::{alloc, realloc, Layout};

// error-pattern: has size 1 and alignment ALIGN, but gave size 2 and alignment ALIGN

fn main() {
    unsafe {
        let x = alloc(Layout::from_size_align_unchecked(1, 1));
        let _y = realloc(x, Layout::from_size_align_unchecked(2, 1), 1);
    }
}
