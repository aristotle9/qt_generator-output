/// C++ type: <span style='color: green;'>```QAbstractNativeEventFilter```</span>
#[repr(C)]
pub struct AbstractNativeEventFilter(u8);

impl AbstractNativeEventFilter {
  /// C++ method: <span style='color: green;'>```pure virtual bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray& eventType, void* message, long* result)```</span>
  ///
  ///
  pub unsafe fn native_event_filter(&mut self,
                                    event_type: &::byte_array::ByteArray,
                                    message: *mut ::libc::c_void,
                                    result: *mut ::libc::c_long)
                                    -> bool {
    ::ffi::qt_core_c_QAbstractNativeEventFilter_nativeEventFilter(self as *mut ::abstract_native_event_filter::AbstractNativeEventFilter, event_type as *const ::byte_array::ByteArray, message, result)
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_native_event_filter::AbstractNativeEventFilter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractNativeEventFilter_delete
  }
}
