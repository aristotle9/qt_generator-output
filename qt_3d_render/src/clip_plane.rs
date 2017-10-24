/// C++ type: <span style='color: green;'>```Qt3DRender::QClipPlane```</span>
#[repr(C)]
pub struct ClipPlane(u8);

impl ClipPlane {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QClipPlane::distance() const```</span>
  ///
  ///
  pub fn distance(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_distance(self as *const ::clip_plane::ClipPlane) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QClipPlane::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_metaObject(self as *const ::clip_plane::ClipPlane) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QClipPlane::QClipPlane()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::clip_plane::ClipPlane> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QClipPlane::QClipPlane(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::clip_plane::ClipPlane> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QClipPlane::normal() const```</span>
  ///
  ///
  pub fn normal(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_normal_as_ptr(self as *const ::clip_plane::ClipPlane) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QClipPlane::planeIndex() const```</span>
  ///
  ///
  pub fn plane_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_planeIndex(self as *const ::clip_plane::ClipPlane) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QClipPlane::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_qt_metacall(self as *mut ::clip_plane::ClipPlane,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QClipPlane::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_qt_metacast(self as *mut ::clip_plane::ClipPlane, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClipPlane::setDistance(float arg1)```</span>
  ///
  ///
  pub fn set_distance(&mut self, arg1: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_setDistance(self as *mut ::clip_plane::ClipPlane, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClipPlane::setNormal(QVector3D arg1)```</span>
  ///
  ///
  pub fn set_normal(&mut self, arg1: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_setNormal(self as *mut ::clip_plane::ClipPlane,
                                                            arg1 as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClipPlane::setPlaneIndex(int arg1)```</span>
  ///
  ///
  pub fn set_plane_index(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_setPlaneIndex(self as *mut ::clip_plane::ClipPlane, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QClipPlane::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QClipPlane::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::clip_plane::ClipPlane {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QClipPlane_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ClipPlane`.
  pub struct Signals<'a>(&'a ::clip_plane::ClipPlane);
  /// Represents a built-in Qt signal `Qt3DRender::QClipPlane::planeIndexChanged`.
  ///
  /// An object of this type can be created from `ClipPlane` with `object.signals().plane_index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClipPlane` object.
  pub struct PlaneIndexChanged<'a>(&'a ::clip_plane::ClipPlane);
  impl<'a> ::qt_core::connection::Receiver for PlaneIndexChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2planeIndexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PlaneIndexChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QClipPlane::normalChanged`.
  ///
  /// An object of this type can be created from `ClipPlane` with `object.signals().normal_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClipPlane` object.
  pub struct NormalChanged<'a>(&'a ::clip_plane::ClipPlane);
  impl<'a> ::qt_core::connection::Receiver for NormalChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2normalChanged(QVector3D)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NormalChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QClipPlane::distanceChanged`.
  ///
  /// An object of this type can be created from `ClipPlane` with `object.signals().distance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClipPlane` object.
  pub struct DistanceChanged<'a>(&'a ::clip_plane::ClipPlane);
  impl<'a> ::qt_core::connection::Receiver for DistanceChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2distanceChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DistanceChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClipPlane::planeIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn plane_index_changed(&self) -> PlaneIndexChanged {
      PlaneIndexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClipPlane::normalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn normal_changed(&self) -> NormalChanged {
      NormalChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClipPlane::distanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn distance_changed(&self) -> DistanceChanged {
      DistanceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ClipPlane`.
  pub struct Slots<'a>(&'a ::clip_plane::ClipPlane);
  /// Represents a built-in Qt slot `Qt3DRender::QClipPlane::setNormal`.
  ///
  /// An object of this type can be created from `ClipPlane` with `object.slots().set_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClipPlane` object.
  pub struct SetNormal<'a>(&'a ::clip_plane::ClipPlane);
  impl<'a> ::qt_core::connection::Receiver for SetNormal<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNormal(QVector3D)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QClipPlane::setPlaneIndex`.
  ///
  /// An object of this type can be created from `ClipPlane` with `object.slots().set_plane_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClipPlane` object.
  pub struct SetPlaneIndex<'a>(&'a ::clip_plane::ClipPlane);
  impl<'a> ::qt_core::connection::Receiver for SetPlaneIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPlaneIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QClipPlane::setDistance`.
  ///
  /// An object of this type can be created from `ClipPlane` with `object.slots().set_distance()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClipPlane` object.
  pub struct SetDistance<'a>(&'a ::clip_plane::ClipPlane);
  impl<'a> ::qt_core::connection::Receiver for SetDistance<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDistance(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClipPlane::setNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_normal(&self) -> SetNormal {
      SetNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClipPlane::setPlaneIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_plane_index(&self) -> SetPlaneIndex {
      SetPlaneIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClipPlane::setDistance`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_distance(&self) -> SetDistance {
      SetDistance(self.0)
    }
  }
  impl ::clip_plane::ClipPlane {
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

impl ::cpp_utils::DynamicCast<::clip_plane::ClipPlane> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::clip_plane::ClipPlane> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_dynamic_cast_Qt3DRender_QClipPlane_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::clip_plane::ClipPlane> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_dynamic_cast_Qt3DRender_QClipPlane_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::clip_plane::ClipPlane {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::clip_plane::ClipPlane)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::clip_plane::ClipPlane as *mut ::clip_plane::ClipPlane) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::clip_plane::ClipPlane {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::clip_plane::ClipPlane)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::clip_plane::ClipPlane as *mut ::clip_plane::ClipPlane) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::clip_plane::ClipPlane {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_QObject_ptr(self as *mut ::clip_plane::ClipPlane) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_QObject_ptr(self as *const ::clip_plane::ClipPlane as *mut ::clip_plane::ClipPlane) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clip_plane::ClipPlane> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clip_plane::ClipPlane {
    let ffi_result = ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clip_plane::ClipPlane {
    let ffi_result = ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clip_plane::ClipPlane> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clip_plane::ClipPlane {
    let ffi_result = ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clip_plane::ClipPlane {
    let ffi_result = ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clip_plane::ClipPlane> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clip_plane::ClipPlane {
    let ffi_result = ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clip_plane::ClipPlane {
    let ffi_result = ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::clip_plane::ClipPlane {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::clip_plane::ClipPlane as *mut ::clip_plane::ClipPlane) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::clip_plane::ClipPlane {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::clip_plane::ClipPlane)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
