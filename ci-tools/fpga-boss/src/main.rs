use ftdi1_sys;
use std::ffi::CStr;
use std::ptr;
use std::time::Duration;

fn reset_fpga() {
    let ftdi = ftdi1_sys::ftdi_new();
    if ftdi == ptr::null_mut() {
        panic!("ftdi_new failed");
    }
    let rv = ftdi1_sys::ftdi_usb_open(ftdi, 0x0403, 0x6011);
    if rv < 0 {
        panic!(
            "ftdi_usb_open failed: {:?}",
            CStr::from_ptr(ftdi1_sys::ftdi_get_error_string(ftdi))
        );
    }

    rv = ftdi1_sys::ftdi_set_interface(ftdi, ftdi1_sys::INTERFACE_A);
    if rv < 0 {
        panic!(
            "ftdi_set_interface failed: {:?}",
            CStr::from_ptr(ftdi1_sys::ftdi_get_error_string(ftdi))
        );
    }

    rv = ftdi1_sys::ftdi_set_bitmode(ftdi, 0xc0, ftdi1_sys::BITMODE_BITBANG);
    if (rv < 0) {
        panic!(
            "ftdi_set_bitmode failed: {:?}",
            CStr::from_ptr(ftdi1_sys::ftdi_get_error_string(ftdi))
        );
    }
    let data: u8 = 0x0d;
    rv = ftdi1_sys::ftdi_write_data(ftdi, &data, 1);
    if (rv < 0) {
        panic!(
            "ftdi_write_data failed: {:?}",
            CStr::from_ptr(ftdi1_sys::ftdi_get_error_string(ftdi))
        );
    }
    std::thread::sleep(Duration::from_millis(1));

    let data: u8 = 0x8d;
    rv = ftdi1_sys::ftdi_write_data(ftdi, &data, 1);
    if (rv < 0) {
        panic!(
            "ftdi_write_data failed: {:?}",
            CStr::from_ptr(ftdi1_sys::ftdi_get_error_string(ftdi))
        );
    }
    std::thread::sleep(Duration::from_millis(1));
    data = 0xcd;
    rv = ftdi1_sys::ftdi_write_data(ftdi, &data, 1);
    if (rv < 0) {
        panic!(
            "ftdi_write_data failed: {:?}",
            CStr::from_ptr(ftdi1_sys::ftdi_get_error_string(ftdi))
        );
    }
}

fn main() {
    println!("Hello, world!");
    reset_fpga();
}
