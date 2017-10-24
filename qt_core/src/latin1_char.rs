/// C++ type: <span style='color: green;'>```QLatin1Char```</span>
#[repr(C)]
pub struct Latin1Char([u8; ::type_sizes::QT_CORE_LATIN_1_CHAR_LATIN_1_CHAR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Latin1Char {
  unsafe fn new_uninitialized() -> Latin1Char {
    Latin1Char(::std::mem::uninitialized())
  }
}

impl Latin1Char {
  /// C++ method: <span style='color: green;'>```[constructor] void QLatin1Char::QLatin1Char(char c)```</span>
  ///
  ///
  pub fn new(c: ::libc::c_char) -> ::latin1_char::Latin1Char {
    {
      let mut object: ::latin1_char::Latin1Char =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLatin1Char_constructor(c, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```char QLatin1Char::toLatin1() const```</span>
  ///
  ///
  pub fn to_latin1(&self) -> ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QLatin1Char_toLatin1(self as *const ::latin1_char::Latin1Char) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short QLatin1Char::unicode() const```</span>
  ///
  ///
  pub fn unicode(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QLatin1Char_unicode(self as *const ::latin1_char::Latin1Char) }
  }
}

impl Drop for ::latin1_char::Latin1Char {
  /// C++ method: <span style='color: green;'>```[destructor] void QLatin1Char::~QLatin1Char()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QLatin1Char_destructor(self as *mut ::latin1_char::Latin1Char) }
  }
}
