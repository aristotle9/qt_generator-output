/// C++ type: <span style='color: green;'>```Qt3DRender::QFrontFace```</span>
#[repr(C)]
pub struct FrontFace(u8);

impl FrontFace {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QFrontFace::WindingDirection Qt3DRender::QFrontFace::direction() const```</span>
  ///
  ///
  pub fn direction(&self) -> ::front_face::WindingDirection {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_direction(self as *const ::front_face::FrontFace) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QFrontFace::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_metaObject(self as *const ::front_face::FrontFace) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFrontFace::QFrontFace()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::front_face::FrontFace> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFrontFace::QFrontFace(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::front_face::FrontFace> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QFrontFace::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_qt_metacall(self as *mut ::front_face::FrontFace,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QFrontFace::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_qt_metacast(self as *mut ::front_face::FrontFace, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QFrontFace::setDirection(Qt3DRender::QFrontFace::WindingDirection direction)```</span>
  ///
  ///
  pub fn set_direction(&mut self, direction: ::front_face::WindingDirection) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_setDirection(self as *mut ::front_face::FrontFace, direction) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QFrontFace::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QFrontFace::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::front_face::FrontFace {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrontFace_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FrontFace`.
  pub struct Signals<'a>(&'a ::front_face::FrontFace);
  /// Represents a built-in Qt signal `Qt3DRender::QFrontFace::directionChanged`.
  ///
  /// An object of this type can be created from `FrontFace` with `object.signals().direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrontFace` object.
  pub struct DirectionChanged<'a>(&'a ::front_face::FrontFace);
  impl<'a> ::qt_core::connection::Receiver for DirectionChanged<'a> {
    type Arguments = (::front_face::WindingDirection,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directionChanged(Qt3DRender::QFrontFace::WindingDirection)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DirectionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFrontFace::directionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn direction_changed(&self) -> DirectionChanged {
      DirectionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FrontFace`.
  pub struct Slots<'a>(&'a ::front_face::FrontFace);
  /// Represents a built-in Qt slot `Qt3DRender::QFrontFace::setDirection`.
  ///
  /// An object of this type can be created from `FrontFace` with `object.slots().set_direction()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrontFace` object.
  pub struct SetDirection<'a>(&'a ::front_face::FrontFace);
  impl<'a> ::qt_core::connection::Receiver for SetDirection<'a> {
    type Arguments = (::front_face::WindingDirection,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDirection(Qt3DRender::QFrontFace::WindingDirection)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFrontFace::setDirection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_direction(&self) -> SetDirection {
      SetDirection(self.0)
    }
  }
  impl ::front_face::FrontFace {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QFrontFace::WindingDirection```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WindingDirection {
  /// C++ enum variant: <span style='color: green;'>```ClockWise = 2304```</span>
  ClockWise = 2304,
  /// C++ enum variant: <span style='color: green;'>```CounterClockWise = 2305```</span>
  CounterClockWise = 2305,
}

impl ::cpp_utils::DynamicCast<::front_face::FrontFace> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::front_face::FrontFace> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_dynamic_cast_Qt3DRender_QFrontFace_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::front_face::FrontFace> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_dynamic_cast_Qt3DRender_QFrontFace_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::front_face::FrontFace {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::front_face::FrontFace)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::front_face::FrontFace as *mut ::front_face::FrontFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::front_face::FrontFace {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::front_face::FrontFace)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::front_face::FrontFace as *mut ::front_face::FrontFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::front_face::FrontFace {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_QObject_ptr(self as *mut ::front_face::FrontFace) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_QObject_ptr(self as *const ::front_face::FrontFace as *mut ::front_face::FrontFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::front_face::FrontFace> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::front_face::FrontFace {
    let ffi_result = ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::front_face::FrontFace {
    let ffi_result = ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::front_face::FrontFace> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::front_face::FrontFace {
    let ffi_result = ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::front_face::FrontFace {
    let ffi_result = ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::front_face::FrontFace> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::front_face::FrontFace {
    let ffi_result = ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::front_face::FrontFace {
    let ffi_result = ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QFrontFace_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::front_face::FrontFace {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::front_face::FrontFace as *mut ::front_face::FrontFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::front_face::FrontFace {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QFrontFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::front_face::FrontFace)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
