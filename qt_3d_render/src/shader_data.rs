/// C++ type: <span style='color: green;'>```Qt3DRender::PropertyReaderInterface```</span>
#[repr(C)]
pub struct PropertyReaderInterface(u8);

impl PropertyReaderInterface {
  /// C++ method: <span style='color: green;'>```pure virtual QVariant Qt3DRender::PropertyReaderInterface::readProperty(const QVariant& v)```</span>
  ///
  ///
  pub fn read_property(&mut self, v: &::qt_core::variant::Variant) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_PropertyReaderInterface_readProperty_to_output(self as *mut ::shader_data::PropertyReaderInterface, v as *const ::qt_core::variant::Variant, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::shader_data::PropertyReaderInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_PropertyReaderInterface_delete
  }
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QShaderData```</span>
#[repr(C)]
pub struct ShaderData(u8);

impl ShaderData {
  /// C++ method: <span style='color: green;'>```virtual bool Qt3DRender::QShaderData::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_event(self as *mut ::shader_data::ShaderData, event)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QShaderData::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_metaObject(self as *const ::shader_data::ShaderData) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QShaderData::QShaderData()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::shader_data::ShaderData> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QShaderData::QShaderData(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::shader_data::ShaderData> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::PropertyReaderInterface> Qt3DRender::QShaderData::propertyReader() const```</span>
  ///
  ///
  pub fn property_reader(&self) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface {
    {
      let mut object: ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_propertyReader_to_output(self as *const ::shader_data::ShaderData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QShaderData::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_qt_metacall(self as *mut ::shader_data::ShaderData,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QShaderData::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_qt_metacast(self as *mut ::shader_data::ShaderData, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QShaderData::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QShaderData::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::shader_data::ShaderData {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderData_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ShaderData`.
  pub struct Signals<'a>(&'a ::shader_data::ShaderData);
  /// Represents a built-in Qt signal `Qt3DRender::QShaderData::removedFromEntity`.
  ///
  /// An object of this type can be created from `ShaderData` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderData` object.
  pub struct RemovedFromEntity<'a>(&'a ::shader_data::ShaderData);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderData::addedToEntity`.
  ///
  /// An object of this type can be created from `ShaderData` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderData` object.
  pub struct AddedToEntity<'a>(&'a ::shader_data::ShaderData);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderData::shareableChanged`.
  ///
  /// An object of this type can be created from `ShaderData` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderData` object.
  pub struct ShareableChanged<'a>(&'a ::shader_data::ShaderData);
  impl<'a> ::qt_core::connection::Receiver for ShareableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shareableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShareableChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderData::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderData::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderData::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ShaderData`.
  pub struct Slots<'a>(&'a ::shader_data::ShaderData);
  /// Represents a built-in Qt slot `Qt3DRender::QShaderData::setShareable`.
  ///
  /// An object of this type can be created from `ShaderData` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderData` object.
  pub struct SetShareable<'a>(&'a ::shader_data::ShaderData);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderData::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::shader_data::ShaderData {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::shader_data::ShaderData {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::shader_data::ShaderData)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::shader_data::ShaderData as *mut ::shader_data::ShaderData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::shader_data::ShaderData {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::shader_data::ShaderData)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::shader_data::ShaderData as *mut ::shader_data::ShaderData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::shader_data::ShaderData {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QShaderData_G_static_cast_QObject_ptr(self as *mut ::shader_data::ShaderData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderData_G_static_cast_QObject_ptr(self as *const ::shader_data::ShaderData as *mut ::shader_data::ShaderData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shader_data::ShaderData> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shader_data::ShaderData {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shader_data::ShaderData {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shader_data::ShaderData> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shader_data::ShaderData {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shader_data::ShaderData {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shader_data::ShaderData> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shader_data::ShaderData {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shader_data::ShaderData {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DRender_QShaderData_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::shader_data::ShaderData {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::shader_data::ShaderData as *mut ::shader_data::ShaderData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::shader_data::ShaderData {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QShaderData_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::shader_data::ShaderData)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
