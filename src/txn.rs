#[allow(dead_code)]
use std::ptr;

use crate::c;
use crate::c::*;
use crate::error::Error;

pub(crate) struct Tx {
    pub(crate) error: Option<Error>,
    pub(crate) obx_txn: *mut OBX_txn,
    pub(crate) ptr_closed: bool,
}

impl Drop for Tx {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr_closed && !self.obx_txn.is_null() {
                self.error = c::call(c::obx_txn_close(self.obx_txn), "Tx::drop".to_string()).err();
                self.obx_txn = std::ptr::null_mut();
            }

            if let Some(err) = &self.error {
                eprintln!("Error: txn: {err}");
            }
        }
    }
}

impl Tx {
    // TODO implement new and new_mut as closures
    // fn _new_op(store: &mut Store, op: Fn(*mut OBX_store) -> *mut OBX_txn) -> Self {
    //   if store.obx_store.is_null() {
    //     panic!("Error: uninitialized store");
    //   }
    //   match c::new_mut(unsafe { op(store.obx_store) }) {
    //     Ok(obx_txn) => Tx { obx_txn, error: None },
    //     Err(e) => Tx {
    //       error: Some(e),
    //       obx_txn: ptr::null_mut(),
    //       obx_store: ptr::null_mut(),
    //     },
    //   }
    // }

    // TODO check memory leak
    // new will clean itself up with drop
    pub(crate) fn new(store: *mut c::OBX_store) -> Self {
        match c::new_mut(unsafe { obx_txn_read(store) }, "Tx::new".to_string()) {
            Ok(obx_txn) => Tx {
                obx_txn,
                error: None,
                ptr_closed: false,
            },
            Err(e) => panic!("{e}"),
        }
    }

    // new_mut requires calling `obx_txn_success`
    pub(crate) fn new_mut(store: *mut c::OBX_store) -> Self {
        match c::new_mut(unsafe { obx_txn_write(store) }, "Tx::new_mut".to_string()) {
            Ok(obx_txn) => Tx {
                obx_txn,
                error: None,
                ptr_closed: false,
            },
            Err(e) => panic!("{e}"),
        }
    }

    // only run on write tx, read tx closes itself on the drop
    pub(crate) fn success(&mut self) {
        self.error = c::call(
            unsafe { obx_txn_success(self.obx_txn) },
            "Tx::success".to_string(),
        )
        .err();
        if let Some(err) = &self.error {
            panic!("{err}");
        } else {
            self.ptr_closed = true;
        }
    }

    fn abort(&mut self) {
        self.error = c::call(
            unsafe { obx_txn_abort(self.obx_txn) },
            "Tx::abort".to_string(),
        )
        .err();
    }

    // TODO open up for debugging
    pub(crate) fn data_size(&mut self) -> (u64, u64) {
        let mut committed_size = 0;
        let mut size_change = 0;
        self.error = c::call(
            unsafe { obx_txn_data_size(self.obx_txn, &mut committed_size, &mut size_change) },
            "Tx::data_size".to_string(),
        )
        .err();
        (committed_size, size_change)
    }
}
