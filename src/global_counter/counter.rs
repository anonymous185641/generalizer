use std::sync::atomic::AtomicUsize;

pub static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(1);
pub fn fresh_number()->usize{
    GLOBAL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub fn reset_counter() {
    GLOBAL_COUNTER.store(1, std::sync::atomic::Ordering::SeqCst); // Resets counter to 0
}