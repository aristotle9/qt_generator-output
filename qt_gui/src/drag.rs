/// C++ type: <span style='color: green;'>```QDrag```</span>
#[repr(C)]
pub struct Drag(u8);

impl Drag {
  /// C++ method: <span style='color: green;'>```static void QDrag::cancel()```</span>
  ///
  ///
  pub fn cancel() {
    unsafe { ::ffi::qt_gui_c_QDrag_cancel() }
  }

  /// C++ method: <span style='color: green;'>```QPixmap QDrag::dragCursor(Qt::DropAction action) const```</span>
  ///
  ///
  pub fn drag_cursor(&self, action: &::qt_core::qt::DropAction) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QDrag_dragCursor_as_ptr(self as *const ::drag::Drag,
                                              action as *const ::qt_core::qt::DropAction)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QDrag::hotSpot() const```</span>
  ///
  ///
  pub fn hot_spot(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QDrag_hotSpot_to_output(self as *const ::drag::Drag, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDrag::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QDrag_metaObject(self as *const ::drag::Drag) }
  }

  /// C++ method: <span style='color: green;'>```QMimeData* QDrag::mimeData() const```</span>
  ///
  ///
  pub fn mime_data(&self) -> *mut ::qt_core::mime_data::MimeData {
    unsafe { ::ffi::qt_gui_c_QDrag_mimeData(self as *const ::drag::Drag) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDrag::QDrag(QObject* dragSource)```</span>
  ///
  ///
  pub unsafe fn new(drag_source: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::drag::Drag> {
    let ffi_result = ::ffi::qt_gui_c_QDrag_new(drag_source);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QPixmap QDrag::pixmap() const```</span>
  ///
  ///
  pub fn pixmap(&self) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDrag_pixmap_as_ptr(self as *const ::drag::Drag) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDrag::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QDrag_qt_metacall(self as *mut ::drag::Drag,
                                      arg1 as *const ::qt_core::meta_object::Call,
                                      arg2,
                                      arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDrag::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QDrag_qt_metacast(self as *mut ::drag::Drag, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDrag::setDragCursor(const QPixmap& cursor, Qt::DropAction action)```</span>
  ///
  ///
  pub fn set_drag_cursor(&mut self, cursor: &::pixmap::Pixmap, action: &::qt_core::qt::DropAction) {
    unsafe {
      ::ffi::qt_gui_c_QDrag_setDragCursor(self as *mut ::drag::Drag,
                                          cursor as *const ::pixmap::Pixmap,
                                          action as *const ::qt_core::qt::DropAction)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDrag::setHotSpot(const QPoint& hotspot)```</span>
  ///
  ///
  pub fn set_hot_spot(&mut self, hotspot: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QDrag_setHotSpot(self as *mut ::drag::Drag,
                                       hotspot as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDrag::setMimeData(QMimeData* data)```</span>
  ///
  ///
  pub unsafe fn set_mime_data(&mut self, data: *mut ::qt_core::mime_data::MimeData) {
    ::ffi::qt_gui_c_QDrag_setMimeData(self as *mut ::drag::Drag, data)
  }

  /// C++ method: <span style='color: green;'>```void QDrag::setPixmap(const QPixmap& arg1)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, arg1: &::pixmap::Pixmap) {
    unsafe { ::ffi::qt_gui_c_QDrag_setPixmap(self as *mut ::drag::Drag, arg1 as *const ::pixmap::Pixmap) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QDrag::source() const```</span>
  ///
  ///
  pub fn source(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QDrag_source(self as *const ::drag::Drag) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QDrag::target() const```</span>
  ///
  ///
  pub fn target(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QDrag_target(self as *const ::drag::Drag) }
  }

  /// C++ method: <span style='color: green;'>```static QString QDrag::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QDrag_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDrag::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QDrag_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::drag::Drag {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDrag_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Drag`.
  pub struct Signals<'a>(&'a ::drag::Drag);
  /// Represents a built-in Qt signal `QDrag::objectNameChanged`.
  ///
  /// An object of this type can be created from `Drag` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Drag` object.
  pub struct ObjectNameChanged<'a>(&'a ::drag::Drag);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QDrag::targetChanged`.
  ///
  /// An object of this type can be created from `Drag` with `object.signals().target_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Drag` object.
  pub struct TargetChanged<'a>(&'a ::drag::Drag);
  impl<'a> ::qt_core::connection::Receiver for TargetChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2targetChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TargetChanged<'a> {}
  /// Represents a built-in Qt signal `QDrag::actionChanged`.
  ///
  /// An object of this type can be created from `Drag` with `object.signals().action_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Drag` object.
  pub struct ActionChanged<'a>(&'a ::drag::Drag);
  impl<'a> ::qt_core::connection::Receiver for ActionChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::DropAction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2actionChanged(Qt::DropAction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDrag::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDrag::targetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_changed(&self) -> TargetChanged {
      TargetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDrag::actionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn action_changed(&self) -> ActionChanged {
      ActionChanged(self.0)
    }
  }
  impl ::drag::Drag {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::drag::Drag {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDrag_G_static_cast_QObject_ptr(self as *mut ::drag::Drag) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QDrag_G_static_cast_QObject_ptr(self as *const ::drag::Drag as *mut ::drag::Drag) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag::Drag> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag::Drag {
    let ffi_result = ::ffi::qt_gui_c_QDrag_G_static_cast_QDrag_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag::Drag {
    let ffi_result = ::ffi::qt_gui_c_QDrag_G_static_cast_QDrag_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::drag::Drag {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QDrag_G_static_cast_QObject_ptr(self as *const ::drag::Drag as *mut ::drag::Drag) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::drag::Drag {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDrag_G_static_cast_QObject_ptr(self as *mut ::drag::Drag) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
