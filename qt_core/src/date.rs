/// C++ type: <span style='color: green;'>```QDate```</span>
#[repr(C)]
pub struct Date([u8; ::type_sizes::QT_CORE_DATE_DATE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Date {
  unsafe fn new_uninitialized() -> Date {
    Date(::std::mem::uninitialized())
  }
}

impl Date {
  /// C++ method: <span style='color: green;'>```QDate QDate::addDays(qint64 days) const```</span>
  ///
  ///
  pub fn add_days(&self, days: i64) -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDate_addDays_to_output(self as *const ::date::Date, days, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDate QDate::addMonths(int months) const```</span>
  ///
  ///
  pub fn add_months(&self, months: ::libc::c_int) -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDate_addMonths_to_output(self as *const ::date::Date, months, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDate QDate::addYears(int years) const```</span>
  ///
  ///
  pub fn add_years(&self, years: ::libc::c_int) -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDate_addYears_to_output(self as *const ::date::Date, years, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QDate QDate::currentDate()```</span>
  ///
  ///
  pub fn current_date() -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDate_currentDate_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QDate::day() const```</span>
  ///
  ///
  pub fn day(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_day(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```int QDate::dayOfWeek() const```</span>
  ///
  ///
  pub fn day_of_week(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_dayOfWeek(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```int QDate::dayOfYear() const```</span>
  ///
  ///
  pub fn day_of_year(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_dayOfYear(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```int QDate::daysInMonth() const```</span>
  ///
  ///
  pub fn days_in_month(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_daysInMonth(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```int QDate::daysInYear() const```</span>
  ///
  ///
  pub fn days_in_year(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_daysInYear(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDate::daysTo(const QDate& arg1) const```</span>
  ///
  ///
  pub fn days_to(&self, arg1: &::date::Date) -> i64 {
    unsafe { ::ffi::qt_core_c_QDate_daysTo(self as *const ::date::Date, arg1 as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```static QDate QDate::fromJulianDay(qint64 jd_)```</span>
  ///
  ///
  pub fn from_julian_day(jd: i64) -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDate_fromJulianDay_to_output(jd, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDate::fromString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_string(&::string::String) -> ::date::Date```<br>
  /// C++ method: <span style='color: green;'>```static QDate QDate::fromString(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_string((&::string::String, &::qt::DateFormat)) -> ::date::Date```<br>
  /// C++ method: <span style='color: green;'>```static QDate QDate::fromString(const QString& s, Qt::DateFormat f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_string((&::string::String, &::string::String)) -> ::date::Date```<br>
  /// C++ method: <span style='color: green;'>```static QDate QDate::fromString(const QString& s, const QString& format)```</span>
  ///
  ///
  pub fn from_string<Args>(args: Args) -> ::date::Date
    where Args: overloading::DateFromStringArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QDate::getDate(int* year, int* month, int* day) const```</span>
  ///
  ///
  pub unsafe fn get_date(&self, year: *mut ::libc::c_int, month: *mut ::libc::c_int, day: *mut ::libc::c_int) {
    ::ffi::qt_core_c_QDate_getDate_const(self as *const ::date::Date, year, month, day)
  }

  /// C++ method: <span style='color: green;'>```void QDate::getDate(int* year, int* month, int* day)```</span>
  ///
  ///
  pub unsafe fn get_date_mut(&mut self, year: *mut ::libc::c_int, month: *mut ::libc::c_int, day: *mut ::libc::c_int) {
    ::ffi::qt_core_c_QDate_getDate(self as *mut ::date::Date, year, month, day)
  }

  /// C++ method: <span style='color: green;'>```static bool QDate::isLeapYear(int year)```</span>
  ///
  ///
  pub fn is_leap_year(year: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_isLeapYear(year) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_isNull(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_isValid_no_args(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```static bool QDate::isValid(int y, int m, int d)```</span>
  ///
  ///
  pub fn is_valid_static(y: ::libc::c_int, m: ::libc::c_int, d: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_isValid_y_m_d(y, m, d) }
  }

  /// C++ method: <span style='color: green;'>```QDate::longDayName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn long_day_name(::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::longDayName(int weekday)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn long_day_name((::libc::c_int, ::date::MonthNameType)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::longDayName(int weekday, QDate::MonthNameType type = ?)```</span>
  ///
  ///
  pub fn long_day_name<Args>(args: Args) -> ::string::String
    where Args: overloading::DateLongDayNameArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDate::longMonthName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn long_month_name(::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::longMonthName(int month)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn long_month_name((::libc::c_int, ::date::MonthNameType)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::longMonthName(int month, QDate::MonthNameType type = ?)```</span>
  ///
  ///
  pub fn long_month_name<Args>(args: Args) -> ::string::String
    where Args: overloading::DateLongMonthNameArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QDate::month() const```</span>
  ///
  ///
  pub fn month(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_month(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```QDate::QDate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::date::Date```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDate::QDate()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::date::Date```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDate::QDate(int y, int m, int d)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::date::Date
    where Args: overloading::DateNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QDate::operator==(const QDate& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::date::Date) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_operator_eq(self as *const ::date::Date, other as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::operator>=(const QDate& other) const```</span>
  ///
  ///
  pub fn op_ge(&self, other: &::date::Date) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_operator_ge(self as *const ::date::Date, other as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::operator>(const QDate& other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::date::Date) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_operator_gt(self as *const ::date::Date, other as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::operator<=(const QDate& other) const```</span>
  ///
  ///
  pub fn op_le(&self, other: &::date::Date) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_operator_le(self as *const ::date::Date, other as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::operator<(const QDate& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::date::Date) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_operator_lt(self as *const ::date::Date, other as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::operator!=(const QDate& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::date::Date) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_operator_neq(self as *const ::date::Date, other as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```bool QDate::setDate(int year, int month, int day)```</span>
  ///
  ///
  pub fn set_date(&mut self, year: ::libc::c_int, month: ::libc::c_int, day: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QDate_setDate(self as *mut ::date::Date, year, month, day) }
  }

  /// C++ method: <span style='color: green;'>```QDate::shortDayName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn short_day_name(::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::shortDayName(int weekday)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn short_day_name((::libc::c_int, ::date::MonthNameType)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::shortDayName(int weekday, QDate::MonthNameType type = ?)```</span>
  ///
  ///
  pub fn short_day_name<Args>(args: Args) -> ::string::String
    where Args: overloading::DateShortDayNameArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDate::shortMonthName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn short_month_name(::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::shortMonthName(int month)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn short_month_name((::libc::c_int, ::date::MonthNameType)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QDate::shortMonthName(int month, QDate::MonthNameType type = ?)```</span>
  ///
  ///
  pub fn short_month_name<Args>(args: Args) -> ::string::String
    where Args: overloading::DateShortMonthNameArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```qint64 QDate::toJulianDay() const```</span>
  ///
  ///
  pub fn to_julian_day(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDate_toJulianDay(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```QDate::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QDate::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, &::qt::DateFormat) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QDate::toString(Qt::DateFormat f = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_string(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QDate::toString(const QString& format) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::DateToStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QDate::weekNumber() const```</span>
  ///
  ///
  pub fn week_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_weekNumber_no_args(self as *const ::date::Date) }
  }

  /// C++ method: <span style='color: green;'>```int QDate::weekNumber(int* yearNum = ?) const```</span>
  ///
  ///
  pub unsafe fn week_number_unsafe(&self, year_num: *mut ::libc::c_int) -> ::libc::c_int {
    ::ffi::qt_core_c_QDate_weekNumber_yearNum(self as *const ::date::Date, year_num)
  }

  /// C++ method: <span style='color: green;'>```int QDate::year() const```</span>
  ///
  ///
  pub fn year(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDate_year(self as *const ::date::Date) }
  }
}

impl Drop for ::date::Date {
  /// C++ method: <span style='color: green;'>```[destructor] void QDate::~QDate()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDate_destructor(self as *mut ::date::Date) }
  }
}

/// C++ type: <span style='color: green;'>```QDate::MonthNameType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MonthNameType {
  /// C++ enum variant: <span style='color: green;'>```DateFormat = 0```</span>
  Date = 0,
  /// C++ enum variant: <span style='color: green;'>```StandaloneFormat = 1```</span>
  Standalone = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Date::from_string](../struct.Date.html#method.from_string) method.
  pub trait DateFromStringArgs {
    fn exec(self) -> ::date::Date;
  }
  impl<'a> DateFromStringArgs for &'a ::string::String {
    fn exec(self) -> ::date::Date {
      let s = self;
      {
        let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_fromString_to_output_s(s as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateFromStringArgs for (&'a ::string::String, &'a ::qt::DateFormat) {
    fn exec(self) -> ::date::Date {
      let s = self.0;
      let f = self.1;
      {
        let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_fromString_to_output_s_f(s as *const ::string::String,
                                                          f as *const ::qt::DateFormat,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateFromStringArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::date::Date {
      let s = self.0;
      let format = self.1;
      {
        let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_fromString_to_output_s_format(s as *const ::string::String,
                                                               format as *const ::string::String,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Date::long_day_name](../struct.Date.html#method.long_day_name) method.
  pub trait DateLongDayNameArgs {
    fn exec(self) -> ::string::String;
  }
  impl DateLongDayNameArgs for ::libc::c_int {
    fn exec(self) -> ::string::String {
      let weekday = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_longDayName_to_output_weekday(weekday, &mut object);
        }
        object
      }
    }
  }
  impl DateLongDayNameArgs for (::libc::c_int, ::date::MonthNameType) {
    fn exec(self) -> ::string::String {
      let weekday = self.0;
      let type_ = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_longDayName_to_output_weekday_type(weekday, type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Date::long_month_name](../struct.Date.html#method.long_month_name) method.
  pub trait DateLongMonthNameArgs {
    fn exec(self) -> ::string::String;
  }
  impl DateLongMonthNameArgs for ::libc::c_int {
    fn exec(self) -> ::string::String {
      let month = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_longMonthName_to_output_month(month, &mut object);
        }
        object
      }
    }
  }
  impl DateLongMonthNameArgs for (::libc::c_int, ::date::MonthNameType) {
    fn exec(self) -> ::string::String {
      let month = self.0;
      let type_ = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_longMonthName_to_output_month_type(month, type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Date::new](../struct.Date.html#method.new) method.
  pub trait DateNewArgs {
    fn exec(self) -> ::date::Date;
  }
  impl DateNewArgs for () {
    fn exec(self) -> ::date::Date {

      {
        let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl DateNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::date::Date {
      let y = self.0;
      let m = self.1;
      let d = self.2;
      {
        let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_constructor_y_m_d(y, m, d, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Date::short_day_name](../struct.Date.html#method.short_day_name) method.
  pub trait DateShortDayNameArgs {
    fn exec(self) -> ::string::String;
  }
  impl DateShortDayNameArgs for ::libc::c_int {
    fn exec(self) -> ::string::String {
      let weekday = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_shortDayName_to_output_weekday(weekday, &mut object);
        }
        object
      }
    }
  }
  impl DateShortDayNameArgs for (::libc::c_int, ::date::MonthNameType) {
    fn exec(self) -> ::string::String {
      let weekday = self.0;
      let type_ = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_shortDayName_to_output_weekday_type(weekday, type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Date::short_month_name](../struct.Date.html#method.short_month_name) method.
  pub trait DateShortMonthNameArgs {
    fn exec(self) -> ::string::String;
  }
  impl DateShortMonthNameArgs for ::libc::c_int {
    fn exec(self) -> ::string::String {
      let month = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_shortMonthName_to_output_month(month, &mut object);
        }
        object
      }
    }
  }
  impl DateShortMonthNameArgs for (::libc::c_int, ::date::MonthNameType) {
    fn exec(self) -> ::string::String {
      let month = self.0;
      let type_ = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_shortMonthName_to_output_month_type(month, type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Date::to_string](../struct.Date.html#method.to_string) method.
  pub trait DateToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::date::Date) -> ::string::String;
  }
  impl<'largs> DateToStringArgs<'largs> for &'largs ::qt::DateFormat {
    fn exec(self, original_self: &'largs ::date::Date) -> ::string::String {
      let f = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_toString_to_output_f(original_self as *const ::date::Date,
                                                      f as *const ::qt::DateFormat,
                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DateToStringArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::date::Date) -> ::string::String {
      let format = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_toString_to_output_format(original_self as *const ::date::Date,
                                                           format as *const ::string::String,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DateToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::date::Date) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDate_toString_to_output_no_args(original_self as *const ::date::Date, &mut object);
        }
        object
      }
    }
  }
}
