/// C++ type: <span style='color: green;'>```Qt3DInput::QInputSettings```</span>
#[repr(C)]
pub struct InputSettings(u8);

impl InputSettings {
  /// C++ method: <span style='color: green;'>```QObject* Qt3DInput::QInputSettings::eventSource() const```</span>
  ///
  ///
  pub fn event_source(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_eventSource(self as *const ::input_settings::InputSettings) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QInputSettings::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_metaObject(self as *const ::input_settings::InputSettings) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QInputSettings::QInputSettings()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::input_settings::InputSettings> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QInputSettings::QInputSettings(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::input_settings::InputSettings> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QInputSettings::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_qt_metacall(self as *mut ::input_settings::InputSettings,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QInputSettings::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_qt_metacast(self as *mut ::input_settings::InputSettings, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QInputSettings::setEventSource(QObject* eventSource)```</span>
  ///
  ///
  pub unsafe fn set_event_source(&mut self, event_source: *mut ::qt_core::object::Object) {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_setEventSource(self as *mut ::input_settings::InputSettings,
                                                                 event_source)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QInputSettings::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QInputSettings::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::input_settings::InputSettings {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSettings_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `InputSettings`.
  pub struct Signals<'a>(&'a ::input_settings::InputSettings);
  /// Represents a built-in Qt signal `Qt3DInput::QInputSettings::shareableChanged`.
  ///
  /// An object of this type can be created from `InputSettings` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSettings` object.
  pub struct ShareableChanged<'a>(&'a ::input_settings::InputSettings);
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
  /// Represents a built-in Qt signal `Qt3DInput::QInputSettings::eventSourceChanged`.
  ///
  /// An object of this type can be created from `InputSettings` with `object.signals().event_source_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSettings` object.
  pub struct EventSourceChanged<'a>(&'a ::input_settings::InputSettings);
  impl<'a> ::qt_core::connection::Receiver for EventSourceChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2eventSourceChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EventSourceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QInputSettings::removedFromEntity`.
  ///
  /// An object of this type can be created from `InputSettings` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSettings` object.
  pub struct RemovedFromEntity<'a>(&'a ::input_settings::InputSettings);
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
  /// Represents a built-in Qt signal `Qt3DInput::QInputSettings::addedToEntity`.
  ///
  /// An object of this type can be created from `InputSettings` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSettings` object.
  pub struct AddedToEntity<'a>(&'a ::input_settings::InputSettings);
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
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QInputSettings::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QInputSettings::eventSourceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn event_source_changed(&self) -> EventSourceChanged {
      EventSourceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QInputSettings::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QInputSettings::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `InputSettings`.
  pub struct Slots<'a>(&'a ::input_settings::InputSettings);
  /// Represents a built-in Qt slot `Qt3DInput::QInputSettings::setEventSource`.
  ///
  /// An object of this type can be created from `InputSettings` with `object.slots().set_event_source()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSettings` object.
  pub struct SetEventSource<'a>(&'a ::input_settings::InputSettings);
  impl<'a> ::qt_core::connection::Receiver for SetEventSource<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEventSource(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QInputSettings::setShareable`.
  ///
  /// An object of this type can be created from `InputSettings` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSettings` object.
  pub struct SetShareable<'a>(&'a ::input_settings::InputSettings);
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
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QInputSettings::setEventSource`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_event_source(&self) -> SetEventSource {
      SetEventSource(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QInputSettings::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::input_settings::InputSettings {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::input_settings::InputSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::input_settings::InputSettings as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::input_settings::InputSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::input_settings::InputSettings as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::input_settings::InputSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_QObject_ptr(self as *mut ::input_settings::InputSettings)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_QObject_ptr(self as *const ::input_settings::InputSettings as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_settings::InputSettings> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_settings::InputSettings {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_settings::InputSettings {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_settings::InputSettings> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_settings::InputSettings {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_settings::InputSettings {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_settings::InputSettings> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_settings::InputSettings {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_settings::InputSettings {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DInput_QInputSettings_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_settings::InputSettings {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::input_settings::InputSettings as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_settings::InputSettings {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::input_settings::InputSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
