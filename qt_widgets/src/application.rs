/// C++ type: <span style='color: green;'>```QApplication```</span>
#[repr(C)]
pub struct Application(u8);

impl Application {
  /// C++ method: <span style='color: green;'>```static [slot] void QApplication::aboutQt()```</span>
  ///
  ///
  pub fn about_qt() {
    unsafe { ::ffi::qt_widgets_c_QApplication_aboutQt() }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::activeModalWidget()```</span>
  ///
  ///
  pub fn active_modal_widget() -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QApplication_activeModalWidget() }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::activePopupWidget()```</span>
  ///
  ///
  pub fn active_popup_widget() -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QApplication_activePopupWidget() }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::activeWindow()```</span>
  ///
  ///
  pub fn active_window() -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QApplication_activeWindow() }
  }

  /// C++ method: <span style='color: green;'>```QApplication::alert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn alert(*mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QApplication::alert(QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn alert((*mut ::widget::Widget, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QApplication::alert(QWidget* widget, int duration = ?)```</span>
  ///
  ///
  pub unsafe fn alert<Args>(args: Args) -> ()
    where Args: overloading::ApplicationAlertArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QList<QWidget*> QApplication::allWidgets()```</span>
  ///
  ///
  pub fn all_widgets() -> ::list::ListWidgetMutPtr {
    {
      let mut object: ::list::ListWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_allWidgets_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] bool QApplication::autoSipEnabled() const```</span>
  ///
  ///
  pub fn auto_sip_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QApplication_autoSipEnabled(self as *const ::application::Application) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::beep()```</span>
  ///
  ///
  pub fn beep() {
    unsafe { ::ffi::qt_widgets_c_QApplication_beep() }
  }

  /// C++ method: <span style='color: green;'>```static [slot] void QApplication::closeAllWindows()```</span>
  ///
  ///
  pub fn close_all_windows() {
    unsafe { ::ffi::qt_widgets_c_QApplication_closeAllWindows() }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::colorSpec()```</span>
  ///
  ///
  pub fn color_spec() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_colorSpec() }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::cursorFlashTime()```</span>
  ///
  ///
  pub fn cursor_flash_time() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_cursorFlashTime() }
  }

  /// C++ method: <span style='color: green;'>```static QDesktopWidget* QApplication::desktop()```</span>
  ///
  ///
  pub fn desktop() -> *mut ::desktop_widget::DesktopWidget {
    unsafe { ::ffi::qt_widgets_c_QApplication_desktop() }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::doubleClickInterval()```</span>
  ///
  ///
  pub fn double_click_interval() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_doubleClickInterval() }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::exec()```</span>
  ///
  ///
  pub fn exec() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_exec() }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::focusWidget()```</span>
  ///
  ///
  pub fn focus_widget() -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QApplication_focusWidget() }
  }

  /// C++ method: <span style='color: green;'>```static QFont QApplication::font()```</span>
  ///
  ///
  pub fn font() -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_font_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QFontMetrics QApplication::fontMetrics()```</span>
  ///
  ///
  pub fn font_metrics() -> ::qt_gui::font_metrics::FontMetrics {
    {
      let mut object: ::qt_gui::font_metrics::FontMetrics =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_fontMetrics_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QApplication::font```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn font_unsafe(*const ::widget::Widget) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QApplication::font(const QWidget* arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn font_unsafe(*const ::libc::c_char) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QApplication::font(const char* className)```</span>
  ///
  ///
  pub unsafe fn font_unsafe<Args>(args: Args) -> ::qt_gui::font::Font
    where Args: overloading::ApplicationFontUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QSize QApplication::globalStrut()```</span>
  ///
  ///
  pub fn global_strut() -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_globalStrut_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QApplication::isEffectEnabled(Qt::UIEffect arg1)```</span>
  ///
  ///
  pub fn is_effect_enabled(arg1: &::qt_core::qt::UIEffect) -> bool {
    unsafe { ::ffi::qt_widgets_c_QApplication_isEffectEnabled(arg1 as *const ::qt_core::qt::UIEffect) }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::keyboardInputInterval()```</span>
  ///
  ///
  pub fn keyboard_input_interval() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_keyboardInputInterval() }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QApplication::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QApplication_metaObject(self as *const ::application::Application) }
  }

  /// C++ method: <span style='color: green;'>```QApplication::QApplication```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((&mut ::libc::c_int, *mut *mut ::libc::c_char)) -> ::cpp_utils::CppBox<::application::Application>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QApplication::QApplication(int& argc, char** argv)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&mut ::libc::c_int, *mut *mut ::libc::c_char, ::libc::c_int)) -> ::cpp_utils::CppBox<::application::Application>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QApplication::QApplication(int& argc, char** argv, int arg3 = ?)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::application::Application>
    where Args: overloading::ApplicationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual bool QApplication::notify(QObject* arg1, QEvent* arg2)```</span>
  ///
  ///
  pub unsafe fn notify(&mut self, arg1: *mut ::qt_core::object::Object, arg2: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QApplication_notify(self as *mut ::application::Application, arg1, arg2)
  }

  /// C++ method: <span style='color: green;'>```QApplication::palette```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn palette(*const ::widget::Widget) -> ::qt_gui::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```static QPalette QApplication::palette(const QWidget* arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn palette(*const ::libc::c_char) -> ::qt_gui::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```static QPalette QApplication::palette(const char* className)```</span>
  ///
  ///
  pub unsafe fn palette<Args>(args: Args) -> ::qt_gui::palette::Palette
    where Args: overloading::ApplicationPaletteArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QApplication::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QApplication_qt_metacall(self as *mut ::application::Application,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QApplication::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QApplication_qt_metacast(self as *mut ::application::Application, arg1)
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setActiveWindow(QWidget* act)```</span>
  ///
  ///
  pub unsafe fn set_active_window(act: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QApplication_setActiveWindow(act)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QApplication::setAutoSipEnabled(const bool enabled)```</span>
  ///
  ///
  pub fn set_auto_sip_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setAutoSipEnabled(self as *mut ::application::Application, enabled) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setColorSpec(int arg1)```</span>
  ///
  ///
  pub fn set_color_spec(arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setColorSpec(arg1) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setCursorFlashTime(int arg1)```</span>
  ///
  ///
  pub fn set_cursor_flash_time(arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setCursorFlashTime(arg1) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setDoubleClickInterval(int arg1)```</span>
  ///
  ///
  pub fn set_double_click_interval(arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setDoubleClickInterval(arg1) }
  }

  /// C++ method: <span style='color: green;'>```QApplication::setEffectEnabled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_effect_enabled(&::qt_core::qt::UIEffect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QApplication::setEffectEnabled(Qt::UIEffect arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_effect_enabled((&::qt_core::qt::UIEffect, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QApplication::setEffectEnabled(Qt::UIEffect arg1, bool enable = ?)```</span>
  ///
  ///
  pub fn set_effect_enabled<Args>(args: Args) -> ()
    where Args: overloading::ApplicationSetEffectEnabledArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static void QApplication::setFont(const QFont& arg1)```</span>
  ///
  ///
  pub fn set_font(arg1: &::qt_gui::font::Font) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setFont_arg1(arg1 as *const ::qt_gui::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setFont(const QFont& arg1, const char* className = ?)```</span>
  ///
  ///
  pub unsafe fn set_font_unsafe(arg1: &::qt_gui::font::Font, class_name: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QApplication_setFont_arg1_className(arg1 as *const ::qt_gui::font::Font, class_name)
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setGlobalStrut(const QSize& arg1)```</span>
  ///
  ///
  pub fn set_global_strut(arg1: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setGlobalStrut(arg1 as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setKeyboardInputInterval(int arg1)```</span>
  ///
  ///
  pub fn set_keyboard_input_interval(arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setKeyboardInputInterval(arg1) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setPalette(const QPalette& arg1)```</span>
  ///
  ///
  pub fn set_palette(arg1: &::qt_gui::palette::Palette) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setPalette_arg1(arg1 as *const ::qt_gui::palette::Palette) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setPalette(const QPalette& arg1, const char* className = ?)```</span>
  ///
  ///
  pub unsafe fn set_palette_unsafe(arg1: &::qt_gui::palette::Palette, class_name: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QApplication_setPalette_arg1_className(arg1 as *const ::qt_gui::palette::Palette, class_name)
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setStartDragDistance(int l)```</span>
  ///
  ///
  pub fn set_start_drag_distance(l: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setStartDragDistance(l) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setStartDragTime(int ms)```</span>
  ///
  ///
  pub fn set_start_drag_time(ms: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setStartDragTime(ms) }
  }

  /// C++ method: <span style='color: green;'>```static QStyle* QApplication::setStyle(const QString& arg1)```</span>
  ///
  ///
  pub fn set_style(arg1: &::qt_core::string::String) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QApplication_setStyle_QString(arg1 as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QApplication::setStyleSheet(const QString& sheet)```</span>
  ///
  ///
  pub fn set_style_sheet(&mut self, sheet: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QApplication_setStyleSheet(self as *mut ::application::Application,
                                                     sheet as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setStyle(QStyle* arg1)```</span>
  ///
  ///
  pub unsafe fn set_style_unsafe(arg1: *mut ::style::Style) {
    ::ffi::qt_widgets_c_QApplication_setStyle_QStyle(arg1)
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setWheelScrollLines(int arg1)```</span>
  ///
  ///
  pub fn set_wheel_scroll_lines(arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setWheelScrollLines(arg1) }
  }

  /// C++ method: <span style='color: green;'>```static void QApplication::setWindowIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_window_icon(icon: &::qt_gui::icon::Icon) {
    unsafe { ::ffi::qt_widgets_c_QApplication_setWindowIcon(icon as *const ::qt_gui::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::startDragDistance()```</span>
  ///
  ///
  pub fn start_drag_distance() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_startDragDistance() }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::startDragTime()```</span>
  ///
  ///
  pub fn start_drag_time() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_startDragTime() }
  }

  /// C++ method: <span style='color: green;'>```static QStyle* QApplication::style()```</span>
  ///
  ///
  pub fn style() -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QApplication_style() }
  }

  /// C++ method: <span style='color: green;'>```QString QApplication::styleSheet() const```</span>
  ///
  ///
  pub fn style_sheet(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_styleSheet_to_output(self as *const ::application::Application, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QApplication::topLevelAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn top_level_at(&::qt_core::point::Point) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::topLevelAt(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn top_level_at((::libc::c_int, ::libc::c_int)) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::topLevelAt(int x, int y)```</span>
  ///
  ///
  pub fn top_level_at<Args>(args: Args) -> *mut ::widget::Widget
    where Args: overloading::ApplicationTopLevelAtArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QList<QWidget*> QApplication::topLevelWidgets()```</span>
  ///
  ///
  pub fn top_level_widgets() -> ::list::ListWidgetMutPtr {
    {
      let mut object: ::list::ListWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_topLevelWidgets_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QApplication::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QApplication_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QApplication::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QApplication_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static int QApplication::wheelScrollLines()```</span>
  ///
  ///
  pub fn wheel_scroll_lines() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QApplication_wheelScrollLines() }
  }

  /// C++ method: <span style='color: green;'>```QApplication::widgetAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn widget_at(&::qt_core::point::Point) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::widgetAt(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn widget_at((::libc::c_int, ::libc::c_int)) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```static QWidget* QApplication::widgetAt(int x, int y)```</span>
  ///
  ///
  pub fn widget_at<Args>(args: Args) -> *mut ::widget::Widget
    where Args: overloading::ApplicationWidgetAtArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QIcon QApplication::windowIcon()```</span>
  ///
  ///
  pub fn window_icon() -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QApplication_windowIcon_to_output(&mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::application::Application {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QApplication_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Application`.
  pub struct Signals<'a>(&'a ::application::Application);
  /// Represents a built-in Qt signal `QApplication::applicationDisplayNameChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().application_display_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct ApplicationDisplayNameChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for ApplicationDisplayNameChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2applicationDisplayNameChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ApplicationDisplayNameChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::focusChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().focus_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct FocusChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for FocusChanged<'a> {
    type Arguments = (*mut ::widget::Widget, *mut ::widget::Widget);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusChanged(QWidget*,QWidget*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::focusObjectChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().focus_object_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct FocusObjectChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for FocusObjectChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusObjectChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusObjectChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::focusWindowChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().focus_window_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct FocusWindowChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for FocusWindowChanged<'a> {
    type Arguments = (*mut ::qt_gui::window::Window,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusWindowChanged(QWindow*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusWindowChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::screenRemoved`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().screen_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct ScreenRemoved<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for ScreenRemoved<'a> {
    type Arguments = (*mut ::qt_gui::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenRemoved(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenRemoved<'a> {}
  /// Represents a built-in Qt signal `QApplication::layoutDirectionChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().layout_direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct LayoutDirectionChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for LayoutDirectionChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::LayoutDirection,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutDirectionChanged(Qt::LayoutDirection)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayoutDirectionChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::primaryScreenChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().primary_screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct PrimaryScreenChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for PrimaryScreenChanged<'a> {
    type Arguments = (*mut ::qt_gui::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primaryScreenChanged(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimaryScreenChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::fontDatabaseChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().font_database_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct FontDatabaseChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for FontDatabaseChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fontDatabaseChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FontDatabaseChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::applicationStateChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().application_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct ApplicationStateChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for ApplicationStateChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::ApplicationState,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2applicationStateChanged(Qt::ApplicationState)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ApplicationStateChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::commitDataRequest`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().commit_data_request()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct CommitDataRequest<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for CommitDataRequest<'a> {
    type Arguments = (&'static mut ::qt_gui::session_manager::SessionManager,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2commitDataRequest(QSessionManager&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CommitDataRequest<'a> {}
  /// Represents a built-in Qt signal `QApplication::screenAdded`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().screen_added()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct ScreenAdded<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for ScreenAdded<'a> {
    type Arguments = (*mut ::qt_gui::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenAdded(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenAdded<'a> {}
  /// Represents a built-in Qt signal `QApplication::saveStateRequest`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().save_state_request()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct SaveStateRequest<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for SaveStateRequest<'a> {
    type Arguments = (&'static mut ::qt_gui::session_manager::SessionManager,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2saveStateRequest(QSessionManager&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SaveStateRequest<'a> {}
  /// Represents a built-in Qt signal `QApplication::paletteChanged`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().palette_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct PaletteChanged<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for PaletteChanged<'a> {
    type Arguments = (&'static ::qt_gui::palette::Palette,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2paletteChanged(const QPalette&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PaletteChanged<'a> {}
  /// Represents a built-in Qt signal `QApplication::lastWindowClosed`.
  ///
  /// An object of this type can be created from `Application` with `object.signals().last_window_closed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct LastWindowClosed<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for LastWindowClosed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2lastWindowClosed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LastWindowClosed<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QApplication::applicationDisplayNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_display_name_changed(&self) -> ApplicationDisplayNameChanged {
      ApplicationDisplayNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::focusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_changed(&self) -> FocusChanged {
      FocusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::focusObjectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_object_changed(&self) -> FocusObjectChanged {
      FocusObjectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::focusWindowChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_window_changed(&self) -> FocusWindowChanged {
      FocusWindowChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::screenRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_removed(&self) -> ScreenRemoved {
      ScreenRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::layoutDirectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_direction_changed(&self) -> LayoutDirectionChanged {
      LayoutDirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::primaryScreenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primary_screen_changed(&self) -> PrimaryScreenChanged {
      PrimaryScreenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::fontDatabaseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn font_database_changed(&self) -> FontDatabaseChanged {
      FontDatabaseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::applicationStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_state_changed(&self) -> ApplicationStateChanged {
      ApplicationStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::commitDataRequest`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data_request(&self) -> CommitDataRequest {
      CommitDataRequest(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::screenAdded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_added(&self) -> ScreenAdded {
      ScreenAdded(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::saveStateRequest`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn save_state_request(&self) -> SaveStateRequest {
      SaveStateRequest(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::paletteChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn palette_changed(&self) -> PaletteChanged {
      PaletteChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QApplication::lastWindowClosed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn last_window_closed(&self) -> LastWindowClosed {
      LastWindowClosed(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Application`.
  pub struct Slots<'a>(&'a ::application::Application);
  /// Represents a built-in Qt slot `QApplication::autoSipEnabled`.
  ///
  /// An object of this type can be created from `Application` with `object.slots().auto_sip_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct AutoSipEnabled<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for AutoSipEnabled<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1autoSipEnabled()\0"
    }
  }
  /// Represents a built-in Qt slot `QApplication::aboutQt`.
  ///
  /// An object of this type can be created from `Application` with `object.slots().about_qt()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct AboutQt<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for AboutQt<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1aboutQt()\0"
    }
  }
  /// Represents a built-in Qt slot `QApplication::setAutoSipEnabled`.
  ///
  /// An object of this type can be created from `Application` with `object.slots().set_auto_sip_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct SetAutoSipEnabled<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for SetAutoSipEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAutoSipEnabled(const bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QApplication::closeAllWindows`.
  ///
  /// An object of this type can be created from `Application` with `object.slots().close_all_windows()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct CloseAllWindows<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for CloseAllWindows<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeAllWindows()\0"
    }
  }
  /// Represents a built-in Qt slot `QApplication::setStyleSheet`.
  ///
  /// An object of this type can be created from `Application` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Application` object.
  pub struct SetStyleSheet<'a>(&'a ::application::Application);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QApplication::autoSipEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn auto_sip_enabled(&self) -> AutoSipEnabled {
      AutoSipEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QApplication::aboutQt`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_qt(&self) -> AboutQt {
      AboutQt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QApplication::setAutoSipEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_auto_sip_enabled(&self) -> SetAutoSipEnabled {
      SetAutoSipEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QApplication::closeAllWindows`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_all_windows(&self) -> CloseAllWindows {
      CloseAllWindows(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QApplication::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
  }
  impl ::application::Application {
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

/// C++ type: <span style='color: green;'>```QApplication::ColorSpec```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ColorSpec {
  /// C++ enum variant: <span style='color: green;'>```NormalColor = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```CustomColor = 1```</span>
  Custom = 1,
  /// C++ enum variant: <span style='color: green;'>```ManyColor = 2```</span>
  Many = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::core_application::CoreApplication> for ::application::Application {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::core_application::CoreApplication {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QApplication_G_static_cast_QCoreApplication_ptr(self as *mut ::application::Application)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::core_application::CoreApplication {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QApplication_G_static_cast_QCoreApplication_ptr(self as *const ::application::Application as *mut ::application::Application) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::application::Application {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QApplication_G_static_cast_QObject_ptr(self as *mut ::application::Application) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QApplication_G_static_cast_QObject_ptr(self as *const ::application::Application as *mut ::application::Application) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::gui_application::GuiApplication> for ::application::Application {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::gui_application::GuiApplication {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QApplication_G_static_cast_QGuiApplication_ptr(self as *mut ::application::Application)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::gui_application::GuiApplication {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QApplication_G_static_cast_QGuiApplication_ptr(self as *const ::application::Application as *mut ::application::Application) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::application::Application> for ::qt_core::core_application::CoreApplication {
  unsafe fn static_cast_mut(&mut self) -> &mut ::application::Application {
    let ffi_result = ::ffi::qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QCoreApplication(self as *mut ::qt_core::core_application::CoreApplication);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::application::Application {
    let ffi_result = ::ffi::qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QCoreApplication(self as *const ::qt_core::core_application::CoreApplication as *mut ::qt_core::core_application::CoreApplication);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::application::Application> for ::qt_gui::gui_application::GuiApplication {
  unsafe fn static_cast_mut(&mut self) -> &mut ::application::Application {
    let ffi_result = ::ffi::qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QGuiApplication(self as *mut ::qt_gui::gui_application::GuiApplication);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::application::Application {
    let ffi_result = ::ffi::qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QGuiApplication(self as *const ::qt_gui::gui_application::GuiApplication as *mut ::qt_gui::gui_application::GuiApplication);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::application::Application> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::application::Application {
    let ffi_result =
      ::ffi::qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::application::Application {
    let ffi_result = ::ffi::qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::application::Application {
  type Target = ::qt_gui::gui_application::GuiApplication;
  fn deref(&self) -> &::qt_gui::gui_application::GuiApplication {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QApplication_G_static_cast_QGuiApplication_ptr(self as *const ::application::Application as *mut ::application::Application) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::application::Application {
  fn deref_mut(&mut self) -> &mut ::qt_gui::gui_application::GuiApplication {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QApplication_G_static_cast_QGuiApplication_ptr(self as *mut ::application::Application)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Application::alert](../struct.Application.html#method.alert) method.
  pub trait ApplicationAlertArgs {
    unsafe fn exec(self) -> ();
  }
  impl ApplicationAlertArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QApplication_alert_widget(widget)
    }
  }
  impl ApplicationAlertArgs for (*mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let widget = self.0;
      let duration = self.1;
      ::ffi::qt_widgets_c_QApplication_alert_widget_duration(widget, duration)
    }
  }
  /// This trait represents a set of arguments accepted by [Application::font_unsafe](../struct.Application.html#method.font_unsafe) method.
  pub trait ApplicationFontUnsafeArgs {
    unsafe fn exec(self) -> ::qt_gui::font::Font;
  }
  impl ApplicationFontUnsafeArgs for *const ::widget::Widget {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let arg1 = self;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QApplication_font_to_output_arg1(arg1, &mut object);
        object
      }
    }
  }
  impl ApplicationFontUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let class_name = self;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QApplication_font_to_output_className(class_name, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Application::new](../struct.Application.html#method.new) method.
  pub trait ApplicationNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::application::Application>;
  }
  impl<'a> ApplicationNewArgs for (&'a mut ::libc::c_int, *mut *mut ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::application::Application> {
      let argc = self.0;
      let argv = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QApplication_new_argc_argv(argc as *mut ::libc::c_int, argv);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ApplicationNewArgs for (&'a mut ::libc::c_int, *mut *mut ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::application::Application> {
      let argc = self.0;
      let argv = self.1;
      let arg3 = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QApplication_new_argc_argv_arg3(argc as *mut ::libc::c_int, argv, arg3);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Application::palette](../struct.Application.html#method.palette) method.
  pub trait ApplicationPaletteArgs {
    unsafe fn exec(self) -> ::qt_gui::palette::Palette;
  }
  impl ApplicationPaletteArgs for *const ::widget::Widget {
    unsafe fn exec(self) -> ::qt_gui::palette::Palette {
      let arg1 = self;
      {
        let mut object: ::qt_gui::palette::Palette =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QApplication_palette_to_output_arg1(arg1, &mut object);
        object
      }
    }
  }
  impl ApplicationPaletteArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::qt_gui::palette::Palette {
      let class_name = self;
      {
        let mut object: ::qt_gui::palette::Palette =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QApplication_palette_to_output_className(class_name, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Application::set_effect_enabled](../struct.Application.html#method.set_effect_enabled) method.
  pub trait ApplicationSetEffectEnabledArgs {
    fn exec(self) -> ();
  }
  impl<'a> ApplicationSetEffectEnabledArgs for &'a ::qt_core::qt::UIEffect {
    fn exec(self) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_widgets_c_QApplication_setEffectEnabled_arg1(arg1 as *const ::qt_core::qt::UIEffect) }
    }
  }
  impl<'a> ApplicationSetEffectEnabledArgs for (&'a ::qt_core::qt::UIEffect, bool) {
    fn exec(self) -> () {
      let arg1 = self.0;
      let enable = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QApplication_setEffectEnabled_arg1_enable(arg1 as *const ::qt_core::qt::UIEffect, enable)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Application::top_level_at](../struct.Application.html#method.top_level_at) method.
  pub trait ApplicationTopLevelAtArgs {
    fn exec(self) -> *mut ::widget::Widget;
  }
  impl<'a> ApplicationTopLevelAtArgs for &'a ::qt_core::point::Point {
    fn exec(self) -> *mut ::widget::Widget {
      let p = self;
      unsafe { ::ffi::qt_widgets_c_QApplication_topLevelAt_p(p as *const ::qt_core::point::Point) }
    }
  }
  impl ApplicationTopLevelAtArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> *mut ::widget::Widget {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QApplication_topLevelAt_x_y(x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Application::widget_at](../struct.Application.html#method.widget_at) method.
  pub trait ApplicationWidgetAtArgs {
    fn exec(self) -> *mut ::widget::Widget;
  }
  impl<'a> ApplicationWidgetAtArgs for &'a ::qt_core::point::Point {
    fn exec(self) -> *mut ::widget::Widget {
      let p = self;
      unsafe { ::ffi::qt_widgets_c_QApplication_widgetAt_p(p as *const ::qt_core::point::Point) }
    }
  }
  impl ApplicationWidgetAtArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> *mut ::widget::Widget {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QApplication_widgetAt_x_y(x, y) }
    }
  }
}



impl ::application::Application {
  pub fn create_and_exit<F: FnOnce(&mut ::application::Application) -> i32>(f: F) -> ! {
    let exit_code = {
      let mut args = ::qt_core::core_application::CoreApplicationArgs::from_real();
      let mut app = unsafe { ::application::Application::new(args.get()) };
      f(app.as_mut())
    };
    ::std::process::exit(exit_code)
  }
}
