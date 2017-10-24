/// C++ type: <span style='color: green;'>```Qt3DRender::QBlendEquation```</span>
#[repr(C)]
pub struct BlendEquation(u8);

impl BlendEquation {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QBlendEquation::BlendFunction Qt3DRender::QBlendEquation::blendFunction() const```</span>
  ///
  ///
  pub fn blend_function(&self) -> ::blend_equation::BlendFunction {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_blendFunction(self as *const ::blend_equation::BlendEquation)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QBlendEquation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_metaObject(self as *const ::blend_equation::BlendEquation)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBlendEquation::QBlendEquation()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::blend_equation::BlendEquation> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBlendEquation::QBlendEquation(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::blend_equation::BlendEquation> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QBlendEquation::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_qt_metacall(self as *mut ::blend_equation::BlendEquation,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QBlendEquation::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_qt_metacast(self as *mut ::blend_equation::BlendEquation, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquation::setBlendFunction(Qt3DRender::QBlendEquation::BlendFunction blendFunction)```</span>
  ///
  ///
  pub fn set_blend_function(&mut self, blend_function: ::blend_equation::BlendFunction) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_setBlendFunction(self as *mut ::blend_equation::BlendEquation,
                                                                       blend_function)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBlendEquation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBlendEquation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::blend_equation::BlendEquation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `BlendEquation`.
  pub struct Signals<'a>(&'a ::blend_equation::BlendEquation);
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquation::blendFunctionChanged`.
  ///
  /// An object of this type can be created from `BlendEquation` with `object.signals().blend_function_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquation` object.
  pub struct BlendFunctionChanged<'a>(&'a ::blend_equation::BlendEquation);
  impl<'a> ::qt_core::connection::Receiver for BlendFunctionChanged<'a> {
    type Arguments = (::blend_equation::BlendFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2blendFunctionChanged(Qt3DRender::QBlendEquation::BlendFunction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BlendFunctionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquation::blendFunctionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn blend_function_changed(&self) -> BlendFunctionChanged {
      BlendFunctionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `BlendEquation`.
  pub struct Slots<'a>(&'a ::blend_equation::BlendEquation);
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquation::setBlendFunction`.
  ///
  /// An object of this type can be created from `BlendEquation` with `object.slots().set_blend_function()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquation` object.
  pub struct SetBlendFunction<'a>(&'a ::blend_equation::BlendEquation);
  impl<'a> ::qt_core::connection::Receiver for SetBlendFunction<'a> {
    type Arguments = (::blend_equation::BlendFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBlendFunction(Qt3DRender::QBlendEquation::BlendFunction)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquation::setBlendFunction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_blend_function(&self) -> SetBlendFunction {
      SetBlendFunction(self.0)
    }
  }
  impl ::blend_equation::BlendEquation {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QBlendEquation::BlendFunction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BlendFunction {
  /// C++ enum variant: <span style='color: green;'>```Add = 32774```</span>
  Add = 32774,
  /// C++ enum variant: <span style='color: green;'>```Min = 32775```</span>
  Min = 32775,
  /// C++ enum variant: <span style='color: green;'>```Max = 32776```</span>
  Max = 32776,
  /// C++ enum variant: <span style='color: green;'>```Subtract = 32778```</span>
  Subtract = 32778,
  /// C++ enum variant: <span style='color: green;'>```ReverseSubtract = 32779```</span>
  ReverseSubtract = 32779,
}

impl ::cpp_utils::DynamicCast<::blend_equation::BlendEquation> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::blend_equation::BlendEquation> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_dynamic_cast_Qt3DRender_QBlendEquation_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::blend_equation::BlendEquation> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_dynamic_cast_Qt3DRender_QBlendEquation_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::blend_equation::BlendEquation {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::blend_equation::BlendEquation as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::blend_equation::BlendEquation {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::blend_equation::BlendEquation as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::blend_equation::BlendEquation {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_QObject_ptr(self as *mut ::blend_equation::BlendEquation)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_QObject_ptr(self as *const ::blend_equation::BlendEquation as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::blend_equation::BlendEquation> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::blend_equation::BlendEquation {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::blend_equation::BlendEquation {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::blend_equation::BlendEquation> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::blend_equation::BlendEquation {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::blend_equation::BlendEquation {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::blend_equation::BlendEquation> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::blend_equation::BlendEquation {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::blend_equation::BlendEquation {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QBlendEquation_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::blend_equation::BlendEquation {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::blend_equation::BlendEquation as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::blend_equation::BlendEquation {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::blend_equation::BlendEquation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
