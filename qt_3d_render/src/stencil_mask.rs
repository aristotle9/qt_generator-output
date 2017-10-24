/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilMask```</span>
#[repr(C)]
pub struct StencilMask(u8);

impl StencilMask {
  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QStencilMask::backOutputMask() const```</span>
  ///
  ///
  pub fn back_output_mask(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_backOutputMask(self as *const ::stencil_mask::StencilMask) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QStencilMask::frontOutputMask() const```</span>
  ///
  ///
  pub fn front_output_mask(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_frontOutputMask(self as *const ::stencil_mask::StencilMask) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QStencilMask::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_metaObject(self as *const ::stencil_mask::StencilMask) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QStencilMask::QStencilMask()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::stencil_mask::StencilMask> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QStencilMask::QStencilMask(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::stencil_mask::StencilMask> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QStencilMask::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_qt_metacall(self as *mut ::stencil_mask::StencilMask,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QStencilMask::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_qt_metacast(self as *mut ::stencil_mask::StencilMask, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilMask::setBackOutputMask(unsigned int backOutputMask)```</span>
  ///
  ///
  pub fn set_back_output_mask(&mut self, back_output_mask: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_setBackOutputMask(self as *mut ::stencil_mask::StencilMask,
                                                                      back_output_mask)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilMask::setFrontOutputMask(unsigned int frontOutputMask)```</span>
  ///
  ///
  pub fn set_front_output_mask(&mut self, front_output_mask: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_setFrontOutputMask(self as *mut ::stencil_mask::StencilMask,
                                                                       front_output_mask)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilMask::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilMask::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::stencil_mask::StencilMask {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilMask_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StencilMask`.
  pub struct Signals<'a>(&'a ::stencil_mask::StencilMask);
  /// Represents a built-in Qt signal `Qt3DRender::QStencilMask::backOutputMaskChanged`.
  ///
  /// An object of this type can be created from `StencilMask` with `object.signals().back_output_mask_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilMask` object.
  pub struct BackOutputMaskChanged<'a>(&'a ::stencil_mask::StencilMask);
  impl<'a> ::qt_core::connection::Receiver for BackOutputMaskChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2backOutputMaskChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BackOutputMaskChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilMask::frontOutputMaskChanged`.
  ///
  /// An object of this type can be created from `StencilMask` with `object.signals().front_output_mask_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilMask` object.
  pub struct FrontOutputMaskChanged<'a>(&'a ::stencil_mask::StencilMask);
  impl<'a> ::qt_core::connection::Receiver for FrontOutputMaskChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2frontOutputMaskChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FrontOutputMaskChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilMask::backOutputMaskChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn back_output_mask_changed(&self) -> BackOutputMaskChanged {
      BackOutputMaskChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilMask::frontOutputMaskChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn front_output_mask_changed(&self) -> FrontOutputMaskChanged {
      FrontOutputMaskChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StencilMask`.
  pub struct Slots<'a>(&'a ::stencil_mask::StencilMask);
  /// Represents a built-in Qt slot `Qt3DRender::QStencilMask::setBackOutputMask`.
  ///
  /// An object of this type can be created from `StencilMask` with `object.slots().set_back_output_mask()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilMask` object.
  pub struct SetBackOutputMask<'a>(&'a ::stencil_mask::StencilMask);
  impl<'a> ::qt_core::connection::Receiver for SetBackOutputMask<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBackOutputMask(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QStencilMask::setFrontOutputMask`.
  ///
  /// An object of this type can be created from `StencilMask` with `object.slots().set_front_output_mask()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilMask` object.
  pub struct SetFrontOutputMask<'a>(&'a ::stencil_mask::StencilMask);
  impl<'a> ::qt_core::connection::Receiver for SetFrontOutputMask<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFrontOutputMask(unsigned int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilMask::setBackOutputMask`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_back_output_mask(&self) -> SetBackOutputMask {
      SetBackOutputMask(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilMask::setFrontOutputMask`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_front_output_mask(&self) -> SetFrontOutputMask {
      SetFrontOutputMask(self.0)
    }
  }
  impl ::stencil_mask::StencilMask {
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

impl ::cpp_utils::DynamicCast<::stencil_mask::StencilMask> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::stencil_mask::StencilMask> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_dynamic_cast_Qt3DRender_QStencilMask_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::stencil_mask::StencilMask> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_dynamic_cast_Qt3DRender_QStencilMask_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::stencil_mask::StencilMask {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::stencil_mask::StencilMask)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::stencil_mask::StencilMask as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::stencil_mask::StencilMask {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::stencil_mask::StencilMask as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::stencil_mask::StencilMask {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_QObject_ptr(self as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_QObject_ptr(self as *const ::stencil_mask::StencilMask as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_mask::StencilMask> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_mask::StencilMask {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_mask::StencilMask {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_mask::StencilMask> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_mask::StencilMask {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_mask::StencilMask {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_mask::StencilMask> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_mask::StencilMask {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_mask::StencilMask {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QStencilMask_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::stencil_mask::StencilMask {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::stencil_mask::StencilMask as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::stencil_mask::StencilMask {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::stencil_mask::StencilMask) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
