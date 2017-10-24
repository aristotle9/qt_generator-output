/// C++ type: <span style='color: green;'>```Qt3DRender::QPickEvent::Buttons```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Buttons {
  /// C++ enum variant: <span style='color: green;'>```NoButton = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftButton = 1```</span>
  Left = 1,
  /// C++ enum variant: <span style='color: green;'>```RightButton = 2```</span>
  Right = 2,
  /// C++ enum variant: <span style='color: green;'>```MiddleButton = 4```</span>
  Middle = 4,
  /// C++ enum variant: <span style='color: green;'>```BackButton = 8```</span>
  Back = 8,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QPickEvent::Modifiers```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Modifiers {
  /// C++ enum variant: <span style='color: green;'>```NoModifier = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ShiftModifier = 33554432```</span>
  Shift = 33554432,
  /// C++ enum variant: <span style='color: green;'>```ControlModifier = 67108864```</span>
  Control = 67108864,
  /// C++ enum variant: <span style='color: green;'>```AltModifier = 134217728```</span>
  Alt = 134217728,
  /// C++ enum variant: <span style='color: green;'>```MetaModifier = 268435456```</span>
  Meta = 268435456,
  /// C++ enum variant: <span style='color: green;'>```KeypadModifier = 536870912```</span>
  Keypad = 536870912,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QPickEvent```</span>
#[repr(C)]
pub struct PickEvent(u8);

impl PickEvent {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickEvent::Buttons Qt3DRender::QPickEvent::button() const```</span>
  ///
  ///
  pub fn button(&self) -> ::pick_event::Buttons {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_button(self as *const ::pick_event::PickEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QPickEvent::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_buttons(self as *const ::pick_event::PickEvent) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPickEvent::distance() const```</span>
  ///
  ///
  pub fn distance(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_distance(self as *const ::pick_event::PickEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QPickEvent::isAccepted() const```</span>
  ///
  ///
  pub fn is_accepted(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_isAccepted(self as *const ::pick_event::PickEvent) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QPickEvent::localIntersection() const```</span>
  ///
  ///
  pub fn local_intersection(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_localIntersection_as_ptr(self as *const ::pick_event::PickEvent)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPickEvent::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_metaObject(self as *const ::pick_event::PickEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QPickEvent::modifiers() const```</span>
  ///
  ///
  pub fn modifiers(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_modifiers(self as *const ::pick_event::PickEvent) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickEvent::QPickEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::pick_event::PickEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickEvent::QPickEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, &::qt_gui::vector_3d::Vector3D, &::qt_gui::vector_3d::Vector3D, ::libc::c_float)) -> ::cpp_utils::CppBox<::pick_event::PickEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickEvent::QPickEvent(const QPointF& position, const QVector3D& worldIntersection, const QVector3D& localIntersection, float distance)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::point_f::PointF, &::qt_gui::vector_3d::Vector3D, &::qt_gui::vector_3d::Vector3D, ::libc::c_float, ::pick_event::Buttons, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pick_event::PickEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickEvent::QPickEvent(const QPointF& position, const QVector3D& worldIntersection, const QVector3D& localIntersection, float distance, Qt3DRender::QPickEvent::Buttons button, int buttons, int modifiers)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::pick_event::PickEvent>
    where Args: overloading::PickEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF Qt3DRender::QPickEvent::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_position_to_output(self as *const ::pick_event::PickEvent,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPickEvent::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_qt_metacall(self as *mut ::pick_event::PickEvent,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPickEvent::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_qt_metacast(self as *mut ::pick_event::PickEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPickEvent::setAccepted(bool accepted)```</span>
  ///
  ///
  pub fn set_accepted(&mut self, accepted: bool) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_setAccepted(self as *mut ::pick_event::PickEvent, accepted) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPickEvent::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPickEvent::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QPickEvent::worldIntersection() const```</span>
  ///
  ///
  pub fn world_intersection(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_worldIntersection_as_ptr(self as *const ::pick_event::PickEvent)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::pick_event::PickEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PickEvent`.
  pub struct Signals<'a>(&'a ::pick_event::PickEvent);
  /// Represents a built-in Qt signal `Qt3DRender::QPickEvent::acceptedChanged`.
  ///
  /// An object of this type can be created from `PickEvent` with `object.signals().accepted_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickEvent` object.
  pub struct AcceptedChanged<'a>(&'a ::pick_event::PickEvent);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPickEvent::objectNameChanged`.
  ///
  /// An object of this type can be created from `PickEvent` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickEvent` object.
  pub struct ObjectNameChanged<'a>(&'a ::pick_event::PickEvent);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickEvent::acceptedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted_changed(&self) -> AcceptedChanged {
      AcceptedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickEvent::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PickEvent`.
  pub struct Slots<'a>(&'a ::pick_event::PickEvent);
  /// Represents a built-in Qt slot `Qt3DRender::QPickEvent::setAccepted`.
  ///
  /// An object of this type can be created from `PickEvent` with `object.slots().set_accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickEvent` object.
  pub struct SetAccepted<'a>(&'a ::pick_event::PickEvent);
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
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickEvent::setAccepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_accepted(&self) -> SetAccepted {
      SetAccepted(self.0)
    }
  }
  impl ::pick_event::PickEvent {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::pick_event::PickEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QPickEvent_G_static_cast_QObject_ptr(self as *mut ::pick_event::PickEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickEvent_G_static_cast_QObject_ptr(self as *const ::pick_event::PickEvent as *mut ::pick_event::PickEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pick_event::PickEvent> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pick_event::PickEvent {
    let ffi_result = ::ffi::qt_3d_render_c_QPickEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pick_event::PickEvent {
    let ffi_result = ::ffi::qt_3d_render_c_QPickEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::pick_event::PickEvent {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickEvent_G_static_cast_QObject_ptr(self as *const ::pick_event::PickEvent as *mut ::pick_event::PickEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::pick_event::PickEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QPickEvent_G_static_cast_QObject_ptr(self as *mut ::pick_event::PickEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PickEvent::new](../struct.PickEvent.html#method.new) method.
  pub trait PickEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_event::PickEvent>;
  }
  impl PickEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_event::PickEvent> {

      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PickEventNewArgs
    for (&'a ::qt_core::point_f::PointF,
                                     &'a ::qt_gui::vector_3d::Vector3D,
                                     &'a ::qt_gui::vector_3d::Vector3D,
                                     ::libc::c_float) {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_event::PickEvent> {
      let position = self.0;
      let world_intersection = self.1;
      let local_intersection = self.2;
      let distance = self.3;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_new_position_worldIntersection_localIntersection_distance(position as *const ::qt_core::point_f::PointF, world_intersection as *const ::qt_gui::vector_3d::Vector3D, local_intersection as *const ::qt_gui::vector_3d::Vector3D, distance) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PickEventNewArgs
    for (&'a ::qt_core::point_f::PointF,
                                     &'a ::qt_gui::vector_3d::Vector3D,
                                     &'a ::qt_gui::vector_3d::Vector3D,
                                     ::libc::c_float,
                                     ::pick_event::Buttons,
                                     ::libc::c_int,
                                     ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::pick_event::PickEvent> {
      let position = self.0;
      let world_intersection = self.1;
      let local_intersection = self.2;
      let distance = self.3;
      let button = self.4;
      let buttons = self.5;
      let modifiers = self.6;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickEvent_new_position_worldIntersection_localIntersection_distance_button_buttons_modifiers(position as *const ::qt_core::point_f::PointF, world_intersection as *const ::qt_gui::vector_3d::Vector3D, local_intersection as *const ::qt_gui::vector_3d::Vector3D, distance, button, buttons, modifiers) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
