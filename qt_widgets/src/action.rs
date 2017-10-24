/// C++ type: <span style='color: green;'>```QAction```</span>
#[repr(C)]
pub struct Action(u8);

impl Action {
  /// C++ method: <span style='color: green;'>```QActionGroup* QAction::actionGroup() const```</span>
  ///
  ///
  pub fn action_group(&self) -> *mut ::action_group::ActionGroup {
    unsafe { ::ffi::qt_widgets_c_QAction_actionGroup(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::activate(QAction::ActionEvent event)```</span>
  ///
  ///
  pub fn activate(&mut self, event: ::action::ActionEvent) {
    unsafe { ::ffi::qt_widgets_c_QAction_activate(self as *mut ::action::Action, event) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*> QAction::associatedGraphicsWidgets() const```</span>
  ///
  ///
  pub fn associated_graphics_widgets(&self) -> ::list::ListGraphicsWidgetMutPtr {
    {
      let mut object: ::list::ListGraphicsWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_associatedGraphicsWidgets_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*> QAction::associatedWidgets() const```</span>
  ///
  ///
  pub fn associated_widgets(&self) -> ::list::ListWidgetMutPtr {
    {
      let mut object: ::list::ListWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_associatedWidgets_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::autoRepeat() const```</span>
  ///
  ///
  pub fn auto_repeat(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_autoRepeat(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QAction::data() const```</span>
  ///
  ///
  pub fn data(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_data_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont QAction::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_font_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::hover()```</span>
  ///
  ///
  pub fn hover(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAction_hover(self as *mut ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QAction::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_icon_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QAction::iconText() const```</span>
  ///
  ///
  pub fn icon_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_iconText_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::isCheckable() const```</span>
  ///
  ///
  pub fn is_checkable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_isCheckable(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::isChecked() const```</span>
  ///
  ///
  pub fn is_checked(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_isChecked(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_isEnabled(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::isIconVisibleInMenu() const```</span>
  ///
  ///
  pub fn is_icon_visible_in_menu(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_isIconVisibleInMenu(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::isSeparator() const```</span>
  ///
  ///
  pub fn is_separator(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_isSeparator(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_isVisible(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QMenu* QAction::menu() const```</span>
  ///
  ///
  pub fn menu(&self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QAction_menu(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QAction::MenuRole QAction::menuRole() const```</span>
  ///
  ///
  pub fn menu_role(&self) -> ::action::MenuRole {
    unsafe { ::ffi::qt_widgets_c_QAction_menuRole(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAction::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QAction_metaObject(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QAction::QAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::action::Action>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAction::QAction()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::action::Action>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAction::QAction(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::action::Action>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAction::QAction(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::action::Action>
    where Args: overloading::ActionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QAction::QAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::action::Action>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAction::QAction(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::icon::Icon, &::qt_core::string::String, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::action::Action>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAction::QAction(const QIcon& icon, const QString& text, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::action::Action>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAction::QAction(const QString& text, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::action::Action>
    where Args: overloading::ActionNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QWidget* QAction::parentWidget() const```</span>
  ///
  ///
  pub fn parent_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QAction_parentWidget(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QAction::Priority QAction::priority() const```</span>
  ///
  ///
  pub fn priority(&self) -> ::action::Priority {
    unsafe { ::ffi::qt_widgets_c_QAction_priority(self as *const ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAction::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QAction_qt_metacall(self as *mut ::action::Action,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAction::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QAction_qt_metacast(self as *mut ::action::Action, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QAction::setActionGroup(QActionGroup* group)```</span>
  ///
  ///
  pub unsafe fn set_action_group(&mut self, group: *mut ::action_group::ActionGroup) {
    ::ffi::qt_widgets_c_QAction_setActionGroup(self as *mut ::action::Action, group)
  }

  /// C++ method: <span style='color: green;'>```void QAction::setAutoRepeat(bool arg1)```</span>
  ///
  ///
  pub fn set_auto_repeat(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setAutoRepeat(self as *mut ::action::Action, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setCheckable(bool arg1)```</span>
  ///
  ///
  pub fn set_checkable(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setCheckable(self as *mut ::action::Action, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::setChecked(bool arg1)```</span>
  ///
  ///
  pub fn set_checked(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setChecked(self as *mut ::action::Action, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setData(const QVariant& var)```</span>
  ///
  ///
  pub fn set_data(&mut self, var: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setData(self as *mut ::action::Action,
                                          var as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::setDisabled(bool b)```</span>
  ///
  ///
  pub fn set_disabled(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setDisabled(self as *mut ::action::Action, b) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::setEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setEnabled(self as *mut ::action::Action, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setFont(self as *mut ::action::Action,
                                          font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setIcon(self as *mut ::action::Action,
                                          icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setIconText(const QString& text)```</span>
  ///
  ///
  pub fn set_icon_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setIconText(self as *mut ::action::Action,
                                              text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setIconVisibleInMenu(bool visible)```</span>
  ///
  ///
  pub fn set_icon_visible_in_menu(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setIconVisibleInMenu(self as *mut ::action::Action, visible) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setMenu(QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn set_menu(&mut self, menu: *mut ::menu::Menu) {
    ::ffi::qt_widgets_c_QAction_setMenu(self as *mut ::action::Action, menu)
  }

  /// C++ method: <span style='color: green;'>```void QAction::setMenuRole(QAction::MenuRole menuRole)```</span>
  ///
  ///
  pub fn set_menu_role(&mut self, menu_role: ::action::MenuRole) {
    unsafe { ::ffi::qt_widgets_c_QAction_setMenuRole(self as *mut ::action::Action, menu_role) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setPriority(QAction::Priority priority)```</span>
  ///
  ///
  pub fn set_priority(&mut self, priority: ::action::Priority) {
    unsafe { ::ffi::qt_widgets_c_QAction_setPriority(self as *mut ::action::Action, priority) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setSeparator(bool b)```</span>
  ///
  ///
  pub fn set_separator(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setSeparator(self as *mut ::action::Action, b) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setShortcut(const QKeySequence& shortcut)```</span>
  ///
  ///
  pub fn set_shortcut(&mut self, shortcut: &::qt_gui::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setShortcut(self as *mut ::action::Action,
                                              shortcut as *const ::qt_gui::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setShortcutContext(Qt::ShortcutContext context)```</span>
  ///
  ///
  pub fn set_shortcut_context(&mut self, context: &::qt_core::qt::ShortcutContext) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setShortcutContext(self as *mut ::action::Action,
                                                     context as *const ::qt_core::qt::ShortcutContext)
    }
  }

  /// C++ method: <span style='color: green;'>```QAction::setShortcuts```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_shortcuts(&mut self, &::qt_gui::key_sequence::StandardKey) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QAction::setShortcuts(QKeySequence::StandardKey arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_shortcuts(&mut self, &::qt_gui::list::ListKeySequence) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QAction::setShortcuts(const QList<QKeySequence>& shortcuts)```</span>
  ///
  ///
  pub fn set_shortcuts<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ActionSetShortcutsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QAction::setStatusTip(const QString& statusTip)```</span>
  ///
  ///
  pub fn set_status_tip(&mut self, status_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setStatusTip(self as *mut ::action::Action,
                                               status_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setText(self as *mut ::action::Action,
                                          text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setToolTip(const QString& tip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setToolTip(self as *mut ::action::Action,
                                             tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::setVisible(bool arg1)```</span>
  ///
  ///
  pub fn set_visible(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QAction_setVisible(self as *mut ::action::Action, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QAction::setWhatsThis(const QString& what)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, what: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QAction_setWhatsThis(self as *mut ::action::Action,
                                               what as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence QAction::shortcut() const```</span>
  ///
  ///
  pub fn shortcut(&self) -> ::qt_gui::key_sequence::KeySequence {
    {
      let mut object: ::qt_gui::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_shortcut_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence> QAction::shortcuts() const```</span>
  ///
  ///
  pub fn shortcuts(&self) -> ::qt_gui::list::ListKeySequence {
    {
      let mut object: ::qt_gui::list::ListKeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_shortcuts_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::showStatusText()```</span>
  ///
  ///
  pub fn show_status_text(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QAction_showStatusText_no_args(self as *mut ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```bool QAction::showStatusText(QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn show_status_text_unsafe(&mut self, widget: *mut ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QAction_showStatusText_widget(self as *mut ::action::Action, widget)
  }

  /// C++ method: <span style='color: green;'>```QString QAction::statusTip() const```</span>
  ///
  ///
  pub fn status_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_statusTip_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QAction::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_text_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::toggle()```</span>
  ///
  ///
  pub fn toggle(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAction_toggle(self as *mut ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QString QAction::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_toolTip_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAction::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAction_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAction::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QAction_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAction::trigger()```</span>
  ///
  ///
  pub fn trigger(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QAction_trigger(self as *mut ::action::Action) }
  }

  /// C++ method: <span style='color: green;'>```QString QAction::whatsThis() const```</span>
  ///
  ///
  pub fn whats_this(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAction_whatsThis_to_output(self as *const ::action::Action, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::action::Action {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAction_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Action`.
  pub struct Signals<'a>(&'a ::action::Action);
  /// Represents a built-in Qt signal `QAction::triggered`.
  ///
  /// An object of this type can be created from `Action` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Triggered<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Triggered<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2triggered(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Triggered<'a> {}
  /// Represents a built-in Qt signal `QAction::changed`.
  ///
  /// An object of this type can be created from `Action` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Changed<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Changed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2changed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Changed<'a> {}
  /// Represents a built-in Qt signal `QAction::objectNameChanged`.
  ///
  /// An object of this type can be created from `Action` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct ObjectNameChanged<'a>(&'a ::action::Action);
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
  /// Represents a built-in Qt signal `QAction::hovered`.
  ///
  /// An object of this type can be created from `Action` with `object.signals().hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Hovered<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Hovered<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hovered()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Hovered<'a> {}
  /// Represents a built-in Qt signal `QAction::toggled`.
  ///
  /// An object of this type can be created from `Action` with `object.signals().toggled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Toggled<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Toggled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2toggled(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Toggled<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAction::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAction::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAction::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAction::hovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hovered(&self) -> Hovered {
      Hovered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAction::toggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggled(&self) -> Toggled {
      Toggled(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Action`.
  pub struct Slots<'a>(&'a ::action::Action);
  /// Represents a built-in Qt slot `QAction::setDisabled`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct SetDisabled<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAction::setChecked`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().set_checked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct SetChecked<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for SetChecked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setChecked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAction::trigger`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().trigger()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Trigger<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Trigger<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1trigger()\0"
    }
  }
  /// Represents a built-in Qt slot `QAction::setVisible`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct SetVisible<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAction::setEnabled`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct SetEnabled<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAction::toggle`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().toggle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Toggle<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Toggle<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggle()\0"
    }
  }
  /// Represents a built-in Qt slot `QAction::hover`.
  ///
  /// An object of this type can be created from `Action` with `object.slots().hover()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Action` object.
  pub struct Hover<'a>(&'a ::action::Action);
  impl<'a> ::qt_core::connection::Receiver for Hover<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hover()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAction::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAction::setChecked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_checked(&self) -> SetChecked {
      SetChecked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAction::trigger`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn trigger(&self) -> Trigger {
      Trigger(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAction::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAction::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAction::toggle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle(&self) -> Toggle {
      Toggle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAction::hover`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hover(&self) -> Hover {
      Hover(self.0)
    }
  }
  impl ::action::Action {
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

/// C++ type: <span style='color: green;'>```QAction::ActionEvent```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ActionEvent {
  /// C++ enum variant: <span style='color: green;'>```Trigger = 0```</span>
  Trigger = 0,
  /// C++ enum variant: <span style='color: green;'>```Hover = 1```</span>
  Hover = 1,
}

/// C++ type: <span style='color: green;'>```QAction::MenuRole```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MenuRole {
  /// C++ enum variant: <span style='color: green;'>```NoRole = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```TextHeuristicRole = 1```</span>
  TextHeuristic = 1,
  /// C++ enum variant: <span style='color: green;'>```ApplicationSpecificRole = 2```</span>
  ApplicationSpecific = 2,
  /// C++ enum variant: <span style='color: green;'>```AboutQtRole = 3```</span>
  AboutQt = 3,
  /// C++ enum variant: <span style='color: green;'>```AboutRole = 4```</span>
  About = 4,
  /// C++ enum variant: <span style='color: green;'>```PreferencesRole = 5```</span>
  Preferences = 5,
  /// C++ enum variant: <span style='color: green;'>```QuitRole = 6```</span>
  Quit = 6,
}

/// C++ type: <span style='color: green;'>```QAction::Priority```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Priority {
  /// C++ enum variant: <span style='color: green;'>```LowPriority = 0```</span>
  Low = 0,
  /// C++ enum variant: <span style='color: green;'>```NormalPriority = 128```</span>
  Normal = 128,
  /// C++ enum variant: <span style='color: green;'>```HighPriority = 256```</span>
  High = 256,
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QAction* arg2)```</span>
///
///
pub unsafe fn op_shl(arg1: &::qt_core::debug::Debug, arg2: *const ::action::Action) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_widgets_c_QAction_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug, arg2, &mut object);
    object
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::action::Action {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAction_G_static_cast_QObject_ptr(self as *mut ::action::Action) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAction_G_static_cast_QObject_ptr(self as *const ::action::Action as *mut ::action::Action)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::action::Action> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::action::Action {
    let ffi_result = ::ffi::qt_widgets_c_QAction_G_static_cast_QAction_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::action::Action {
    let ffi_result = ::ffi::qt_widgets_c_QAction_G_static_cast_QAction_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::action::Action {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QAction_G_static_cast_QObject_ptr(self as *const ::action::Action as *mut ::action::Action)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::action::Action {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAction_G_static_cast_QObject_ptr(self as *mut ::action::Action) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Action::new](../struct.Action.html#method.new) method.
  pub trait ActionNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::action::Action>;
  }
  impl<'a> ActionNewArgs for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::action::Action> {
      let icon = self.0;
      let text = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QAction_new_icon_text(icon as *const ::qt_gui::icon::Icon,
                                                  text as *const ::qt_core::string::String)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ActionNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::action::Action> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QAction_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ActionNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::action::Action> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QAction_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Action::new_unsafe](../struct.Action.html#method.new_unsafe) method.
  pub trait ActionNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::action::Action>;
  }
  impl<'a> ActionNewUnsafeArgs
    for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::action::Action> {
      let icon = self.0;
      let text = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QAction_new_icon_text_parent(icon as *const ::qt_gui::icon::Icon,
                                                                        text as *const ::qt_core::string::String,
                                                                        parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ActionNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::action::Action> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QAction_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ActionNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::action::Action> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QAction_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Action::set_shortcuts](../struct.Action.html#method.set_shortcuts) method.
  pub trait ActionSetShortcutsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::action::Action) -> ();
  }
  impl<'largs> ActionSetShortcutsArgs<'largs> for &'largs ::qt_gui::key_sequence::StandardKey {
    fn exec(self, original_self: &'largs mut ::action::Action) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QAction_setShortcuts_arg1(original_self as *mut ::action::Action,
                                                      arg1 as *const ::qt_gui::key_sequence::StandardKey)
      }
    }
  }
  impl<'largs> ActionSetShortcutsArgs<'largs> for &'largs ::qt_gui::list::ListKeySequence {
    fn exec(self, original_self: &'largs mut ::action::Action) -> () {
      let shortcuts = self;
      unsafe {
        ::ffi::qt_widgets_c_QAction_setShortcuts_shortcuts(original_self as *mut ::action::Action,
                                                           shortcuts as *const ::qt_gui::list::ListKeySequence)
      }
    }
  }
}
