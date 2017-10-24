/// C++ type: <span style='color: green;'>```QTextCodec::ConversionFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ConversionFlag {
  /// C++ enum variant: <span style='color: green;'>```ConvertInvalidToNull = -2147483648```</span>
  ConvertInvalidToNull = -2147483648,
  /// C++ enum variant: <span style='color: green;'>```DefaultConversion = 0```</span>
  DefaultConversion = 0,
  /// C++ enum variant: <span style='color: green;'>```IgnoreHeader = 1```</span>
  IgnoreHeader = 1,
  /// C++ enum variant: <span style='color: green;'>```FreeFunction = 2```</span>
  FreeFunction = 2,
}

impl ::flags::FlaggableEnum for ConversionFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ConversionFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTextCodec::ConverterState```</span>
#[repr(C)]
pub struct ConverterState(u8);

impl ConverterState {
  /// C++ method: <span style='color: green;'>```void* QTextCodec::ConverterState::d() const```</span>
  ///
  ///
  pub fn d(&self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QTextCodec_ConverterState_d(self as *const ::text_codec::ConverterState) }
  }

  /// C++ method: <span style='color: green;'>```const QFlags<QTextCodec::ConversionFlag>& QTextCodec::ConverterState::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::flags::Flags<::text_codec::ConversionFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTextCodec_ConverterState_flags(self as *const ::text_codec::ConverterState) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```int QTextCodec::ConverterState::invalidChars() const```</span>
  ///
  ///
  pub fn invalid_chars(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextCodec_ConverterState_invalidChars(self as *const ::text_codec::ConverterState) }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec::ConverterState::ConverterState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::text_codec::ConverterState>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCodec::ConverterState::ConverterState()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::flags::Flags<::text_codec::ConversionFlag>) -> ::cpp_utils::CppBox<::text_codec::ConverterState>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCodec::ConverterState::ConverterState(QFlags<QTextCodec::ConversionFlag> f = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::text_codec::ConverterState>
    where Args: overloading::ConverterStateNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QTextCodec::ConverterState::remainingChars() const```</span>
  ///
  ///
  pub fn remaining_chars(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextCodec_ConverterState_remainingChars(self as *const ::text_codec::ConverterState) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCodec::ConverterState::set_d(void* value)```</span>
  ///
  ///
  pub unsafe fn set_d(&mut self, value: *mut ::libc::c_void) {
    ::ffi::qt_core_c_QTextCodec_ConverterState_set_d(self as *mut ::text_codec::ConverterState, value)
  }

  /// C++ method: <span style='color: green;'>```void QTextCodec::ConverterState::set_flags(QFlags<QTextCodec::ConversionFlag> value)```</span>
  ///
  ///
  pub fn set_flags(&mut self, value: ::flags::Flags<::text_codec::ConversionFlag>) {
    unsafe {
      ::ffi::qt_core_c_QTextCodec_ConverterState_set_flags(self as *mut ::text_codec::ConverterState,
                                                           value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCodec::ConverterState::set_invalidChars(int value)```</span>
  ///
  ///
  pub fn set_invalid_chars(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QTextCodec_ConverterState_set_invalidChars(self as *mut ::text_codec::ConverterState, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCodec::ConverterState::set_remainingChars(int value)```</span>
  ///
  ///
  pub fn set_remaining_chars(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QTextCodec_ConverterState_set_remainingChars(self as *mut ::text_codec::ConverterState, value)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::text_codec::ConverterState {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTextCodec_ConverterState_delete
  }
}

/// C++ type: <span style='color: green;'>```QTextCodec```</span>
#[repr(C)]
pub struct TextCodec(u8);

impl TextCodec {
  /// C++ method: <span style='color: green;'>```virtual QList<QByteArray> QTextCodec::aliases() const```</span>
  ///
  ///
  pub fn aliases(&self) -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextCodec_aliases_to_output(self as *const ::text_codec::TextCodec, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTextCodec::availableCodecs()```</span>
  ///
  ///
  pub fn available_codecs() -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextCodec_availableCodecs_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<int> QTextCodec::availableMibs()```</span>
  ///
  ///
  pub fn available_mibs() -> ::list::ListCInt {
    {
      let mut object: ::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextCodec_availableMibs_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec::canEncode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn can_encode(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextCodec::canEncode(QChar arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn can_encode(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextCodec::canEncode(const QString& arg1) const```</span>
  ///
  ///
  pub fn can_encode<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::TextCodecCanEncodeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForHtml(const QByteArray& ba)```</span>
  ///
  ///
  pub fn codec_for_html(ba: &::byte_array::ByteArray) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QTextCodec_codecForHtml_ba(ba as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForHtml(const QByteArray& ba, QTextCodec* defaultCodec)```</span>
  ///
  ///
  pub unsafe fn codec_for_html_unsafe(ba: &::byte_array::ByteArray,
                                      default_codec: *mut ::text_codec::TextCodec)
                                      -> *mut ::text_codec::TextCodec {
    ::ffi::qt_core_c_QTextCodec_codecForHtml_ba_defaultCodec(ba as *const ::byte_array::ByteArray, default_codec)
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForLocale()```</span>
  ///
  ///
  pub fn codec_for_locale() -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QTextCodec_codecForLocale() }
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForMib(int mib)```</span>
  ///
  ///
  pub fn codec_for_mib(mib: ::libc::c_int) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QTextCodec_codecForMib(mib) }
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForName(const QByteArray& name)```</span>
  ///
  ///
  pub fn codec_for_name(name: &::byte_array::ByteArray) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QTextCodec_codecForName_QByteArray(name as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForName(const char* name)```</span>
  ///
  ///
  pub unsafe fn codec_for_name_unsafe(name: *const ::libc::c_char) -> *mut ::text_codec::TextCodec {
    ::ffi::qt_core_c_QTextCodec_codecForName_char(name)
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForUtfText(const QByteArray& ba)```</span>
  ///
  ///
  pub fn codec_for_utf_text(ba: &::byte_array::ByteArray) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QTextCodec_codecForUtfText_ba(ba as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```static QTextCodec* QTextCodec::codecForUtfText(const QByteArray& ba, QTextCodec* defaultCodec)```</span>
  ///
  ///
  pub unsafe fn codec_for_utf_text_unsafe(ba: &::byte_array::ByteArray,
                                          default_codec: *mut ::text_codec::TextCodec)
                                          -> *mut ::text_codec::TextCodec {
    ::ffi::qt_core_c_QTextCodec_codecForUtfText_ba_defaultCodec(ba as *const ::byte_array::ByteArray, default_codec)
  }

  /// C++ method: <span style='color: green;'>```QByteArray QTextCodec::fromUnicode(const QString& uc) const```</span>
  ///
  ///
  pub fn from_unicode(&self, uc: &::string::String) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextCodec_fromUnicode_to_output_uc(self as *const ::text_codec::TextCodec,
                                                             uc as *const ::string::String,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec::fromUnicode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_unicode_unsafe(&self, (*const ::char::Char, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QTextCodec::fromUnicode(const QChar* in, int length) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_unicode_unsafe(&self, (*const ::char::Char, ::libc::c_int, *mut ::text_codec::ConverterState)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QTextCodec::fromUnicode(const QChar* in, int length, QTextCodec::ConverterState* state = ?) const```</span>
  ///
  ///
  pub unsafe fn from_unicode_unsafe<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::TextCodecFromUnicodeUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCodec::makeDecoder```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn make_decoder(&self, ()) -> *mut ::text_decoder::TextDecoder```<br>
  /// C++ method: <span style='color: green;'>```QTextDecoder* QTextCodec::makeDecoder() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn make_decoder(&self, ::flags::Flags<::text_codec::ConversionFlag>) -> *mut ::text_decoder::TextDecoder```<br>
  /// C++ method: <span style='color: green;'>```QTextDecoder* QTextCodec::makeDecoder(QFlags<QTextCodec::ConversionFlag> flags = ?) const```</span>
  ///
  ///
  pub fn make_decoder<'largs, Args>(&'largs self, args: Args) -> *mut ::text_decoder::TextDecoder
    where Args: overloading::TextCodecMakeDecoderArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCodec::makeEncoder```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn make_encoder(&self, ()) -> *mut ::text_encoder::TextEncoder```<br>
  /// C++ method: <span style='color: green;'>```QTextEncoder* QTextCodec::makeEncoder() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn make_encoder(&self, ::flags::Flags<::text_codec::ConversionFlag>) -> *mut ::text_encoder::TextEncoder```<br>
  /// C++ method: <span style='color: green;'>```QTextEncoder* QTextCodec::makeEncoder(QFlags<QTextCodec::ConversionFlag> flags = ?) const```</span>
  ///
  ///
  pub fn make_encoder<'largs, Args>(&'largs self, args: Args) -> *mut ::text_encoder::TextEncoder
    where Args: overloading::TextCodecMakeEncoderArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```pure virtual int QTextCodec::mibEnum() const```</span>
  ///
  ///
  pub fn mib_enum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextCodec_mibEnum(self as *const ::text_codec::TextCodec) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QByteArray QTextCodec::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextCodec_name_to_output(self as *const ::text_codec::TextCodec, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QTextCodec::setCodecForLocale(QTextCodec* c)```</span>
  ///
  ///
  pub unsafe fn set_codec_for_locale(c: *mut ::text_codec::TextCodec) {
    ::ffi::qt_core_c_QTextCodec_setCodecForLocale(c)
  }

  /// C++ method: <span style='color: green;'>```QString QTextCodec::toUnicode(const QByteArray& arg1) const```</span>
  ///
  ///
  pub fn to_unicode(&self, arg1: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextCodec_toUnicode_to_output_arg1(self as *const ::text_codec::TextCodec,
                                                             arg1 as *const ::byte_array::ByteArray,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec::toUnicode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_unicode_unsafe(&self, *const ::libc::c_char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextCodec::toUnicode(const char* chars) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_unicode_unsafe(&self, (*const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextCodec::toUnicode(const char* in, int length) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_unicode_unsafe(&self, (*const ::libc::c_char, ::libc::c_int, *mut ::text_codec::ConverterState)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextCodec::toUnicode(const char* in, int length, QTextCodec::ConverterState* state = ?) const```</span>
  ///
  ///
  pub unsafe fn to_unicode_unsafe<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::TextCodecToUnicodeUnsafeArgs<'largs>
  {
    args.exec(self)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ConverterState::new](../struct.ConverterState.html#method.new) method.
  pub trait ConverterStateNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::text_codec::ConverterState>;
  }
  impl ConverterStateNewArgs for ::flags::Flags<::text_codec::ConversionFlag> {
    fn exec(self) -> ::cpp_utils::CppBox<::text_codec::ConverterState> {
      let f = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QTextCodec_ConverterState_new_f(f.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ConverterStateNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::text_codec::ConverterState> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QTextCodec_ConverterState_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCodec::can_encode](../struct.TextCodec.html#method.can_encode) method.
  pub trait TextCodecCanEncodeArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> bool;
  }
  impl<'largs> TextCodecCanEncodeArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> bool {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QTextCodec_canEncode_QChar(original_self as *const ::text_codec::TextCodec,
                                                    arg1 as *const ::char::Char)
      }
    }
  }
  impl<'largs> TextCodecCanEncodeArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> bool {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QTextCodec_canEncode_QString(original_self as *const ::text_codec::TextCodec,
                                                      arg1 as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCodec::from_unicode_unsafe](../struct.TextCodec.html#method.from_unicode_unsafe) method.
  pub trait TextCodecFromUnicodeUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::byte_array::ByteArray;
  }
  impl<'largs> TextCodecFromUnicodeUnsafeArgs<'largs> for (*const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::byte_array::ByteArray {
      let in_ = self.0;
      let length = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextCodec_fromUnicode_to_output_in_length(original_self as *const ::text_codec::TextCodec,
                                                                    in_,
                                                                    length,
                                                                    &mut object);
        object
      }
    }
  }
  impl<'largs> TextCodecFromUnicodeUnsafeArgs<'largs>
    for (*const ::char::Char, ::libc::c_int, *mut ::text_codec::ConverterState) {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::byte_array::ByteArray {
      let in_ = self.0;
      let length = self.1;
      let state = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextCodec_fromUnicode_to_output_in_length_state(original_self as *const ::text_codec::TextCodec, in_, length, state, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCodec::make_decoder](../struct.TextCodec.html#method.make_decoder) method.
  pub trait TextCodecMakeDecoderArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> *mut ::text_decoder::TextDecoder;
  }
  impl<'largs> TextCodecMakeDecoderArgs<'largs> for ::flags::Flags<::text_codec::ConversionFlag> {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> *mut ::text_decoder::TextDecoder {
      let flags = self;
      unsafe {
        ::ffi::qt_core_c_QTextCodec_makeDecoder_flags(original_self as *const ::text_codec::TextCodec,
                                                      flags.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> TextCodecMakeDecoderArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> *mut ::text_decoder::TextDecoder {

      unsafe { ::ffi::qt_core_c_QTextCodec_makeDecoder_no_args(original_self as *const ::text_codec::TextCodec) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCodec::make_encoder](../struct.TextCodec.html#method.make_encoder) method.
  pub trait TextCodecMakeEncoderArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> *mut ::text_encoder::TextEncoder;
  }
  impl<'largs> TextCodecMakeEncoderArgs<'largs> for ::flags::Flags<::text_codec::ConversionFlag> {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> *mut ::text_encoder::TextEncoder {
      let flags = self;
      unsafe {
        ::ffi::qt_core_c_QTextCodec_makeEncoder_flags(original_self as *const ::text_codec::TextCodec,
                                                      flags.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> TextCodecMakeEncoderArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> *mut ::text_encoder::TextEncoder {

      unsafe { ::ffi::qt_core_c_QTextCodec_makeEncoder_no_args(original_self as *const ::text_codec::TextCodec) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCodec::to_unicode_unsafe](../struct.TextCodec.html#method.to_unicode_unsafe) method.
  pub trait TextCodecToUnicodeUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::string::String;
  }
  impl<'largs> TextCodecToUnicodeUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::string::String {
      let chars = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextCodec_toUnicode_to_output_chars(original_self as *const ::text_codec::TextCodec,
                                                              chars,
                                                              &mut object);
        object
      }
    }
  }
  impl<'largs> TextCodecToUnicodeUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::string::String {
      let in_ = self.0;
      let length = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextCodec_toUnicode_to_output_in_length(original_self as *const ::text_codec::TextCodec,
                                                                  in_,
                                                                  length,
                                                                  &mut object);
        object
      }
    }
  }
  impl<'largs> TextCodecToUnicodeUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_int, *mut ::text_codec::ConverterState) {
    unsafe fn exec(self, original_self: &'largs ::text_codec::TextCodec) -> ::string::String {
      let in_ = self.0;
      let length = self.1;
      let state = self.2;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextCodec_toUnicode_to_output_in_length_state(original_self as *const ::text_codec::TextCodec, in_, length, state, &mut object);
        object
      }
    }
  }
}
