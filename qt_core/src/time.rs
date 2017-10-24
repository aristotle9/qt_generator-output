/// C++ type: <span style='color: green;'>```QTime```</span>
#[repr(C)]
pub struct Time([u8; ::type_sizes::QT_CORE_TIME_TIME]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Time {
  unsafe fn new_uninitialized() -> Time {
    Time(::std::mem::uninitialized())
  }
}

impl Time {
  /// C++ method: <span style='color: green;'>```QTime QTime::addMSecs(int ms) const```</span>
  ///
  ///
  pub fn add_m_secs(&self, ms: ::libc::c_int) -> ::time::Time {
    {
      let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTime_addMSecs_to_output(self as *const ::time::Time, ms, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTime QTime::addSecs(int secs) const```</span>
  ///
  ///
  pub fn add_secs(&self, secs: ::libc::c_int) -> ::time::Time {
    {
      let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTime_addSecs_to_output(self as *const ::time::Time, secs, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QTime QTime::currentTime()```</span>
  ///
  ///
  pub fn current_time() -> ::time::Time {
    {
      let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTime_currentTime_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTime::elapsed() const```</span>
  ///
  ///
  pub fn elapsed(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_elapsed(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```static QTime QTime::fromMSecsSinceStartOfDay(int msecs)```</span>
  ///
  ///
  pub fn from_m_secs_since_start_of_day(msecs: ::libc::c_int) -> ::time::Time {
    {
      let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTime_fromMSecsSinceStartOfDay_to_output(msecs, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTime::fromString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_string(&::string::String) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```static QTime QTime::fromString(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_string((&::string::String, &::qt::DateFormat)) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```static QTime QTime::fromString(const QString& s, Qt::DateFormat f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_string((&::string::String, &::string::String)) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```static QTime QTime::fromString(const QString& s, const QString& format)```</span>
  ///
  ///
  pub fn from_string<Args>(args: Args) -> ::time::Time
    where Args: overloading::TimeFromStringArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QTime::hour() const```</span>
  ///
  ///
  pub fn hour(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_hour(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_isNull(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_isValid_no_args(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```QTime::isValid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_valid_static((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QTime::isValid(int h, int m, int s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_valid_static((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QTime::isValid(int h, int m, int s, int ms = ?)```</span>
  ///
  ///
  pub fn is_valid_static<Args>(args: Args) -> bool
    where Args: overloading::TimeIsValidStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QTime::minute() const```</span>
  ///
  ///
  pub fn minute(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_minute(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```int QTime::msec() const```</span>
  ///
  ///
  pub fn msec(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_msec(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```int QTime::msecsSinceStartOfDay() const```</span>
  ///
  ///
  pub fn msecs_since_start_of_day(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_msecsSinceStartOfDay(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```int QTime::msecsTo(const QTime& arg1) const```</span>
  ///
  ///
  pub fn msecs_to(&self, arg1: &::time::Time) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_msecsTo(self as *const ::time::Time, arg1 as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```QTime::QTime```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTime::QTime()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTime::QTime(int h, int m)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTime::QTime(int h, int m, int s = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::time::Time```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTime::QTime(int h, int m, int s = ?, int ms = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::time::Time
    where Args: overloading::TimeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QTime::operator==(const QTime& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::time::Time) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_operator_eq(self as *const ::time::Time, other as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::operator>=(const QTime& other) const```</span>
  ///
  ///
  pub fn op_ge(&self, other: &::time::Time) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_operator_ge(self as *const ::time::Time, other as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::operator>(const QTime& other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::time::Time) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_operator_gt(self as *const ::time::Time, other as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::operator<=(const QTime& other) const```</span>
  ///
  ///
  pub fn op_le(&self, other: &::time::Time) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_operator_le(self as *const ::time::Time, other as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::operator<(const QTime& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::time::Time) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_operator_lt(self as *const ::time::Time, other as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```bool QTime::operator!=(const QTime& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::time::Time) -> bool {
    unsafe { ::ffi::qt_core_c_QTime_operator_neq(self as *const ::time::Time, other as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```int QTime::restart()```</span>
  ///
  ///
  pub fn restart(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_restart(self as *mut ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```int QTime::second() const```</span>
  ///
  ///
  pub fn second(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_second(self as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```int QTime::secsTo(const QTime& arg1) const```</span>
  ///
  ///
  pub fn secs_to(&self, arg1: &::time::Time) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTime_secsTo(self as *const ::time::Time, arg1 as *const ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```QTime::setHMS```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_h_m_s(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTime::setHMS(int h, int m, int s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_h_m_s(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTime::setHMS(int h, int m, int s, int ms = ?)```</span>
  ///
  ///
  pub fn set_h_m_s<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::TimeSetHMSArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTime::start()```</span>
  ///
  ///
  pub fn start(&mut self) {
    unsafe { ::ffi::qt_core_c_QTime_start(self as *mut ::time::Time) }
  }

  /// C++ method: <span style='color: green;'>```QTime::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTime::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, &::qt::DateFormat) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTime::toString(Qt::DateFormat f = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_string(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTime::toString(const QString& format) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::TimeToStringArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::time::Time {
  /// C++ method: <span style='color: green;'>```[destructor] void QTime::~QTime()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTime_destructor(self as *mut ::time::Time) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Time::from_string](../struct.Time.html#method.from_string) method.
  pub trait TimeFromStringArgs {
    fn exec(self) -> ::time::Time;
  }
  impl<'a> TimeFromStringArgs for &'a ::string::String {
    fn exec(self) -> ::time::Time {
      let s = self;
      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_fromString_to_output_s(s as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeFromStringArgs for (&'a ::string::String, &'a ::qt::DateFormat) {
    fn exec(self) -> ::time::Time {
      let s = self.0;
      let f = self.1;
      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_fromString_to_output_s_f(s as *const ::string::String,
                                                          f as *const ::qt::DateFormat,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeFromStringArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::time::Time {
      let s = self.0;
      let format = self.1;
      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_fromString_to_output_s_format(s as *const ::string::String,
                                                               format as *const ::string::String,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Time::is_valid_static](../struct.Time.html#method.is_valid_static) method.
  pub trait TimeIsValidStaticArgs {
    fn exec(self) -> bool;
  }
  impl TimeIsValidStaticArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> bool {
      let h = self.0;
      let m = self.1;
      let s = self.2;
      unsafe { ::ffi::qt_core_c_QTime_isValid_h_m_s(h, m, s) }
    }
  }
  impl TimeIsValidStaticArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> bool {
      let h = self.0;
      let m = self.1;
      let s = self.2;
      let ms = self.3;
      unsafe { ::ffi::qt_core_c_QTime_isValid_h_m_s_ms(h, m, s, ms) }
    }
  }
  /// This trait represents a set of arguments accepted by [Time::new](../struct.Time.html#method.new) method.
  pub trait TimeNewArgs {
    fn exec(self) -> ::time::Time;
  }
  impl TimeNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::time::Time {
      let h = self.0;
      let m = self.1;
      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_constructor_h_m(h, m, &mut object);
        }
        object
      }
    }
  }
  impl TimeNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::time::Time {
      let h = self.0;
      let m = self.1;
      let s = self.2;
      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_constructor_h_m_s(h, m, s, &mut object);
        }
        object
      }
    }
  }
  impl TimeNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::time::Time {
      let h = self.0;
      let m = self.1;
      let s = self.2;
      let ms = self.3;
      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_constructor_h_m_s_ms(h, m, s, ms, &mut object);
        }
        object
      }
    }
  }
  impl TimeNewArgs for () {
    fn exec(self) -> ::time::Time {

      {
        let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Time::set_h_m_s](../struct.Time.html#method.set_h_m_s) method.
  pub trait TimeSetHMSArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::time::Time) -> bool;
  }
  impl<'largs> TimeSetHMSArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::time::Time) -> bool {
      let h = self.0;
      let m = self.1;
      let s = self.2;
      unsafe { ::ffi::qt_core_c_QTime_setHMS_h_m_s(original_self as *mut ::time::Time, h, m, s) }
    }
  }
  impl<'largs> TimeSetHMSArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::time::Time) -> bool {
      let h = self.0;
      let m = self.1;
      let s = self.2;
      let ms = self.3;
      unsafe { ::ffi::qt_core_c_QTime_setHMS_h_m_s_ms(original_self as *mut ::time::Time, h, m, s, ms) }
    }
  }
  /// This trait represents a set of arguments accepted by [Time::to_string](../struct.Time.html#method.to_string) method.
  pub trait TimeToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::time::Time) -> ::string::String;
  }
  impl<'largs> TimeToStringArgs<'largs> for &'largs ::qt::DateFormat {
    fn exec(self, original_self: &'largs ::time::Time) -> ::string::String {
      let f = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_toString_to_output_f(original_self as *const ::time::Time,
                                                      f as *const ::qt::DateFormat,
                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeToStringArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::time::Time) -> ::string::String {
      let format = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_toString_to_output_format(original_self as *const ::time::Time,
                                                           format as *const ::string::String,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::time::Time) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTime_toString_to_output_no_args(original_self as *const ::time::Time, &mut object);
        }
        object
      }
    }
  }
}
