/// C++ type: <span style='color: green;'>```QTextDecoder```</span>
#[repr(C)]
pub struct TextDecoder(u8);

impl TextDecoder {
  /// C++ method: <span style='color: green;'>```bool QTextDecoder::hasFailure() const```</span>
  ///
  ///
  pub fn has_failure(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTextDecoder_hasFailure(self as *const ::text_decoder::TextDecoder) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextDecoder::QTextDecoder(const QTextCodec* codec)```</span>
  ///
  ///
  pub unsafe fn new(codec: *const ::text_codec::TextCodec) -> ::cpp_utils::CppBox<::text_decoder::TextDecoder> {
    let ffi_result = ::ffi::qt_core_c_QTextDecoder_new(codec);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QString QTextDecoder::toUnicode(const QByteArray& ba)```</span>
  ///
  ///
  pub fn to_unicode(&mut self, ba: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextDecoder_toUnicode_to_output_ba(self as *mut ::text_decoder::TextDecoder,
                                                             ba as *const ::byte_array::ByteArray,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDecoder::toUnicode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_unicode_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextDecoder::toUnicode(const char* chars, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_unicode_unsafe(&mut self, (*mut ::string::String, *const ::libc::c_char, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextDecoder::toUnicode(QString* target, const char* chars, int len)```</span>
  ///
  ///
  pub unsafe fn to_unicode_unsafe<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::TextDecoderToUnicodeUnsafeArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::text_decoder::TextDecoder {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTextDecoder_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextDecoder::to_unicode_unsafe](../struct.TextDecoder.html#method.to_unicode_unsafe) method.
  pub trait TextDecoderToUnicodeUnsafeArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self, original_self: &'largs mut ::text_decoder::TextDecoder) -> Self::ReturnType;
  }
  impl<'largs> TextDecoderToUnicodeUnsafeArgs<'largs> for (*mut ::string::String, *const ::libc::c_char, ::libc::c_int) {
    type ReturnType = ();
    unsafe fn exec(self, original_self: &'largs mut ::text_decoder::TextDecoder) -> () {
      let target = self.0;
      let chars = self.1;
      let len = self.2;
      ::ffi::qt_core_c_QTextDecoder_toUnicode(original_self as *mut ::text_decoder::TextDecoder,
                                              target,
                                              chars,
                                              len)
    }
  }
  impl<'largs> TextDecoderToUnicodeUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    type ReturnType = ::string::String;
    unsafe fn exec(self, original_self: &'largs mut ::text_decoder::TextDecoder) -> ::string::String {
      let chars = self.0;
      let len = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextDecoder_toUnicode_to_output_chars_len(original_self as *mut ::text_decoder::TextDecoder, chars, len, &mut object);
        object
      }
    }
  }
}
