/// C++ type: <span style='color: green;'>```QCalendarWidget```</span>
#[repr(C)]
pub struct CalendarWidget(u8);

impl CalendarWidget {
  /// C++ method: <span style='color: green;'>```int QCalendarWidget::dateEditAcceptDelay() const```</span>
  ///
  ///
  pub fn date_edit_accept_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_dateEditAcceptDelay(self as *const ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```QCalendarWidget::dateTextFormat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn date_text_format(&self, ()) -> ::map::MapQtCoreDateQtGuiTextCharFormat```<br>
  /// C++ method: <span style='color: green;'>```QMap<QDate, QTextCharFormat> QCalendarWidget::dateTextFormat() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn date_text_format(&self, &::qt_core::date::Date) -> ::qt_gui::text_char_format::TextCharFormat```<br>
  /// C++ method: <span style='color: green;'>```QTextCharFormat QCalendarWidget::dateTextFormat(const QDate& date) const```</span>
  ///
  ///
  pub fn date_text_format<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::CalendarWidgetDateTextFormatArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCharFormat QCalendarWidget::headerTextFormat() const```</span>
  ///
  ///
  pub fn header_text_format(&self) -> ::qt_gui::text_char_format::TextCharFormat {
    {
      let mut object: ::qt_gui::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_headerTextFormat_to_output(self as *const ::calendar_widget::CalendarWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCalendarWidget::HorizontalHeaderFormat QCalendarWidget::horizontalHeaderFormat() const```</span>
  ///
  ///
  pub fn horizontal_header_format(&self) -> ::calendar_widget::HorizontalHeaderFormat {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_horizontalHeaderFormat(self as *const ::calendar_widget::CalendarWidget)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QCalendarWidget::isDateEditEnabled() const```</span>
  ///
  ///
  pub fn is_date_edit_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_isDateEditEnabled(self as *const ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QCalendarWidget::isGridVisible() const```</span>
  ///
  ///
  pub fn is_grid_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_isGridVisible(self as *const ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QCalendarWidget::isNavigationBarVisible() const```</span>
  ///
  ///
  pub fn is_navigation_bar_visible(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_isNavigationBarVisible(self as *const ::calendar_widget::CalendarWidget)
    }
  }

  /// C++ method: <span style='color: green;'>```QDate QCalendarWidget::maximumDate() const```</span>
  ///
  ///
  pub fn maximum_date(&self) -> ::qt_core::date::Date {
    {
      let mut object: ::qt_core::date::Date =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_maximumDate_to_output(self as *const ::calendar_widget::CalendarWidget,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QCalendarWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_metaObject(self as *const ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```QDate QCalendarWidget::minimumDate() const```</span>
  ///
  ///
  pub fn minimum_date(&self) -> ::qt_core::date::Date {
    {
      let mut object: ::qt_core::date::Date =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_minimumDate_to_output(self as *const ::calendar_widget::CalendarWidget,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QCalendarWidget::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_minimumSizeHint_to_output(self as *const ::calendar_widget::CalendarWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QCalendarWidget::monthShown() const```</span>
  ///
  ///
  pub fn month_shown(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_monthShown(self as *const ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QCalendarWidget::QCalendarWidget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::calendar_widget::CalendarWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QCalendarWidget::QCalendarWidget(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::calendar_widget::CalendarWidget> {
    let ffi_result = ::ffi::qt_widgets_c_QCalendarWidget_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QCalendarWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QCalendarWidget_qt_metacall(self as *mut ::calendar_widget::CalendarWidget,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QCalendarWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QCalendarWidget_qt_metacast(self as *mut ::calendar_widget::CalendarWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```QDate QCalendarWidget::selectedDate() const```</span>
  ///
  ///
  pub fn selected_date(&self) -> ::qt_core::date::Date {
    {
      let mut object: ::qt_core::date::Date =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_selectedDate_to_output(self as *const ::calendar_widget::CalendarWidget,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCalendarWidget::SelectionMode QCalendarWidget::selectionMode() const```</span>
  ///
  ///
  pub fn selection_mode(&self) -> ::calendar_widget::SelectionMode {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_selectionMode(self as *const ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::setCurrentPage(int year, int month)```</span>
  ///
  ///
  pub fn set_current_page(&mut self, year: ::libc::c_int, month: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setCurrentPage(self as *mut ::calendar_widget::CalendarWidget, year, month)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setDateEditAcceptDelay(int delay)```</span>
  ///
  ///
  pub fn set_date_edit_accept_delay(&mut self, delay: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setDateEditAcceptDelay(self as *mut ::calendar_widget::CalendarWidget, delay)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setDateEditEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_date_edit_enabled(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setDateEditEnabled(self as *mut ::calendar_widget::CalendarWidget, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::setDateRange(const QDate& min, const QDate& max)```</span>
  ///
  ///
  pub fn set_date_range(&mut self, min: &::qt_core::date::Date, max: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setDateRange(self as *mut ::calendar_widget::CalendarWidget,
                                                       min as *const ::qt_core::date::Date,
                                                       max as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setDateTextFormat(const QDate& date, const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_date_text_format(&mut self,
                              date: &::qt_core::date::Date,
                              format: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setDateTextFormat(self as *mut ::calendar_widget::CalendarWidget, date as *const ::qt_core::date::Date, format as *const ::qt_gui::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setFirstDayOfWeek(Qt::DayOfWeek dayOfWeek)```</span>
  ///
  ///
  pub fn set_first_day_of_week(&mut self, day_of_week: &::qt_core::qt::DayOfWeek) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setFirstDayOfWeek(self as *mut ::calendar_widget::CalendarWidget,
                                                            day_of_week as *const ::qt_core::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::setGridVisible(bool show)```</span>
  ///
  ///
  pub fn set_grid_visible(&mut self, show: bool) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_setGridVisible(self as *mut ::calendar_widget::CalendarWidget, show) }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setHeaderTextFormat(const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_header_text_format(&mut self, format: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_setHeaderTextFormat(self as *mut ::calendar_widget::CalendarWidget, format as *const ::qt_gui::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setHorizontalHeaderFormat(QCalendarWidget::HorizontalHeaderFormat format)```</span>
  ///
  ///
  pub fn set_horizontal_header_format(&mut self, format: ::calendar_widget::HorizontalHeaderFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setHorizontalHeaderFormat(self as *mut ::calendar_widget::CalendarWidget,
                                                                    format)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setMaximumDate(const QDate& date)```</span>
  ///
  ///
  pub fn set_maximum_date(&mut self, date: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setMaximumDate(self as *mut ::calendar_widget::CalendarWidget,
                                                         date as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setMinimumDate(const QDate& date)```</span>
  ///
  ///
  pub fn set_minimum_date(&mut self, date: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setMinimumDate(self as *mut ::calendar_widget::CalendarWidget,
                                                         date as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::setNavigationBarVisible(bool visible)```</span>
  ///
  ///
  pub fn set_navigation_bar_visible(&mut self, visible: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setNavigationBarVisible(self as *mut ::calendar_widget::CalendarWidget,
                                                                  visible)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::setSelectedDate(const QDate& date)```</span>
  ///
  ///
  pub fn set_selected_date(&mut self, date: &::qt_core::date::Date) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setSelectedDate(self as *mut ::calendar_widget::CalendarWidget,
                                                          date as *const ::qt_core::date::Date)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setSelectionMode(QCalendarWidget::SelectionMode mode)```</span>
  ///
  ///
  pub fn set_selection_mode(&mut self, mode: ::calendar_widget::SelectionMode) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setSelectionMode(self as *mut ::calendar_widget::CalendarWidget, mode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setVerticalHeaderFormat(QCalendarWidget::VerticalHeaderFormat format)```</span>
  ///
  ///
  pub fn set_vertical_header_format(&mut self, format: ::calendar_widget::VerticalHeaderFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_setVerticalHeaderFormat(self as *mut ::calendar_widget::CalendarWidget,
                                                                  format)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCalendarWidget::setWeekdayTextFormat(Qt::DayOfWeek dayOfWeek, const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_weekday_text_format(&mut self,
                                 day_of_week: &::qt_core::qt::DayOfWeek,
                                 format: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_setWeekdayTextFormat(self as *mut ::calendar_widget::CalendarWidget, day_of_week as *const ::qt_core::qt::DayOfWeek, format as *const ::qt_gui::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::showNextMonth()```</span>
  ///
  ///
  pub fn show_next_month(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_showNextMonth(self as *mut ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::showNextYear()```</span>
  ///
  ///
  pub fn show_next_year(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_showNextYear(self as *mut ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::showPreviousMonth()```</span>
  ///
  ///
  pub fn show_previous_month(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_showPreviousMonth(self as *mut ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::showPreviousYear()```</span>
  ///
  ///
  pub fn show_previous_year(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_showPreviousYear(self as *mut ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::showSelectedDate()```</span>
  ///
  ///
  pub fn show_selected_date(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_showSelectedDate(self as *mut ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QCalendarWidget::showToday()```</span>
  ///
  ///
  pub fn show_today(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_showToday(self as *mut ::calendar_widget::CalendarWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QCalendarWidget::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_sizeHint_to_output(self as *const ::calendar_widget::CalendarWidget,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCalendarWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCalendarWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCalendarWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCalendarWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCalendarWidget::VerticalHeaderFormat QCalendarWidget::verticalHeaderFormat() const```</span>
  ///
  ///
  pub fn vertical_header_format(&self) -> ::calendar_widget::VerticalHeaderFormat {
    unsafe {
      ::ffi::qt_widgets_c_QCalendarWidget_verticalHeaderFormat(self as *const ::calendar_widget::CalendarWidget)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QCalendarWidget::weekdayTextFormat(Qt::DayOfWeek dayOfWeek) const```</span>
  ///
  ///
  pub fn weekday_text_format(&self,
                             day_of_week: &::qt_core::qt::DayOfWeek)
                             -> ::qt_gui::text_char_format::TextCharFormat {
    {
      let mut object: ::qt_gui::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_weekdayTextFormat_to_output(self as *const ::calendar_widget::CalendarWidget, day_of_week as *const ::qt_core::qt::DayOfWeek, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QCalendarWidget::yearShown() const```</span>
  ///
  ///
  pub fn year_shown(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCalendarWidget_yearShown(self as *const ::calendar_widget::CalendarWidget) }
  }
}

impl ::cpp_utils::CppDeletable for ::calendar_widget::CalendarWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QCalendarWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CalendarWidget`.
  pub struct Signals<'a>(&'a ::calendar_widget::CalendarWidget);
  /// Represents a built-in Qt signal `QCalendarWidget::currentPageChanged`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().current_page_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct CurrentPageChanged<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentPageChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentPageChanged(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentPageChanged<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::windowIconChanged`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct WindowIconChanged<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for WindowIconChanged<'a> {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconChanged(const QIcon&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconChanged<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct WindowIconTextChanged<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for WindowIconTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconTextChanged<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::clicked`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Clicked<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(const QDate&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for CustomContextMenuRequested<'a> {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customContextMenuRequested(const QPoint&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomContextMenuRequested<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::windowTitleChanged`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct WindowTitleChanged<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::activated`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Activated<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Activated<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(const QDate&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Activated<'a> {}
  /// Represents a built-in Qt signal `QCalendarWidget::selectionChanged`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SelectionChanged<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::currentPageChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_page_changed(&self) -> CurrentPageChanged {
      CurrentPageChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCalendarWidget::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CalendarWidget`.
  pub struct Slots<'a>(&'a ::calendar_widget::CalendarWidget);
  /// Represents a built-in Qt slot `QCalendarWidget::setWindowTitle`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetWindowTitle<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showNextMonth`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_next_month()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowNextMonth<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNextMonth<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNextMonth()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setGridVisible`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_grid_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetGridVisible<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetGridVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGridVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setHidden`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetHidden<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showMaximized`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowMaximized<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showPreviousYear`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_previous_year()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowPreviousYear<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowPreviousYear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showPreviousYear()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showSelectedDate`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_selected_date()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowSelectedDate<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowSelectedDate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showSelectedDate()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::update`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Update<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showFullScreen`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowFullScreen<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setDisabled`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetDisabled<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setStyleSheet`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetStyleSheet<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::show`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Show<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showNextYear`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_next_year()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowNextYear<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNextYear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNextYear()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showPreviousMonth`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_previous_month()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowPreviousMonth<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowPreviousMonth<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showPreviousMonth()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setVisible`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetVisible<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::lower`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Lower<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setSelectedDate`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_selected_date()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetSelectedDate<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetSelectedDate<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSelectedDate(const QDate&)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showMinimized`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowMinimized<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setEnabled`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetEnabled<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::hide`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Hide<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showNormal`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowNormal<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::repaint`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Repaint<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setWindowModified`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetWindowModified<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::close`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Close<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::showToday`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().show_today()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct ShowToday<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowToday<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showToday()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setCurrentPage`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_current_page()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetCurrentPage<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentPage<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentPage(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::raise`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct Raise<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setDateRange`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_date_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetDateRange<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetDateRange<'a> {
    type Arguments = (&'static ::qt_core::date::Date, &'static ::qt_core::date::Date);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDateRange(const QDate&,const QDate&)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setNavigationBarVisible`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_navigation_bar_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetNavigationBarVisible<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetNavigationBarVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNavigationBarVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCalendarWidget::setFocus`.
  ///
  /// An object of this type can be created from `CalendarWidget` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CalendarWidget` object.
  pub struct SetFocus<'a>(&'a ::calendar_widget::CalendarWidget);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showNextMonth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_next_month(&self) -> ShowNextMonth {
      ShowNextMonth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setGridVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_grid_visible(&self) -> SetGridVisible {
      SetGridVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showPreviousYear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_previous_year(&self) -> ShowPreviousYear {
      ShowPreviousYear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showSelectedDate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_selected_date(&self) -> ShowSelectedDate {
      ShowSelectedDate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showNextYear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_next_year(&self) -> ShowNextYear {
      ShowNextYear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showPreviousMonth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_previous_month(&self) -> ShowPreviousMonth {
      ShowPreviousMonth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setSelectedDate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_selected_date(&self) -> SetSelectedDate {
      SetSelectedDate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::showToday`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_today(&self) -> ShowToday {
      ShowToday(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setCurrentPage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_page(&self) -> SetCurrentPage {
      SetCurrentPage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setDateRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date_range(&self) -> SetDateRange {
      SetDateRange(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setNavigationBarVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_navigation_bar_visible(&self) -> SetNavigationBarVisible {
      SetNavigationBarVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCalendarWidget::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
  }
  impl ::calendar_widget::CalendarWidget {
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

/// C++ type: <span style='color: green;'>```QCalendarWidget::HorizontalHeaderFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum HorizontalHeaderFormat {
  /// C++ enum variant: <span style='color: green;'>```NoHorizontalHeader = 0```</span>
  NoHorizontalHeader = 0,
  /// C++ enum variant: <span style='color: green;'>```SingleLetterDayNames = 1```</span>
  SingleLetterDayNames = 1,
  /// C++ enum variant: <span style='color: green;'>```ShortDayNames = 2```</span>
  ShortDayNames = 2,
  /// C++ enum variant: <span style='color: green;'>```LongDayNames = 3```</span>
  LongDayNames = 3,
}

/// C++ type: <span style='color: green;'>```QCalendarWidget::SelectionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectionMode {
  /// C++ enum variant: <span style='color: green;'>```NoSelection = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```SingleSelection = 1```</span>
  Single = 1,
}

/// C++ type: <span style='color: green;'>```QCalendarWidget::VerticalHeaderFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum VerticalHeaderFormat {
  /// C++ enum variant: <span style='color: green;'>```NoVerticalHeader = 0```</span>
  NoVerticalHeader = 0,
  /// C++ enum variant: <span style='color: green;'>```ISOWeekNumbers = 1```</span>
  ISOWeekNumbers = 1,
}

impl ::cpp_utils::DynamicCast<::calendar_widget::CalendarWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::calendar_widget::CalendarWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_dynamic_cast_QCalendarWidget_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::calendar_widget::CalendarWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_dynamic_cast_QCalendarWidget_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::calendar_widget::CalendarWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QObject_ptr(self as *mut ::calendar_widget::CalendarWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QObject_ptr(self as *const ::calendar_widget::CalendarWidget as *mut ::calendar_widget::CalendarWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::calendar_widget::CalendarWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::calendar_widget::CalendarWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QPaintDevice_ptr(self as *const ::calendar_widget::CalendarWidget as *mut ::calendar_widget::CalendarWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::calendar_widget::CalendarWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QWidget_ptr(self as *mut ::calendar_widget::CalendarWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QWidget_ptr(self as *const ::calendar_widget::CalendarWidget as *mut ::calendar_widget::CalendarWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::calendar_widget::CalendarWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::calendar_widget::CalendarWidget {
    let ffi_result = ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::calendar_widget::CalendarWidget {
    let ffi_result = ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::calendar_widget::CalendarWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::calendar_widget::CalendarWidget {
    let ffi_result = ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::calendar_widget::CalendarWidget {
    let ffi_result = ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::calendar_widget::CalendarWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::calendar_widget::CalendarWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::calendar_widget::CalendarWidget {
    let ffi_result = ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::calendar_widget::CalendarWidget {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QWidget_ptr(self as *const ::calendar_widget::CalendarWidget as *mut ::calendar_widget::CalendarWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::calendar_widget::CalendarWidget {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QCalendarWidget_G_static_cast_QWidget_ptr(self as *mut ::calendar_widget::CalendarWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CalendarWidget::date_text_format](../struct.CalendarWidget.html#method.date_text_format) method.
  pub trait CalendarWidgetDateTextFormatArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::calendar_widget::CalendarWidget) -> Self::ReturnType;
  }
  impl<'largs> CalendarWidgetDateTextFormatArgs<'largs> for &'largs ::qt_core::date::Date {
    type ReturnType = ::qt_gui::text_char_format::TextCharFormat;
    fn exec(self,
            original_self: &'largs ::calendar_widget::CalendarWidget)
            -> ::qt_gui::text_char_format::TextCharFormat {
      let date = self;
      {
        let mut object: ::qt_gui::text_char_format::TextCharFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QCalendarWidget_dateTextFormat_to_output_date(original_self as *const ::calendar_widget::CalendarWidget, date as *const ::qt_core::date::Date, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> CalendarWidgetDateTextFormatArgs<'largs> for () {
    type ReturnType = ::map::MapQtCoreDateQtGuiTextCharFormat;
    fn exec(self, original_self: &'largs ::calendar_widget::CalendarWidget) -> ::map::MapQtCoreDateQtGuiTextCharFormat {

      {
        let mut object: ::map::MapQtCoreDateQtGuiTextCharFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QCalendarWidget_dateTextFormat_to_output_no_args(original_self as *const ::calendar_widget::CalendarWidget, &mut object);
        }
        object
      }
    }
  }
}
