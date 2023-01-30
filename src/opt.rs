#[allow(dead_code)]

use crate::c;
use crate::c::*;
use crate::error::Error;

pub(crate) struct Opt {
  error: Option<Error>,
  pub(crate) obx_opt: *mut OBX_store_options,
}

impl Drop for Opt {
  fn drop(&mut self) {
    unsafe {
      if !self.obx_opt.is_null() {
        obx_opt_free(self.obx_opt);
        self.obx_opt = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        println!("Error: {}", err);
      }
    }
  }
}

impl Opt {
  fn new() -> Self {
    let obx_opt = unsafe { obx_opt() };
    Opt {
      error: None,
      obx_opt,
    }
  }

  fn directory(&mut self, dir: *const ::std::os::raw::c_char) {
    self.error = c::call(unsafe { obx_opt_directory(self.obx_opt, dir) }).err();
  }

  fn max_db_size_in_kb(&mut self, size_in_kb: u64) {
    unsafe {
      obx_opt_max_db_size_in_kb(self.obx_opt, size_in_kb);
    }
  }

  fn max_data_size_in_kb(&mut self, size_in_kb: u64) {
    unsafe {
      obx_opt_max_data_size_in_kb(self.obx_opt, size_in_kb);
    }
  }

  fn file_mode(&mut self, file_mode: ::std::os::raw::c_uint) {
    unsafe {
      obx_opt_file_mode(self.obx_opt, file_mode);
    }
  }

  fn max_readers(&mut self, max_readers: ::std::os::raw::c_uint) {
    unsafe {
      obx_opt_max_readers(self.obx_opt, max_readers);
    }
  }

  fn no_reader_thread_locals(&mut self, flag: bool) {
    unsafe {
      obx_opt_no_reader_thread_locals(self.obx_opt, flag);
    }
  }

  // fn model(&mut self, model: *mut OBX_model) {
  fn model(&mut self, model: &mut OBX_model) {
    self.error = c::call(unsafe { obx_opt_model(self.obx_opt, model) }).err();
  }

  fn model_bytes(&mut self, bytes: *const ::std::os::raw::c_void, size: usize) {
    self.error = c::call(unsafe { obx_opt_model_bytes(self.obx_opt, bytes, size) }).err();
  }

  fn model_bytes_direct(&mut self, bytes: *const ::std::os::raw::c_void, size: usize) {
    self.error = c::call(unsafe { obx_opt_model_bytes_direct(self.obx_opt, bytes, size) }).err();
  }

  fn validate_on_open(&mut self, page_limit: usize, leaf_level: bool) {
    unsafe {
      obx_opt_validate_on_open(self.obx_opt, page_limit, leaf_level);
    }
  }

  fn put_padding_mode(&mut self, mode: OBXPutPaddingMode) {
    unsafe {
      obx_opt_put_padding_mode(self.obx_opt, mode);
    }
  }

  fn read_schema(&mut self, value: bool) {
    unsafe {
      obx_opt_read_schema(self.obx_opt, value);
    }
  }

  fn use_previous_commit(&mut self, value: bool) {
    unsafe {
      obx_opt_use_previous_commit(self.obx_opt, value);
    }
  }

  fn read_only(&mut self, value: bool) {
    unsafe {
      obx_opt_read_only(self.obx_opt, value);
    }
  }

  fn debug_flags(&mut self, flags: u32) {
    unsafe {
      obx_opt_debug_flags(self.obx_opt, flags);
    }
  }

  fn add_debug_flags(&mut self, flags: u32) {
    unsafe {
      obx_opt_add_debug_flags(self.obx_opt, flags);
    }
  }

  fn async_max_queue_length(&mut self, value: usize) {
    unsafe {
      obx_opt_async_max_queue_length(self.obx_opt, value);
    }
  }

  fn async_throttle_at_queue_length(&mut self, value: usize) {
    unsafe {
      obx_opt_async_throttle_at_queue_length(self.obx_opt, value);
    }
  }

  fn async_throttle_micros(&mut self, value: u32) {
    unsafe {
      obx_opt_async_throttle_micros(self.obx_opt, value);
    }
  }

  fn async_max_in_tx_duration(&mut self, micros: u32) {
    unsafe {
      obx_opt_async_max_in_tx_duration(self.obx_opt, micros);
    }
  }

  fn async_max_in_tx_operations(&mut self, value: u32) {
    unsafe {
      obx_opt_async_max_in_tx_operations(self.obx_opt, value);
    }
  }

  fn async_pre_txn_delay(&mut self, delay_micros: u32) {
    unsafe {
      obx_opt_async_pre_txn_delay(self.obx_opt, delay_micros);
    }
  }

  fn async_pre_txn_delay4(&mut self, delay_micros: u32, delay2_micros: u32, min_queue_length_for_delay2: usize) {
    unsafe {
      obx_opt_async_pre_txn_delay4(self.obx_opt, delay_micros, delay2_micros, min_queue_length_for_delay2);
    }
  }

  fn async_post_txn_delay(&mut self, delay_micros: u32) {
    unsafe {
      obx_opt_async_post_txn_delay(self.obx_opt, delay_micros);
    }
  }

  fn async_post_txn_delay5(&mut self, delay_micros: u32, delay2_micros: u32, min_queue_length_for_delay2: usize, subtract_processing_time: bool) {
    unsafe {
      obx_opt_async_post_txn_delay5(self.obx_opt, delay_micros, delay2_micros, min_queue_length_for_delay2, subtract_processing_time);
    }
  }

  fn async_minor_refill_threshold(&mut self, queue_length: usize) {
    unsafe {
      obx_opt_async_minor_refill_threshold(self.obx_opt, queue_length);
    }
  }

  fn async_minor_refill_max_count(&mut self, value: u32) {
    unsafe {
      obx_opt_async_minor_refill_max_count(self.obx_opt, value);
    }
  }

  fn async_max_tx_pool_size(&mut self, value: usize) {
    unsafe {
      obx_opt_async_max_tx_pool_size(self.obx_opt, value);
    }
  }

  fn async_object_bytes_max_cache_size(&mut self, value: u64) {
    unsafe {
      obx_opt_async_object_bytes_max_cache_size(self.obx_opt, value);
    }
  }

  fn async_object_bytes_max_size_to_cache(&mut self, value: u64) {
    unsafe {
      obx_opt_async_object_bytes_max_size_to_cache(self.obx_opt, value);
    }
  }

  // TODO repair later
  // fn get_directory(&self) -> &str {
  //   let c_str = unsafe { obx_opt_get_directory(self.obx_opt) };
  //   let c_str_slice = unsafe { from_raw_parts(c_str as *const u8, strlen(c_str)) };
  //   str::from_utf8(c_str_slice).unwrap()
  // }

  fn get_max_db_size_in_kb(&self) -> u64 {
    unsafe { obx_opt_get_max_db_size_in_kb(self.obx_opt) }
  }

  fn get_max_data_size_in_kb(&self) -> u64 {
    unsafe { obx_opt_get_max_data_size_in_kb(self.obx_opt) }
  }

  fn get_debug_flags(&self) -> u32 {
    unsafe { obx_opt_get_debug_flags(self.obx_opt) }
  }
}
