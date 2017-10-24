/// C++ type: <span style='color: green;'>```QDateTime```</span>
#[repr(C)]
pub struct DateTime([u8; ::type_sizes::QT_CORE_DATE_TIME_DATE_TIME]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for DateTime {
  unsafe fn new_uninitialized() -> DateTime {
    DateTime(::std::mem::uninitialized())
  }
}

impl DateTime {
  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::addDays(qint64 days) const```</span>
  ///
  ///
  pub fn add_days(&self, days: i64) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_addDays_to_output(self as *const ::date_time::DateTime, days, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::addMSecs(qint64 msecs) const```</span>
  ///
  ///
  pub fn add_m_secs(&self, msecs: i64) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_addMSecs_to_output(self as *const ::date_time::DateTime, msecs, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::addMonths(int months) const```</span>
  ///
  ///
  pub fn add_months(&self, months: ::libc::c_int) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_addMonths_to_output(self as *const ::date_time::DateTime, months, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::addSecs(qint64 secs) const```</span>
  ///
  ///
  pub fn add_secs(&self, secs: i64) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_addSecs_to_output(self as *const ::date_time::DateTime, secs, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::addYears(int years) const```</span>
  ///
  ///
  pub fn add_years(&self, years: ::libc::c_int) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_addYears_to_output(self as *const ::date_time::DateTime, years, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::currentDateTime()```</span>
  ///
  ///
  pub fn current_date_time() -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_currentDateTime_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::currentDateTimeUtc()```</span>
  ///
  ///
  pub fn current_date_time_utc() -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_currentDateTimeUtc_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static qint64 QDateTime::currentMSecsSinceEpoch()```</span>
  ///
  ///
  pub fn current_m_secs_since_epoch() -> i64 {
    unsafe { ::ffi::qt_core_c_QDateTime_currentMSecsSinceEpoch() }
  }

  /// C++ method: <span style='color: green;'>```static qint64 QDateTime::currentSecsSinceEpoch()```</span>
  ///
  ///
  pub fn current_secs_since_epoch() -> i64 {
    unsafe { ::ffi::qt_core_c_QDateTime_currentSecsSinceEpoch() }
  }

  /// C++ method: <span style='color: green;'>```QDate QDateTime::date() const```</span>
  ///
  ///
  pub fn date(&self) -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_date_to_output(self as *const ::date_time::DateTime, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDateTime::daysTo(const QDateTime& arg1) const```</span>
  ///
  ///
  pub fn days_to(&self, arg1: &::date_time::DateTime) -> i64 {
    unsafe {
      ::ffi::qt_core_c_QDateTime_daysTo(self as *const ::date_time::DateTime,
                                        arg1 as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime::fromMSecsSinceEpoch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_m_secs_since_epoch(i64) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_m_secs_since_epoch((i64, &::qt::TimeSpec)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, Qt::TimeSpec spec)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_m_secs_since_epoch((i64, &::qt::TimeSpec, ::libc::c_int)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, Qt::TimeSpec spec, int offsetFromUtc = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn from_m_secs_since_epoch((i64, &::time_zone::TimeZone)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromMSecsSinceEpoch(qint64 msecs, const QTimeZone& timeZone)```</span>
  ///
  ///
  pub fn from_m_secs_since_epoch<Args>(args: Args) -> ::date_time::DateTime
    where Args: overloading::DateTimeFromMSecsSinceEpochArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDateTime::fromSecsSinceEpoch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_secs_since_epoch(i64) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromSecsSinceEpoch(qint64 secs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_secs_since_epoch((i64, &::qt::TimeSpec)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromSecsSinceEpoch(qint64 secs, Qt::TimeSpec spe = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_secs_since_epoch((i64, &::qt::TimeSpec, ::libc::c_int)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromSecsSinceEpoch(qint64 secs, Qt::TimeSpec spe = ?, int offsetFromUtc = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn from_secs_since_epoch((i64, &::time_zone::TimeZone)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromSecsSinceEpoch(qint64 secs, const QTimeZone& timeZone)```</span>
  ///
  ///
  pub fn from_secs_since_epoch<Args>(args: Args) -> ::date_time::DateTime
    where Args: overloading::DateTimeFromSecsSinceEpochArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDateTime::fromString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_string(&::string::String) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromString(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_string((&::string::String, &::qt::DateFormat)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromString(const QString& s, Qt::DateFormat f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_string((&::string::String, &::string::String)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromString(const QString& s, const QString& format)```</span>
  ///
  ///
  pub fn from_string<Args>(args: Args) -> ::date_time::DateTime
    where Args: overloading::DateTimeFromStringArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDateTime::fromTime_t```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_time_t(::libc::c_uint) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromTime_t(unsigned int secsSince1Jan1970UTC)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_time_t((::libc::c_uint, &::qt::TimeSpec)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromTime_t(unsigned int secsSince1Jan1970UTC, Qt::TimeSpec spec)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_time_t((::libc::c_uint, &::qt::TimeSpec, ::libc::c_int)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromTime_t(unsigned int secsSince1Jan1970UTC, Qt::TimeSpec spec, int offsetFromUtc = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn from_time_t((::libc::c_uint, &::time_zone::TimeZone)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```static QDateTime QDateTime::fromTime_t(unsigned int secsSince1Jan1970UTC, const QTimeZone& timeZone)```</span>
  ///
  ///
  pub fn from_time_t<Args>(args: Args) -> ::date_time::DateTime
    where Args: overloading::DateTimeFromTimeTArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QDateTime::isDaylightTime() const```</span>
  ///
  ///
  pub fn is_daylight_time(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDateTime_isDaylightTime(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDateTime_isNull(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDateTime_isValid(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDateTime::msecsTo(const QDateTime& arg1) const```</span>
  ///
  ///
  pub fn msecs_to(&self, arg1: &::date_time::DateTime) -> i64 {
    unsafe {
      ::ffi::qt_core_c_QDateTime_msecsTo(self as *const ::date_time::DateTime,
                                         arg1 as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime::QDateTime```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::date::Date) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime(const QDate& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::date::Date, &::time::Time)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime(const QDate& arg1, const QTime& arg2)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::date::Date, &::time::Time, &::qt::TimeSpec)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime(const QDate& arg1, const QTime& arg2, Qt::TimeSpec spec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::date::Date, &::time::Time, &::qt::TimeSpec, ::libc::c_int)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime(const QDate& date, const QTime& time, Qt::TimeSpec spec, int offsetSeconds)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::date::Date, &::time::Time, &::time_zone::TimeZone)) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime(const QDate& date, const QTime& time, const QTimeZone& timeZone)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::date_time::DateTime) -> ::date_time::DateTime```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTime::QDateTime(const QDateTime& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::date_time::DateTime
    where Args: overloading::DateTimeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QDateTime::offsetFromUtc() const```</span>
  ///
  ///
  pub fn offset_from_utc(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDateTime_offsetFromUtc(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime& QDateTime::operator=(const QDateTime& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::date_time::DateTime) -> &'l0 mut ::date_time::DateTime {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QDateTime_operator_assign(self as *mut ::date_time::DateTime,
                                                 other as *const ::date_time::DateTime)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::operator==(const QDateTime& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDateTime_operator_eq(self as *const ::date_time::DateTime,
                                             other as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::operator>=(const QDateTime& other) const```</span>
  ///
  ///
  pub fn op_ge(&self, other: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDateTime_operator_ge(self as *const ::date_time::DateTime,
                                             other as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::operator>(const QDateTime& other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDateTime_operator_gt(self as *const ::date_time::DateTime,
                                             other as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::operator<=(const QDateTime& other) const```</span>
  ///
  ///
  pub fn op_le(&self, other: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDateTime_operator_le(self as *const ::date_time::DateTime,
                                             other as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::operator<(const QDateTime& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDateTime_operator_lt(self as *const ::date_time::DateTime,
                                             other as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDateTime::operator!=(const QDateTime& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDateTime_operator_neq(self as *const ::date_time::DateTime,
                                              other as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDateTime::secsTo(const QDateTime& arg1) const```</span>
  ///
  ///
  pub fn secs_to(&self, arg1: &::date_time::DateTime) -> i64 {
    unsafe {
      ::ffi::qt_core_c_QDateTime_secsTo(self as *const ::date_time::DateTime,
                                        arg1 as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setDate(const QDate& date)```</span>
  ///
  ///
  pub fn set_date(&mut self, date: &::date::Date) {
    unsafe {
      ::ffi::qt_core_c_QDateTime_setDate(self as *mut ::date_time::DateTime,
                                         date as *const ::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setMSecsSinceEpoch(qint64 msecs)```</span>
  ///
  ///
  pub fn set_m_secs_since_epoch(&mut self, msecs: i64) {
    unsafe { ::ffi::qt_core_c_QDateTime_setMSecsSinceEpoch(self as *mut ::date_time::DateTime, msecs) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setOffsetFromUtc(int offsetSeconds)```</span>
  ///
  ///
  pub fn set_offset_from_utc(&mut self, offset_seconds: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QDateTime_setOffsetFromUtc(self as *mut ::date_time::DateTime, offset_seconds) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setSecsSinceEpoch(qint64 secs)```</span>
  ///
  ///
  pub fn set_secs_since_epoch(&mut self, secs: i64) {
    unsafe { ::ffi::qt_core_c_QDateTime_setSecsSinceEpoch(self as *mut ::date_time::DateTime, secs) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setTime(const QTime& time)```</span>
  ///
  ///
  pub fn set_time(&mut self, time: &::time::Time) {
    unsafe {
      ::ffi::qt_core_c_QDateTime_setTime(self as *mut ::date_time::DateTime,
                                         time as *const ::time::Time)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setTimeSpec(Qt::TimeSpec spec)```</span>
  ///
  ///
  pub fn set_time_spec(&mut self, spec: &::qt::TimeSpec) {
    unsafe {
      ::ffi::qt_core_c_QDateTime_setTimeSpec(self as *mut ::date_time::DateTime,
                                             spec as *const ::qt::TimeSpec)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setTime_t(unsigned int secsSince1Jan1970UTC)```</span>
  ///
  ///
  pub fn set_time_t(&mut self, secs_since1_jan1970_u_t_c: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_core_c_QDateTime_setTime_t(self as *mut ::date_time::DateTime,
                                           secs_since1_jan1970_u_t_c)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setTimeZone(const QTimeZone& toZone)```</span>
  ///
  ///
  pub fn set_time_zone(&mut self, to_zone: &::time_zone::TimeZone) {
    unsafe {
      ::ffi::qt_core_c_QDateTime_setTimeZone(self as *mut ::date_time::DateTime,
                                             to_zone as *const ::time_zone::TimeZone)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::setUtcOffset(int seconds)```</span>
  ///
  ///
  pub fn set_utc_offset(&mut self, seconds: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QDateTime_setUtcOffset(self as *mut ::date_time::DateTime, seconds) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTime::swap(QDateTime& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::date_time::DateTime) {
    unsafe {
      ::ffi::qt_core_c_QDateTime_swap(self as *mut ::date_time::DateTime,
                                      other as *mut ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```QTime QDateTime::time() const```</span>
  ///
  ///
  pub fn time(&self) -> ::time::Time {
    {
      let mut object: ::time::Time = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_time_to_output(self as *const ::date_time::DateTime, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone QDateTime::timeZone() const```</span>
  ///
  ///
  pub fn time_zone(&self) -> ::time_zone::TimeZone {
    {
      let mut object: ::time_zone::TimeZone =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_timeZone_to_output(self as *const ::date_time::DateTime, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDateTime::timeZoneAbbreviation() const```</span>
  ///
  ///
  pub fn time_zone_abbreviation(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_timeZoneAbbreviation_to_output(self as *const ::date_time::DateTime, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::toLocalTime() const```</span>
  ///
  ///
  pub fn to_local_time(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_toLocalTime_to_output(self as *const ::date_time::DateTime, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDateTime::toMSecsSinceEpoch() const```</span>
  ///
  ///
  pub fn to_m_secs_since_epoch(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDateTime_toMSecsSinceEpoch(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::toOffsetFromUtc(int offsetSeconds) const```</span>
  ///
  ///
  pub fn to_offset_from_utc(&self, offset_seconds: ::libc::c_int) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_toOffsetFromUtc_to_output(self as *const ::date_time::DateTime,
                                                             offset_seconds,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDateTime::toSecsSinceEpoch() const```</span>
  ///
  ///
  pub fn to_secs_since_epoch(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDateTime_toSecsSinceEpoch(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QDateTime::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, &::qt::DateFormat) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QDateTime::toString(Qt::DateFormat f = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_string(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QDateTime::toString(const QString& format) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::DateTimeToStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::toTimeSpec(Qt::TimeSpec spec) const```</span>
  ///
  ///
  pub fn to_time_spec(&self, spec: &::qt::TimeSpec) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_toTimeSpec_to_output(self as *const ::date_time::DateTime,
                                                        spec as *const ::qt::TimeSpec,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QDateTime::toTime_t() const```</span>
  ///
  ///
  pub fn to_time_t(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QDateTime_toTime_t(self as *const ::date_time::DateTime) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::toTimeZone(const QTimeZone& toZone) const```</span>
  ///
  ///
  pub fn to_time_zone(&self, to_zone: &::time_zone::TimeZone) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_toTimeZone_to_output(self as *const ::date_time::DateTime,
                                                        to_zone as *const ::time_zone::TimeZone,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTime::toUTC() const```</span>
  ///
  ///
  pub fn to_u_t_c(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDateTime_toUTC_to_output(self as *const ::date_time::DateTime, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QDateTime::utcOffset() const```</span>
  ///
  ///
  pub fn utc_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDateTime_utcOffset(self as *const ::date_time::DateTime) }
  }
}

impl Drop for ::date_time::DateTime {
  /// C++ method: <span style='color: green;'>```[destructor] void QDateTime::~QDateTime()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDateTime_destructor(self as *mut ::date_time::DateTime) }
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::date::Date) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QDate& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::date::Date, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QDate& key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash(&::date_time::DateTime) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QDateTime& key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash((&::date_time::DateTime, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QDateTime& key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn hash(&::time::Time) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QTime& key)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn hash((&::time::Time, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QTime& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::date::Date)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QDate& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::time::Time)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTime& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> ::debug::Debug
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void swap(QDateTime& value1, QDateTime& value2)```</span>
///
///
pub fn swap(value1: &mut ::date_time::DateTime, value2: &mut ::date_time::DateTime) {
  unsafe {
    ::ffi::qt_core_c_QDateTime_G_swap(value1 as *mut ::date_time::DateTime,
                                      value2 as *mut ::date_time::DateTime)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DateTime::from_m_secs_since_epoch](../struct.DateTime.html#method.from_m_secs_since_epoch) method.
  pub trait DateTimeFromMSecsSinceEpochArgs {
    fn exec(self) -> ::date_time::DateTime;
  }
  impl DateTimeFromMSecsSinceEpochArgs for i64 {
    fn exec(self) -> ::date_time::DateTime {
      let msecs = self;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs(msecs, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromMSecsSinceEpochArgs for (i64, &'a ::qt::TimeSpec) {
    fn exec(self) -> ::date_time::DateTime {
      let msecs = self.0;
      let spec = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_spec(msecs,
                                                                              spec as *const ::qt::TimeSpec,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromMSecsSinceEpochArgs for (i64, &'a ::qt::TimeSpec, ::libc::c_int) {
    fn exec(self) -> ::date_time::DateTime {
      let msecs = self.0;
      let spec = self.1;
      let offset_from_utc = self.2;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_spec_offsetFromUtc(msecs, spec as *const ::qt::TimeSpec, offset_from_utc, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromMSecsSinceEpochArgs for (i64, &'a ::time_zone::TimeZone) {
    fn exec(self) -> ::date_time::DateTime {
      let msecs = self.0;
      let time_zone = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_timeZone(msecs, time_zone as *const ::time_zone::TimeZone, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DateTime::from_secs_since_epoch](../struct.DateTime.html#method.from_secs_since_epoch) method.
  pub trait DateTimeFromSecsSinceEpochArgs {
    fn exec(self) -> ::date_time::DateTime;
  }
  impl DateTimeFromSecsSinceEpochArgs for i64 {
    fn exec(self) -> ::date_time::DateTime {
      let secs = self;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs(secs, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromSecsSinceEpochArgs for (i64, &'a ::qt::TimeSpec) {
    fn exec(self) -> ::date_time::DateTime {
      let secs = self.0;
      let spe = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_spe(secs,
                                                                           spe as *const ::qt::TimeSpec,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromSecsSinceEpochArgs for (i64, &'a ::qt::TimeSpec, ::libc::c_int) {
    fn exec(self) -> ::date_time::DateTime {
      let secs = self.0;
      let spe = self.1;
      let offset_from_utc = self.2;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_spe_offsetFromUtc(secs, spe as *const ::qt::TimeSpec, offset_from_utc, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromSecsSinceEpochArgs for (i64, &'a ::time_zone::TimeZone) {
    fn exec(self) -> ::date_time::DateTime {
      let secs = self.0;
      let time_zone = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_timeZone(secs, time_zone as *const ::time_zone::TimeZone, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DateTime::from_string](../struct.DateTime.html#method.from_string) method.
  pub trait DateTimeFromStringArgs {
    fn exec(self) -> ::date_time::DateTime;
  }
  impl<'a> DateTimeFromStringArgs for &'a ::string::String {
    fn exec(self) -> ::date_time::DateTime {
      let s = self;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromString_to_output_s(s as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromStringArgs for (&'a ::string::String, &'a ::qt::DateFormat) {
    fn exec(self) -> ::date_time::DateTime {
      let s = self.0;
      let f = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromString_to_output_s_f(s as *const ::string::String,
                                                              f as *const ::qt::DateFormat,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromStringArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::date_time::DateTime {
      let s = self.0;
      let format = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromString_to_output_s_format(s as *const ::string::String,
                                                                   format as *const ::string::String,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DateTime::from_time_t](../struct.DateTime.html#method.from_time_t) method.
  pub trait DateTimeFromTimeTArgs {
    fn exec(self) -> ::date_time::DateTime;
  }
  impl DateTimeFromTimeTArgs for ::libc::c_uint {
    fn exec(self) -> ::date_time::DateTime {
      let secs_since1_jan1970_u_t_c = self;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC(secs_since1_jan1970_u_t_c, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromTimeTArgs for (::libc::c_uint, &'a ::qt::TimeSpec) {
    fn exec(self) -> ::date_time::DateTime {
      let secs_since1_jan1970_u_t_c = self.0;
      let spec = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_spec(secs_since1_jan1970_u_t_c,
                                                                                    spec as *const ::qt::TimeSpec,
                                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromTimeTArgs for (::libc::c_uint, &'a ::qt::TimeSpec, ::libc::c_int) {
    fn exec(self) -> ::date_time::DateTime {
      let secs_since1_jan1970_u_t_c = self.0;
      let spec = self.1;
      let offset_from_utc = self.2;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_spec_offsetFromUtc(secs_since1_jan1970_u_t_c, spec as *const ::qt::TimeSpec, offset_from_utc, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeFromTimeTArgs for (::libc::c_uint, &'a ::time_zone::TimeZone) {
    fn exec(self) -> ::date_time::DateTime {
      let secs_since1_jan1970_u_t_c = self.0;
      let time_zone = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_timeZone(secs_since1_jan1970_u_t_c, time_zone as *const ::time_zone::TimeZone, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DateTime::new](../struct.DateTime.html#method.new) method.
  pub trait DateTimeNewArgs {
    fn exec(self) -> ::date_time::DateTime;
  }
  impl<'a> DateTimeNewArgs for &'a ::date::Date {
    fn exec(self) -> ::date_time::DateTime {
      let arg1 = self;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_arg1(arg1 as *const ::date::Date, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeNewArgs for (&'a ::date::Date, &'a ::time::Time) {
    fn exec(self) -> ::date_time::DateTime {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_arg1_arg2(arg1 as *const ::date::Date,
                                                           arg2 as *const ::time::Time,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeNewArgs for (&'a ::date::Date, &'a ::time::Time, &'a ::qt::TimeSpec) {
    fn exec(self) -> ::date_time::DateTime {
      let arg1 = self.0;
      let arg2 = self.1;
      let spec = self.2;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_arg1_arg2_spec(arg1 as *const ::date::Date,
                                                                arg2 as *const ::time::Time,
                                                                spec as *const ::qt::TimeSpec,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeNewArgs for (&'a ::date::Date, &'a ::time::Time, &'a ::qt::TimeSpec, ::libc::c_int) {
    fn exec(self) -> ::date_time::DateTime {
      let date = self.0;
      let time = self.1;
      let spec = self.2;
      let offset_seconds = self.3;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_date_time_spec_offsetSeconds(date as *const ::date::Date,
                                                                              time as *const ::time::Time,
                                                                              spec as *const ::qt::TimeSpec,
                                                                              offset_seconds,
                                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeNewArgs for (&'a ::date::Date, &'a ::time::Time, &'a ::time_zone::TimeZone) {
    fn exec(self) -> ::date_time::DateTime {
      let date = self.0;
      let time = self.1;
      let time_zone = self.2;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_date_time_timeZone(date as *const ::date::Date,
                                                                    time as *const ::time::Time,
                                                                    time_zone as *const ::time_zone::TimeZone,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl DateTimeNewArgs for () {
    fn exec(self) -> ::date_time::DateTime {

      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> DateTimeNewArgs for &'a ::date_time::DateTime {
    fn exec(self) -> ::date_time::DateTime {
      let other = self;
      {
        let mut object: ::date_time::DateTime =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_constructor_other(other as *const ::date_time::DateTime, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DateTime::to_string](../struct.DateTime.html#method.to_string) method.
  pub trait DateTimeToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::date_time::DateTime) -> ::string::String;
  }
  impl<'largs> DateTimeToStringArgs<'largs> for &'largs ::qt::DateFormat {
    fn exec(self, original_self: &'largs ::date_time::DateTime) -> ::string::String {
      let f = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_toString_to_output_f(original_self as *const ::date_time::DateTime,
                                                          f as *const ::qt::DateFormat,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DateTimeToStringArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::date_time::DateTime) -> ::string::String {
      let format = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_toString_to_output_format(original_self as *const ::date_time::DateTime,
                                                               format as *const ::string::String,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DateTimeToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::date_time::DateTime) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_toString_to_output_no_args(original_self as *const ::date_time::DateTime,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::date::Date {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QDateTime_G_qHash_QDate(key as *const ::date::Date) }
    }
  }
  impl<'a> HashArgs for &'a ::date_time::DateTime {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QDateTime_G_qHash_QDateTime(key as *const ::date_time::DateTime) }
    }
  }
  impl<'a> HashArgs for (&'a ::date_time::DateTime, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QDateTime_G_qHash_QDateTime_unsigned_int(key as *const ::date_time::DateTime, seed) }
    }
  }
  impl<'a> HashArgs for (&'a ::date::Date, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QDateTime_G_qHash_QDate_unsigned_int(key as *const ::date::Date, seed) }
    }
  }
  impl<'a> HashArgs for &'a ::time::Time {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QDateTime_G_qHash_QTime(key as *const ::time::Time) }
    }
  }
  impl<'a> HashArgs for (&'a ::time::Time, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QDateTime_G_qHash_QTime_unsigned_int(key as *const ::time::Time, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    fn exec(self) -> ::debug::Debug;
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::date::Date) {
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_G_operator_shl_to_output_QDebug_QDate(arg1 as *const ::debug::Debug,
                                                                           arg2 as *const ::date::Date,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::time::Time) {
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDateTime_G_operator_shl_to_output_QDebug_QTime(arg1 as *const ::debug::Debug,
                                                                           arg2 as *const ::time::Time,
                                                                           &mut object);
        }
        object
      }
    }
  }
}
