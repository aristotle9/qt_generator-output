/// C++ type: <span style='color: green;'>```QGuiApplication```</span>
#[repr(C)]
pub struct GuiApplication(u8);

impl GuiApplication {
  /// C++ method: <span style='color: green;'>```static QList<QWindow*> QGuiApplication::allWindows()```</span>
  ///
  ///
  pub fn all_windows() -> ::list::ListWindowMutPtr {
    {
      let mut object: ::list::ListWindowMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_allWindows_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGuiApplication::applicationDisplayName()```</span>
  ///
  ///
  pub fn application_display_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_applicationDisplayName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::changeOverrideCursor(const QCursor& arg1)```</span>
  ///
  ///
  pub fn change_override_cursor(arg1: &::cursor::Cursor) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_changeOverrideCursor(arg1 as *const ::cursor::Cursor) }
  }

  /// C++ method: <span style='color: green;'>```static QClipboard* QGuiApplication::clipboard()```</span>
  ///
  ///
  pub fn clipboard() -> *mut ::clipboard::Clipboard {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_clipboard() }
  }

  /// C++ method: <span style='color: green;'>```static QString QGuiApplication::desktopFileName()```</span>
  ///
  ///
  pub fn desktop_file_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_desktopFileName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QGuiApplication::desktopSettingsAware()```</span>
  ///
  ///
  pub fn desktop_settings_aware() -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_desktopSettingsAware() }
  }

  /// C++ method: <span style='color: green;'>```double QGuiApplication::devicePixelRatio() const```</span>
  ///
  ///
  pub fn device_pixel_ratio(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_devicePixelRatio(self as *const ::gui_application::GuiApplication) }
  }

  /// C++ method: <span style='color: green;'>```static int QGuiApplication::exec()```</span>
  ///
  ///
  pub fn exec() -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_exec() }
  }

  /// C++ method: <span style='color: green;'>```static QObject* QGuiApplication::focusObject()```</span>
  ///
  ///
  pub fn focus_object() -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_focusObject() }
  }

  /// C++ method: <span style='color: green;'>```static QWindow* QGuiApplication::focusWindow()```</span>
  ///
  ///
  pub fn focus_window() -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_focusWindow() }
  }

  /// C++ method: <span style='color: green;'>```static QFont QGuiApplication::font()```</span>
  ///
  ///
  pub fn font() -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_font_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QInputMethod* QGuiApplication::inputMethod()```</span>
  ///
  ///
  pub fn input_method() -> *mut ::input_method::InputMethod {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_inputMethod() }
  }

  /// C++ method: <span style='color: green;'>```static bool QGuiApplication::isFallbackSessionManagementEnabled()```</span>
  ///
  ///
  pub fn is_fallback_session_management_enabled() -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_isFallbackSessionManagementEnabled() }
  }

  /// C++ method: <span style='color: green;'>```static bool QGuiApplication::isLeftToRight()```</span>
  ///
  ///
  pub fn is_left_to_right() -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_isLeftToRight() }
  }

  /// C++ method: <span style='color: green;'>```static bool QGuiApplication::isRightToLeft()```</span>
  ///
  ///
  pub fn is_right_to_left() -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_isRightToLeft() }
  }

  /// C++ method: <span style='color: green;'>```bool QGuiApplication::isSavingSession() const```</span>
  ///
  ///
  pub fn is_saving_session(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_isSavingSession(self as *const ::gui_application::GuiApplication) }
  }

  /// C++ method: <span style='color: green;'>```bool QGuiApplication::isSessionRestored() const```</span>
  ///
  ///
  pub fn is_session_restored(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_isSessionRestored(self as *const ::gui_application::GuiApplication) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGuiApplication::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_metaObject(self as *const ::gui_application::GuiApplication) }
  }

  /// C++ method: <span style='color: green;'>```static QWindow* QGuiApplication::modalWindow()```</span>
  ///
  ///
  pub fn modal_window() -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_modalWindow() }
  }

  /// C++ method: <span style='color: green;'>```QGuiApplication::QGuiApplication```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((&mut ::libc::c_int, *mut *mut ::libc::c_char)) -> ::cpp_utils::CppBox<::gui_application::GuiApplication>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGuiApplication::QGuiApplication(int& argc, char** argv)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&mut ::libc::c_int, *mut *mut ::libc::c_char, ::libc::c_int)) -> ::cpp_utils::CppBox<::gui_application::GuiApplication>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGuiApplication::QGuiApplication(int& argc, char** argv, int arg3 = ?)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::gui_application::GuiApplication>
    where Args: overloading::GuiApplicationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual bool QGuiApplication::notify(QObject* arg1, QEvent* arg2)```</span>
  ///
  ///
  pub unsafe fn notify(&mut self, arg1: *mut ::qt_core::object::Object, arg2: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_gui_c_QGuiApplication_notify(self as *mut ::gui_application::GuiApplication, arg1, arg2)
  }

  /// C++ method: <span style='color: green;'>```static QCursor* QGuiApplication::overrideCursor()```</span>
  ///
  ///
  pub fn override_cursor() -> *mut ::cursor::Cursor {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_overrideCursor() }
  }

  /// C++ method: <span style='color: green;'>```static QPalette QGuiApplication::palette()```</span>
  ///
  ///
  pub fn palette() -> ::palette::Palette {
    {
      let mut object: ::palette::Palette =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_palette_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)() QGuiApplication::platformFunction(const QByteArray& function)```</span>
  ///
  ///
  pub fn platform_function(function: &::qt_core::byte_array::ByteArray) -> extern "C" fn() {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_platformFunction(function as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGuiApplication::platformName()```</span>
  ///
  ///
  pub fn platform_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_platformName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QScreen* QGuiApplication::primaryScreen()```</span>
  ///
  ///
  pub fn primary_screen() -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_primaryScreen() }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGuiApplication::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QGuiApplication_qt_metacall(self as *mut ::gui_application::GuiApplication,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGuiApplication::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QGuiApplication_qt_metacast(self as *mut ::gui_application::GuiApplication, arg1)
  }

  /// C++ method: <span style='color: green;'>```static bool QGuiApplication::quitOnLastWindowClosed()```</span>
  ///
  ///
  pub fn quit_on_last_window_closed() -> bool {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_quitOnLastWindowClosed() }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::restoreOverrideCursor()```</span>
  ///
  ///
  pub fn restore_override_cursor() {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_restoreOverrideCursor() }
  }

  /// C++ method: <span style='color: green;'>```static QList<QScreen*> QGuiApplication::screens()```</span>
  ///
  ///
  pub fn screens() -> ::list::ListScreenMutPtr {
    {
      let mut object: ::list::ListScreenMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_screens_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QGuiApplication::sessionId() const```</span>
  ///
  ///
  pub fn session_id(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_sessionId_to_output(self as *const ::gui_application::GuiApplication,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QGuiApplication::sessionKey() const```</span>
  ///
  ///
  pub fn session_key(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_sessionKey_to_output(self as *const ::gui_application::GuiApplication,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setApplicationDisplayName(const QString& name)```</span>
  ///
  ///
  pub fn set_application_display_name(name: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setApplicationDisplayName(name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setDesktopFileName(const QString& name)```</span>
  ///
  ///
  pub fn set_desktop_file_name(name: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setDesktopFileName(name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setDesktopSettingsAware(bool on)```</span>
  ///
  ///
  pub fn set_desktop_settings_aware(on: bool) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setDesktopSettingsAware(on) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setFallbackSessionManagementEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_fallback_session_management_enabled(arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setFallbackSessionManagementEnabled(arg1) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setFont(const QFont& arg1)```</span>
  ///
  ///
  pub fn set_font(arg1: &::font::Font) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setFont(arg1 as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setLayoutDirection(Qt::LayoutDirection direction)```</span>
  ///
  ///
  pub fn set_layout_direction(direction: &::qt_core::qt::LayoutDirection) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setLayoutDirection(direction as *const ::qt_core::qt::LayoutDirection) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setOverrideCursor(const QCursor& arg1)```</span>
  ///
  ///
  pub fn set_override_cursor(arg1: &::cursor::Cursor) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setOverrideCursor(arg1 as *const ::cursor::Cursor) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setPalette(const QPalette& pal)```</span>
  ///
  ///
  pub fn set_palette(pal: &::palette::Palette) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setPalette(pal as *const ::palette::Palette) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setQuitOnLastWindowClosed(bool quit)```</span>
  ///
  ///
  pub fn set_quit_on_last_window_closed(quit: bool) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setQuitOnLastWindowClosed(quit) }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::setWindowIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_window_icon(icon: &::icon::Icon) {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_setWindowIcon(icon as *const ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```static QStyleHints* QGuiApplication::styleHints()```</span>
  ///
  ///
  pub fn style_hints() -> *mut ::style_hints::StyleHints {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_styleHints() }
  }

  /// C++ method: <span style='color: green;'>```static void QGuiApplication::sync()```</span>
  ///
  ///
  pub fn sync() {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_sync() }
  }

  /// C++ method: <span style='color: green;'>```static QWindow* QGuiApplication::topLevelAt(const QPoint& pos)```</span>
  ///
  ///
  pub fn top_level_at(pos: &::qt_core::point::Point) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QGuiApplication_topLevelAt(pos as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```static QList<QWindow*> QGuiApplication::topLevelWindows()```</span>
  ///
  ///
  pub fn top_level_windows() -> ::list::ListWindowMutPtr {
    {
      let mut object: ::list::ListWindowMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_topLevelWindows_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGuiApplication::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QGuiApplication_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGuiApplication::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QGuiApplication_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QIcon QGuiApplication::windowIcon()```</span>
  ///
  ///
  pub fn window_icon() -> ::icon::Icon {
    {
      let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_windowIcon_to_output(&mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::gui_application::GuiApplication {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QGuiApplication_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GuiApplication`.
  pub struct Signals<'a>(&'a ::gui_application::GuiApplication);
  /// Represents a built-in Qt signal `QGuiApplication::saveStateRequest`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().save_state_request()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct SaveStateRequest<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for SaveStateRequest<'a> {
    type Arguments = (&'static mut ::session_manager::SessionManager,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2saveStateRequest(QSessionManager&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SaveStateRequest<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::commitDataRequest`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().commit_data_request()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct CommitDataRequest<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for CommitDataRequest<'a> {
    type Arguments = (&'static mut ::session_manager::SessionManager,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2commitDataRequest(QSessionManager&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CommitDataRequest<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::organizationNameChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().organization_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct OrganizationNameChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for OrganizationNameChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2organizationNameChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OrganizationNameChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::primaryScreenChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().primary_screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct PrimaryScreenChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for PrimaryScreenChanged<'a> {
    type Arguments = (*mut ::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primaryScreenChanged(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimaryScreenChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::organizationDomainChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().organization_domain_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct OrganizationDomainChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for OrganizationDomainChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2organizationDomainChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OrganizationDomainChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::applicationNameChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().application_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct ApplicationNameChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for ApplicationNameChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2applicationNameChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ApplicationNameChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::applicationDisplayNameChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().application_display_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct ApplicationDisplayNameChanged<'a>(&'a ::gui_application::GuiApplication);
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
  /// Represents a built-in Qt signal `QGuiApplication::paletteChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().palette_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct PaletteChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for PaletteChanged<'a> {
    type Arguments = (&'static ::palette::Palette,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2paletteChanged(const QPalette&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PaletteChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::screenAdded`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().screen_added()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct ScreenAdded<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for ScreenAdded<'a> {
    type Arguments = (*mut ::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenAdded(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenAdded<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::layoutDirectionChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().layout_direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct LayoutDirectionChanged<'a>(&'a ::gui_application::GuiApplication);
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
  /// Represents a built-in Qt signal `QGuiApplication::aboutToQuit`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().about_to_quit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct AboutToQuit<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for AboutToQuit<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToQuit()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToQuit<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::applicationVersionChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().application_version_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct ApplicationVersionChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for ApplicationVersionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2applicationVersionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ApplicationVersionChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::screenRemoved`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().screen_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct ScreenRemoved<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for ScreenRemoved<'a> {
    type Arguments = (*mut ::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenRemoved(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenRemoved<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::focusObjectChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().focus_object_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct FocusObjectChanged<'a>(&'a ::gui_application::GuiApplication);
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
  /// Represents a built-in Qt signal `QGuiApplication::fontDatabaseChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().font_database_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct FontDatabaseChanged<'a>(&'a ::gui_application::GuiApplication);
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
  /// Represents a built-in Qt signal `QGuiApplication::applicationStateChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().application_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct ApplicationStateChanged<'a>(&'a ::gui_application::GuiApplication);
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
  /// Represents a built-in Qt signal `QGuiApplication::focusWindowChanged`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().focus_window_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct FocusWindowChanged<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for FocusWindowChanged<'a> {
    type Arguments = (*mut ::window::Window,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusWindowChanged(QWindow*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusWindowChanged<'a> {}
  /// Represents a built-in Qt signal `QGuiApplication::lastWindowClosed`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.signals().last_window_closed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct LastWindowClosed<'a>(&'a ::gui_application::GuiApplication);
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
    /// Returns an object representing a built-in Qt signal `QGuiApplication::saveStateRequest`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn save_state_request(&self) -> SaveStateRequest {
      SaveStateRequest(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::commitDataRequest`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn commit_data_request(&self) -> CommitDataRequest {
      CommitDataRequest(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::organizationNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn organization_name_changed(&self) -> OrganizationNameChanged {
      OrganizationNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::primaryScreenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primary_screen_changed(&self) -> PrimaryScreenChanged {
      PrimaryScreenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::organizationDomainChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn organization_domain_changed(&self) -> OrganizationDomainChanged {
      OrganizationDomainChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::applicationNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_name_changed(&self) -> ApplicationNameChanged {
      ApplicationNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::applicationDisplayNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_display_name_changed(&self) -> ApplicationDisplayNameChanged {
      ApplicationDisplayNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::paletteChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn palette_changed(&self) -> PaletteChanged {
      PaletteChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::screenAdded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_added(&self) -> ScreenAdded {
      ScreenAdded(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::layoutDirectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_direction_changed(&self) -> LayoutDirectionChanged {
      LayoutDirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::aboutToQuit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_quit(&self) -> AboutToQuit {
      AboutToQuit(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::applicationVersionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_version_changed(&self) -> ApplicationVersionChanged {
      ApplicationVersionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::screenRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_removed(&self) -> ScreenRemoved {
      ScreenRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::focusObjectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_object_changed(&self) -> FocusObjectChanged {
      FocusObjectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::fontDatabaseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn font_database_changed(&self) -> FontDatabaseChanged {
      FontDatabaseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::applicationStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_state_changed(&self) -> ApplicationStateChanged {
      ApplicationStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::focusWindowChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_window_changed(&self) -> FocusWindowChanged {
      FocusWindowChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGuiApplication::lastWindowClosed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn last_window_closed(&self) -> LastWindowClosed {
      LastWindowClosed(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GuiApplication`.
  pub struct Slots<'a>(&'a ::gui_application::GuiApplication);
  /// Represents a built-in Qt slot `QGuiApplication::quit`.
  ///
  /// An object of this type can be created from `GuiApplication` with `object.slots().quit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GuiApplication` object.
  pub struct Quit<'a>(&'a ::gui_application::GuiApplication);
  impl<'a> ::qt_core::connection::Receiver for Quit<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1quit()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGuiApplication::quit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn quit(&self) -> Quit {
      Quit(self.0)
    }
  }
  impl ::gui_application::GuiApplication {
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

impl ::cpp_utils::StaticCast<::qt_core::core_application::CoreApplication> for ::gui_application::GuiApplication {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::core_application::CoreApplication {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QCoreApplication_ptr(self as *mut ::gui_application::GuiApplication) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::core_application::CoreApplication {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QCoreApplication_ptr(self as *const ::gui_application::GuiApplication as *mut ::gui_application::GuiApplication) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::gui_application::GuiApplication {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QObject_ptr(self as *mut ::gui_application::GuiApplication)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QObject_ptr(self as *const ::gui_application::GuiApplication as *mut ::gui_application::GuiApplication) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gui_application::GuiApplication> for ::qt_core::core_application::CoreApplication {
unsafe fn static_cast_mut(&mut self) -> &mut ::gui_application::GuiApplication {
let ffi_result = ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QCoreApplication(self as *mut ::qt_core::core_application::CoreApplication);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::gui_application::GuiApplication {
let ffi_result = ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QCoreApplication(self as *const ::qt_core::core_application::CoreApplication as *mut ::qt_core::core_application::CoreApplication);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::gui_application::GuiApplication> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gui_application::GuiApplication {
    let ffi_result = ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gui_application::GuiApplication {
    let ffi_result = ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::gui_application::GuiApplication {
  type Target = ::qt_core::core_application::CoreApplication;
  fn deref(&self) -> &::qt_core::core_application::CoreApplication {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QCoreApplication_ptr(self as *const ::gui_application::GuiApplication as *mut ::gui_application::GuiApplication) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::gui_application::GuiApplication {
  fn deref_mut(&mut self) -> &mut ::qt_core::core_application::CoreApplication {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGuiApplication_G_static_cast_QCoreApplication_ptr(self as *mut ::gui_application::GuiApplication) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GuiApplication::new](../struct.GuiApplication.html#method.new) method.
  pub trait GuiApplicationNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::gui_application::GuiApplication>;
  }
  impl<'a> GuiApplicationNewArgs for (&'a mut ::libc::c_int, *mut *mut ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::gui_application::GuiApplication> {
      let argc = self.0;
      let argv = self.1;
      let ffi_result = ::ffi::qt_gui_c_QGuiApplication_new_argc_argv(argc as *mut ::libc::c_int, argv);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GuiApplicationNewArgs for (&'a mut ::libc::c_int, *mut *mut ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::gui_application::GuiApplication> {
      let argc = self.0;
      let argv = self.1;
      let arg3 = self.2;
      let ffi_result = ::ffi::qt_gui_c_QGuiApplication_new_argc_argv_arg3(argc as *mut ::libc::c_int, argv, arg3);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}



impl ::gui_application::GuiApplication {
  pub fn create_and_exit<F: FnOnce(&mut ::gui_application::GuiApplication) -> i32>(f: F) -> ! {
    let exit_code = {
      let mut args = ::qt_core::core_application::CoreApplicationArgs::from_real();
      let mut app = unsafe { ::gui_application::GuiApplication::new(args.get()) };
      f(app.as_mut())
    };
    ::std::process::exit(exit_code)
  }
}
