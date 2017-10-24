/// C++ type: <span style='color: green;'>```QMenu```</span>
#[repr(C)]
pub struct Menu(u8);

impl Menu {
  /// C++ method: <span style='color: green;'>```QAction* QMenu::actionAt(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn action_at(&self, arg1: &::qt_core::point::Point) -> *mut ::action::Action {
    unsafe {
      ::ffi::qt_widgets_c_QMenu_actionAt(self as *const ::menu::Menu,
                                         arg1 as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QMenu::actionGeometry(QAction* arg1) const```</span>
  ///
  ///
  pub unsafe fn action_geometry(&self, arg1: *mut ::action::Action) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMenu_actionGeometry_to_output(self as *const ::menu::Menu, arg1, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenu::activeAction() const```</span>
  ///
  ///
  pub fn active_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QMenu_activeAction(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```QMenu::addAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_action(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addAction(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_action(&mut self, &::qt_core::string::String) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addAction(const QString& text)```</span>
  ///
  ///
  pub fn add_action<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::MenuAddActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMenu::addAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_action_unsafe(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addAction(const QIcon& icon, const QString& text, const QObject* receiver, const char* member)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_action_unsafe(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char, &::qt_gui::key_sequence::KeySequence)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addAction(const QIcon& icon, const QString& text, const QObject* receiver, const char* member, const QKeySequence& shortcut = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_action_unsafe(&mut self, (&::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addAction(const QString& text, const QObject* receiver, const char* member)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_action_unsafe(&mut self, (&::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char, &::qt_gui::key_sequence::KeySequence)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addAction(const QString& text, const QObject* receiver, const char* member, const QKeySequence& shortcut = ?)```</span>
  ///
  ///
  pub unsafe fn add_action_unsafe<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::MenuAddActionUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMenu::addMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_menu(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QMenu::addMenu(const QIcon& icon, const QString& title)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_menu(&mut self, &::qt_core::string::String) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QMenu::addMenu(const QString& title)```</span>
  ///
  ///
  pub fn add_menu<'largs, Args>(&'largs mut self, args: Args) -> *mut ::menu::Menu
    where Args: overloading::MenuAddMenuArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addMenu(QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn add_menu_unsafe(&mut self, menu: *mut ::menu::Menu) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenu_addMenu_menu(self as *mut ::menu::Menu, menu)
  }

  /// C++ method: <span style='color: green;'>```QMenu::addSection```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_section(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addSection(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_section(&mut self, &::qt_core::string::String) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addSection(const QString& text)```</span>
  ///
  ///
  pub fn add_section<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::MenuAddSectionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QMenu::addSeparator()```</span>
  ///
  ///
  pub fn add_separator(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QMenu_addSeparator(self as *mut ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMenu_clear(self as *mut ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenu::defaultAction() const```</span>
  ///
  ///
  pub fn default_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QMenu_defaultAction(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```QMenu::exec```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exec2(&mut self, ()) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::exec()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exec2(&mut self, &::qt_core::point::Point) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::exec(const QPoint& pos)```</span>
  ///
  ///
  pub fn exec2<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::MenuExec2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QMenu::exec(const QPoint& pos, QAction* at = ?)```</span>
  ///
  ///
  pub unsafe fn exec3(&mut self, pos: &::qt_core::point::Point, at: *mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenu_exec_pos_at(self as *mut ::menu::Menu,
                                          pos as *const ::qt_core::point::Point,
                                          at)
  }

  /// C++ method: <span style='color: green;'>```static QAction* QMenu::exec(QList<QAction*> actions, const QPoint& pos)```</span>
  ///
  ///
  pub fn exec_static_0(actions: &::list::ListActionMutPtr, pos: &::qt_core::point::Point) -> *mut ::action::Action {
    unsafe {
      ::ffi::qt_widgets_c_QMenu_exec_actions_pos(actions as *const ::list::ListActionMutPtr,
                                                 pos as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QMenu::exec```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exec_static_1((&::list::ListActionMutPtr, &::qt_core::point::Point, *mut ::action::Action)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```static QAction* QMenu::exec(QList<QAction*> actions, const QPoint& pos, QAction* at = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exec_static_1((&::list::ListActionMutPtr, &::qt_core::point::Point, *mut ::action::Action, *mut ::widget::Widget)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```static QAction* QMenu::exec(QList<QAction*> actions, const QPoint& pos, QAction* at = ?, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn exec_static_1<Args>(args: Args) -> *mut ::action::Action
    where Args: overloading::MenuExecStatic1Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QMenu::hideTearOffMenu()```</span>
  ///
  ///
  pub fn hide_tear_off_menu(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMenu_hideTearOffMenu(self as *mut ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QMenu::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMenu_icon_to_output(self as *const ::menu::Menu, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenu::insertMenu(QAction* before, QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn insert_menu(&mut self,
                            before: *mut ::action::Action,
                            menu: *mut ::menu::Menu)
                            -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenu_insertMenu(self as *mut ::menu::Menu, before, menu)
  }

  /// C++ method: <span style='color: green;'>```QMenu::insertSection```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_section(&mut self, (*mut ::action::Action, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::insertSection(QAction* before, const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_section(&mut self, (*mut ::action::Action, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QMenu::insertSection(QAction* before, const QString& text)```</span>
  ///
  ///
  pub unsafe fn insert_section<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::MenuInsertSectionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QMenu::insertSeparator(QAction* before)```</span>
  ///
  ///
  pub unsafe fn insert_separator(&mut self, before: *mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenu_insertSeparator(self as *mut ::menu::Menu, before)
  }

  /// C++ method: <span style='color: green;'>```bool QMenu::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenu_isEmpty(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```bool QMenu::isTearOffEnabled() const```</span>
  ///
  ///
  pub fn is_tear_off_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenu_isTearOffEnabled(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```bool QMenu::isTearOffMenuVisible() const```</span>
  ///
  ///
  pub fn is_tear_off_menu_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenu_isTearOffMenuVisible(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenu::menuAction() const```</span>
  ///
  ///
  pub fn menu_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QMenu_menuAction(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMenu::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMenu_metaObject(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```QMenu::QMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::menu::Menu>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMenu::QMenu()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::menu::Menu>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMenu::QMenu(const QString& title)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::menu::Menu>
    where Args: overloading::MenuNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMenu::QMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::menu::Menu>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMenu::QMenu(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::menu::Menu>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMenu::QMenu(const QString& title, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::menu::Menu>
    where Args: overloading::MenuNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QMenu::popup(const QPoint& pos)```</span>
  ///
  ///
  pub fn popup(&mut self, pos: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_widgets_c_QMenu_popup_pos(self as *mut ::menu::Menu,
                                          pos as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::popup(const QPoint& pos, QAction* at = ?)```</span>
  ///
  ///
  pub unsafe fn popup_unsafe(&mut self, pos: &::qt_core::point::Point, at: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QMenu_popup_pos_at(self as *mut ::menu::Menu,
                                           pos as *const ::qt_core::point::Point,
                                           at)
  }

  /// C++ method: <span style='color: green;'>```virtual int QMenu::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMenu_qt_metacall(self as *mut ::menu::Menu,
                                          arg1 as *const ::qt_core::meta_object::Call,
                                          arg2,
                                          arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMenu::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMenu_qt_metacast(self as *mut ::menu::Menu, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QMenu::separatorsCollapsible() const```</span>
  ///
  ///
  pub fn separators_collapsible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenu_separatorsCollapsible(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setActiveAction(QAction* act)```</span>
  ///
  ///
  pub unsafe fn set_active_action(&mut self, act: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QMenu_setActiveAction(self as *mut ::menu::Menu, act)
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setAsDockMenu()```</span>
  ///
  ///
  pub fn set_as_dock_menu(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMenu_setAsDockMenu(self as *mut ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setDefaultAction(QAction* arg1)```</span>
  ///
  ///
  pub unsafe fn set_default_action(&mut self, arg1: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QMenu_setDefaultAction(self as *mut ::menu::Menu, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QMenu_setIcon(self as *mut ::menu::Menu,
                                        icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setNoReplayFor(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_no_replay_for(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QMenu_setNoReplayFor(self as *mut ::menu::Menu, widget)
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setSeparatorsCollapsible(bool collapse)```</span>
  ///
  ///
  pub fn set_separators_collapsible(&mut self, collapse: bool) {
    unsafe { ::ffi::qt_widgets_c_QMenu_setSeparatorsCollapsible(self as *mut ::menu::Menu, collapse) }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setTearOffEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_tear_off_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QMenu_setTearOffEnabled(self as *mut ::menu::Menu, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QMenu_setTitle(self as *mut ::menu::Menu,
                                         title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMenu::setToolTipsVisible(bool visible)```</span>
  ///
  ///
  pub fn set_tool_tips_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QMenu_setToolTipsVisible(self as *mut ::menu::Menu, visible) }
  }

  /// C++ method: <span style='color: green;'>```QMenu::showTearOffMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_tear_off_menu(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMenu::showTearOffMenu()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_tear_off_menu(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMenu::showTearOffMenu(const QPoint& pos)```</span>
  ///
  ///
  pub fn show_tear_off_menu<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MenuShowTearOffMenuArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QSize QMenu::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMenu_sizeHint_to_output(self as *const ::menu::Menu, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMenu::title() const```</span>
  ///
  ///
  pub fn title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMenu_title_to_output(self as *const ::menu::Menu, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMenu::toolTipsVisible() const```</span>
  ///
  ///
  pub fn tool_tips_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenu_toolTipsVisible(self as *const ::menu::Menu) }
  }

  /// C++ method: <span style='color: green;'>```static QString QMenu::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMenu_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMenu::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMenu_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::menu::Menu {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMenu_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Menu`.
  pub struct Signals<'a>(&'a ::menu::Menu);
  /// Represents a built-in Qt signal `QMenu::triggered`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Triggered<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Triggered<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2triggered(QAction*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Triggered<'a> {}
  /// Represents a built-in Qt signal `QMenu::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct WindowIconTextChanged<'a>(&'a ::menu::Menu);
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
  /// Represents a built-in Qt signal `QMenu::aboutToShow`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().about_to_show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct AboutToShow<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for AboutToShow<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToShow()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToShow<'a> {}
  /// Represents a built-in Qt signal `QMenu::aboutToHide`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().about_to_hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct AboutToHide<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for AboutToHide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToHide()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToHide<'a> {}
  /// Represents a built-in Qt signal `QMenu::windowIconChanged`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct WindowIconChanged<'a>(&'a ::menu::Menu);
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
  /// Represents a built-in Qt signal `QMenu::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::menu::Menu);
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
  /// Represents a built-in Qt signal `QMenu::windowTitleChanged`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct WindowTitleChanged<'a>(&'a ::menu::Menu);
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
  /// Represents a built-in Qt signal `QMenu::hovered`.
  ///
  /// An object of this type can be created from `Menu` with `object.signals().hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Hovered<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Hovered<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hovered(QAction*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Hovered<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QMenu::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::aboutToShow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_show(&self) -> AboutToShow {
      AboutToShow(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::aboutToHide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_hide(&self) -> AboutToHide {
      AboutToHide(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenu::hovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hovered(&self) -> Hovered {
      Hovered(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Menu`.
  pub struct Slots<'a>(&'a ::menu::Menu);
  /// Represents a built-in Qt slot `QMenu::show`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Show<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::hide`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Hide<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::raise`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Raise<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setVisible`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetVisible<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setWindowModified`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetWindowModified<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::showNormal`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct ShowNormal<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::showMinimized`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct ShowMinimized<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::close`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Close<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setDisabled`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetDisabled<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setEnabled`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetEnabled<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::showFullScreen`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct ShowFullScreen<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setWindowTitle`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetWindowTitle<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::update`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Update<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::internalDelayedPopup`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().internal_delayed_popup()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct InternalDelayedPopup<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for InternalDelayedPopup<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1internalDelayedPopup()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setStyleSheet`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetStyleSheet<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::repaint`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Repaint<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::showMaximized`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct ShowMaximized<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::lower`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct Lower<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setFocus`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetFocus<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::updateMicroFocus`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct UpdateMicroFocus<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenu::setHidden`.
  ///
  /// An object of this type can be created from `Menu` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Menu` object.
  pub struct SetHidden<'a>(&'a ::menu::Menu);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMenu::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::internalDelayedPopup`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn internal_delayed_popup(&self) -> InternalDelayedPopup {
      InternalDelayedPopup(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenu::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
  }
  impl ::menu::Menu {
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

/// C++ method: <span style='color: green;'>```void qt_mac_set_dock_menu(QMenu* menu)```</span>
///
///
pub unsafe fn qt_mac_set_dock_menu(menu: *mut ::menu::Menu) {
  ::ffi::qt_widgets_c_QMenu_G_qt_mac_set_dock_menu(menu)
}

impl ::cpp_utils::DynamicCast<::menu::Menu> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::menu::Menu> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_G_dynamic_cast_QMenu_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::menu::Menu> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMenu_G_dynamic_cast_QMenu_ptr(self as *const ::widget::Widget as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::menu::Menu {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QObject_ptr(self as *mut ::menu::Menu) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QObject_ptr(self as *const ::menu::Menu as *mut ::menu::Menu) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::menu::Menu {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QPaintDevice_ptr(self as *mut ::menu::Menu) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMenu_G_static_cast_QPaintDevice_ptr(self as *const ::menu::Menu as *mut ::menu::Menu)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::menu::Menu {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QWidget_ptr(self as *mut ::menu::Menu) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QWidget_ptr(self as *const ::menu::Menu as *mut ::menu::Menu) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::menu::Menu> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::menu::Menu {
    let ffi_result =
      ::ffi::qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::menu::Menu {
    let ffi_result = ::ffi::qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::menu::Menu> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::menu::Menu {
    let ffi_result = ::ffi::qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::menu::Menu {
    let ffi_result = ::ffi::qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::menu::Menu> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::menu::Menu {
    let ffi_result = ::ffi::qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::menu::Menu {
    let ffi_result = ::ffi::qt_widgets_c_QMenu_G_static_cast_QMenu_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::menu::Menu {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QWidget_ptr(self as *const ::menu::Menu as *mut ::menu::Menu) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::menu::Menu {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_G_static_cast_QWidget_ptr(self as *mut ::menu::Menu) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Menu::add_action](../struct.Menu.html#method.add_action) method.
  pub trait MenuAddActionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action;
  }
  impl<'largs> MenuAddActionArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_addAction_icon_text(original_self as *mut ::menu::Menu,
                                                      icon as *const ::qt_gui::icon::Icon,
                                                      text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> MenuAddActionArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_addAction_text(original_self as *mut ::menu::Menu,
                                                 text as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::add_action_unsafe](../struct.Menu.html#method.add_action_unsafe) method.
  pub trait MenuAddActionUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action;
  }
  impl<'largs> MenuAddActionUnsafeArgs<'largs>
    for (&'largs ::qt_gui::icon::Icon,
                                                        &'largs ::qt_core::string::String,
                                                        *const ::qt_core::object::Object,
                                                        *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      let receiver = self.2;
      let member = self.3;
      ::ffi::qt_widgets_c_QMenu_addAction_icon_text_receiver_member(original_self as *mut ::menu::Menu,
                                                                    icon as *const ::qt_gui::icon::Icon,
                                                                    text as *const ::qt_core::string::String,
                                                                    receiver,
                                                                    member)
    }
  }
  impl<'largs> MenuAddActionUnsafeArgs<'largs>
    for (&'largs ::qt_gui::icon::Icon,
                                                        &'largs ::qt_core::string::String,
                                                        *const ::qt_core::object::Object,
                                                        *const ::libc::c_char,
                                                        &'largs ::qt_gui::key_sequence::KeySequence) {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      let receiver = self.2;
      let member = self.3;
      let shortcut = self.4;
      ::ffi::qt_widgets_c_QMenu_addAction_icon_text_receiver_member_shortcut(original_self as *mut ::menu::Menu, icon as *const ::qt_gui::icon::Icon, text as *const ::qt_core::string::String, receiver, member, shortcut as *const ::qt_gui::key_sequence::KeySequence)
    }
  }
  impl<'largs> MenuAddActionUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let text = self.0;
      let receiver = self.1;
      let member = self.2;
      ::ffi::qt_widgets_c_QMenu_addAction_text_receiver_member(original_self as *mut ::menu::Menu,
                                                               text as *const ::qt_core::string::String,
                                                               receiver,
                                                               member)
    }
  }
  impl<'largs> MenuAddActionUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String,
                                                        *const ::qt_core::object::Object,
                                                        *const ::libc::c_char,
                                                        &'largs ::qt_gui::key_sequence::KeySequence) {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let text = self.0;
      let receiver = self.1;
      let member = self.2;
      let shortcut = self.3;
      ::ffi::qt_widgets_c_QMenu_addAction_text_receiver_member_shortcut(original_self as *mut ::menu::Menu, text as *const ::qt_core::string::String, receiver, member, shortcut as *const ::qt_gui::key_sequence::KeySequence)
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::add_menu](../struct.Menu.html#method.add_menu) method.
  pub trait MenuAddMenuArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::menu::Menu;
  }
  impl<'largs> MenuAddMenuArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::menu::Menu {
      let icon = self.0;
      let title = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_addMenu_icon_title(original_self as *mut ::menu::Menu,
                                                     icon as *const ::qt_gui::icon::Icon,
                                                     title as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> MenuAddMenuArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::menu::Menu {
      let title = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_addMenu_title(original_self as *mut ::menu::Menu,
                                                title as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::add_section](../struct.Menu.html#method.add_section) method.
  pub trait MenuAddSectionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action;
  }
  impl<'largs> MenuAddSectionArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_addSection_icon_text(original_self as *mut ::menu::Menu,
                                                       icon as *const ::qt_gui::icon::Icon,
                                                       text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> MenuAddSectionArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_addSection_text(original_self as *mut ::menu::Menu,
                                                  text as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::exec2](../struct.Menu.html#method.exec2) method.
  pub trait MenuExec2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action;
  }
  impl<'largs> MenuExec2Args<'largs> for () {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {

      unsafe { ::ffi::qt_widgets_c_QMenu_exec_no_args(original_self as *mut ::menu::Menu) }
    }
  }
  impl<'largs> MenuExec2Args<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_exec_pos(original_self as *mut ::menu::Menu,
                                           pos as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::exec_static_1](../struct.Menu.html#method.exec_static_1) method.
  pub trait MenuExecStatic1Args {
    unsafe fn exec(self) -> *mut ::action::Action;
  }
  impl<'a> MenuExecStatic1Args for (&'a ::list::ListActionMutPtr, &'a ::qt_core::point::Point, *mut ::action::Action) {
    unsafe fn exec(self) -> *mut ::action::Action {
      let actions = self.0;
      let pos = self.1;
      let at = self.2;
      ::ffi::qt_widgets_c_QMenu_exec_actions_pos_at(actions as *const ::list::ListActionMutPtr,
                                                    pos as *const ::qt_core::point::Point,
                                                    at)
    }
  }
  impl<'a> MenuExecStatic1Args
    for (&'a ::list::ListActionMutPtr, &'a ::qt_core::point::Point, *mut ::action::Action, *mut ::widget::Widget) {
    unsafe fn exec(self) -> *mut ::action::Action {
      let actions = self.0;
      let pos = self.1;
      let at = self.2;
      let parent = self.3;
      ::ffi::qt_widgets_c_QMenu_exec_actions_pos_at_parent(actions as *const ::list::ListActionMutPtr,
                                                           pos as *const ::qt_core::point::Point,
                                                           at,
                                                           parent)
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::insert_section](../struct.Menu.html#method.insert_section) method.
  pub trait MenuInsertSectionArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action;
  }
  impl<'largs> MenuInsertSectionArgs<'largs>
    for (*mut ::action::Action, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let before = self.0;
      let icon = self.1;
      let text = self.2;
      ::ffi::qt_widgets_c_QMenu_insertSection_before_icon_text(original_self as *mut ::menu::Menu,
                                                               before,
                                                               icon as *const ::qt_gui::icon::Icon,
                                                               text as *const ::qt_core::string::String)
    }
  }
  impl<'largs> MenuInsertSectionArgs<'largs> for (*mut ::action::Action, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::menu::Menu) -> *mut ::action::Action {
      let before = self.0;
      let text = self.1;
      ::ffi::qt_widgets_c_QMenu_insertSection_before_text(original_self as *mut ::menu::Menu,
                                                          before,
                                                          text as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::new](../struct.Menu.html#method.new) method.
  pub trait MenuNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::menu::Menu>;
  }
  impl MenuNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::menu::Menu> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> MenuNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::menu::Menu> {
      let title = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenu_new_title(title as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::new_unsafe](../struct.Menu.html#method.new_unsafe) method.
  pub trait MenuNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::menu::Menu>;
  }
  impl MenuNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::menu::Menu> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QMenu_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> MenuNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::menu::Menu> {
      let title = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QMenu_new_title_parent(title as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Menu::show_tear_off_menu](../struct.Menu.html#method.show_tear_off_menu) method.
  pub trait MenuShowTearOffMenuArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> ();
  }
  impl<'largs> MenuShowTearOffMenuArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> () {

      unsafe { ::ffi::qt_widgets_c_QMenu_showTearOffMenu_no_args(original_self as *mut ::menu::Menu) }
    }
  }
  impl<'largs> MenuShowTearOffMenuArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::menu::Menu) -> () {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenu_showTearOffMenu_pos(original_self as *mut ::menu::Menu,
                                                      pos as *const ::qt_core::point::Point)
      }
    }
  }
}
