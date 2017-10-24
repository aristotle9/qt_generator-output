/// C++ type: <span style='color: green;'>```QDateTimeEdit```</span>
#[repr(C)]
pub struct DateTimeEdit(u8);

impl DateTimeEdit {
  /// C++ method: <span style='color: green;'>```bool QDateTimeEdit::calendarPopup() const```</span>
  ///
  ///
  pub fn calendar_popup(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_calendarPopup(self as *const ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```QCalendarWidget* QDateTimeEdit::calendarWidget() const```</span>
  ///
  ///
  pub fn calendar_widget(&self) -> *mut ::calendar_widget::CalendarWidget {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_calendarWidget(self as *const ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QDateTimeEdit::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clear(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::clearMaximumDate()```</span>
  ///
  ///
  pub fn clear_maximum_date(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clearMaximumDate(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::clearMaximumDateTime()```</span>
  ///
  ///
  pub fn clear_maximum_date_time(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clearMaximumDateTime(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::clearMaximumTime()```</span>
  ///
  ///
  pub fn clear_maximum_time(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clearMaximumTime(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::clearMinimumDate()```</span>
  ///
  ///
  pub fn clear_minimum_date(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clearMinimumDate(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::clearMinimumDateTime()```</span>
  ///
  ///
  pub fn clear_minimum_date_time(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clearMinimumDateTime(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::clearMinimumTime()```</span>
  ///
  ///
  pub fn clear_minimum_time(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_clearMinimumTime(self as *mut ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```QDateTimeEdit::Section QDateTimeEdit::currentSection() const```</span>
  ///
  ///
  pub fn current_section(&self) -> ::date_time_edit::Section {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_currentSection(self as *const ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QDateTimeEdit::currentSectionIndex() const```</span>
  ///
  ///
  pub fn current_section_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_currentSectionIndex(self as *const ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```QDate QDateTimeEdit::date() const```</span>
  ///
  ///
  pub fn date(&self) -> ::qt_core::date::Date {
    {
      let mut object: ::qt_core::date::Date =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_date_to_output(self as *const ::date_time_edit::DateTimeEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTimeEdit::dateTime() const```</span>
  ///
  ///
  pub fn date_time(&self) -> ::qt_core::date_time::DateTime {
    {
      let mut object: ::qt_core::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_dateTime_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDateTimeEdit::displayFormat() const```</span>
  ///
  ///
  pub fn display_format(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_displayFormat_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QDateTimeEdit::Section> QDateTimeEdit::displayedSections() const```</span>
  ///
  ///
  pub fn displayed_sections(&self) -> ::qt_core::flags::Flags<::date_time_edit::Section> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_displayedSections(self as *const ::date_time_edit::DateTimeEdit) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QDateTimeEdit::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QDateTimeEdit_event(self as *mut ::date_time_edit::DateTimeEdit, event)
  }

  /// C++ method: <span style='color: green;'>```QDate QDateTimeEdit::maximumDate() const```</span>
  ///
  ///
  pub fn maximum_date(&self) -> ::qt_core::date::Date {
    {
      let mut object: ::qt_core::date::Date =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_maximumDate_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTimeEdit::maximumDateTime() const```</span>
  ///
  ///
  pub fn maximum_date_time(&self) -> ::qt_core::date_time::DateTime {
    {
      let mut object: ::qt_core::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_maximumDateTime_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTime QDateTimeEdit::maximumTime() const```</span>
  ///
  ///
  pub fn maximum_time(&self) -> ::qt_core::time::Time {
    {
      let mut object: ::qt_core::time::Time =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_maximumTime_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDateTimeEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_metaObject(self as *const ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```QDate QDateTimeEdit::minimumDate() const```</span>
  ///
  ///
  pub fn minimum_date(&self) -> ::qt_core::date::Date {
    {
      let mut object: ::qt_core::date::Date =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_minimumDate_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QDateTimeEdit::minimumDateTime() const```</span>
  ///
  ///
  pub fn minimum_date_time(&self) -> ::qt_core::date_time::DateTime {
    {
      let mut object: ::qt_core::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_minimumDateTime_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTime QDateTimeEdit::minimumTime() const```</span>
  ///
  ///
  pub fn minimum_time(&self) -> ::qt_core::time::Time {
    {
      let mut object: ::qt_core::time::Time =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_minimumTime_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTimeEdit::QDateTimeEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::date::Date) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(const QDate& d)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::date_time::DateTime) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(const QDateTime& dt)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::time::Time) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(const QTime& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>
    where Args: overloading::DateTimeEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDateTimeEdit::QDateTimeEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::date::Date, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(const QDate& d, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::date_time::DateTime, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(const QDateTime& dt, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::time::Time, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateTimeEdit::QDateTimeEdit(const QTime& t, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>
    where Args: overloading::DateTimeEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QDateTimeEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDateTimeEdit_qt_metacall(self as *mut ::date_time_edit::DateTimeEdit,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDateTimeEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDateTimeEdit_qt_metacast(self as *mut ::date_time_edit::DateTimeEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```QDateTimeEdit::Section QDateTimeEdit::sectionAt(int index) const```</span>
  ///
  ///
  pub fn section_at(&self, index: ::libc::c_int) -> ::date_time_edit::Section {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_sectionAt(self as *const ::date_time_edit::DateTimeEdit, index) }
  }

  /// C++ method: <span style='color: green;'>```int QDateTimeEdit::sectionCount() const```</span>
  ///
  ///
  pub fn section_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_sectionCount(self as *const ::date_time_edit::DateTimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QDateTimeEdit::sectionText(QDateTimeEdit::Section section) const```</span>
  ///
  ///
  pub fn section_text(&self, section: ::date_time_edit::Section) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_sectionText_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                                section,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setCalendarPopup(bool enable)```</span>
  ///
  ///
  pub fn set_calendar_popup(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_setCalendarPopup(self as *mut ::date_time_edit::DateTimeEdit, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setCalendarWidget(QCalendarWidget* calendarWidget)```</span>
  ///
  ///
  pub unsafe fn set_calendar_widget(&mut self, calendar_widget: *mut ::calendar_widget::CalendarWidget) {
    ::ffi::qt_widgets_c_QDateTimeEdit_setCalendarWidget(self as *mut ::date_time_edit::DateTimeEdit, calendar_widget)
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setCurrentSection(QDateTimeEdit::Section section)```</span>
  ///
  ///
  pub fn set_current_section(&mut self, section: ::date_time_edit::Section) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_setCurrentSection(self as *mut ::date_time_edit::DateTimeEdit, section) }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setCurrentSectionIndex(int index)```</span>
  ///
  ///
  pub fn set_current_section_index(&mut self, index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setCurrentSectionIndex(self as *mut ::date_time_edit::DateTimeEdit, index)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDateTimeEdit::setDate(const QDate& date)```</span>
  ///
  ///
  pub fn set_date(&mut self, date: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setDate(self as *mut ::date_time_edit::DateTimeEdit,
                                                date as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setDateRange(const QDate& min, const QDate& max)```</span>
  ///
  ///
  pub fn set_date_range(&mut self, min: &::qt_core::date::Date, max: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setDateRange(self as *mut ::date_time_edit::DateTimeEdit,
                                                     min as *const ::qt_core::date::Date,
                                                     max as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDateTimeEdit::setDateTime(const QDateTime& dateTime)```</span>
  ///
  ///
  pub fn set_date_time(&mut self, date_time: &::qt_core::date_time::DateTime) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setDateTime(self as *mut ::date_time_edit::DateTimeEdit,
                                                    date_time as *const ::qt_core::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setDateTimeRange(const QDateTime& min, const QDateTime& max)```</span>
  ///
  ///
  pub fn set_date_time_range(&mut self, min: &::qt_core::date_time::DateTime, max: &::qt_core::date_time::DateTime) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setDateTimeRange(self as *mut ::date_time_edit::DateTimeEdit,
                                                         min as *const ::qt_core::date_time::DateTime,
                                                         max as *const ::qt_core::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setDisplayFormat(const QString& format)```</span>
  ///
  ///
  pub fn set_display_format(&mut self, format: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setDisplayFormat(self as *mut ::date_time_edit::DateTimeEdit,
                                                         format as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setMaximumDate(const QDate& max)```</span>
  ///
  ///
  pub fn set_maximum_date(&mut self, max: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setMaximumDate(self as *mut ::date_time_edit::DateTimeEdit,
                                                       max as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setMaximumDateTime(const QDateTime& dt)```</span>
  ///
  ///
  pub fn set_maximum_date_time(&mut self, dt: &::qt_core::date_time::DateTime) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setMaximumDateTime(self as *mut ::date_time_edit::DateTimeEdit,
                                                           dt as *const ::qt_core::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setMaximumTime(const QTime& max)```</span>
  ///
  ///
  pub fn set_maximum_time(&mut self, max: &::qt_core::time::Time) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setMaximumTime(self as *mut ::date_time_edit::DateTimeEdit,
                                                       max as *const ::qt_core::time::Time)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setMinimumDate(const QDate& min)```</span>
  ///
  ///
  pub fn set_minimum_date(&mut self, min: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setMinimumDate(self as *mut ::date_time_edit::DateTimeEdit,
                                                       min as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setMinimumDateTime(const QDateTime& dt)```</span>
  ///
  ///
  pub fn set_minimum_date_time(&mut self, dt: &::qt_core::date_time::DateTime) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setMinimumDateTime(self as *mut ::date_time_edit::DateTimeEdit,
                                                           dt as *const ::qt_core::date_time::DateTime)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setMinimumTime(const QTime& min)```</span>
  ///
  ///
  pub fn set_minimum_time(&mut self, min: &::qt_core::time::Time) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setMinimumTime(self as *mut ::date_time_edit::DateTimeEdit,
                                                       min as *const ::qt_core::time::Time)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setSelectedSection(QDateTimeEdit::Section section)```</span>
  ///
  ///
  pub fn set_selected_section(&mut self, section: ::date_time_edit::Section) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setSelectedSection(self as *mut ::date_time_edit::DateTimeEdit, section)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDateTimeEdit::setTime(const QTime& time)```</span>
  ///
  ///
  pub fn set_time(&mut self, time: &::qt_core::time::Time) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setTime(self as *mut ::date_time_edit::DateTimeEdit,
                                                time as *const ::qt_core::time::Time)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setTimeRange(const QTime& min, const QTime& max)```</span>
  ///
  ///
  pub fn set_time_range(&mut self, min: &::qt_core::time::Time, max: &::qt_core::time::Time) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setTimeRange(self as *mut ::date_time_edit::DateTimeEdit,
                                                     min as *const ::qt_core::time::Time,
                                                     max as *const ::qt_core::time::Time)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDateTimeEdit::setTimeSpec(Qt::TimeSpec spec)```</span>
  ///
  ///
  pub fn set_time_spec(&mut self, spec: &::qt_core::qt::TimeSpec) {
    unsafe {
      ::ffi::qt_widgets_c_QDateTimeEdit_setTimeSpec(self as *mut ::date_time_edit::DateTimeEdit,
                                                    spec as *const ::qt_core::qt::TimeSpec)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QDateTimeEdit::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_sizeHint_to_output(self as *const ::date_time_edit::DateTimeEdit,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QDateTimeEdit::stepBy(int steps)```</span>
  ///
  ///
  pub fn step_by(&mut self, steps: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_stepBy(self as *mut ::date_time_edit::DateTimeEdit, steps) }
  }

  /// C++ method: <span style='color: green;'>```QTime QDateTimeEdit::time() const```</span>
  ///
  ///
  pub fn time(&self) -> ::qt_core::time::Time {
    {
      let mut object: ::qt_core::time::Time =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_time_to_output(self as *const ::date_time_edit::DateTimeEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDateTimeEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDateTimeEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDateTimeEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDateTimeEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::date_time_edit::DateTimeEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDateTimeEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DateTimeEdit`.
  pub struct Signals<'a>(&'a ::date_time_edit::DateTimeEdit);
  /// Represents a built-in Qt signal `QDateTimeEdit::dateTimeChanged`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.signals().date_time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct DateTimeChanged<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for DateTimeChanged<'a> {
    type Arguments = (&'static ::qt_core::date_time::DateTime,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dateTimeChanged(const QDateTime&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DateTimeChanged<'a> {}
  /// Represents a built-in Qt signal `QDateTimeEdit::dateChanged`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.signals().date_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct DateChanged<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for DateChanged<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dateChanged(const QDate&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DateChanged<'a> {}
  /// Represents a built-in Qt signal `QDateTimeEdit::timeChanged`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.signals().time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct TimeChanged<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for TimeChanged<'a> {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2timeChanged(const QTime&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TimeChanged<'a> {}
  /// Represents a built-in Qt signal `QDateTimeEdit::editingFinished`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.signals().editing_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct EditingFinished<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for EditingFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editingFinished()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditingFinished<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDateTimeEdit::dateTimeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn date_time_changed(&self) -> DateTimeChanged {
      DateTimeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDateTimeEdit::dateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn date_changed(&self) -> DateChanged {
      DateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDateTimeEdit::timeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn time_changed(&self) -> TimeChanged {
      TimeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDateTimeEdit::editingFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editing_finished(&self) -> EditingFinished {
      EditingFinished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DateTimeEdit`.
  pub struct Slots<'a>(&'a ::date_time_edit::DateTimeEdit);
  /// Represents a built-in Qt slot `QDateTimeEdit::setTime`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.slots().set_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct SetTime<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SetTime<'a> {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTime(const QTime&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDateTimeEdit::setDate`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.slots().set_date()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct SetDate<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDate<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDate(const QDate&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDateTimeEdit::setDateTime`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.slots().set_date_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct SetDateTime<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDateTime<'a> {
    type Arguments = (&'static ::qt_core::date_time::DateTime,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDateTime(const QDateTime&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDateTimeEdit::stepDown`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.slots().step_down()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct StepDown<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for StepDown<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepDown()\0"
    }
  }
  /// Represents a built-in Qt slot `QDateTimeEdit::selectAll`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct SelectAll<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QDateTimeEdit::stepUp`.
  ///
  /// An object of this type can be created from `DateTimeEdit` with `object.slots().step_up()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateTimeEdit` object.
  pub struct StepUp<'a>(&'a ::date_time_edit::DateTimeEdit);
  impl<'a> ::qt_core::connection::Receiver for StepUp<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepUp()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDateTimeEdit::setTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_time(&self) -> SetTime {
      SetTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateTimeEdit::setDate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date(&self) -> SetDate {
      SetDate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateTimeEdit::setDateTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date_time(&self) -> SetDateTime {
      SetDateTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateTimeEdit::stepDown`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_down(&self) -> StepDown {
      StepDown(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateTimeEdit::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateTimeEdit::stepUp`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_up(&self) -> StepUp {
      StepUp(self.0)
    }
  }
  impl ::date_time_edit::DateTimeEdit {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QDateTimeEdit::Section```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Section {
  /// C++ enum variant: <span style='color: green;'>```NoSection = 0```</span>
  NoSection = 0,
  /// C++ enum variant: <span style='color: green;'>```AmPmSection = 1```</span>
  AmPmSection = 1,
  /// C++ enum variant: <span style='color: green;'>```MSecSection = 2```</span>
  MSecSection = 2,
  /// C++ enum variant: <span style='color: green;'>```SecondSection = 4```</span>
  SecondSection = 4,
  /// C++ enum variant: <span style='color: green;'>```MinuteSection = 8```</span>
  MinuteSection = 8,
  /// C++ enum variant: <span style='color: green;'>```HourSection = 16```</span>
  HourSection = 16,
  /// C++ enum variant: <span style='color: green;'>```TimeSections_Mask = 31```</span>
  TimeSectionsMask = 31,
  /// C++ enum variant: <span style='color: green;'>```DaySection = 256```</span>
  DaySection = 256,
  /// C++ enum variant: <span style='color: green;'>```MonthSection = 512```</span>
  MonthSection = 512,
  /// C++ enum variant: <span style='color: green;'>```YearSection = 1024```</span>
  YearSection = 1024,
  /// C++ enum variant: <span style='color: green;'>```DateSections_Mask = 1792```</span>
  DateSectionsMask = 1792,
}

impl ::qt_core::flags::FlaggableEnum for Section {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Section"
  }
}

impl ::cpp_utils::DynamicCast<::date_time_edit::DateTimeEdit> for ::abstract_spin_box::AbstractSpinBox {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::date_time_edit::DateTimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::date_time_edit::DateTimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::date_time_edit::DateTimeEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::date_time_edit::DateTimeEdit> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::date_time_edit::DateTimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::date_time_edit::DateTimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QObject_ptr(self as *mut ::date_time_edit::DateTimeEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QObject_ptr(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::date_time_edit::DateTimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::date_time_edit::DateTimeEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QPaintDevice_ptr(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_spin_box::AbstractSpinBox> for ::date_time_edit::DateTimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QAbstractSpinBox_ptr(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::date_time_edit::DateTimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QWidget_ptr(self as *mut ::date_time_edit::DateTimeEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QWidget_ptr(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_time_edit::DateTimeEdit> for ::abstract_spin_box::AbstractSpinBox {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_time_edit::DateTimeEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_time_edit::DateTimeEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_time_edit::DateTimeEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::date_time_edit::DateTimeEdit {
  type Target = ::abstract_spin_box::AbstractSpinBox;
  fn deref(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QAbstractSpinBox_ptr(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::date_time_edit::DateTimeEdit {
  fn deref_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DateTimeEdit::new](../struct.DateTimeEdit.html#method.new) method.
  pub trait DateTimeEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>;
  }
  impl<'a> DateTimeEditNewArgs for &'a ::qt_core::date::Date {
    fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let d = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_new_d(d as *const ::qt_core::date::Date) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> DateTimeEditNewArgs for &'a ::qt_core::date_time::DateTime {
    fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let dt = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_new_dt(dt as *const ::qt_core::date_time::DateTime) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl DateTimeEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> DateTimeEditNewArgs for &'a ::qt_core::time::Time {
    fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateTimeEdit_new_t(t as *const ::qt_core::time::Time) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [DateTimeEdit::new_unsafe](../struct.DateTimeEdit.html#method.new_unsafe) method.
  pub trait DateTimeEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit>;
  }
  impl<'a> DateTimeEditNewUnsafeArgs for (&'a ::qt_core::date::Date, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let d = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_new_d_parent(d as *const ::qt_core::date::Date, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> DateTimeEditNewUnsafeArgs for (&'a ::qt_core::date_time::DateTime, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let dt = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_new_dt_parent(dt as *const ::qt_core::date_time::DateTime,
                                                                       parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl DateTimeEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> DateTimeEditNewUnsafeArgs for (&'a ::qt_core::time::Time, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_time_edit::DateTimeEdit> {
      let t = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QDateTimeEdit_new_t_parent(t as *const ::qt_core::time::Time, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
