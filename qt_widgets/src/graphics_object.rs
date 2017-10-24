/// C++ type: <span style='color: green;'>```QGraphicsObject```</span>
#[repr(C)]
pub struct GraphicsObject(u8);

impl GraphicsObject {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsObject::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsObject_metaObject(self as *const ::graphics_object::GraphicsObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsObject::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsObject_qt_metacall(self as *mut ::graphics_object::GraphicsObject,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsObject::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsObject_qt_metacast(self as *mut ::graphics_object::GraphicsObject, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsObject::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsObject_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsObject::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsObject_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsObject::ungrabGesture(Qt::GestureType type)```</span>
  ///
  ///
  pub fn ungrab_gesture(&mut self, type_: &::qt_core::qt::GestureType) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsObject_ungrabGesture(self as *mut ::graphics_object::GraphicsObject,
                                                        type_ as *const ::qt_core::qt::GestureType)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_object::GraphicsObject {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsObject_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsObject`.
  pub struct Signals<'a>(&'a ::graphics_object::GraphicsObject);
  /// Represents a built-in Qt signal `QGraphicsObject::widthChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct WidthChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::parentChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct ParentChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::childrenChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().children_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct ChildrenChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for ChildrenChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2childrenChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ChildrenChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_object::GraphicsObject);
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
  /// Represents a built-in Qt signal `QGraphicsObject::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::heightChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct HeightChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::xChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct XChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for XChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::yChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct YChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for YChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::scaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct ScaleChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for ScaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::opacityChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct OpacityChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for OpacityChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2opacityChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OpacityChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::zChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct ZChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for ZChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2zChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ZChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::visibleChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct VisibleChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for VisibleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsObject::rotationChanged`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.signals().rotation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct RotationChanged<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for RotationChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rotationChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RotationChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::childrenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn children_changed(&self) -> ChildrenChanged {
      ChildrenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::zChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_changed(&self) -> ZChanged {
      ZChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsObject::rotationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_changed(&self) -> RotationChanged {
      RotationChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsObject`.
  pub struct Slots<'a>(&'a ::graphics_object::GraphicsObject);
  /// Represents a built-in Qt slot `QGraphicsObject::updateMicroFocus`.
  ///
  /// An object of this type can be created from `GraphicsObject` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsObject` object.
  pub struct UpdateMicroFocus<'a>(&'a ::graphics_object::GraphicsObject);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsObject::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
  }
  impl ::graphics_object::GraphicsObject {
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

impl ::cpp_utils::DynamicCast<::graphics_object::GraphicsObject> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_object::GraphicsObject> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsObject_G_dynamic_cast_QGraphicsObject_ptr(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_object::GraphicsObject> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsObject_G_dynamic_cast_QGraphicsObject_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_object::GraphicsObject {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QObject_ptr(self as *mut ::graphics_object::GraphicsObject)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QObject_ptr(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_object::GraphicsObject {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_object::GraphicsObject> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_object::GraphicsObject {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_object::GraphicsObject {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_object::GraphicsObject> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_object::GraphicsObject {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_object::GraphicsObject {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}
