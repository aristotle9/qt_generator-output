/// C++ type: <span style='color: green;'>```QSplitterHandle```</span>
#[repr(C)]
pub struct SplitterHandle(u8);

impl SplitterHandle {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSplitterHandle::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSplitterHandle_metaObject(self as *const ::splitter_handle::SplitterHandle) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSplitterHandle::QSplitterHandle(Qt::Orientation o, QSplitter* parent)```</span>
  ///
  ///
  pub unsafe fn new(o: &::qt_core::qt::Orientation,
                    parent: *mut ::splitter::Splitter)
                    -> ::cpp_utils::CppBox<::splitter_handle::SplitterHandle> {
    let ffi_result = ::ffi::qt_widgets_c_QSplitterHandle_new(o as *const ::qt_core::qt::Orientation, parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool QSplitterHandle::opaqueResize() const```</span>
  ///
  ///
  pub fn opaque_resize(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QSplitterHandle_opaqueResize(self as *const ::splitter_handle::SplitterHandle) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSplitterHandle::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSplitterHandle_qt_metacall(self as *mut ::splitter_handle::SplitterHandle,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSplitterHandle::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSplitterHandle_qt_metacast(self as *mut ::splitter_handle::SplitterHandle, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSplitterHandle::setOrientation(Qt::Orientation o)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, o: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QSplitterHandle_setOrientation(self as *mut ::splitter_handle::SplitterHandle,
                                                         o as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSplitterHandle::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSplitterHandle_sizeHint_to_output(self as *const ::splitter_handle::SplitterHandle,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSplitter* QSplitterHandle::splitter() const```</span>
  ///
  ///
  pub fn splitter(&self) -> *mut ::splitter::Splitter {
    unsafe { ::ffi::qt_widgets_c_QSplitterHandle_splitter(self as *const ::splitter_handle::SplitterHandle) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSplitterHandle::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSplitterHandle_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSplitterHandle::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSplitterHandle_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::splitter_handle::SplitterHandle {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSplitterHandle_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SplitterHandle`.
  pub struct Signals<'a>(&'a ::splitter_handle::SplitterHandle);
  /// Represents a built-in Qt signal `QSplitterHandle::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::splitter_handle::SplitterHandle);
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
  /// Represents a built-in Qt signal `QSplitterHandle::windowTitleChanged`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct WindowTitleChanged<'a>(&'a ::splitter_handle::SplitterHandle);
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
  /// Represents a built-in Qt signal `QSplitterHandle::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct WindowIconTextChanged<'a>(&'a ::splitter_handle::SplitterHandle);
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
  /// Represents a built-in Qt signal `QSplitterHandle::windowIconChanged`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct WindowIconChanged<'a>(&'a ::splitter_handle::SplitterHandle);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSplitterHandle::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplitterHandle::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplitterHandle::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplitterHandle::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SplitterHandle`.
  pub struct Slots<'a>(&'a ::splitter_handle::SplitterHandle);
  /// Represents a built-in Qt slot `QSplitterHandle::repaint`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Repaint<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::showMaximized`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct ShowMaximized<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setVisible`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetVisible<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setHidden`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetHidden<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::showMinimized`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct ShowMinimized<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::showFullScreen`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct ShowFullScreen<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::update`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Update<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setWindowModified`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetWindowModified<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setWindowTitle`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetWindowTitle<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::updateMicroFocus`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct UpdateMicroFocus<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setEnabled`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetEnabled<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::close`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Close<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setFocus`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetFocus<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setStyleSheet`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetStyleSheet<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::raise`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Raise<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::showNormal`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct ShowNormal<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::hide`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Hide<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::setDisabled`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct SetDisabled<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::show`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Show<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplitterHandle::lower`.
  ///
  /// An object of this type can be created from `SplitterHandle` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplitterHandle` object.
  pub struct Lower<'a>(&'a ::splitter_handle::SplitterHandle);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplitterHandle::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
  }
  impl ::splitter_handle::SplitterHandle {
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

impl ::cpp_utils::DynamicCast<::splitter_handle::SplitterHandle> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::splitter_handle::SplitterHandle> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_dynamic_cast_QSplitterHandle_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::splitter_handle::SplitterHandle> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_dynamic_cast_QSplitterHandle_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::splitter_handle::SplitterHandle {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QObject_ptr(self as *mut ::splitter_handle::SplitterHandle)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QObject_ptr(self as *const ::splitter_handle::SplitterHandle as *mut ::splitter_handle::SplitterHandle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::splitter_handle::SplitterHandle {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QPaintDevice_ptr(self as *mut ::splitter_handle::SplitterHandle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QPaintDevice_ptr(self as *const ::splitter_handle::SplitterHandle as *mut ::splitter_handle::SplitterHandle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::splitter_handle::SplitterHandle {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QWidget_ptr(self as *mut ::splitter_handle::SplitterHandle)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QWidget_ptr(self as *const ::splitter_handle::SplitterHandle as *mut ::splitter_handle::SplitterHandle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter_handle::SplitterHandle> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter_handle::SplitterHandle {
    let ffi_result = ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter_handle::SplitterHandle {
    let ffi_result = ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter_handle::SplitterHandle> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter_handle::SplitterHandle {
    let ffi_result = ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter_handle::SplitterHandle {
    let ffi_result = ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splitter_handle::SplitterHandle> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splitter_handle::SplitterHandle {
    let ffi_result =
      ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splitter_handle::SplitterHandle {
    let ffi_result = ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::splitter_handle::SplitterHandle {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QWidget_ptr(self as *const ::splitter_handle::SplitterHandle as *mut ::splitter_handle::SplitterHandle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::splitter_handle::SplitterHandle {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplitterHandle_G_static_cast_QWidget_ptr(self as *mut ::splitter_handle::SplitterHandle)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
