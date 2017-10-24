/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderTarget```</span>
#[repr(C)]
pub struct RenderTarget(u8);

impl RenderTarget {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderTarget::addOutput(Qt3DRender::QRenderTargetOutput* output)```</span>
  ///
  ///
  pub unsafe fn add_output(&mut self, output: *mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_addOutput(self as *mut ::render_target::RenderTarget, output)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderTarget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_metaObject(self as *const ::render_target::RenderTarget) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderTarget::QRenderTarget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_target::RenderTarget> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderTarget::QRenderTarget(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_target::RenderTarget> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*> Qt3DRender::QRenderTarget::outputs() const```</span>
  ///
  ///
  pub fn outputs(&self) -> ::vector::VectorRenderTargetOutputMutPtr {
    {
      let mut object: ::vector::VectorRenderTargetOutputMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_outputs_to_output(self as *const ::render_target::RenderTarget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderTarget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_qt_metacall(self as *mut ::render_target::RenderTarget,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderTarget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_qt_metacast(self as *mut ::render_target::RenderTarget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderTarget::removeOutput(Qt3DRender::QRenderTargetOutput* output)```</span>
  ///
  ///
  pub unsafe fn remove_output(&mut self, output: *mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_removeOutput(self as *mut ::render_target::RenderTarget, output)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderTarget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderTarget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_target::RenderTarget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTarget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RenderTarget`.
  pub struct Signals<'a>(&'a ::render_target::RenderTarget);
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTarget::addedToEntity`.
  ///
  /// An object of this type can be created from `RenderTarget` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTarget` object.
  pub struct AddedToEntity<'a>(&'a ::render_target::RenderTarget);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTarget::shareableChanged`.
  ///
  /// An object of this type can be created from `RenderTarget` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTarget` object.
  pub struct ShareableChanged<'a>(&'a ::render_target::RenderTarget);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTarget::removedFromEntity`.
  ///
  /// An object of this type can be created from `RenderTarget` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTarget` object.
  pub struct RemovedFromEntity<'a>(&'a ::render_target::RenderTarget);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTarget::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTarget::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTarget::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RenderTarget`.
  pub struct Slots<'a>(&'a ::render_target::RenderTarget);
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTarget::setShareable`.
  ///
  /// An object of this type can be created from `RenderTarget` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTarget` object.
  pub struct SetShareable<'a>(&'a ::render_target::RenderTarget);
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
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTarget::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::render_target::RenderTarget {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::render_target::RenderTarget {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::render_target::RenderTarget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::render_target::RenderTarget as *mut ::render_target::RenderTarget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_target::RenderTarget {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_target::RenderTarget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_target::RenderTarget as *mut ::render_target::RenderTarget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_target::RenderTarget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_QObject_ptr(self as *mut ::render_target::RenderTarget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_QObject_ptr(self as *const ::render_target::RenderTarget as *mut ::render_target::RenderTarget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target::RenderTarget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target::RenderTarget {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target::RenderTarget {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target::RenderTarget> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target::RenderTarget {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target::RenderTarget {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target::RenderTarget> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target::RenderTarget {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target::RenderTarget {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DRender_QRenderTarget_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_target::RenderTarget {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::render_target::RenderTarget as *mut ::render_target::RenderTarget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_target::RenderTarget {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTarget_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::render_target::RenderTarget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
