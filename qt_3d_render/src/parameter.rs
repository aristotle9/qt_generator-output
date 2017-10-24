/// C++ type: <span style='color: green;'>```Qt3DRender::QParameter```</span>
#[repr(C)]
pub struct Parameter(u8);

impl Parameter {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QParameter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QParameter_metaObject(self as *const ::parameter::Parameter) }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DRender::QParameter::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QParameter_name_to_output(self as *const ::parameter::Parameter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter::QParameter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::parameter::Parameter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QParameter::QParameter()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::qt_core::variant::Variant)) -> ::cpp_utils::CppBox<::parameter::Parameter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QParameter::QParameter(const QString& name, const QVariant& value)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::parameter::Parameter>
    where Args: overloading::ParameterNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter::QParameter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::parameter::Parameter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QParameter::QParameter(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::abstract_texture::AbstractTexture)) -> ::cpp_utils::CppBox<::parameter::Parameter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QParameter::QParameter(const QString& name, Qt3DRender::QAbstractTexture* texture)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::abstract_texture::AbstractTexture, *mut ::qt_3d_core::node::Node)) -> ::cpp_utils::CppBox<::parameter::Parameter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QParameter::QParameter(const QString& name, Qt3DRender::QAbstractTexture* texture, Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, &::qt_core::variant::Variant, *mut ::qt_3d_core::node::Node)) -> ::cpp_utils::CppBox<::parameter::Parameter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QParameter::QParameter(const QString& name, const QVariant& value, Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::parameter::Parameter>
    where Args: overloading::ParameterNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QParameter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QParameter_qt_metacall(self as *mut ::parameter::Parameter,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QParameter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QParameter_qt_metacast(self as *mut ::parameter::Parameter, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QParameter::setName(const QString& name)```</span>
  ///
  ///
  pub fn set_name(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QParameter_setName(self as *mut ::parameter::Parameter,
                                                          name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QParameter::setValue(const QVariant& dv)```</span>
  ///
  ///
  pub fn set_value(&mut self, dv: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QParameter_setValue(self as *mut ::parameter::Parameter,
                                                           dv as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QParameter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QParameter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QParameter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QParameter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant Qt3DRender::QParameter::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QParameter_value_to_output(self as *const ::parameter::Parameter, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::parameter::Parameter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QParameter_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Parameter`.
  pub struct Signals<'a>(&'a ::parameter::Parameter);
  /// Represents a built-in Qt signal `Qt3DRender::QParameter::enabledChanged`.
  ///
  /// An object of this type can be created from `Parameter` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct EnabledChanged<'a>(&'a ::parameter::Parameter);
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
  /// Represents a built-in Qt signal `Qt3DRender::QParameter::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Parameter` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct NodeDestroyed<'a>(&'a ::parameter::Parameter);
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
  /// Represents a built-in Qt signal `Qt3DRender::QParameter::valueChanged`.
  ///
  /// An object of this type can be created from `Parameter` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct ValueChanged<'a>(&'a ::parameter::Parameter);
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
  /// Represents a built-in Qt signal `Qt3DRender::QParameter::parentChanged`.
  ///
  /// An object of this type can be created from `Parameter` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct ParentChanged<'a>(&'a ::parameter::Parameter);
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
  /// Represents a built-in Qt signal `Qt3DRender::QParameter::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Parameter` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::parameter::Parameter);
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
  /// Represents a built-in Qt signal `Qt3DRender::QParameter::nameChanged`.
  ///
  /// An object of this type can be created from `Parameter` with `object.signals().name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct NameChanged<'a>(&'a ::parameter::Parameter);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QParameter::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QParameter::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QParameter::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QParameter::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QParameter::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QParameter::nameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn name_changed(&self) -> NameChanged {
      NameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Parameter`.
  pub struct Slots<'a>(&'a ::parameter::Parameter);
  /// Represents a built-in Qt slot `Qt3DRender::QParameter::setEnabled`.
  ///
  /// An object of this type can be created from `Parameter` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct SetEnabled<'a>(&'a ::parameter::Parameter);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QParameter::setParent`.
  ///
  /// An object of this type can be created from `Parameter` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct SetParent<'a>(&'a ::parameter::Parameter);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QParameter::setName`.
  ///
  /// An object of this type can be created from `Parameter` with `object.slots().set_name()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct SetName<'a>(&'a ::parameter::Parameter);
  impl<'a> ::qt_core::connection::Receiver for SetName<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setName(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QParameter::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Parameter` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::parameter::Parameter);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QParameter::setValue`.
  ///
  /// An object of this type can be created from `Parameter` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Parameter` object.
  pub struct SetValue<'a>(&'a ::parameter::Parameter);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (&'static ::qt_core::variant::Variant,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(const QVariant&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QParameter::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QParameter::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QParameter::setName`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_name(&self) -> SetName {
      SetName(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QParameter::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QParameter::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
  }
  impl ::parameter::Parameter {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::parameter::Parameter {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::parameter::Parameter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::parameter::Parameter as *mut ::parameter::Parameter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::parameter::Parameter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QParameter_G_static_cast_QObject_ptr(self as *mut ::parameter::Parameter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QParameter_G_static_cast_QObject_ptr(self as *const ::parameter::Parameter as *mut ::parameter::Parameter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::parameter::Parameter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::parameter::Parameter {
    let ffi_result = ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::parameter::Parameter {
    let ffi_result = ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::parameter::Parameter> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::parameter::Parameter {
    let ffi_result = ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::parameter::Parameter {
    let ffi_result = ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::parameter::Parameter {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::parameter::Parameter as *mut ::parameter::Parameter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::parameter::Parameter {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QParameter_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::parameter::Parameter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Parameter::new](../struct.Parameter.html#method.new) method.
  pub trait ParameterNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter>;
  }
  impl<'a> ParameterNewArgs for (&'a ::qt_core::string::String, &'a ::qt_core::variant::Variant) {
    fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter> {
      let name = self.0;
      let value = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QParameter_new_name_value(name as *const ::qt_core::string::String,
                                                                     value as *const ::qt_core::variant::Variant)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ParameterNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter> {

      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QParameter_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Parameter::new_unsafe](../struct.Parameter.html#method.new_unsafe) method.
  pub trait ParameterNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter>;
  }
  impl<'a> ParameterNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::abstract_texture::AbstractTexture) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter> {
      let name = self.0;
      let texture = self.1;
      let ffi_result =
        ::ffi::qt_3d_render_c_Qt3DRender_QParameter_new_name_texture(name as *const ::qt_core::string::String, texture);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ParameterNewUnsafeArgs
    for (&'a ::qt_core::string::String, *mut ::abstract_texture::AbstractTexture, *mut ::qt_3d_core::node::Node) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter> {
      let name = self.0;
      let texture = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_3d_render_c_Qt3DRender_QParameter_new_name_texture_parent(name as *const ::qt_core::string::String,
                                                                            texture,
                                                                            parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ParameterNewUnsafeArgs
    for (&'a ::qt_core::string::String, &'a ::qt_core::variant::Variant, *mut ::qt_3d_core::node::Node) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter> {
      let name = self.0;
      let value = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QParameter_new_name_value_parent(name as *const ::qt_core::string::String, value as *const ::qt_core::variant::Variant, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ParameterNewUnsafeArgs for *mut ::qt_3d_core::node::Node {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::parameter::Parameter> {
      let parent = self;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QParameter_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
