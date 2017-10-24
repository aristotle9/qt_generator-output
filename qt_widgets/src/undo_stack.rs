/// C++ type: <span style='color: green;'>```QUndoStack```</span>
#[repr(C)]
pub struct UndoStack(u8);

impl UndoStack {
  /// C++ method: <span style='color: green;'>```void QUndoStack::beginMacro(const QString& text)```</span>
  ///
  ///
  pub fn begin_macro(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QUndoStack_beginMacro(self as *mut ::undo_stack::UndoStack,
                                                text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUndoStack::canRedo() const```</span>
  ///
  ///
  pub fn can_redo(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_canRedo(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```bool QUndoStack::canUndo() const```</span>
  ///
  ///
  pub fn can_undo(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_canUndo(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```int QUndoStack::cleanIndex() const```</span>
  ///
  ///
  pub fn clean_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_cleanIndex(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```void QUndoStack::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_clear(self as *mut ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```const QUndoCommand* QUndoStack::command(int index) const```</span>
  ///
  ///
  pub fn command(&self, index: ::libc::c_int) -> *const ::undo_command::UndoCommand {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_command(self as *const ::undo_stack::UndoStack, index) }
  }

  /// C++ method: <span style='color: green;'>```int QUndoStack::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_count(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack::createRedoAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_redo_action(&self, *mut ::qt_core::object::Object) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoStack::createRedoAction(QObject* parent) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_redo_action(&self, (*mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoStack::createRedoAction(QObject* parent, const QString& prefix = ?) const```</span>
  ///
  ///
  pub unsafe fn create_redo_action<'largs, Args>(&'largs self, args: Args) -> *mut ::action::Action
    where Args: overloading::UndoStackCreateRedoActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUndoStack::createUndoAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_undo_action(&self, *mut ::qt_core::object::Object) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoStack::createUndoAction(QObject* parent) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_undo_action(&self, (*mut ::qt_core::object::Object, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QUndoStack::createUndoAction(QObject* parent, const QString& prefix = ?) const```</span>
  ///
  ///
  pub unsafe fn create_undo_action<'largs, Args>(&'largs self, args: Args) -> *mut ::action::Action
    where Args: overloading::UndoStackCreateUndoActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QUndoStack::endMacro()```</span>
  ///
  ///
  pub fn end_macro(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_endMacro(self as *mut ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```int QUndoStack::index() const```</span>
  ///
  ///
  pub fn index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_index(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```bool QUndoStack::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_isActive(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```bool QUndoStack::isClean() const```</span>
  ///
  ///
  pub fn is_clean(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_isClean(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QUndoStack::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_metaObject(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUndoStack::QUndoStack()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::undo_stack::UndoStack> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoStack_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QUndoStack::QUndoStack(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::undo_stack::UndoStack> {
    let ffi_result = ::ffi::qt_widgets_c_QUndoStack_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QUndoStack::push(QUndoCommand* cmd)```</span>
  ///
  ///
  pub unsafe fn push(&mut self, cmd: *mut ::undo_command::UndoCommand) {
    ::ffi::qt_widgets_c_QUndoStack_push(self as *mut ::undo_stack::UndoStack, cmd)
  }

  /// C++ method: <span style='color: green;'>```virtual int QUndoStack::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QUndoStack_qt_metacall(self as *mut ::undo_stack::UndoStack,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QUndoStack::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QUndoStack_qt_metacast(self as *mut ::undo_stack::UndoStack, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_redo(self as *mut ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoStack::redoText() const```</span>
  ///
  ///
  pub fn redo_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoStack_redoText_to_output(self as *const ::undo_stack::UndoStack, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::resetClean()```</span>
  ///
  ///
  pub fn reset_clean(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_resetClean(self as *mut ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack::setActive```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_active(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::setActive()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_active(&mut self, bool) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::setActive(bool active = ?)```</span>
  ///
  ///
  pub fn set_active<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UndoStackSetActiveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::setClean()```</span>
  ///
  ///
  pub fn set_clean(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_setClean(self as *mut ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::setIndex(int idx)```</span>
  ///
  ///
  pub fn set_index(&mut self, idx: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_setIndex(self as *mut ::undo_stack::UndoStack, idx) }
  }

  /// C++ method: <span style='color: green;'>```void QUndoStack::setUndoLimit(int limit)```</span>
  ///
  ///
  pub fn set_undo_limit(&mut self, limit: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_setUndoLimit(self as *mut ::undo_stack::UndoStack, limit) }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoStack::text(int idx) const```</span>
  ///
  ///
  pub fn text(&self, idx: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoStack_text_to_output(self as *const ::undo_stack::UndoStack, idx, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUndoStack::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QUndoStack_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUndoStack::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QUndoStack_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QUndoStack::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_undo(self as *mut ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```int QUndoStack::undoLimit() const```</span>
  ///
  ///
  pub fn undo_limit(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QUndoStack_undoLimit(self as *const ::undo_stack::UndoStack) }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoStack::undoText() const```</span>
  ///
  ///
  pub fn undo_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoStack_undoText_to_output(self as *const ::undo_stack::UndoStack, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::undo_stack::UndoStack {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QUndoStack_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `UndoStack`.
  pub struct Signals<'a>(&'a ::undo_stack::UndoStack);
  /// Represents a built-in Qt signal `QUndoStack::indexChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct IndexChanged<'a>(&'a ::undo_stack::UndoStack);
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
  /// Represents a built-in Qt signal `QUndoStack::cleanChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().clean_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct CleanChanged<'a>(&'a ::undo_stack::UndoStack);
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
  /// Represents a built-in Qt signal `QUndoStack::undoTextChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().undo_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct UndoTextChanged<'a>(&'a ::undo_stack::UndoStack);
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
  /// Represents a built-in Qt signal `QUndoStack::canRedoChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().can_redo_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct CanRedoChanged<'a>(&'a ::undo_stack::UndoStack);
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
  /// Represents a built-in Qt signal `QUndoStack::canUndoChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().can_undo_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct CanUndoChanged<'a>(&'a ::undo_stack::UndoStack);
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
  /// Represents a built-in Qt signal `QUndoStack::redoTextChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().redo_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct RedoTextChanged<'a>(&'a ::undo_stack::UndoStack);
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
  /// Represents a built-in Qt signal `QUndoStack::objectNameChanged`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct ObjectNameChanged<'a>(&'a ::undo_stack::UndoStack);
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
    /// Returns an object representing a built-in Qt signal `QUndoStack::indexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_changed(&self) -> IndexChanged {
      IndexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoStack::cleanChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clean_changed(&self) -> CleanChanged {
      CleanChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoStack::undoTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_text_changed(&self) -> UndoTextChanged {
      UndoTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoStack::canRedoChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn can_redo_changed(&self) -> CanRedoChanged {
      CanRedoChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoStack::canUndoChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn can_undo_changed(&self) -> CanUndoChanged {
      CanUndoChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoStack::redoTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo_text_changed(&self) -> RedoTextChanged {
      RedoTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QUndoStack::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `UndoStack`.
  pub struct Slots<'a>(&'a ::undo_stack::UndoStack);
  /// Represents a built-in Qt slot `QUndoStack::setClean`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.slots().set_clean()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct SetClean<'a>(&'a ::undo_stack::UndoStack);
  impl<'a> ::qt_core::connection::Receiver for SetClean<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setClean()\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoStack::setIndex`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.slots().set_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct SetIndex<'a>(&'a ::undo_stack::UndoStack);
  impl<'a> ::qt_core::connection::Receiver for SetIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoStack::redo`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct Redo<'a>(&'a ::undo_stack::UndoStack);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoStack::undo`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct Undo<'a>(&'a ::undo_stack::UndoStack);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoStack::resetClean`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.slots().reset_clean()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct ResetClean<'a>(&'a ::undo_stack::UndoStack);
  impl<'a> ::qt_core::connection::Receiver for ResetClean<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetClean()\0"
    }
  }
  /// Represents a built-in Qt slot `QUndoStack::setActive`.
  ///
  /// An object of this type can be created from `UndoStack` with `object.slots().set_active()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `UndoStack` object.
  pub struct SetActive<'a>(&'a ::undo_stack::UndoStack);
  impl<'a> ::qt_core::connection::Receiver for SetActive<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setActive(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QUndoStack::setClean`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_clean(&self) -> SetClean {
      SetClean(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoStack::setIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_index(&self) -> SetIndex {
      SetIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoStack::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoStack::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoStack::resetClean`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_clean(&self) -> ResetClean {
      ResetClean(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QUndoStack::setActive`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_active(&self) -> SetActive {
      SetActive(self.0)
    }
  }
  impl ::undo_stack::UndoStack {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::undo_stack::UndoStack {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoStack_G_static_cast_QObject_ptr(self as *mut ::undo_stack::UndoStack) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoStack_G_static_cast_QObject_ptr(self as *const ::undo_stack::UndoStack as *mut ::undo_stack::UndoStack) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::undo_stack::UndoStack> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::undo_stack::UndoStack {
    let ffi_result =
      ::ffi::qt_widgets_c_QUndoStack_G_static_cast_QUndoStack_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::undo_stack::UndoStack {
    let ffi_result = ::ffi::qt_widgets_c_QUndoStack_G_static_cast_QUndoStack_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::undo_stack::UndoStack {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoStack_G_static_cast_QObject_ptr(self as *const ::undo_stack::UndoStack as *mut ::undo_stack::UndoStack) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::undo_stack::UndoStack {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QUndoStack_G_static_cast_QObject_ptr(self as *mut ::undo_stack::UndoStack) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UndoStack::create_redo_action](../struct.UndoStack.html#method.create_redo_action) method.
  pub trait UndoStackCreateRedoActionArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::undo_stack::UndoStack) -> *mut ::action::Action;
  }
  impl<'largs> UndoStackCreateRedoActionArgs<'largs> for *mut ::qt_core::object::Object {
    unsafe fn exec(self, original_self: &'largs ::undo_stack::UndoStack) -> *mut ::action::Action {
      let parent = self;
      ::ffi::qt_widgets_c_QUndoStack_createRedoAction_parent(original_self as *const ::undo_stack::UndoStack, parent)
    }
  }
  impl<'largs> UndoStackCreateRedoActionArgs<'largs>
    for (*mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs ::undo_stack::UndoStack) -> *mut ::action::Action {
      let parent = self.0;
      let prefix = self.1;
      ::ffi::qt_widgets_c_QUndoStack_createRedoAction_parent_prefix(original_self as *const ::undo_stack::UndoStack,
                                                                    parent,
                                                                    prefix as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UndoStack::create_undo_action](../struct.UndoStack.html#method.create_undo_action) method.
  pub trait UndoStackCreateUndoActionArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::undo_stack::UndoStack) -> *mut ::action::Action;
  }
  impl<'largs> UndoStackCreateUndoActionArgs<'largs> for *mut ::qt_core::object::Object {
    unsafe fn exec(self, original_self: &'largs ::undo_stack::UndoStack) -> *mut ::action::Action {
      let parent = self;
      ::ffi::qt_widgets_c_QUndoStack_createUndoAction_parent(original_self as *const ::undo_stack::UndoStack, parent)
    }
  }
  impl<'largs> UndoStackCreateUndoActionArgs<'largs>
    for (*mut ::qt_core::object::Object, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs ::undo_stack::UndoStack) -> *mut ::action::Action {
      let parent = self.0;
      let prefix = self.1;
      ::ffi::qt_widgets_c_QUndoStack_createUndoAction_parent_prefix(original_self as *const ::undo_stack::UndoStack,
                                                                    parent,
                                                                    prefix as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [UndoStack::set_active](../struct.UndoStack.html#method.set_active) method.
  pub trait UndoStackSetActiveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::undo_stack::UndoStack) -> ();
  }
  impl<'largs> UndoStackSetActiveArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::undo_stack::UndoStack) -> () {
      let active = self;
      unsafe { ::ffi::qt_widgets_c_QUndoStack_setActive_active(original_self as *mut ::undo_stack::UndoStack, active) }
    }
  }
  impl<'largs> UndoStackSetActiveArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::undo_stack::UndoStack) -> () {

      unsafe { ::ffi::qt_widgets_c_QUndoStack_setActive_no_args(original_self as *mut ::undo_stack::UndoStack) }
    }
  }
}
