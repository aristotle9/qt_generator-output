/// C++ type: <span style='color: green;'>```Qt3DRender::QFilterKey```</span>
#[repr(C)]
pub struct FilterKey(u8);

impl FilterKey {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QFilterKey::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_metaObject(self as *const ::filter_key::FilterKey) }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DRender::QFilterKey::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_name_to_output(self as *const ::filter_key::FilterKey, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFilterKey::QFilterKey()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::filter_key::FilterKey> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFilterKey::QFilterKey(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::filter_key::FilterKey> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QFilterKey::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_qt_metacall(self as *mut ::filter_key::FilterKey,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QFilterKey::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_qt_metacast(self as *mut ::filter_key::FilterKey, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QFilterKey::setName(const QString& customType)```</span>
  ///
  ///
  pub fn set_name(&mut self, custom_type: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_setName(self as *mut ::filter_key::FilterKey,
                                                          custom_type as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QFilterKey::setValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_setValue(self as *mut ::filter_key::FilterKey,
                                                           value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QFilterKey::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QFilterKey::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant Qt3DRender::QFilterKey::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_value_to_output(self as *const ::filter_key::FilterKey,
                                                                    &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::filter_key::FilterKey {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QFilterKey_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FilterKey`.
  pub struct Signals<'a>(&'a ::filter_key::FilterKey);
  /// Represents a built-in Qt signal `Qt3DRender::QFilterKey::valueChanged`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct ValueChanged<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for ValueChanged<'a> {
    type Arguments = (&'static ::qt_core::variant::Variant,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(const QVariant&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QFilterKey::enabledChanged`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct EnabledChanged<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QFilterKey::nodeDestroyed`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct NodeDestroyed<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QFilterKey::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QFilterKey::nameChanged`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.signals().name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct NameChanged<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for NameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NameChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QFilterKey::parentChanged`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct ParentChanged<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFilterKey::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFilterKey::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFilterKey::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFilterKey::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFilterKey::nameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn name_changed(&self) -> NameChanged {
      NameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFilterKey::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FilterKey`.
  pub struct Slots<'a>(&'a ::filter_key::FilterKey);
  /// Represents a built-in Qt slot `Qt3DRender::QFilterKey::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QFilterKey::setValue`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct SetValue<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (&'static ::qt_core::variant::Variant,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(const QVariant&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QFilterKey::setName`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.slots().set_name()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct SetName<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for SetName<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setName(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QFilterKey::setParent`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct SetParent<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QFilterKey::setEnabled`.
  ///
  /// An object of this type can be created from `FilterKey` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FilterKey` object.
  pub struct SetEnabled<'a>(&'a ::filter_key::FilterKey);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFilterKey::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFilterKey::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFilterKey::setName`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_name(&self) -> SetName {
      SetName(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFilterKey::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFilterKey::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::filter_key::FilterKey {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::filter_key::FilterKey {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::filter_key::FilterKey)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::filter_key::FilterKey as *mut ::filter_key::FilterKey) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::filter_key::FilterKey {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_QObject_ptr(self as *mut ::filter_key::FilterKey) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_QObject_ptr(self as *const ::filter_key::FilterKey as *mut ::filter_key::FilterKey) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::filter_key::FilterKey> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::filter_key::FilterKey {
    let ffi_result = ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DRender_QFilterKey_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::filter_key::FilterKey {
    let ffi_result = ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DRender_QFilterKey_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::filter_key::FilterKey> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::filter_key::FilterKey {
    let ffi_result = ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DRender_QFilterKey_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::filter_key::FilterKey {
    let ffi_result = ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DRender_QFilterKey_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::filter_key::FilterKey {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::filter_key::FilterKey as *mut ::filter_key::FilterKey) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::filter_key::FilterKey {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QFilterKey_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::filter_key::FilterKey)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
