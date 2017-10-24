/// C++ type: <span style='color: green;'>```QUndoCommand```</span>
#[repr(C)]
pub struct UndoCommand(u8);

impl UndoCommand {
  /// C++ method: <span style='color: green;'>```QString QUndoCommand::actionText() const```</span>
  ///
  ///
  pub fn action_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoCommand_actionText_to_output(self as *const ::undo_command::UndoCommand, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QUndoCommand* QUndoCommand::child(int index) const```</span>
  ///
  ///
  pub fn child(&self, index: ::libc::c_int) -> *const ::undo_command::UndoCommand {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_child(self as *const ::undo_command::UndoCommand, index) }
  }

  /// C++ method: <span style='color: green;'>```int QUndoCommand::childCount() const```</span>
  ///
  ///
  pub fn child_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_childCount(self as *const ::undo_command::UndoCommand) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QUndoCommand::id() const```</span>
  ///
  ///
  pub fn id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_id(self as *const ::undo_command::UndoCommand) }
  }

  /// C++ method: <span style='color: green;'>```bool QUndoCommand::isObsolete() const```</span>
  ///
  ///
  pub fn is_obsolete(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_isObsolete(self as *const ::undo_command::UndoCommand) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QUndoCommand::mergeWith(const QUndoCommand* other)```</span>
  ///
  ///
  pub unsafe fn merge_with(&mut self, other: *const ::undo_command::UndoCommand) -> bool {
    ::ffi::qt_widgets_c_QUndoCommand_mergeWith(self as *mut ::undo_command::UndoCommand, other)
  }

  /// C++ method: <span style='color: green;'>```QUndoCommand::QUndoCommand```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoCommand::QUndoCommand()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoCommand::QUndoCommand(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>
    where Args: overloading::UndoCommandNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUndoCommand::QUndoCommand```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::undo_command::UndoCommand) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoCommand::QUndoCommand(QUndoCommand* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::undo_command::UndoCommand)) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUndoCommand::QUndoCommand(const QString& text, QUndoCommand* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>
    where Args: overloading::UndoCommandNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual void QUndoCommand::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_redo(self as *mut ::undo_command::UndoCommand) }
  }

  /// C++ method: <span style='color: green;'>```void QUndoCommand::setObsolete(bool obsolete)```</span>
  ///
  ///
  pub fn set_obsolete(&mut self, obsolete: bool) {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_setObsolete(self as *mut ::undo_command::UndoCommand, obsolete) }
  }

  /// C++ method: <span style='color: green;'>```void QUndoCommand::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QUndoCommand_setText(self as *mut ::undo_command::UndoCommand,
                                               text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QUndoCommand::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QUndoCommand_text_to_output(self as *const ::undo_command::UndoCommand, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QUndoCommand::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QUndoCommand_undo(self as *mut ::undo_command::UndoCommand) }
  }
}

impl ::cpp_utils::CppDeletable for ::undo_command::UndoCommand {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QUndoCommand_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UndoCommand::new](../struct.UndoCommand.html#method.new) method.
  pub trait UndoCommandNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>;
  }
  impl UndoCommandNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::undo_command::UndoCommand> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoCommand_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> UndoCommandNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::undo_command::UndoCommand> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QUndoCommand_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [UndoCommand::new_unsafe](../struct.UndoCommand.html#method.new_unsafe) method.
  pub trait UndoCommandNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_command::UndoCommand>;
  }
  impl UndoCommandNewUnsafeArgs for *mut ::undo_command::UndoCommand {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_command::UndoCommand> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QUndoCommand_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> UndoCommandNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::undo_command::UndoCommand) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::undo_command::UndoCommand> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QUndoCommand_new_text_parent(text as *const ::qt_core::string::String,
                                                                        parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
