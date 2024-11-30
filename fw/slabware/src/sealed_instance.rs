use embassy_sync::waitqueue::AtomicWaker;

pub(crate) trait SealedInstance {
    fn waker() -> &'static AtomicWaker;
}
