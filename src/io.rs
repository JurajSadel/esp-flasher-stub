use heapless::Deque;

pub mod uart;
#[cfg(any(
    feature = "esp32c3",
    feature = "esp32s3",
    feature = "esp32c6",
    feature = "esp32h2"
))]
pub mod usb_serial_jtag;

const RX_QUEUE_SIZE: usize = crate::targets::MAX_WRITE_BLOCK + 0x400;

static mut RX_QUEUE: Deque<u8, RX_QUEUE_SIZE> = Deque::new();