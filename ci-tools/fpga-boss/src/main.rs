use libftdi1_sys::{self, ftdi_interface};
use std::ffi::{CStr, c_uchar};
use std::ptr;
use std::time::Duration;

const BITMODE_BITBANG: c_uchar = 0x01;

fn reset_fpga() {
    unsafe { 
        let ftdi = libftdi1_sys::ftdi_new();
        if ftdi == ptr::null_mut() {
            panic!("ftdi_new failed");
        }
        let mut rv = libftdi1_sys::ftdi_usb_open(ftdi, 0x0403, 0x6011);
        if rv < 0 {
            panic!(
                "ftdi_usb_open failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }

        rv = libftdi1_sys::ftdi_set_interface(ftdi, ftdi_interface::INTERFACE_A);
        if rv < 0 {
            panic!(
                "ftdi_set_interface failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }

        rv = libftdi1_sys::ftdi_set_bitmode(ftdi, 0xc0, BITMODE_BITBANG);
        if (rv < 0) {
            panic!(
                "ftdi_set_bitmode failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        let data: u8 = 0x0d;
        rv = libftdi1_sys::ftdi_write_data(ftdi, &data, 1);
        if (rv < 0) {
            panic!(
                "ftdi_write_data failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        std::thread::sleep(Duration::from_millis(1));

        let data: u8 = 0x8d;
        rv = libftdi1_sys::ftdi_write_data(ftdi, &data, 1);
        if (rv < 0) {
            panic!(
                "ftdi_write_data failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
        std::thread::sleep(Duration::from_millis(1));
        let data = 0xcd;
        rv = libftdi1_sys::ftdi_write_data(ftdi, &data, 1);
        if (rv < 0) {
            panic!(
                "ftdi_write_data failed: {:?}",
                CStr::from_ptr(libftdi1_sys::ftdi_get_error_string(ftdi))
            );
        }
    }
}

fn main() {
    println!("Hello, world!");
    reset_fpga();
}
