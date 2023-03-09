#![allow(dead_code)]
use crate::c::*;
use crate::{c, error};

pub(crate) struct Tx {
    // pub(crate) error: Option<Error>,
    pub(crate) obx_txn: *mut OBX_txn,
    pub(crate) ptr_closed: bool,
}

impl Drop for Tx {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr_closed && !self.obx_txn.is_null() {
                match c::call(c::obx_txn_close(self.obx_txn), Some("Tx::drop")).err() {
                    Some(err) => eprintln!("Error: txn: {err}"),
                    _ => ()
                }
                self.obx_txn = std::ptr::null_mut();
            }
        }
    }
}

impl Tx {

    // TODO check memory leak
    // new will clean itself up with drop
    pub(crate) fn new(store: *mut c::OBX_store) -> error::Result<Self> {
        c::new_mut(unsafe { obx_txn_read(store) }, Some("Tx::new"))
        .map(|obx_txn|
            Tx {
                obx_txn,
                ptr_closed: false,
            }
        )
    }

    // new_mut requires calling `obx_txn_success`
    pub(crate) fn new_mut(store: *mut c::OBX_store) -> error::Result<Self> {
        c::new_mut(unsafe { obx_txn_write(store) }, Some("Tx::new_mut"))
        .map(|obx_txn|
            Tx {
                obx_txn,
                ptr_closed: false,
            }
        )
    }

    // only run on write tx, read tx closes itself on the drop
    pub(crate) fn success(&mut self) -> error::Result<()> {
        let r = unsafe { obx_txn_success(self.obx_txn) };

        if r == 0 {
            self.ptr_closed = true;
            return Ok(());
        }

        c::call(r, Some("Tx::success"))
    }

    fn abort(&mut self) -> error::Result<()> {
        c::call(unsafe { obx_txn_abort(self.obx_txn) }, Some("Tx::abort"))
    }

    // TODO write test
    pub(crate) fn data_size(&mut self) -> error::Result<(u64, u64)> {
        let mut committed_size = 0;
        let mut size_change = 0;
        c::call(
            unsafe { obx_txn_data_size(self.obx_txn, &mut committed_size, &mut size_change) },
            Some("Tx::data_size"),
        )
        .map(|_| (committed_size, size_change))
    }
}
