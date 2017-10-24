/// C++ type: <span style='color: green;'>```QFrame```</span>
#[repr(C)]
pub struct Frame(u8);

impl Frame {
  /// C++ method: <span style='color: green;'>```QRect QFrame::frameRect() const```</span>
  ///
  ///
  pub fn frame_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFrame_frameRect_to_output(self as *const ::frame::Frame, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFrame::Shadow QFrame::frameShadow() const```</span>
  ///
  ///
  pub fn frame_shadow(&self) -> ::frame::Shadow {
    unsafe { ::ffi::qt_widgets_c_QFrame_frameShadow(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```QFrame::Shape QFrame::frameShape() const```</span>
  ///
  ///
  pub fn frame_shape(&self) -> ::frame::Shape {
    unsafe { ::ffi::qt_widgets_c_QFrame_frameShape(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```int QFrame::frameStyle() const```</span>
  ///
  ///
  pub fn frame_style(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFrame_frameStyle(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```int QFrame::frameWidth() const```</span>
  ///
  ///
  pub fn frame_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFrame_frameWidth(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```int QFrame::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFrame_lineWidth(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFrame::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QFrame_metaObject(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```int QFrame::midLineWidth() const```</span>
  ///
  ///
  pub fn mid_line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QFrame_midLineWidth(self as *const ::frame::Frame) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QFrame::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QFrame_qt_metacall(self as *mut ::frame::Frame,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QFrame::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QFrame_qt_metacast(self as *mut ::frame::Frame, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QFrame::setFrameRect(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_frame_rect(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QFrame_setFrameRect(self as *mut ::frame::Frame,
                                              arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFrame::setFrameShadow(QFrame::Shadow arg1)```</span>
  ///
  ///
  pub fn set_frame_shadow(&mut self, arg1: ::frame::Shadow) {
    unsafe { ::ffi::qt_widgets_c_QFrame_setFrameShadow(self as *mut ::frame::Frame, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFrame::setFrameShape(QFrame::Shape arg1)```</span>
  ///
  ///
  pub fn set_frame_shape(&mut self, arg1: ::frame::Shape) {
    unsafe { ::ffi::qt_widgets_c_QFrame_setFrameShape(self as *mut ::frame::Frame, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFrame::setFrameStyle(int arg1)```</span>
  ///
  ///
  pub fn set_frame_style(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFrame_setFrameStyle(self as *mut ::frame::Frame, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFrame::setLineWidth(int arg1)```</span>
  ///
  ///
  pub fn set_line_width(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFrame_setLineWidth(self as *mut ::frame::Frame, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFrame::setMidLineWidth(int arg1)```</span>
  ///
  ///
  pub fn set_mid_line_width(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QFrame_setMidLineWidth(self as *mut ::frame::Frame, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QFrame::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFrame_sizeHint_to_output(self as *const ::frame::Frame, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFrame::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFrame_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFrame::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFrame_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::frame::Frame {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFrame_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Frame`.
  pub struct Signals<'a>(&'a ::frame::Frame);
  /// Represents a built-in Qt signal `QFrame::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `Frame` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct WindowIconTextChanged<'a>(&'a ::frame::Frame);
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
  /// Represents a built-in Qt signal `QFrame::windowIconChanged`.
  ///
  /// An object of this type can be created from `Frame` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct WindowIconChanged<'a>(&'a ::frame::Frame);
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
  /// Represents a built-in Qt signal `QFrame::windowTitleChanged`.
  ///
  /// An object of this type can be created from `Frame` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct WindowTitleChanged<'a>(&'a ::frame::Frame);
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
  /// Represents a built-in Qt signal `QFrame::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `Frame` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::frame::Frame);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFrame::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFrame::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFrame::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFrame::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Frame`.
  pub struct Slots<'a>(&'a ::frame::Frame);
  /// Represents a built-in Qt slot `QFrame::showNormal`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct ShowNormal<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::close`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Close<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::repaint`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Repaint<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::showFullScreen`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct ShowFullScreen<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setWindowTitle`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetWindowTitle<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setHidden`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetHidden<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setVisible`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetVisible<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::hide`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Hide<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::update`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Update<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::updateMicroFocus`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct UpdateMicroFocus<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setDisabled`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetDisabled<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setWindowModified`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetWindowModified<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::showMinimized`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct ShowMinimized<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::show`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Show<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::lower`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Lower<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::showMaximized`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct ShowMaximized<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setEnabled`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetEnabled<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setStyleSheet`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetStyleSheet<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::setFocus`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct SetFocus<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QFrame::raise`.
  ///
  /// An object of this type can be created from `Frame` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Frame` object.
  pub struct Raise<'a>(&'a ::frame::Frame);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QFrame::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFrame::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
  }
  impl ::frame::Frame {
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

/// C++ type: <span style='color: green;'>```QFrame::Shadow```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Shadow {
  /// C++ enum variant: <span style='color: green;'>```Plain = 16```</span>
  Plain = 16,
  /// C++ enum variant: <span style='color: green;'>```Raised = 32```</span>
  Raised = 32,
  /// C++ enum variant: <span style='color: green;'>```Sunken = 48```</span>
  Sunken = 48,
}

/// C++ type: <span style='color: green;'>```QFrame::Shape```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Shape {
  /// C++ enum variant: <span style='color: green;'>```NoFrame = 0```</span>
  NoFrame = 0,
  /// C++ enum variant: <span style='color: green;'>```Box = 1```</span>
  Box = 1,
  /// C++ enum variant: <span style='color: green;'>```Panel = 2```</span>
  Panel = 2,
  /// C++ enum variant: <span style='color: green;'>```WinPanel = 3```</span>
  WinPanel = 3,
  /// C++ enum variant: <span style='color: green;'>```HLine = 4```</span>
  HLine = 4,
  /// C++ enum variant: <span style='color: green;'>```VLine = 5```</span>
  VLine = 5,
  /// C++ enum variant: <span style='color: green;'>```StyledPanel = 6```</span>
  StyledPanel = 6,
}

/// C++ type: <span style='color: green;'>```QFrame::StyleMask```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleMask {
  /// C++ enum variant: <span style='color: green;'>```Shape_Mask = 15```</span>
  Shape = 15,
  /// C++ enum variant: <span style='color: green;'>```Shadow_Mask = 240```</span>
  Shadow = 240,
}

impl ::cpp_utils::DynamicCast<::frame::Frame> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::frame::Frame> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFrame_G_dynamic_cast_QFrame_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::frame::Frame> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFrame_G_dynamic_cast_QFrame_ptr(self as *const ::widget::Widget as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::frame::Frame {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFrame_G_static_cast_QObject_ptr(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFrame_G_static_cast_QObject_ptr(self as *const ::frame::Frame as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::frame::Frame {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFrame_G_static_cast_QPaintDevice_ptr(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFrame_G_static_cast_QPaintDevice_ptr(self as *const ::frame::Frame as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::frame::Frame {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFrame_G_static_cast_QWidget_ptr(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFrame_G_static_cast_QWidget_ptr(self as *const ::frame::Frame as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::frame::Frame> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      ::ffi::qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = ::ffi::qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::frame::Frame> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = ::ffi::qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = ::ffi::qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::frame::Frame> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = ::ffi::qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = ::ffi::qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::frame::Frame {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFrame_G_static_cast_QWidget_ptr(self as *const ::frame::Frame as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::frame::Frame {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFrame_G_static_cast_QWidget_ptr(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
