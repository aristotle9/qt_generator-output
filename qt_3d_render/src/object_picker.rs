/// C++ type: <span style='color: green;'>```Qt3DRender::QObjectPicker```</span>
#[repr(C)]
pub struct ObjectPicker(u8);

impl ObjectPicker {
  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QObjectPicker::containsMouse() const```</span>
  ///
  ///
  pub fn contains_mouse(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_containsMouse(self as *const ::object_picker::ObjectPicker)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QObjectPicker::isDragEnabled() const```</span>
  ///
  ///
  pub fn is_drag_enabled(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_isDragEnabled(self as *const ::object_picker::ObjectPicker)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QObjectPicker::isHoverEnabled() const```</span>
  ///
  ///
  pub fn is_hover_enabled(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_isHoverEnabled(self as *const ::object_picker::ObjectPicker)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QObjectPicker::isPressed() const```</span>
  ///
  ///
  pub fn is_pressed(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_isPressed(self as *const ::object_picker::ObjectPicker) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QObjectPicker::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_metaObject(self as *const ::object_picker::ObjectPicker) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QObjectPicker::QObjectPicker()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::object_picker::ObjectPicker> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QObjectPicker::QObjectPicker(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::object_picker::ObjectPicker> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QObjectPicker::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_qt_metacall(self as *mut ::object_picker::ObjectPicker,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QObjectPicker::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_qt_metacast(self as *mut ::object_picker::ObjectPicker, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QObjectPicker::setDragEnabled(bool dragEnabled)```</span>
  ///
  ///
  pub fn set_drag_enabled(&mut self, drag_enabled: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_setDragEnabled(self as *mut ::object_picker::ObjectPicker,
                                                                    drag_enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QObjectPicker::setHoverEnabled(bool hoverEnabled)```</span>
  ///
  ///
  pub fn set_hover_enabled(&mut self, hover_enabled: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_setHoverEnabled(self as *mut ::object_picker::ObjectPicker,
                                                                     hover_enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QObjectPicker::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QObjectPicker::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::object_picker::ObjectPicker {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QObjectPicker_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ObjectPicker`.
  pub struct Signals<'a>(&'a ::object_picker::ObjectPicker);
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::moved`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct Moved<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for Moved<'a> {
    type Arguments = (*mut ::pick_event::PickEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2moved(Qt3DRender::QPickEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Moved<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::pressed`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct Pressed<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for Pressed<'a> {
    type Arguments = (*mut ::pick_event::PickEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressed(Qt3DRender::QPickEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::clicked`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct Clicked<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (*mut ::pick_event::PickEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(Qt3DRender::QPickEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::hoverEnabledChanged`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().hover_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct HoverEnabledChanged<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for HoverEnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hoverEnabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HoverEnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::pressedChanged`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().pressed_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct PressedChanged<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for PressedChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressedChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PressedChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::containsMouseChanged`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().contains_mouse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct ContainsMouseChanged<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for ContainsMouseChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2containsMouseChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ContainsMouseChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::shareableChanged`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct ShareableChanged<'a>(&'a ::object_picker::ObjectPicker);
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
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::addedToEntity`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct AddedToEntity<'a>(&'a ::object_picker::ObjectPicker);
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
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::dragEnabledChanged`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().drag_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct DragEnabledChanged<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for DragEnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dragEnabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DragEnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::exited`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().exited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct Exited<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for Exited<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2exited()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Exited<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::released`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct Released<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for Released<'a> {
    type Arguments = (*mut ::pick_event::PickEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2released(Qt3DRender::QPickEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Released<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::entered`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct Entered<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for Entered<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2entered()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Entered<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QObjectPicker::removedFromEntity`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct RemovedFromEntity<'a>(&'a ::object_picker::ObjectPicker);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::moved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn moved(&self) -> Moved {
      Moved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::hoverEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hover_enabled_changed(&self) -> HoverEnabledChanged {
      HoverEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::pressedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed_changed(&self) -> PressedChanged {
      PressedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::containsMouseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn contains_mouse_changed(&self) -> ContainsMouseChanged {
      ContainsMouseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::dragEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn drag_enabled_changed(&self) -> DragEnabledChanged {
      DragEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::exited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exited(&self) -> Exited {
      Exited(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QObjectPicker::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ObjectPicker`.
  pub struct Slots<'a>(&'a ::object_picker::ObjectPicker);
  /// Represents a built-in Qt slot `Qt3DRender::QObjectPicker::setHoverEnabled`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.slots().set_hover_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct SetHoverEnabled<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for SetHoverEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHoverEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QObjectPicker::setShareable`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct SetShareable<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QObjectPicker::setDragEnabled`.
  ///
  /// An object of this type can be created from `ObjectPicker` with `object.slots().set_drag_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectPicker` object.
  pub struct SetDragEnabled<'a>(&'a ::object_picker::ObjectPicker);
  impl<'a> ::qt_core::connection::Receiver for SetDragEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDragEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QObjectPicker::setHoverEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hover_enabled(&self) -> SetHoverEnabled {
      SetHoverEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QObjectPicker::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QObjectPicker::setDragEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_drag_enabled(&self) -> SetDragEnabled {
      SetDragEnabled(self.0)
    }
  }
  impl ::object_picker::ObjectPicker {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::object_picker::ObjectPicker {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::object_picker::ObjectPicker) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::object_picker::ObjectPicker as *mut ::object_picker::ObjectPicker) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::object_picker::ObjectPicker {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::object_picker::ObjectPicker)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::object_picker::ObjectPicker as *mut ::object_picker::ObjectPicker) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::object_picker::ObjectPicker {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_QObject_ptr(self as *mut ::object_picker::ObjectPicker)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_QObject_ptr(self as *const ::object_picker::ObjectPicker as *mut ::object_picker::ObjectPicker) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::object_picker::ObjectPicker> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::object_picker::ObjectPicker {
    let ffi_result = ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::object_picker::ObjectPicker {
    let ffi_result = ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::object_picker::ObjectPicker> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::object_picker::ObjectPicker {
    let ffi_result = ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::object_picker::ObjectPicker {
    let ffi_result = ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::object_picker::ObjectPicker> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::object_picker::ObjectPicker {
    let ffi_result = ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::object_picker::ObjectPicker {
    let ffi_result = ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::object_picker::ObjectPicker {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::object_picker::ObjectPicker as *mut ::object_picker::ObjectPicker) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::object_picker::ObjectPicker {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::object_picker::ObjectPicker) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
