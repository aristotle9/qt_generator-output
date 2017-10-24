/// C++ type: <span style='color: green;'>```Qt3DInput::QAxisAccumulator```</span>
#[repr(C)]
pub struct AxisAccumulator(u8);

impl AxisAccumulator {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QAxisAccumulator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_metaObject(self as *const ::axis_accumulator::AxisAccumulator)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAxisAccumulator::QAxisAccumulator()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::axis_accumulator::AxisAccumulator> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAxisAccumulator::QAxisAccumulator(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::axis_accumulator::AxisAccumulator> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAxisAccumulator::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_qt_metacall(self as *mut ::axis_accumulator::AxisAccumulator,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QAxisAccumulator::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_qt_metacast(self as *mut ::axis_accumulator::AxisAccumulator, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QAxisAccumulator::scale() const```</span>
  ///
  ///
  pub fn scale(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_scale(self as *const ::axis_accumulator::AxisAccumulator) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAxisAccumulator::setScale(float scale)```</span>
  ///
  ///
  pub fn set_scale(&mut self, scale: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_setScale(self as *mut ::axis_accumulator::AxisAccumulator, scale)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAxisAccumulator::setSourceAxis(Qt3DInput::QAxis* sourceAxis)```</span>
  ///
  ///
  pub unsafe fn set_source_axis(&mut self, source_axis: *mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_setSourceAxis(self as *mut ::axis_accumulator::AxisAccumulator,
                                                                  source_axis)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAxisAccumulator::setSourceAxisType(Qt3DInput::QAxisAccumulator::SourceAxisType sourceAxisType)```</span>
  ///
  ///
  pub fn set_source_axis_type(&mut self, source_axis_type: &::axis_accumulator::SourceAxisType) {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_setSourceAxisType(self as *mut ::axis_accumulator::AxisAccumulator, source_axis_type as *const ::axis_accumulator::SourceAxisType) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* Qt3DInput::QAxisAccumulator::sourceAxis() const```</span>
  ///
  ///
  pub fn source_axis(&self) -> *mut ::axis::Axis {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_sourceAxis(self as *const ::axis_accumulator::AxisAccumulator)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisAccumulator::SourceAxisType Qt3DInput::QAxisAccumulator::sourceAxisType() const```</span>
  ///
  ///
  pub fn source_axis_type(&self) -> ::axis_accumulator::SourceAxisType {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_sourceAxisType(self as *const ::axis_accumulator::AxisAccumulator)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAxisAccumulator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAxisAccumulator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QAxisAccumulator::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_value(self as *const ::axis_accumulator::AxisAccumulator) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QAxisAccumulator::velocity() const```</span>
  ///
  ///
  pub fn velocity(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_velocity(self as *const ::axis_accumulator::AxisAccumulator)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::axis_accumulator::AxisAccumulator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisAccumulator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AxisAccumulator`.
  pub struct Signals<'a>(&'a ::axis_accumulator::AxisAccumulator);
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::removedFromEntity`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct RemovedFromEntity<'a>(&'a ::axis_accumulator::AxisAccumulator);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::addedToEntity`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct AddedToEntity<'a>(&'a ::axis_accumulator::AxisAccumulator);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::sourceAxisTypeChanged`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().source_axis_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct SourceAxisTypeChanged<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for SourceAxisTypeChanged<'a> {
    type Arguments = (&'static ::axis_accumulator::SourceAxisType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceAxisTypeChanged(Qt3DInput::QAxisAccumulator::SourceAxisType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceAxisTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::velocityChanged`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().velocity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct VelocityChanged<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for VelocityChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2velocityChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VelocityChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::valueChanged`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct ValueChanged<'a>(&'a ::axis_accumulator::AxisAccumulator);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::scaleChanged`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct ScaleChanged<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for ScaleChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::sourceAxisChanged`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().source_axis_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct SourceAxisChanged<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for SourceAxisChanged<'a> {
    type Arguments = (*mut ::axis::Axis,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceAxisChanged(Qt3DInput::QAxis*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceAxisChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisAccumulator::shareableChanged`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct ShareableChanged<'a>(&'a ::axis_accumulator::AxisAccumulator);
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
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::sourceAxisTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_axis_type_changed(&self) -> SourceAxisTypeChanged {
      SourceAxisTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::velocityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn velocity_changed(&self) -> VelocityChanged {
      VelocityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::sourceAxisChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_axis_changed(&self) -> SourceAxisChanged {
      SourceAxisChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisAccumulator::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AxisAccumulator`.
  pub struct Slots<'a>(&'a ::axis_accumulator::AxisAccumulator);
  /// Represents a built-in Qt slot `Qt3DInput::QAxisAccumulator::setSourceAxis`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.slots().set_source_axis()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct SetSourceAxis<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for SetSourceAxis<'a> {
    type Arguments = (*mut ::axis::Axis,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceAxis(Qt3DInput::QAxis*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisAccumulator::setScale`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.slots().set_scale()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct SetScale<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for SetScale<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setScale(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisAccumulator::setShareable`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct SetShareable<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisAccumulator::setSourceAxisType`.
  ///
  /// An object of this type can be created from `AxisAccumulator` with `object.slots().set_source_axis_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisAccumulator` object.
  pub struct SetSourceAxisType<'a>(&'a ::axis_accumulator::AxisAccumulator);
  impl<'a> ::qt_core::connection::Receiver for SetSourceAxisType<'a> {
    type Arguments = (&'static ::axis_accumulator::SourceAxisType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceAxisType(Qt3DInput::QAxisAccumulator::SourceAxisType)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisAccumulator::setSourceAxis`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_axis(&self) -> SetSourceAxis {
      SetSourceAxis(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisAccumulator::setScale`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_scale(&self) -> SetScale {
      SetScale(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisAccumulator::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisAccumulator::setSourceAxisType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_axis_type(&self) -> SetSourceAxisType {
      SetSourceAxisType(self.0)
    }
  }
  impl ::axis_accumulator::AxisAccumulator {
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

/// C++ type: <span style='color: green;'>```Qt3DInput::QAxisAccumulator::SourceAxisType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SourceAxisType {
  /// C++ enum variant: <span style='color: green;'>```Velocity = 0```</span>
  Velocity = 0,
  /// C++ enum variant: <span style='color: green;'>```Acceleration = 1```</span>
  Acceleration = 1,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::axis_accumulator::AxisAccumulator {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::axis_accumulator::AxisAccumulator as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::axis_accumulator::AxisAccumulator {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::axis_accumulator::AxisAccumulator as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::axis_accumulator::AxisAccumulator {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_QObject_ptr(self as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_QObject_ptr(self as *const ::axis_accumulator::AxisAccumulator as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::axis_accumulator::AxisAccumulator> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::axis_accumulator::AxisAccumulator {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::axis_accumulator::AxisAccumulator {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::axis_accumulator::AxisAccumulator> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::axis_accumulator::AxisAccumulator {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::axis_accumulator::AxisAccumulator {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::axis_accumulator::AxisAccumulator> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::axis_accumulator::AxisAccumulator {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::axis_accumulator::AxisAccumulator {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::axis_accumulator::AxisAccumulator {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::axis_accumulator::AxisAccumulator as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::axis_accumulator::AxisAccumulator {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::axis_accumulator::AxisAccumulator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
