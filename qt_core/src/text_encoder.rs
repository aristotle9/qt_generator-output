/// C++ type: <span style='color: green;'>```QTextEncoder```</span>
#[repr(C)]
pub struct TextEncoder(u8);

impl TextEncoder {
  /// C++ method: <span style='color: green;'>```QByteArray QTextEncoder::fromUnicode(const QString& str)```</span>
  ///
  ///
  pub fn from_unicode(&mut self, str: &::string::String) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextEncoder_fromUnicode_to_output_str(self as *mut ::text_encoder::TextEncoder,
                                                                str as *const ::string::String,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QTextEncoder::fromUnicode(const QChar* uc, int len)```</span>
  ///
  ///
  pub unsafe fn from_unicode_unsafe(&mut self, uc: *const ::char::Char, len: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTextEncoder_fromUnicode_to_output_uc_len(self as *mut ::text_encoder::TextEncoder,
                                                                 uc,
                                                                 len,
                                                                 &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextEncoder::hasFailure() const```</span>
  ///
  ///
  pub fn has_failure(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTextEncoder_hasFailure(self as *const ::text_encoder::TextEncoder) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextEncoder::QTextEncoder(const QTextCodec* codec)```</span>
  ///
  ///
  pub unsafe fn new(codec: *const ::text_codec::TextCodec) -> ::cpp_utils::CppBox<::text_encoder::TextEncoder> {
    let ffi_result = ::ffi::qt_core_c_QTextEncoder_new(codec);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::text_encoder::TextEncoder {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTextEncoder_delete
  }
}
