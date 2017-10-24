/// C++ type: <span style='color: green;'>```Qt3DRender::QPointSize```</span>
#[repr(C)]
pub struct PointSize(u8);

impl PointSize {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPointSize::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_metaObject(self as *const ::point_size::PointSize) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPointSize::QPointSize()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::point_size::PointSize> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPointSize::QPointSize(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::point_size::PointSize> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPointSize::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_qt_metacall(self as *mut ::point_size::PointSize,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPointSize::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_qt_metacast(self as *mut ::point_size::PointSize, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPointSize::setSizeMode(Qt3DRender::QPointSize::SizeMode sizeMode)```</span>
  ///
  ///
  pub fn set_size_mode(&mut self, size_mode: ::point_size::SizeMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_setSizeMode(self as *mut ::point_size::PointSize, size_mode) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPointSize::setValue(float value)```</span>
  ///
  ///
  pub fn set_value(&mut self, value: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_setValue(self as *mut ::point_size::PointSize, value) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPointSize::SizeMode Qt3DRender::QPointSize::sizeMode() const```</span>
  ///
  ///
  pub fn size_mode(&self) -> ::point_size::SizeMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_sizeMode(self as *const ::point_size::PointSize) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPointSize::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPointSize::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPointSize::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_value(self as *const ::point_size::PointSize) }
  }
}

impl ::cpp_utils::CppDeletable for ::point_size::PointSize {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPointSize_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PointSize`.
  pub struct Signals<'a>(&'a ::point_size::PointSize);
  /// Represents a built-in Qt signal `Qt3DRender::QPointSize::sizeModeChanged`.
  ///
  /// An object of this type can be created from `PointSize` with `object.signals().size_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointSize` object.
  pub struct SizeModeChanged<'a>(&'a ::point_size::PointSize);
  impl<'a> ::qt_core::connection::Receiver for SizeModeChanged<'a> {
    type Arguments = (::point_size::SizeMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sizeModeChanged(Qt3DRender::QPointSize::SizeMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SizeModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPointSize::valueChanged`.
  ///
  /// An object of this type can be created from `PointSize` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointSize` object.
  pub struct ValueChanged<'a>(&'a ::point_size::PointSize);
  impl<'a> ::qt_core::connection::Receiver for ValueChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointSize::sizeModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn size_mode_changed(&self) -> SizeModeChanged {
      SizeModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointSize::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PointSize`.
  pub struct Slots<'a>(&'a ::point_size::PointSize);
  /// Represents a built-in Qt slot `Qt3DRender::QPointSize::setSizeMode`.
  ///
  /// An object of this type can be created from `PointSize` with `object.slots().set_size_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointSize` object.
  pub struct SetSizeMode<'a>(&'a ::point_size::PointSize);
  impl<'a> ::qt_core::connection::Receiver for SetSizeMode<'a> {
    type Arguments = (::point_size::SizeMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSizeMode(Qt3DRender::QPointSize::SizeMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPointSize::setValue`.
  ///
  /// An object of this type can be created from `PointSize` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointSize` object.
  pub struct SetValue<'a>(&'a ::point_size::PointSize);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointSize::setSizeMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_size_mode(&self) -> SetSizeMode {
      SetSizeMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointSize::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
  }
  impl ::point_size::PointSize {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QPointSize::SizeMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeMode {
  /// C++ enum variant: <span style='color: green;'>```Fixed = 0```</span>
  Fixed = 0,
  /// C++ enum variant: <span style='color: green;'>```Programmable = 1```</span>
  Programmable = 1,
}

impl ::cpp_utils::DynamicCast<::point_size::PointSize> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::point_size::PointSize> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointSize_G_dynamic_cast_Qt3DRender_QPointSize_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::point_size::PointSize> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointSize_G_dynamic_cast_Qt3DRender_QPointSize_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::point_size::PointSize {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::point_size::PointSize)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::point_size::PointSize as *mut ::point_size::PointSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::point_size::PointSize {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::point_size::PointSize)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::point_size::PointSize as *mut ::point_size::PointSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::point_size::PointSize {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QPointSize_G_static_cast_QObject_ptr(self as *mut ::point_size::PointSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointSize_G_static_cast_QObject_ptr(self as *const ::point_size::PointSize as *mut ::point_size::PointSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_size::PointSize> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_size::PointSize {
    let ffi_result = ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_size::PointSize {
    let ffi_result = ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_size::PointSize> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_size::PointSize {
    let ffi_result = ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_size::PointSize {
    let ffi_result = ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_size::PointSize> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_size::PointSize {
    let ffi_result = ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_size::PointSize {
    let ffi_result = ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QPointSize_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::point_size::PointSize {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::point_size::PointSize as *mut ::point_size::PointSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::point_size::PointSize {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QPointSize_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::point_size::PointSize)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
