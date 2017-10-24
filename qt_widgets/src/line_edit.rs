/// C++ type: <span style='color: green;'>```QLineEdit::ActionPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ActionPosition {
  /// C++ enum variant: <span style='color: green;'>```LeadingPosition = 0```</span>
  Leading = 0,
  /// C++ enum variant: <span style='color: green;'>```TrailingPosition = 1```</span>
  Trailing = 1,
}

/// C++ type: <span style='color: green;'>```QLineEdit::EchoMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum EchoMode {
  /// C++ enum variant: <span style='color: green;'>```Normal = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```NoEcho = 1```</span>
  NoEcho = 1,
  /// C++ enum variant: <span style='color: green;'>```Password = 2```</span>
  Password = 2,
  /// C++ enum variant: <span style='color: green;'>```PasswordEchoOnEdit = 3```</span>
  PasswordEchoOnEdit = 3,
}

/// C++ type: <span style='color: green;'>```QLineEdit```</span>
#[repr(C)]
pub struct LineEdit(u8);

impl LineEdit {
  /// C++ method: <span style='color: green;'>```QAction* QLineEdit::addAction(const QIcon& icon, QLineEdit::ActionPosition position)```</span>
  ///
  ///
  pub fn add_action(&mut self,
                    icon: &::qt_gui::icon::Icon,
                    position: ::line_edit::ActionPosition)
                    -> *mut ::action::Action {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_addAction_icon_position(self as *mut ::line_edit::LineEdit,
                                                            icon as *const ::qt_gui::icon::Icon,
                                                            position)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::addAction(QAction* action, QLineEdit::ActionPosition position)```</span>
  ///
  ///
  pub unsafe fn add_action_unsafe(&mut self, action: *mut ::action::Action, position: ::line_edit::ActionPosition) {
    ::ffi::qt_widgets_c_QLineEdit_addAction_action_position(self as *mut ::line_edit::LineEdit, action, position)
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::backspace()```</span>
  ///
  ///
  pub fn backspace(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_backspace(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_clear(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QCompleter* QLineEdit::completer() const```</span>
  ///
  ///
  pub fn completer(&self) -> *mut ::completer::Completer {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_completer(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::copy() const```</span>
  ///
  ///
  pub fn copy(&self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_copy(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QMenu* QLineEdit::createStandardContextMenu()```</span>
  ///
  ///
  pub fn create_standard_context_menu(&mut self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_createStandardContextMenu(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QLineEdit::cursorBackward```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cursor_backward(&mut self, bool) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineEdit::cursorBackward(bool mark)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cursor_backward(&mut self, (bool, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineEdit::cursorBackward(bool mark, int steps = ?)```</span>
  ///
  ///
  pub fn cursor_backward<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LineEditCursorBackwardArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLineEdit::cursorForward```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cursor_forward(&mut self, bool) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineEdit::cursorForward(bool mark)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cursor_forward(&mut self, (bool, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineEdit::cursorForward(bool mark, int steps = ?)```</span>
  ///
  ///
  pub fn cursor_forward<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LineEditCursorForwardArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QLineEdit::cursorPosition() const```</span>
  ///
  ///
  pub fn cursor_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_cursorPosition(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QLineEdit::cursorPositionAt(const QPoint& pos)```</span>
  ///
  ///
  pub fn cursor_position_at(&mut self, pos: &::qt_core::point::Point) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_cursorPositionAt(self as *mut ::line_edit::LineEdit,
                                                     pos as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::cursorWordBackward(bool mark)```</span>
  ///
  ///
  pub fn cursor_word_backward(&mut self, mark: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_cursorWordBackward(self as *mut ::line_edit::LineEdit, mark) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::cursorWordForward(bool mark)```</span>
  ///
  ///
  pub fn cursor_word_forward(&mut self, mark: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_cursorWordForward(self as *mut ::line_edit::LineEdit, mark) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::cut()```</span>
  ///
  ///
  pub fn cut(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_cut(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::del()```</span>
  ///
  ///
  pub fn del(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_del(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::deselect()```</span>
  ///
  ///
  pub fn deselect(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_deselect(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QLineEdit::displayText() const```</span>
  ///
  ///
  pub fn display_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_displayText_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::dragEnabled() const```</span>
  ///
  ///
  pub fn drag_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_dragEnabled(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QLineEdit::EchoMode QLineEdit::echoMode() const```</span>
  ///
  ///
  pub fn echo_mode(&self) -> ::line_edit::EchoMode {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_echoMode(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::end(bool mark)```</span>
  ///
  ///
  pub fn end(&mut self, mark: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_end(self as *mut ::line_edit::LineEdit, mark) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QLineEdit::event(QEvent* arg1)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, arg1: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QLineEdit_event(self as *mut ::line_edit::LineEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::getTextMargins(int* left, int* top, int* right, int* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_text_margins(&self,
                                 left: *mut ::libc::c_int,
                                 top: *mut ::libc::c_int,
                                 right: *mut ::libc::c_int,
                                 bottom: *mut ::libc::c_int) {
    ::ffi::qt_widgets_c_QLineEdit_getTextMargins(self as *const ::line_edit::LineEdit,
                                                 left,
                                                 top,
                                                 right,
                                                 bottom)
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::hasAcceptableInput() const```</span>
  ///
  ///
  pub fn has_acceptable_input(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_hasAcceptableInput(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::hasFrame() const```</span>
  ///
  ///
  pub fn has_frame(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_hasFrame(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::hasSelectedText() const```</span>
  ///
  ///
  pub fn has_selected_text(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_hasSelectedText(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::home(bool mark)```</span>
  ///
  ///
  pub fn home(&mut self, mark: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_home(self as *mut ::line_edit::LineEdit, mark) }
  }

  /// C++ method: <span style='color: green;'>```QString QLineEdit::inputMask() const```</span>
  ///
  ///
  pub fn input_mask(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_inputMask_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineEdit::inputMethodQuery```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn input_method_query(&self, (&::qt_core::qt::InputMethodQuery, &::qt_core::variant::Variant)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QLineEdit::inputMethodQuery(Qt::InputMethodQuery property, QVariant argument) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn input_method_query(&self, &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QLineEdit::inputMethodQuery(Qt::InputMethodQuery arg1) const```</span>
  ///
  ///
  pub fn input_method_query<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::LineEditInputMethodQueryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QLineEdit::insert(const QString& arg1)```</span>
  ///
  ///
  pub fn insert(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_insert(self as *mut ::line_edit::LineEdit,
                                           arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::isClearButtonEnabled() const```</span>
  ///
  ///
  pub fn is_clear_button_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_isClearButtonEnabled(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::isModified() const```</span>
  ///
  ///
  pub fn is_modified(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_isModified(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_isReadOnly(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::isRedoAvailable() const```</span>
  ///
  ///
  pub fn is_redo_available(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_isRedoAvailable(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineEdit::isUndoAvailable() const```</span>
  ///
  ///
  pub fn is_undo_available(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_isUndoAvailable(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QLineEdit::maxLength() const```</span>
  ///
  ///
  pub fn max_length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_maxLength(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QLineEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_metaObject(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLineEdit::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_minimumSizeHint_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineEdit::QLineEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::line_edit::LineEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineEdit::QLineEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::line_edit::LineEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineEdit::QLineEdit(const QString& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::line_edit::LineEdit>
    where Args: overloading::LineEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLineEdit::QLineEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::line_edit::LineEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineEdit::QLineEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::line_edit::LineEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineEdit::QLineEdit(const QString& arg1, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::line_edit::LineEdit>
    where Args: overloading::LineEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::paste()```</span>
  ///
  ///
  pub fn paste(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_paste(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QLineEdit::placeholderText() const```</span>
  ///
  ///
  pub fn placeholder_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_placeholderText_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QLineEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QLineEdit_qt_metacall(self as *mut ::line_edit::LineEdit,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QLineEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QLineEdit_qt_metacast(self as *mut ::line_edit::LineEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_redo(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_selectAll(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QLineEdit::selectedText() const```</span>
  ///
  ///
  pub fn selected_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_selectedText_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QLineEdit::selectionStart() const```</span>
  ///
  ///
  pub fn selection_start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_selectionStart(self as *const ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setClearButtonEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_clear_button_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setClearButtonEnabled(self as *mut ::line_edit::LineEdit, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setCompleter(QCompleter* completer)```</span>
  ///
  ///
  pub unsafe fn set_completer(&mut self, completer: *mut ::completer::Completer) {
    ::ffi::qt_widgets_c_QLineEdit_setCompleter(self as *mut ::line_edit::LineEdit, completer)
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setCursorMoveStyle(Qt::CursorMoveStyle style)```</span>
  ///
  ///
  pub fn set_cursor_move_style(&mut self, style: &::qt_core::qt::CursorMoveStyle) {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_setCursorMoveStyle(self as *mut ::line_edit::LineEdit,
                                                       style as *const ::qt_core::qt::CursorMoveStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setCursorPosition(int arg1)```</span>
  ///
  ///
  pub fn set_cursor_position(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setCursorPosition(self as *mut ::line_edit::LineEdit, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setDragEnabled(bool b)```</span>
  ///
  ///
  pub fn set_drag_enabled(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setDragEnabled(self as *mut ::line_edit::LineEdit, b) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setEchoMode(QLineEdit::EchoMode arg1)```</span>
  ///
  ///
  pub fn set_echo_mode(&mut self, arg1: ::line_edit::EchoMode) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setEchoMode(self as *mut ::line_edit::LineEdit, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setFrame(bool arg1)```</span>
  ///
  ///
  pub fn set_frame(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setFrame(self as *mut ::line_edit::LineEdit, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setInputMask(const QString& inputMask)```</span>
  ///
  ///
  pub fn set_input_mask(&mut self, input_mask: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_setInputMask(self as *mut ::line_edit::LineEdit,
                                                 input_mask as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setMaxLength(int arg1)```</span>
  ///
  ///
  pub fn set_max_length(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setMaxLength(self as *mut ::line_edit::LineEdit, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setModified(bool arg1)```</span>
  ///
  ///
  pub fn set_modified(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setModified(self as *mut ::line_edit::LineEdit, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setPlaceholderText(const QString& arg1)```</span>
  ///
  ///
  pub fn set_placeholder_text(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_setPlaceholderText(self as *mut ::line_edit::LineEdit,
                                                       arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setReadOnly(bool arg1)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setReadOnly(self as *mut ::line_edit::LineEdit, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QLineEdit::setSelection(int arg1, int arg2)```</span>
  ///
  ///
  pub fn set_selection(&mut self, arg1: ::libc::c_int, arg2: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_setSelection(self as *mut ::line_edit::LineEdit, arg1, arg2) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::setText(const QString& arg1)```</span>
  ///
  ///
  pub fn set_text(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QLineEdit_setText(self as *mut ::line_edit::LineEdit,
                                            arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QLineEdit::setTextMargins```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_text_margins(&mut self, &::qt_core::margins::Margins) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineEdit::setTextMargins(const QMargins& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_text_margins(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineEdit::setTextMargins(int left, int top, int right, int bottom)```</span>
  ///
  ///
  pub fn set_text_margins<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LineEditSetTextMarginsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QLineEdit::setValidator(const QValidator* arg1)```</span>
  ///
  ///
  pub unsafe fn set_validator(&mut self, arg1: *const ::qt_gui::validator::Validator) {
    ::ffi::qt_widgets_c_QLineEdit_setValidator(self as *mut ::line_edit::LineEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QLineEdit::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_sizeHint_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QLineEdit::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_text_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMargins QLineEdit::textMargins() const```</span>
  ///
  ///
  pub fn text_margins(&self) -> ::qt_core::margins::Margins {
    {
      let mut object: ::qt_core::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_textMargins_to_output(self as *const ::line_edit::LineEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLineEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLineEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLineEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QLineEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QLineEdit::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_undo(self as *mut ::line_edit::LineEdit) }
  }

  /// C++ method: <span style='color: green;'>```const QValidator* QLineEdit::validator() const```</span>
  ///
  ///
  pub fn validator(&self) -> *const ::qt_gui::validator::Validator {
    unsafe { ::ffi::qt_widgets_c_QLineEdit_validator(self as *const ::line_edit::LineEdit) }
  }
}

impl ::cpp_utils::CppDeletable for ::line_edit::LineEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QLineEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `LineEdit`.
  pub struct Signals<'a>(&'a ::line_edit::LineEdit);
  /// Represents a built-in Qt signal `QLineEdit::returnPressed`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().return_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct ReturnPressed<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for ReturnPressed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2returnPressed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ReturnPressed<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::editingFinished`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().editing_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct EditingFinished<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for EditingFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editingFinished()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditingFinished<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::windowTitleChanged`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct WindowTitleChanged<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct WindowIconTextChanged<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for WindowIconTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconTextChanged<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for CustomContextMenuRequested<'a> {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customContextMenuRequested(const QPoint&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomContextMenuRequested<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::selectionChanged`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SelectionChanged<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectionChanged<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::textEdited`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().text_edited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct TextEdited<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for TextEdited<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textEdited(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextEdited<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::cursorPositionChanged`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().cursor_position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct CursorPositionChanged<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for CursorPositionChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cursorPositionChanged(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CursorPositionChanged<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::textChanged`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct TextChanged<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for TextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextChanged<'a> {}
  /// Represents a built-in Qt signal `QLineEdit::windowIconChanged`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct WindowIconChanged<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for WindowIconChanged<'a> {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconChanged(const QIcon&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QLineEdit::returnPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn return_pressed(&self) -> ReturnPressed {
      ReturnPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::editingFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editing_finished(&self) -> EditingFinished {
      EditingFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::textEdited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_edited(&self) -> TextEdited {
      TextEdited(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::cursorPositionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_position_changed(&self) -> CursorPositionChanged {
      CursorPositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QLineEdit::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `LineEdit`.
  pub struct Slots<'a>(&'a ::line_edit::LineEdit);
  /// Represents a built-in Qt slot `QLineEdit::setEnabled`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetEnabled<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::updateMicroFocus`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct UpdateMicroFocus<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::cut`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().cut()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Cut<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Cut<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1cut()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::paste`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().paste()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Paste<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Paste<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1paste()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::update`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Update<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::showMinimized`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct ShowMinimized<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::close`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Close<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::hide`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Hide<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::undo`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Undo<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::copy`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().copy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Copy<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Copy<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1copy()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::showMaximized`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct ShowMaximized<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setWindowModified`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetWindowModified<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::show`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Show<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::raise`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Raise<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::clear`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Clear<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::redo`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Redo<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::repaint`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Repaint<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setText`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetText<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setDisabled`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetDisabled<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::showNormal`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct ShowNormal<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::lower`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct Lower<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setVisible`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetVisible<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::showFullScreen`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct ShowFullScreen<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setFocus`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetFocus<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setHidden`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetHidden<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::selectAll`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SelectAll<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setWindowTitle`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetWindowTitle<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QLineEdit::setStyleSheet`.
  ///
  /// An object of this type can be created from `LineEdit` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LineEdit` object.
  pub struct SetStyleSheet<'a>(&'a ::line_edit::LineEdit);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QLineEdit::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::cut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cut(&self) -> Cut {
      Cut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::paste`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn paste(&self) -> Paste {
      Paste(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::copy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy(&self) -> Copy {
      Copy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text(&self) -> SetText {
      SetText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QLineEdit::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
  }
  impl ::line_edit::LineEdit {
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

impl ::cpp_utils::DynamicCast<::line_edit::LineEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::line_edit::LineEdit> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLineEdit_G_dynamic_cast_QLineEdit_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::line_edit::LineEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_G_dynamic_cast_QLineEdit_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::line_edit::LineEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QObject_ptr(self as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QObject_ptr(self as *const ::line_edit::LineEdit as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::line_edit::LineEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QPaintDevice_ptr(self as *const ::line_edit::LineEdit as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::line_edit::LineEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QWidget_ptr(self as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QWidget_ptr(self as *const ::line_edit::LineEdit as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::line_edit::LineEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::line_edit::LineEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::line_edit::LineEdit {
    let ffi_result = ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::line_edit::LineEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::line_edit::LineEdit {
    let ffi_result = ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::line_edit::LineEdit {
    let ffi_result = ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::line_edit::LineEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::line_edit::LineEdit {
    let ffi_result = ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::line_edit::LineEdit {
    let ffi_result = ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::line_edit::LineEdit {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QWidget_ptr(self as *const ::line_edit::LineEdit as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::line_edit::LineEdit {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QLineEdit_G_static_cast_QWidget_ptr(self as *mut ::line_edit::LineEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LineEdit::cursor_backward](../struct.LineEdit.html#method.cursor_backward) method.
  pub trait LineEditCursorBackwardArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> ();
  }
  impl<'largs> LineEditCursorBackwardArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> () {
      let mark = self;
      unsafe { ::ffi::qt_widgets_c_QLineEdit_cursorBackward_mark(original_self as *mut ::line_edit::LineEdit, mark) }
    }
  }
  impl<'largs> LineEditCursorBackwardArgs<'largs> for (bool, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> () {
      let mark = self.0;
      let steps = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_cursorBackward_mark_steps(original_self as *mut ::line_edit::LineEdit,
                                                                mark,
                                                                steps)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LineEdit::cursor_forward](../struct.LineEdit.html#method.cursor_forward) method.
  pub trait LineEditCursorForwardArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> ();
  }
  impl<'largs> LineEditCursorForwardArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> () {
      let mark = self;
      unsafe { ::ffi::qt_widgets_c_QLineEdit_cursorForward_mark(original_self as *mut ::line_edit::LineEdit, mark) }
    }
  }
  impl<'largs> LineEditCursorForwardArgs<'largs> for (bool, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> () {
      let mark = self.0;
      let steps = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_cursorForward_mark_steps(original_self as *mut ::line_edit::LineEdit, mark, steps)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LineEdit::input_method_query](../struct.LineEdit.html#method.input_method_query) method.
  pub trait LineEditInputMethodQueryArgs<'largs> {
    fn exec(self, original_self: &'largs ::line_edit::LineEdit) -> ::qt_core::variant::Variant;
  }
  impl<'largs> LineEditInputMethodQueryArgs<'largs> for &'largs ::qt_core::qt::InputMethodQuery {
    fn exec(self, original_self: &'largs ::line_edit::LineEdit) -> ::qt_core::variant::Variant {
      let arg1 = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QLineEdit_inputMethodQuery_to_output_arg1(original_self as *const ::line_edit::LineEdit, arg1 as *const ::qt_core::qt::InputMethodQuery, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> LineEditInputMethodQueryArgs<'largs>
    for (&'largs ::qt_core::qt::InputMethodQuery, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs ::line_edit::LineEdit) -> ::qt_core::variant::Variant {
      let property = self.0;
      let argument = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QLineEdit_inputMethodQuery_to_output_property_argument(original_self as *const ::line_edit::LineEdit, property as *const ::qt_core::qt::InputMethodQuery, argument as *const ::qt_core::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LineEdit::new](../struct.LineEdit.html#method.new) method.
  pub trait LineEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::line_edit::LineEdit>;
  }
  impl<'a> LineEditNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::line_edit::LineEdit> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_new_arg1(arg1 as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl LineEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::line_edit::LineEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QLineEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [LineEdit::new_unsafe](../struct.LineEdit.html#method.new_unsafe) method.
  pub trait LineEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::line_edit::LineEdit>;
  }
  impl<'a> LineEditNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::line_edit::LineEdit> {
      let arg1 = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QLineEdit_new_arg1_parent(arg1 as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl LineEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::line_edit::LineEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QLineEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [LineEdit::set_text_margins](../struct.LineEdit.html#method.set_text_margins) method.
  pub trait LineEditSetTextMarginsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> ();
  }
  impl<'largs> LineEditSetTextMarginsArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> () {
      let left = self.0;
      let top = self.1;
      let right = self.2;
      let bottom = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_setTextMargins_left_top_right_bottom(original_self as *mut ::line_edit::LineEdit, left, top, right, bottom)
      }
    }
  }
  impl<'largs> LineEditSetTextMarginsArgs<'largs> for &'largs ::qt_core::margins::Margins {
    fn exec(self, original_self: &'largs mut ::line_edit::LineEdit) -> () {
      let margins = self;
      unsafe {
        ::ffi::qt_widgets_c_QLineEdit_setTextMargins_margins(original_self as *mut ::line_edit::LineEdit,
                                                             margins as *const ::qt_core::margins::Margins)
      }
    }
  }
}
