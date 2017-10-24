/// C++ type: <span style='color: green;'>```Qt3DRender::QComputeCommand```</span>
#[repr(C)]
pub struct ComputeCommand(u8);

impl ComputeCommand {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QComputeCommand::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_metaObject(self as *const ::compute_command::ComputeCommand)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QComputeCommand::QComputeCommand()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::compute_command::ComputeCommand> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QComputeCommand::QComputeCommand(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::compute_command::ComputeCommand> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QComputeCommand::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_qt_metacall(self as *mut ::compute_command::ComputeCommand,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QComputeCommand::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_qt_metacast(self as *mut ::compute_command::ComputeCommand, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QComputeCommand::setWorkGroupX(int workGroupX)```</span>
  ///
  ///
  pub fn set_work_group_x(&mut self, work_group_x: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_setWorkGroupX(self as *mut ::compute_command::ComputeCommand,
                                                                     work_group_x)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QComputeCommand::setWorkGroupY(int workGroupY)```</span>
  ///
  ///
  pub fn set_work_group_y(&mut self, work_group_y: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_setWorkGroupY(self as *mut ::compute_command::ComputeCommand,
                                                                     work_group_y)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QComputeCommand::setWorkGroupZ(int workGroupZ)```</span>
  ///
  ///
  pub fn set_work_group_z(&mut self, work_group_z: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_setWorkGroupZ(self as *mut ::compute_command::ComputeCommand,
                                                                     work_group_z)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QComputeCommand::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QComputeCommand::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QComputeCommand::workGroupX() const```</span>
  ///
  ///
  pub fn work_group_x(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_workGroupX(self as *const ::compute_command::ComputeCommand)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QComputeCommand::workGroupY() const```</span>
  ///
  ///
  pub fn work_group_y(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_workGroupY(self as *const ::compute_command::ComputeCommand)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QComputeCommand::workGroupZ() const```</span>
  ///
  ///
  pub fn work_group_z(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_workGroupZ(self as *const ::compute_command::ComputeCommand)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::compute_command::ComputeCommand {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QComputeCommand_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ComputeCommand`.
  pub struct Signals<'a>(&'a ::compute_command::ComputeCommand);
  /// Represents a built-in Qt signal `Qt3DRender::QComputeCommand::workGroupXChanged`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.signals().work_group_x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct WorkGroupXChanged<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for WorkGroupXChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workGroupXChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkGroupXChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QComputeCommand::workGroupZChanged`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.signals().work_group_z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct WorkGroupZChanged<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for WorkGroupZChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workGroupZChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkGroupZChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QComputeCommand::shareableChanged`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct ShareableChanged<'a>(&'a ::compute_command::ComputeCommand);
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
  /// Represents a built-in Qt signal `Qt3DRender::QComputeCommand::workGroupYChanged`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.signals().work_group_y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct WorkGroupYChanged<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for WorkGroupYChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workGroupYChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkGroupYChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QComputeCommand::removedFromEntity`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct RemovedFromEntity<'a>(&'a ::compute_command::ComputeCommand);
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
  /// Represents a built-in Qt signal `Qt3DRender::QComputeCommand::addedToEntity`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct AddedToEntity<'a>(&'a ::compute_command::ComputeCommand);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QComputeCommand::workGroupXChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_group_x_changed(&self) -> WorkGroupXChanged {
      WorkGroupXChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QComputeCommand::workGroupZChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_group_z_changed(&self) -> WorkGroupZChanged {
      WorkGroupZChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QComputeCommand::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QComputeCommand::workGroupYChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_group_y_changed(&self) -> WorkGroupYChanged {
      WorkGroupYChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QComputeCommand::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QComputeCommand::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ComputeCommand`.
  pub struct Slots<'a>(&'a ::compute_command::ComputeCommand);
  /// Represents a built-in Qt slot `Qt3DRender::QComputeCommand::setWorkGroupY`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.slots().set_work_group_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct SetWorkGroupY<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for SetWorkGroupY<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorkGroupY(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QComputeCommand::setShareable`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct SetShareable<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QComputeCommand::setWorkGroupX`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.slots().set_work_group_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct SetWorkGroupX<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for SetWorkGroupX<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorkGroupX(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QComputeCommand::setWorkGroupZ`.
  ///
  /// An object of this type can be created from `ComputeCommand` with `object.slots().set_work_group_z()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ComputeCommand` object.
  pub struct SetWorkGroupZ<'a>(&'a ::compute_command::ComputeCommand);
  impl<'a> ::qt_core::connection::Receiver for SetWorkGroupZ<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorkGroupZ(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QComputeCommand::setWorkGroupY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_work_group_y(&self) -> SetWorkGroupY {
      SetWorkGroupY(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QComputeCommand::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QComputeCommand::setWorkGroupX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_work_group_x(&self) -> SetWorkGroupX {
      SetWorkGroupX(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QComputeCommand::setWorkGroupZ`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_work_group_z(&self) -> SetWorkGroupZ {
      SetWorkGroupZ(self.0)
    }
  }
  impl ::compute_command::ComputeCommand {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::compute_command::ComputeCommand {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::compute_command::ComputeCommand as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::compute_command::ComputeCommand {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::compute_command::ComputeCommand as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::compute_command::ComputeCommand {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_QObject_ptr(self as *mut ::compute_command::ComputeCommand)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_QObject_ptr(self as *const ::compute_command::ComputeCommand as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::compute_command::ComputeCommand> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::compute_command::ComputeCommand {
    let ffi_result = ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::compute_command::ComputeCommand {
    let ffi_result = ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::compute_command::ComputeCommand> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::compute_command::ComputeCommand {
    let ffi_result = ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::compute_command::ComputeCommand {
    let ffi_result = ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::compute_command::ComputeCommand> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::compute_command::ComputeCommand {
    let ffi_result = ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::compute_command::ComputeCommand {
    let ffi_result = ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DRender_QComputeCommand_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::compute_command::ComputeCommand {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::compute_command::ComputeCommand as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::compute_command::ComputeCommand {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QComputeCommand_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::compute_command::ComputeCommand) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
