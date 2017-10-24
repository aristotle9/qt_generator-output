/// C++ type: <span style='color: green;'>```QTimeZone::NameType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum NameType {
  /// C++ enum variant: <span style='color: green;'>```DefaultName = 0```</span>
  Default = 0,
  /// C++ enum variant: <span style='color: green;'>```LongName = 1```</span>
  Long = 1,
  /// C++ enum variant: <span style='color: green;'>```ShortName = 2```</span>
  Short = 2,
  /// C++ enum variant: <span style='color: green;'>```OffsetName = 3```</span>
  Offset = 3,
}

/// C++ type: <span style='color: green;'>```QTimeZone::OffsetData```</span>
#[repr(C)]
pub struct OffsetData([u8; ::type_sizes::QT_CORE_TIME_ZONE_OFFSET_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OffsetData {
  unsafe fn new_uninitialized() -> OffsetData {
    OffsetData(::std::mem::uninitialized())
  }
}

impl OffsetData {
  /// C++ method: <span style='color: green;'>```const QString& QTimeZone::OffsetData::abbreviation() const```</span>
  ///
  ///
  pub fn abbreviation<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_abbreviation(self as *const ::time_zone::OffsetData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QTimeZone::OffsetData::abbreviation_mut()```</span>
  ///
  ///
  pub fn abbreviation_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_abbreviation_mut(self as *mut ::time_zone::OffsetData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QDateTime& QTimeZone::OffsetData::atUtc() const```</span>
  ///
  ///
  pub fn at_utc<'l0>(&'l0 self) -> &'l0 ::date_time::DateTime {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_atUtc(self as *const ::time_zone::OffsetData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDateTime& QTimeZone::OffsetData::atUtc_mut()```</span>
  ///
  ///
  pub fn at_utc_mut<'l0>(&'l0 mut self) -> &'l0 mut ::date_time::DateTime {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_atUtc_mut(self as *mut ::time_zone::OffsetData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QTimeZone::OffsetData::daylightTimeOffset() const```</span>
  ///
  ///
  pub fn daylight_time_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_daylightTimeOffset(self as *const ::time_zone::OffsetData) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeZone::OffsetData::offsetFromUtc() const```</span>
  ///
  ///
  pub fn offset_from_utc(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_offsetFromUtc(self as *const ::time_zone::OffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeZone::OffsetData::set_abbreviation(QString value)```</span>
  ///
  ///
  pub fn set_abbreviation(&mut self, value: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_OffsetData_set_abbreviation(self as *mut ::time_zone::OffsetData,
                                                             value as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTimeZone::OffsetData::set_atUtc(QDateTime value)```</span>
  ///
  ///
  pub fn set_at_utc(&mut self, value: &::date_time::DateTime) {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_OffsetData_set_atUtc(self as *mut ::time_zone::OffsetData,
                                                      value as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTimeZone::OffsetData::set_daylightTimeOffset(int value)```</span>
  ///
  ///
  pub fn set_daylight_time_offset(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_set_daylightTimeOffset(self as *mut ::time_zone::OffsetData, value) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeZone::OffsetData::set_offsetFromUtc(int value)```</span>
  ///
  ///
  pub fn set_offset_from_utc(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_set_offsetFromUtc(self as *mut ::time_zone::OffsetData, value) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeZone::OffsetData::set_standardTimeOffset(int value)```</span>
  ///
  ///
  pub fn set_standard_time_offset(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_set_standardTimeOffset(self as *mut ::time_zone::OffsetData, value) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeZone::OffsetData::standardTimeOffset() const```</span>
  ///
  ///
  pub fn standard_time_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_standardTimeOffset(self as *const ::time_zone::OffsetData) }
  }
}

impl Drop for ::time_zone::OffsetData {
  /// C++ method: <span style='color: green;'>```[destructor] void QTimeZone::OffsetData::~QTimeZone::OffsetData()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimeZone_OffsetData_destructor(self as *mut ::time_zone::OffsetData) }
  }
}

/// C++ type: <span style='color: green;'>```QTimeZone::TimeType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TimeType {
  /// C++ enum variant: <span style='color: green;'>```StandardTime = 0```</span>
  Standard = 0,
  /// C++ enum variant: <span style='color: green;'>```DaylightTime = 1```</span>
  Daylight = 1,
  /// C++ enum variant: <span style='color: green;'>```GenericTime = 2```</span>
  Generic = 2,
}

/// C++ type: <span style='color: green;'>```QTimeZone```</span>
#[repr(C)]
pub struct TimeZone([u8; ::type_sizes::QT_CORE_TIME_ZONE_TIME_ZONE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TimeZone {
  unsafe fn new_uninitialized() -> TimeZone {
    TimeZone(::std::mem::uninitialized())
  }
}

impl TimeZone {
  /// C++ method: <span style='color: green;'>```QString QTimeZone::abbreviation(const QDateTime& atDateTime) const```</span>
  ///
  ///
  pub fn abbreviation(&self, at_date_time: &::date_time::DateTime) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_abbreviation_to_output(self as *const ::time_zone::TimeZone,
                                                          at_date_time as *const ::date_time::DateTime,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::availableTimeZoneIds```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn available_time_zone_ids(()) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTimeZone::availableTimeZoneIds()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn available_time_zone_ids(&::locale::Country) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTimeZone::availableTimeZoneIds(QLocale::Country country)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn available_time_zone_ids(::libc::c_int) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTimeZone::availableTimeZoneIds(int offsetSeconds)```</span>
  ///
  ///
  pub fn available_time_zone_ids<Args>(args: Args) -> ::list::ListByteArray
    where Args: overloading::TimeZoneAvailableTimeZoneIdsArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QTimeZone::comment() const```</span>
  ///
  ///
  pub fn comment(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_comment_to_output(self as *const ::time_zone::TimeZone, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTimeZone::daylightTimeOffset(const QDateTime& atDateTime) const```</span>
  ///
  ///
  pub fn daylight_time_offset(&self, at_date_time: &::date_time::DateTime) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_daylightTimeOffset(self as *const ::time_zone::TimeZone,
                                                    at_date_time as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::displayName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn display_name(&self, &::time_zone::TimeType) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTimeZone::displayName(QTimeZone::TimeType timeType) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn display_name(&self, (&::time_zone::TimeType, &::time_zone::NameType)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTimeZone::displayName(QTimeZone::TimeType timeType, QTimeZone::NameType nameType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn display_name(&self, (&::time_zone::TimeType, &::time_zone::NameType, &::locale::Locale)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTimeZone::displayName(QTimeZone::TimeType timeType, QTimeZone::NameType nameType = ?, const QLocale& locale = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn display_name(&self, &::date_time::DateTime) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTimeZone::displayName(const QDateTime& atDateTime) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn display_name(&self, (&::date_time::DateTime, &::time_zone::NameType)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTimeZone::displayName(const QDateTime& atDateTime, QTimeZone::NameType nameType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn display_name(&self, (&::date_time::DateTime, &::time_zone::NameType, &::locale::Locale)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTimeZone::displayName(const QDateTime& atDateTime, QTimeZone::NameType nameType = ?, const QLocale& locale = ?) const```</span>
  ///
  ///
  pub fn display_name<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::TimeZoneDisplayNameArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTimeZone::hasDaylightTime() const```</span>
  ///
  ///
  pub fn has_daylight_time(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTimeZone_hasDaylightTime(self as *const ::time_zone::TimeZone) }
  }

  /// C++ method: <span style='color: green;'>```bool QTimeZone::hasTransitions() const```</span>
  ///
  ///
  pub fn has_transitions(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTimeZone_hasTransitions(self as *const ::time_zone::TimeZone) }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QTimeZone::ianaIdToWindowsId(const QByteArray& ianaId)```</span>
  ///
  ///
  pub fn iana_id_to_windows_id(iana_id: &::byte_array::ByteArray) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_ianaIdToWindowsId_to_output(iana_id as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QTimeZone::id() const```</span>
  ///
  ///
  pub fn id(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_id_to_output(self as *const ::time_zone::TimeZone, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTimeZone::isDaylightTime(const QDateTime& atDateTime) const```</span>
  ///
  ///
  pub fn is_daylight_time(&self, at_date_time: &::date_time::DateTime) -> bool {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_isDaylightTime(self as *const ::time_zone::TimeZone,
                                                at_date_time as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QTimeZone::isTimeZoneIdAvailable(const QByteArray& ianaId)```</span>
  ///
  ///
  pub fn is_time_zone_id_available(iana_id: &::byte_array::ByteArray) -> bool {
    unsafe { ::ffi::qt_core_c_QTimeZone_isTimeZoneIdAvailable(iana_id as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QTimeZone::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTimeZone_isValid(self as *const ::time_zone::TimeZone) }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::QTimeZone```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone(const QByteArray& ianaId)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::byte_array::ByteArray, ::libc::c_int, &::string::String, &::string::String)) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone(const QByteArray& zoneId, int offsetSeconds, const QString& name, const QString& abbreviation)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::byte_array::ByteArray, ::libc::c_int, &::string::String, &::string::String, &::locale::Country)) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone(const QByteArray& zoneId, int offsetSeconds, const QString& name, const QString& abbreviation, QLocale::Country country = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::byte_array::ByteArray, ::libc::c_int, &::string::String, &::string::String, &::locale::Country, &::string::String)) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone(const QByteArray& zoneId, int offsetSeconds, const QString& name, const QString& abbreviation, QLocale::Country country = ?, const QString& comment = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::time_zone::TimeZone) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone(const QTimeZone& other)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::time_zone::TimeZone```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeZone::QTimeZone(int offsetSeconds)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::time_zone::TimeZone
    where Args: overloading::TimeZoneNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QTimeZone::nextTransition(const QDateTime& afterDateTime) const```</span>
  ///
  ///
  pub fn next_transition(&self, after_date_time: &::date_time::DateTime) -> ::time_zone::OffsetData {
    {
      let mut object: ::time_zone::OffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_nextTransition_to_output(self as *const ::time_zone::TimeZone,
                                                            after_date_time as *const ::date_time::DateTime,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QTimeZone::offsetData(const QDateTime& forDateTime) const```</span>
  ///
  ///
  pub fn offset_data(&self, for_date_time: &::date_time::DateTime) -> ::time_zone::OffsetData {
    {
      let mut object: ::time_zone::OffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_offsetData_to_output(self as *const ::time_zone::TimeZone,
                                                        for_date_time as *const ::date_time::DateTime,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTimeZone::offsetFromUtc(const QDateTime& atDateTime) const```</span>
  ///
  ///
  pub fn offset_from_utc(&self, at_date_time: &::date_time::DateTime) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_offsetFromUtc(self as *const ::time_zone::TimeZone,
                                               at_date_time as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone& QTimeZone::operator=(const QTimeZone& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::time_zone::TimeZone) -> &'l0 mut ::time_zone::TimeZone {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QTimeZone_operator_assign(self as *mut ::time_zone::TimeZone,
                                                 other as *const ::time_zone::TimeZone)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTimeZone::operator==(const QTimeZone& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::time_zone::TimeZone) -> bool {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_operator_eq(self as *const ::time_zone::TimeZone,
                                             other as *const ::time_zone::TimeZone)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTimeZone::operator!=(const QTimeZone& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::time_zone::TimeZone) -> bool {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_operator_neq(self as *const ::time_zone::TimeZone,
                                              other as *const ::time_zone::TimeZone)
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QTimeZone::previousTransition(const QDateTime& beforeDateTime) const```</span>
  ///
  ///
  pub fn previous_transition(&self, before_date_time: &::date_time::DateTime) -> ::time_zone::OffsetData {
    {
      let mut object: ::time_zone::OffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_previousTransition_to_output(self as *const ::time_zone::TimeZone,
                                                                before_date_time as *const ::date_time::DateTime,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTimeZone::standardTimeOffset(const QDateTime& atDateTime) const```</span>
  ///
  ///
  pub fn standard_time_offset(&self, at_date_time: &::date_time::DateTime) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_standardTimeOffset(self as *const ::time_zone::TimeZone,
                                                    at_date_time as *const ::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTimeZone::swap(QTimeZone& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::time_zone::TimeZone) {
    unsafe {
      ::ffi::qt_core_c_QTimeZone_swap(self as *mut ::time_zone::TimeZone,
                                      other as *mut ::time_zone::TimeZone)
    }
  }

  /// C++ method: <span style='color: green;'>```static QTimeZone QTimeZone::systemTimeZone()```</span>
  ///
  ///
  pub fn system_time_zone() -> ::time_zone::TimeZone {
    {
      let mut object: ::time_zone::TimeZone =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_systemTimeZone_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QTimeZone::systemTimeZoneId()```</span>
  ///
  ///
  pub fn system_time_zone_id() -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_systemTimeZoneId_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData> QTimeZone::transitions(const QDateTime& fromDateTime, const QDateTime& toDateTime) const```</span>
  ///
  ///
  pub fn transitions(&self,
                     from_date_time: &::date_time::DateTime,
                     to_date_time: &::date_time::DateTime)
                     -> ::vector::VectorTimeZoneOffsetData {
    {
      let mut object: ::vector::VectorTimeZoneOffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_transitions_to_output(self as *const ::time_zone::TimeZone,
                                                         from_date_time as *const ::date_time::DateTime,
                                                         to_date_time as *const ::date_time::DateTime,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QTimeZone QTimeZone::utc()```</span>
  ///
  ///
  pub fn utc() -> ::time_zone::TimeZone {
    {
      let mut object: ::time_zone::TimeZone =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeZone_utc_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::windowsIdToDefaultIanaId```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn windows_id_to_default_iana_id(&::byte_array::ByteArray) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray& windowsId)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn windows_id_to_default_iana_id((&::byte_array::ByteArray, &::locale::Country)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QTimeZone::windowsIdToDefaultIanaId(const QByteArray& windowsId, QLocale::Country country)```</span>
  ///
  ///
  pub fn windows_id_to_default_iana_id<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::TimeZoneWindowsIdToDefaultIanaIdArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTimeZone::windowsIdToIanaIds```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn windows_id_to_iana_ids(&::byte_array::ByteArray) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray& windowsId)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn windows_id_to_iana_ids((&::byte_array::ByteArray, &::locale::Country)) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QTimeZone::windowsIdToIanaIds(const QByteArray& windowsId, QLocale::Country country)```</span>
  ///
  ///
  pub fn windows_id_to_iana_ids<Args>(args: Args) -> ::list::ListByteArray
    where Args: overloading::TimeZoneWindowsIdToIanaIdsArgs
  {
    args.exec()
  }
}

impl Drop for ::time_zone::TimeZone {
  /// C++ method: <span style='color: green;'>```[destructor] void QTimeZone::~QTimeZone()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimeZone_destructor(self as *mut ::time_zone::TimeZone) }
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QTimeZone& tz)```</span>
///
///
pub fn op_shl(dbg: &::debug::Debug, tz: &::time_zone::TimeZone) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QTimeZone_G_operator_shl_to_output(dbg as *const ::debug::Debug,
                                                          tz as *const ::time_zone::TimeZone,
                                                          &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& ds, QTimeZone& tz)```</span>
///
///
pub fn op_shr<'l0, 'l1>(ds: &'l0 mut ::data_stream::DataStream,
                        tz: &'l1 mut ::time_zone::TimeZone)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QTimeZone_G_operator_shr(ds as *mut ::data_stream::DataStream,
                                              tz as *mut ::time_zone::TimeZone)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QTimeZone& value1, QTimeZone& value2)```</span>
///
///
pub fn swap(value1: &mut ::time_zone::TimeZone, value2: &mut ::time_zone::TimeZone) {
  unsafe {
    ::ffi::qt_core_c_QTimeZone_G_swap(value1 as *mut ::time_zone::TimeZone,
                                      value2 as *mut ::time_zone::TimeZone)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TimeZone::available_time_zone_ids](../struct.TimeZone.html#method.available_time_zone_ids) method.
  pub trait TimeZoneAvailableTimeZoneIdsArgs {
    fn exec(self) -> ::list::ListByteArray;
  }
  impl<'a> TimeZoneAvailableTimeZoneIdsArgs for &'a ::locale::Country {
    fn exec(self) -> ::list::ListByteArray {
      let country = self;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_availableTimeZoneIds_to_output_country(country as *const ::locale::Country,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl TimeZoneAvailableTimeZoneIdsArgs for () {
    fn exec(self) -> ::list::ListByteArray {

      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_availableTimeZoneIds_to_output_no_args(&mut object);
        }
        object
      }
    }
  }
  impl TimeZoneAvailableTimeZoneIdsArgs for ::libc::c_int {
    fn exec(self) -> ::list::ListByteArray {
      let offset_seconds = self;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_availableTimeZoneIds_to_output_offsetSeconds(offset_seconds, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TimeZone::display_name](../struct.TimeZone.html#method.display_name) method.
  pub trait TimeZoneDisplayNameArgs<'largs> {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String;
  }
  impl<'largs> TimeZoneDisplayNameArgs<'largs> for &'largs ::date_time::DateTime {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String {
      let at_date_time = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_displayName_to_output_atDateTime(original_self as *const ::time_zone::TimeZone,
                                                                      at_date_time as *const ::date_time::DateTime,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeZoneDisplayNameArgs<'largs> for (&'largs ::date_time::DateTime, &'largs ::time_zone::NameType) {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String {
      let at_date_time = self.0;
      let name_type = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_displayName_to_output_atDateTime_nameType(original_self as *const ::time_zone::TimeZone, at_date_time as *const ::date_time::DateTime, name_type as *const ::time_zone::NameType, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeZoneDisplayNameArgs<'largs>
    for (&'largs ::date_time::DateTime, &'largs ::time_zone::NameType, &'largs ::locale::Locale) {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String {
      let at_date_time = self.0;
      let name_type = self.1;
      let locale = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_displayName_to_output_atDateTime_nameType_locale(original_self as *const ::time_zone::TimeZone, at_date_time as *const ::date_time::DateTime, name_type as *const ::time_zone::NameType, locale as *const ::locale::Locale, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeZoneDisplayNameArgs<'largs> for &'largs ::time_zone::TimeType {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String {
      let time_type = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_displayName_to_output_timeType(original_self as *const ::time_zone::TimeZone,
                                                                    time_type as *const ::time_zone::TimeType,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeZoneDisplayNameArgs<'largs> for (&'largs ::time_zone::TimeType, &'largs ::time_zone::NameType) {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String {
      let time_type = self.0;
      let name_type = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_displayName_to_output_timeType_nameType(original_self as *const ::time_zone::TimeZone, time_type as *const ::time_zone::TimeType, name_type as *const ::time_zone::NameType, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TimeZoneDisplayNameArgs<'largs>
    for (&'largs ::time_zone::TimeType, &'largs ::time_zone::NameType, &'largs ::locale::Locale) {
    fn exec(self, original_self: &'largs ::time_zone::TimeZone) -> ::string::String {
      let time_type = self.0;
      let name_type = self.1;
      let locale = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_displayName_to_output_timeType_nameType_locale(original_self as *const ::time_zone::TimeZone, time_type as *const ::time_zone::TimeType, name_type as *const ::time_zone::NameType, locale as *const ::locale::Locale, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TimeZone::new](../struct.TimeZone.html#method.new) method.
  pub trait TimeZoneNewArgs {
    fn exec(self) -> ::time_zone::TimeZone;
  }
  impl<'a> TimeZoneNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::time_zone::TimeZone {
      let iana_id = self;
      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_ianaId(iana_id as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl TimeZoneNewArgs for () {
    fn exec(self) -> ::time_zone::TimeZone {

      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl TimeZoneNewArgs for ::libc::c_int {
    fn exec(self) -> ::time_zone::TimeZone {
      let offset_seconds = self;
      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_offsetSeconds(offset_seconds, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeZoneNewArgs for &'a ::time_zone::TimeZone {
    fn exec(self) -> ::time_zone::TimeZone {
      let other = self;
      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_other(other as *const ::time_zone::TimeZone, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeZoneNewArgs for (&'a ::byte_array::ByteArray, ::libc::c_int, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::time_zone::TimeZone {
      let zone_id = self.0;
      let offset_seconds = self.1;
      let name = self.2;
      let abbreviation = self.3;
      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation(zone_id as *const ::byte_array::ByteArray, offset_seconds, name as *const ::string::String, abbreviation as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeZoneNewArgs
    for (&'a ::byte_array::ByteArray,
                                    ::libc::c_int,
                                    &'a ::string::String,
                                    &'a ::string::String,
                                    &'a ::locale::Country) {
    fn exec(self) -> ::time_zone::TimeZone {
      let zone_id = self.0;
      let offset_seconds = self.1;
      let name = self.2;
      let abbreviation = self.3;
      let country = self.4;
      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation_country(zone_id as *const ::byte_array::ByteArray, offset_seconds, name as *const ::string::String, abbreviation as *const ::string::String, country as *const ::locale::Country, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeZoneNewArgs
    for (&'a ::byte_array::ByteArray,
                                    ::libc::c_int,
                                    &'a ::string::String,
                                    &'a ::string::String,
                                    &'a ::locale::Country,
                                    &'a ::string::String) {
    fn exec(self) -> ::time_zone::TimeZone {
      let zone_id = self.0;
      let offset_seconds = self.1;
      let name = self.2;
      let abbreviation = self.3;
      let country = self.4;
      let comment = self.5;
      {
        let mut object: ::time_zone::TimeZone =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation_country_comment(zone_id as *const ::byte_array::ByteArray, offset_seconds, name as *const ::string::String, abbreviation as *const ::string::String, country as *const ::locale::Country, comment as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TimeZone::windows_id_to_default_iana_id](../struct.TimeZone.html#method.windows_id_to_default_iana_id) method.
  pub trait TimeZoneWindowsIdToDefaultIanaIdArgs {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> TimeZoneWindowsIdToDefaultIanaIdArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::byte_array::ByteArray {
      let windows_id = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_windowsIdToDefaultIanaId_to_output_windowsId(windows_id as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeZoneWindowsIdToDefaultIanaIdArgs for (&'a ::byte_array::ByteArray, &'a ::locale::Country) {
    fn exec(self) -> ::byte_array::ByteArray {
      let windows_id = self.0;
      let country = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_windowsIdToDefaultIanaId_to_output_windowsId_country(windows_id as *const ::byte_array::ByteArray, country as *const ::locale::Country, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TimeZone::windows_id_to_iana_ids](../struct.TimeZone.html#method.windows_id_to_iana_ids) method.
  pub trait TimeZoneWindowsIdToIanaIdsArgs {
    fn exec(self) -> ::list::ListByteArray;
  }
  impl<'a> TimeZoneWindowsIdToIanaIdsArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::list::ListByteArray {
      let windows_id = self;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_windowsIdToIanaIds_to_output_windowsId(windows_id as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TimeZoneWindowsIdToIanaIdsArgs for (&'a ::byte_array::ByteArray, &'a ::locale::Country) {
    fn exec(self) -> ::list::ListByteArray {
      let windows_id = self.0;
      let country = self.1;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTimeZone_windowsIdToIanaIds_to_output_windowsId_country(windows_id as *const ::byte_array::ByteArray, country as *const ::locale::Country, &mut object);
        }
        object
      }
    }
  }
}
