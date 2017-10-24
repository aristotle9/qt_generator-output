/// C++ type: <span style='color: green;'>```Qt3DRender::QPickTriangleEvent```</span>
#[repr(C)]
pub struct PickTriangleEvent(u8);

impl PickTriangleEvent {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPickTriangleEvent::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_metaObject(self as *const ::pick_triangle_event::PickTriangleEvent) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickTriangleEvent::QPickTriangleEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickTriangleEvent::QPickTriangleEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, &::qt_gui::vector_3d::Vector3D, &::qt_gui::vector_3d::Vector3D, ::libc::c_float, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickTriangleEvent::QPickTriangleEvent(const QPointF& position, const QVector3D& worldIntersection, const QVector3D& localIntersection, float distance, unsigned int triangleIndex, unsigned int vertex1Index, unsigned int vertex2Index, unsigned int vertex3Index)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, &::qt_gui::vector_3d::Vector3D, &::qt_gui::vector_3d::Vector3D, ::libc::c_float, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::pick_event::Buttons, ::libc::c_int, ::libc::c_int, &::qt_gui::vector_3d::Vector3D)) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickTriangleEvent::QPickTriangleEvent(const QPointF& position, const QVector3D& worldIntersection, const QVector3D& localIntersection, float distance, unsigned int triangleIndex, unsigned int vertex1Index, unsigned int vertex2Index, unsigned int vertex3Index, Qt3DRender::QPickEvent::Buttons button, int buttons, int modifiers, const QVector3D& uvw)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent>
    where Args: overloading::PickTriangleEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPickTriangleEvent::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_qt_metacall(self as *mut ::pick_triangle_event::PickTriangleEvent, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPickTriangleEvent::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_qt_metacast(self as *mut ::pick_triangle_event::PickTriangleEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPickTriangleEvent::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPickTriangleEvent::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QPickTriangleEvent::triangleIndex() const```</span>
  ///
  ///
  pub fn triangle_index(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_triangleIndex(self as *const ::pick_triangle_event::PickTriangleEvent) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QPickTriangleEvent::uvw() const```</span>
  ///
  ///
  pub fn uvw(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_uvw_as_ptr(self as *const ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QPickTriangleEvent::vertex1Index() const```</span>
  ///
  ///
  pub fn vertex1_index(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex1Index(self as *const ::pick_triangle_event::PickTriangleEvent) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QPickTriangleEvent::vertex2Index() const```</span>
  ///
  ///
  pub fn vertex2_index(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex2Index(self as *const ::pick_triangle_event::PickTriangleEvent) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QPickTriangleEvent::vertex3Index() const```</span>
  ///
  ///
  pub fn vertex3_index(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex3Index(self as *const ::pick_triangle_event::PickTriangleEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::pick_triangle_event::PickTriangleEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PickTriangleEvent`.
  pub struct Signals<'a>(&'a ::pick_triangle_event::PickTriangleEvent);
  /// Represents a built-in Qt signal `Qt3DRender::QPickTriangleEvent::acceptedChanged`.
  ///
  /// An object of this type can be created from `PickTriangleEvent` with `object.signals().accepted_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickTriangleEvent` object.
  pub struct AcceptedChanged<'a>(&'a ::pick_triangle_event::PickTriangleEvent);
  impl<'a> ::qt_core::connection::Receiver for AcceptedChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2acceptedChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AcceptedChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickTriangleEvent::acceptedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted_changed(&self) -> AcceptedChanged {
      AcceptedChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PickTriangleEvent`.
  pub struct Slots<'a>(&'a ::pick_triangle_event::PickTriangleEvent);
  /// Represents a built-in Qt slot `Qt3DRender::QPickTriangleEvent::setAccepted`.
  ///
  /// An object of this type can be created from `PickTriangleEvent` with `object.slots().set_accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickTriangleEvent` object.
  pub struct SetAccepted<'a>(&'a ::pick_triangle_event::PickTriangleEvent);
  impl<'a> ::qt_core::connection::Receiver for SetAccepted<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAccepted(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickTriangleEvent::setAccepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_accepted(&self) -> SetAccepted {
      SetAccepted(self.0)
    }
  }
  impl ::pick_triangle_event::PickTriangleEvent {
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

impl ::cpp_utils::DynamicCast<::pick_triangle_event::PickTriangleEvent> for ::pick_event::PickEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pick_triangle_event::PickTriangleEvent> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_dynamic_cast_Qt3DRender_QPickTriangleEvent_ptr(self as *mut ::pick_event::PickEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pick_triangle_event::PickTriangleEvent> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_dynamic_cast_Qt3DRender_QPickTriangleEvent_ptr(self as *const ::pick_event::PickEvent as *mut ::pick_event::PickEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::pick_event::PickEvent> for ::pick_triangle_event::PickTriangleEvent {
  fn static_cast_mut(&mut self) -> &mut ::pick_event::PickEvent {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(self as *mut ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::pick_event::PickEvent {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(self as *const ::pick_triangle_event::PickTriangleEvent as *mut ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::pick_triangle_event::PickTriangleEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_QObject_ptr(self as *mut ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_QObject_ptr(self as *const ::pick_triangle_event::PickTriangleEvent as *mut ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pick_triangle_event::PickTriangleEvent> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pick_triangle_event::PickTriangleEvent {
    let ffi_result = ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pick_triangle_event::PickTriangleEvent {
    let ffi_result = ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pick_triangle_event::PickTriangleEvent> for ::pick_event::PickEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pick_triangle_event::PickTriangleEvent {
    let ffi_result = ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_Qt3DRender_QPickEvent(self as *mut ::pick_event::PickEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pick_triangle_event::PickTriangleEvent {
    let ffi_result = ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_Qt3DRender_QPickEvent(self as *const ::pick_event::PickEvent as *mut ::pick_event::PickEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::pick_triangle_event::PickTriangleEvent {
  type Target = ::pick_event::PickEvent;
  fn deref(&self) -> &::pick_event::PickEvent {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(self as *const ::pick_triangle_event::PickTriangleEvent as *mut ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::pick_triangle_event::PickTriangleEvent {
  fn deref_mut(&mut self) -> &mut ::pick_event::PickEvent {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(self as *mut ::pick_triangle_event::PickTriangleEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PickTriangleEvent::new](../struct.PickTriangleEvent.html#method.new) method.
  pub trait PickTriangleEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent>;
  }
  impl PickTriangleEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent> {

      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PickTriangleEventNewArgs
    for (&'a ::qt_core::point_f::PointF,
                                             &'a ::qt_gui::vector_3d::Vector3D,
                                             &'a ::qt_gui::vector_3d::Vector3D,
                                             ::libc::c_float,
                                             ::libc::c_uint,
                                             ::libc::c_uint,
                                             ::libc::c_uint,
                                             ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent> {
      let position = self.0;
      let world_intersection = self.1;
      let local_intersection = self.2;
      let distance = self.3;
      let triangle_index = self.4;
      let vertex1_index = self.5;
      let vertex2_index = self.6;
      let vertex3_index = self.7;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_position_worldIntersection_localIntersection_distance_triangleIndex_vertex1Index_vertex2Index_vertex3Index(position as *const ::qt_core::point_f::PointF, world_intersection as *const ::qt_gui::vector_3d::Vector3D, local_intersection as *const ::qt_gui::vector_3d::Vector3D, distance, triangle_index, vertex1_index, vertex2_index, vertex3_index) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PickTriangleEventNewArgs
    for (&'a ::qt_core::point_f::PointF,
                                             &'a ::qt_gui::vector_3d::Vector3D,
                                             &'a ::qt_gui::vector_3d::Vector3D,
                                             ::libc::c_float,
                                             ::libc::c_uint,
                                             ::libc::c_uint,
                                             ::libc::c_uint,
                                             ::libc::c_uint,
                                             ::pick_event::Buttons,
                                             ::libc::c_int,
                                             ::libc::c_int,
                                             &'a ::qt_gui::vector_3d::Vector3D) {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_triangle_event::PickTriangleEvent> {
      let position = self.0;
      let world_intersection = self.1;
      let local_intersection = self.2;
      let distance = self.3;
      let triangle_index = self.4;
      let vertex1_index = self.5;
      let vertex2_index = self.6;
      let vertex3_index = self.7;
      let button = self.8;
      let buttons = self.9;
      let modifiers = self.10;
      let uvw = self.11;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_position_worldIntersection_localIntersection_distance_triangleIndex_vertex1Index_vertex2Index_vertex3Index_button_buttons_modifiers_uvw(position as *const ::qt_core::point_f::PointF, world_intersection as *const ::qt_gui::vector_3d::Vector3D, local_intersection as *const ::qt_gui::vector_3d::Vector3D, distance, triangle_index, vertex1_index, vertex2_index, vertex3_index, button, buttons, modifiers, uvw as *const ::qt_gui::vector_3d::Vector3D) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
