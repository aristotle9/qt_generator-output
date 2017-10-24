/// C++ type: <span style='color: green;'>```QUndoGroup```</span>
#[repr(C)]
pub struct UndoGroup(u8);

impl UndoGroup {
  /// C++ method: <span style='color: green;'>```QUndoStack* QUndoGroup::activeStack() const```</span>
  ///
  ///
  pub fn active_stack(&self) -> *mut ::undo_stack::UndoStack {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_activeStack(self as *const ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```void QUndoGroup::addStack(QUndoStack* stack)```</span>
  ///
  ///
  pub unsafe fn add_stack(&mut self, stack: *mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QUndoGroup_addStack(self as *mut ::undo_group::UndoGroup, stack)
  }

  /// C++ method: <span style='color: green;'>```bool QUndoGroup::canRedo() const```</span>
  ///
  ///
  pub fn can_redo(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_canRedo(self as *const ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```bool QUndoGroup::canUndo() const```</span>
  ///
  ///
  pub fn can_undo(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_canUndo(self as *const ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```QUndoGroup::createRedoAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_redo_action(&self, *mut ::qt_core::object::Object) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoGroup::createRedoAction(QObject* parent) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_redo_action(&self, (*mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoGroup::createRedoAction(QObject* parent, const QString& prefix = ?) const```</span>
  ///
  ///
  pub unsafe fn create_redo_action<'largs, Args>(&'largs self, args: Args) -> *mut ::action::Action
    where Args: overloading::UndoGroupCreateRedoActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUndoGroup::createUndoAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_undo_action(&self, *mut ::qt_core::object::Object) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoGroup::createUndoAction(QObject* parent) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_undo_action(&self, (*mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoGroup::createUndoAction(QObject* parent, const QString& prefix = ?) const```</span>
  ///
  ///
  pub unsafe fn create_undo_action<'largs, Args>(&'largs self, args: Args) -> *mut ::action::Action
    where Args: overloading::UndoGroupCreateUndoActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QUndoGroup::isClean() const```</span>
  ///
  ///
  pub fn is_clean(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_isClean(self as *const ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QUndoGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_metaObject(self as *const ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUndoGroup::QUndoGroup()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::undo_group::UndoGroup> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoGroup_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUndoGroup::QUndoGroup(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::undo_group::UndoGroup> {
    let ffi_result = ::ffi::qt_widgets_c_QUndoGroup_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QUndoGroup::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QUndoGroup_qt_metacall(self as *mut ::undo_group::UndoGroup,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QUndoGroup::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QUndoGroup_qt_metacast(self as *mut ::undo_group::UndoGroup, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoGroup::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_redo(self as *mut ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoGroup::redoText() const```</span>
  ///
  ///
  pub fn redo_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoGroup_redoText_to_output(self as *const ::undo_group::UndoGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QUndoGroup::removeStack(QUndoStack* stack)```</span>
  ///
  ///
  pub unsafe fn remove_stack(&mut self, stack: *mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QUndoGroup_removeStack(self as *mut ::undo_group::UndoGroup, stack)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoGroup::setActiveStack(QUndoStack* stack)```</span>
  ///
  ///
  pub unsafe fn set_active_stack(&mut self, stack: *mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QUndoGroup_setActiveStack(self as *mut ::undo_group::UndoGroup, stack)
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*> QUndoGroup::stacks() const```</span>
  ///
  ///
  pub fn stacks(&self) -> ::list::ListUndoStackMutPtr {
    {
      let mut object: ::list::ListUndoStackMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoGroup_stacks_to_output(self as *const ::undo_group::UndoGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUndoGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QUndoGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUndoGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QUndoGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoGroup::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoGroup_undo(self as *mut ::undo_group::UndoGroup) }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoGroup::undoText() const```</span>
  ///
  ///
  pub fn undo_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoGroup_undoText_to_output(self as *const ::undo_group::UndoGroup, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::undo_group::UndoGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QUndoGroup_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `UndoGroup`.
  pub struct Signals<'a>(&'a ::undo_group::UndoGroup);
  /// Represents a built-in Qt signal `QUndoGroup::undoTextChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().undo_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct UndoTextChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for UndoTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2undoTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UndoTextChanged<'a> {}
  /// Represents a built-in Qt signal `QUndoGroup::activeStackChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().active_stack_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct ActiveStackChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for ActiveStackChanged<'a> {
    type Arguments = (*mut ::undo_stack::UndoStack,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeStackChanged(QUndoStack*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActiveStackChanged<'a> {}
  /// Represents a built-in Qt signal `QUndoGroup::objectNameChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct ObjectNameChanged<'a>(&'a ::undo_group::UndoGroup);
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
  /// Represents a built-in Qt signal `QUndoGroup::cleanChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().clean_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct CleanChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for CleanChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cleanChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CleanChanged<'a> {}
  /// Represents a built-in Qt signal `QUndoGroup::canUndoChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().can_undo_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct CanUndoChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for CanUndoChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2canUndoChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CanUndoChanged<'a> {}
  /// Represents a built-in Qt signal `QUndoGroup::redoTextChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().redo_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct RedoTextChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for RedoTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2redoTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RedoTextChanged<'a> {}
  /// Represents a built-in Qt signal `QUndoGroup::indexChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct IndexChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for IndexChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2indexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IndexChanged<'a> {}
  /// Represents a built-in Qt signal `QUndoGroup::canRedoChanged`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.signals().can_redo_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct CanRedoChanged<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for CanRedoChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2canRedoChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CanRedoChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QUndoGroup::undoTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_text_changed(&self) -> UndoTextChanged {
      UndoTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::activeStackChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_stack_changed(&self) -> ActiveStackChanged {
      ActiveStackChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::cleanChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clean_changed(&self) -> CleanChanged {
      CleanChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::canUndoChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn can_undo_changed(&self) -> CanUndoChanged {
      CanUndoChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::redoTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo_text_changed(&self) -> RedoTextChanged {
      RedoTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::indexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_changed(&self) -> IndexChanged {
      IndexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoGroup::canRedoChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn can_redo_changed(&self) -> CanRedoChanged {
      CanRedoChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `UndoGroup`.
  pub struct Slots<'a>(&'a ::undo_group::UndoGroup);
  /// Represents a built-in Qt slot `QUndoGroup::redo`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct Redo<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoGroup::setActiveStack`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.slots().set_active_stack()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct SetActiveStack<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for SetActiveStack<'a> {
    type Arguments = (*mut ::undo_stack::UndoStack,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setActiveStack(QUndoStack*)\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoGroup::undo`.
  ///
  /// An object of this type can be created from `UndoGroup` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoGroup` object.
  pub struct Undo<'a>(&'a ::undo_group::UndoGroup);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QUndoGroup::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoGroup::setActiveStack`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_active_stack(&self) -> SetActiveStack {
      SetActiveStack(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoGroup::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
  }
  impl ::undo_group::UndoGroup {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::undo_group::UndoGroup {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoGroup_G_static_cast_QObject_ptr(self as *mut ::undo_group::UndoGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoGroup_G_static_cast_QObject_ptr(self as *const ::undo_group::UndoGroup as *mut ::undo_group::UndoGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_group::UndoGroup> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_group::UndoGroup {
    let ffi_result =
      ::ffi::qt_widgets_c_QUndoGroup_G_static_cast_QUndoGroup_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_group::UndoGroup {
    let ffi_result = ::ffi::qt_widgets_c_QUndoGroup_G_static_cast_QUndoGroup_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::undo_group::UndoGroup {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoGroup_G_static_cast_QObject_ptr(self as *const ::undo_group::UndoGroup as *mut ::undo_group::UndoGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::undo_group::UndoGroup {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoGroup_G_static_cast_QObject_ptr(self as *mut ::undo_group::UndoGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UndoGroup::create_redo_action](../struct.UndoGroup.html#method.create_redo_action) method.
  pub trait UndoGroupCreateRedoActionArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::undo_group::UndoGroup) -> *mut ::action::Action;
  }
  impl<'largs> UndoGroupCreateRedoActionArgs<'largs> for *mut ::qt_core::object::Object {
    unsafe fn exec(self, original_self: &'largs ::undo_group::UndoGroup) -> *mut ::action::Action {
      let parent = self;
      ::ffi::qt_widgets_c_QUndoGroup_createRedoAction_parent(original_self as *const ::undo_group::UndoGroup, parent)
    }
  }
  impl<'largs> UndoGroupCreateRedoActionArgs<'largs>
    for (*mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs ::undo_group::UndoGroup) -> *mut ::action::Action {
      let parent = self.0;
      let prefix = self.1;
      ::ffi::qt_widgets_c_QUndoGroup_createRedoAction_parent_prefix(original_self as *const ::undo_group::UndoGroup,
                                                                    parent,
                                                                    prefix as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UndoGroup::create_undo_action](../struct.UndoGroup.html#method.create_undo_action) method.
  pub trait UndoGroupCreateUndoActionArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::undo_group::UndoGroup) -> *mut ::action::Action;
  }
  impl<'largs> UndoGroupCreateUndoActionArgs<'largs> for *mut ::qt_core::object::Object {
    unsafe fn exec(self, original_self: &'largs ::undo_group::UndoGroup) -> *mut ::action::Action {
      let parent = self;
      ::ffi::qt_widgets_c_QUndoGroup_createUndoAction_parent(original_self as *const ::undo_group::UndoGroup, parent)
    }
  }
  impl<'largs> UndoGroupCreateUndoActionArgs<'largs>
    for (*mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs ::undo_group::UndoGroup) -> *mut ::action::Action {
      let parent = self.0;
      let prefix = self.1;
      ::ffi::qt_widgets_c_QUndoGroup_createUndoAction_parent_prefix(original_self as *const ::undo_group::UndoGroup,
                                                                    parent,
                                                                    prefix as *const ::qt_core::string::String)
    }
  }
}
