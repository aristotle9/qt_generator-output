/// C++ type: <span style='color: green;'>```QOpenGLWidget```</span>
#[repr(C)]
pub struct OpenGLWidget(u8);

impl OpenGLWidget {
  /// C++ method: <span style='color: green;'>```QOpenGLContext* QOpenGLWidget::context() const```</span>
  ///
  ///
  pub fn context(&self) -> *mut ::qt_gui::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_context(self as *const ::opengl_widget::OpenGLWidget) }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLWidget::defaultFramebufferObject() const```</span>
  ///
  ///
  pub fn default_framebuffer_object(&self) -> u32 {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_defaultFramebufferObject(self as *const ::opengl_widget::OpenGLWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLWidget::doneCurrent()```</span>
  ///
  ///
  pub fn done_current(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_doneCurrent(self as *mut ::opengl_widget::OpenGLWidget) }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat QOpenGLWidget::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_gui::surface_format::SurfaceFormat {
    {
      let mut object: ::qt_gui::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QOpenGLWidget_format_to_output(self as *const ::opengl_widget::OpenGLWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QImage QOpenGLWidget::grabFramebuffer()```</span>
  ///
  ///
  pub fn grab_framebuffer(&mut self) -> ::cpp_utils::CppBox<::qt_gui::image::Image> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_grabFramebuffer_as_ptr(self as *mut ::opengl_widget::OpenGLWidget) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLWidget::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_isValid(self as *const ::opengl_widget::OpenGLWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLWidget::makeCurrent()```</span>
  ///
  ///
  pub fn make_current(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_makeCurrent(self as *mut ::opengl_widget::OpenGLWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_metaObject(self as *const ::opengl_widget::OpenGLWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QOpenGLWidget_qt_metacall(self as *mut ::opengl_widget::OpenGLWidget,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QOpenGLWidget_qt_metacast(self as *mut ::opengl_widget::OpenGLWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLWidget::setFormat(const QSurfaceFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_gui::surface_format::SurfaceFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QOpenGLWidget_setFormat(self as *mut ::opengl_widget::OpenGLWidget,
                                                  format as *const ::qt_gui::surface_format::SurfaceFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLWidget::setUpdateBehavior(QOpenGLWidget::UpdateBehavior updateBehavior)```</span>
  ///
  ///
  pub fn set_update_behavior(&mut self, update_behavior: ::opengl_widget::UpdateBehavior) {
    unsafe {
      ::ffi::qt_widgets_c_QOpenGLWidget_setUpdateBehavior(self as *mut ::opengl_widget::OpenGLWidget, update_behavior)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QOpenGLWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QOpenGLWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLWidget::UpdateBehavior QOpenGLWidget::updateBehavior() const```</span>
  ///
  ///
  pub fn update_behavior(&self) -> ::opengl_widget::UpdateBehavior {
    unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_updateBehavior(self as *const ::opengl_widget::OpenGLWidget) }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_widget::OpenGLWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QOpenGLWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLWidget`.
  pub struct Signals<'a>(&'a ::opengl_widget::OpenGLWidget);
  /// Represents a built-in Qt signal `QOpenGLWidget::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct WindowIconTextChanged<'a>(&'a ::opengl_widget::OpenGLWidget);
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
  /// Represents a built-in Qt signal `QOpenGLWidget::aboutToResize`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().about_to_resize()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct AboutToResize<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for AboutToResize<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToResize()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToResize<'a> {}
  /// Represents a built-in Qt signal `QOpenGLWidget::aboutToCompose`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().about_to_compose()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct AboutToCompose<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for AboutToCompose<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToCompose()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToCompose<'a> {}
  /// Represents a built-in Qt signal `QOpenGLWidget::resized`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Resized<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Resized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2resized()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Resized<'a> {}
  /// Represents a built-in Qt signal `QOpenGLWidget::windowIconChanged`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct WindowIconChanged<'a>(&'a ::opengl_widget::OpenGLWidget);
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
  /// Represents a built-in Qt signal `QOpenGLWidget::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::opengl_widget::OpenGLWidget);
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
  /// Represents a built-in Qt signal `QOpenGLWidget::frameSwapped`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().frame_swapped()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct FrameSwapped<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for FrameSwapped<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2frameSwapped()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FrameSwapped<'a> {}
  /// Represents a built-in Qt signal `QOpenGLWidget::windowTitleChanged`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct WindowTitleChanged<'a>(&'a ::opengl_widget::OpenGLWidget);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::aboutToResize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_resize(&self) -> AboutToResize {
      AboutToResize(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::aboutToCompose`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_compose(&self) -> AboutToCompose {
      AboutToCompose(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::resized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resized(&self) -> Resized {
      Resized(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::frameSwapped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn frame_swapped(&self) -> FrameSwapped {
      FrameSwapped(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLWidget::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `OpenGLWidget`.
  pub struct Slots<'a>(&'a ::opengl_widget::OpenGLWidget);
  /// Represents a built-in Qt slot `QOpenGLWidget::show`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Show<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setDisabled`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetDisabled<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setEnabled`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetEnabled<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::showFullScreen`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct ShowFullScreen<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::update`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Update<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::showMaximized`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct ShowMaximized<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setVisible`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetVisible<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::lower`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Lower<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setFocus`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetFocus<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::showNormal`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct ShowNormal<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setStyleSheet`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetStyleSheet<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::raise`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Raise<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::hide`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Hide<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setWindowTitle`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetWindowTitle<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::showMinimized`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct ShowMinimized<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setWindowModified`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetWindowModified<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::repaint`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Repaint<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::close`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct Close<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLWidget::setHidden`.
  ///
  /// An object of this type can be created from `OpenGLWidget` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWidget` object.
  pub struct SetHidden<'a>(&'a ::opengl_widget::OpenGLWidget);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLWidget::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
  }
  impl ::opengl_widget::OpenGLWidget {
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

/// C++ type: <span style='color: green;'>```QOpenGLWidget::UpdateBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UpdateBehavior {
  /// C++ enum variant: <span style='color: green;'>```NoPartialUpdate = 0```</span>
  NoPartialUpdate = 0,
  /// C++ enum variant: <span style='color: green;'>```PartialUpdate = 1```</span>
  PartialUpdate = 1,
}

impl ::cpp_utils::DynamicCast<::opengl_widget::OpenGLWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::opengl_widget::OpenGLWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_G_dynamic_cast_QOpenGLWidget_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::opengl_widget::OpenGLWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_G_dynamic_cast_QOpenGLWidget_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_widget::OpenGLWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QObject_ptr(self as *mut ::opengl_widget::OpenGLWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QObject_ptr(self as *const ::opengl_widget::OpenGLWidget as *mut ::opengl_widget::OpenGLWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::opengl_widget::OpenGLWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::opengl_widget::OpenGLWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QPaintDevice_ptr(self as *const ::opengl_widget::OpenGLWidget as *mut ::opengl_widget::OpenGLWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::opengl_widget::OpenGLWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QWidget_ptr(self as *mut ::opengl_widget::OpenGLWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QWidget_ptr(self as *const ::opengl_widget::OpenGLWidget as *mut ::opengl_widget::OpenGLWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_widget::OpenGLWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_widget::OpenGLWidget {
    let ffi_result = ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_widget::OpenGLWidget {
    let ffi_result = ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_widget::OpenGLWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_widget::OpenGLWidget {
    let ffi_result = ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_widget::OpenGLWidget {
    let ffi_result = ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_widget::OpenGLWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_widget::OpenGLWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_widget::OpenGLWidget {
    let ffi_result = ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_widget::OpenGLWidget {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QWidget_ptr(self as *const ::opengl_widget::OpenGLWidget as *mut ::opengl_widget::OpenGLWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_widget::OpenGLWidget {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QOpenGLWidget_G_static_cast_QWidget_ptr(self as *mut ::opengl_widget::OpenGLWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
