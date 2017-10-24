/// C++ type: <span style='color: green;'>```QMenuBar```</span>
#[repr(C)]
pub struct MenuBar(u8);

impl MenuBar {
  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::actionAt(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn action_at(&self, arg1: &::qt_core::point::Point) -> *mut ::action::Action {
    unsafe {
      ::ffi::qt_widgets_c_QMenuBar_actionAt(self as *const ::menu_bar::MenuBar,
                                            arg1 as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QMenuBar::actionGeometry(QAction* arg1) const```</span>
  ///
  ///
  pub unsafe fn action_geometry(&self, arg1: *mut ::action::Action) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMenuBar_actionGeometry_to_output(self as *const ::menu_bar::MenuBar, arg1, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::activeAction() const```</span>
  ///
  ///
  pub fn active_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_activeAction(self as *const ::menu_bar::MenuBar) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::addAction(const QString& text)```</span>
  ///
  ///
  pub fn add_action(&mut self, text: &::qt_core::string::String) -> *mut ::action::Action {
    unsafe {
      ::ffi::qt_widgets_c_QMenuBar_addAction_text(self as *mut ::menu_bar::MenuBar,
                                                  text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::addAction(const QString& text, const QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn add_action_unsafe(&mut self,
                                  text: &::qt_core::string::String,
                                  receiver: *const ::qt_core::object::Object,
                                  member: *const ::libc::c_char)
                                  -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenuBar_addAction_text_receiver_member(self as *mut ::menu_bar::MenuBar,
                                                                text as *const ::qt_core::string::String,
                                                                receiver,
                                                                member)
  }

  /// C++ method: <span style='color: green;'>```QMenuBar::addMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_menu(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QMenuBar::addMenu(const QIcon& icon, const QString& title)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_menu(&mut self, &::qt_core::string::String) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QMenuBar::addMenu(const QString& title)```</span>
  ///
  ///
  pub fn add_menu<'largs, Args>(&'largs mut self, args: Args) -> *mut ::menu::Menu
    where Args: overloading::MenuBarAddMenuArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::addMenu(QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn add_menu_unsafe(&mut self, menu: *mut ::menu::Menu) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenuBar_addMenu_menu(self as *mut ::menu_bar::MenuBar, menu)
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::addSeparator()```</span>
  ///
  ///
  pub fn add_separator(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_addSeparator(self as *mut ::menu_bar::MenuBar) }
  }

  /// C++ method: <span style='color: green;'>```void QMenuBar::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_clear(self as *mut ::menu_bar::MenuBar) }
  }

  /// C++ method: <span style='color: green;'>```QMenuBar::cornerWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn corner_widget(&self, ()) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QMenuBar::cornerWidget() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn corner_widget(&self, &::qt_core::qt::Corner) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QMenuBar::cornerWidget(Qt::Corner corner = ?) const```</span>
  ///
  ///
  pub fn corner_widget<'largs, Args>(&'largs self, args: Args) -> *mut ::widget::Widget
    where Args: overloading::MenuBarCornerWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QMenuBar::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_heightForWidth(self as *const ::menu_bar::MenuBar, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::insertMenu(QAction* before, QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn insert_menu(&mut self,
                            before: *mut ::action::Action,
                            menu: *mut ::menu::Menu)
                            -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenuBar_insertMenu(self as *mut ::menu_bar::MenuBar, before, menu)
  }

  /// C++ method: <span style='color: green;'>```QAction* QMenuBar::insertSeparator(QAction* before)```</span>
  ///
  ///
  pub unsafe fn insert_separator(&mut self, before: *mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QMenuBar_insertSeparator(self as *mut ::menu_bar::MenuBar, before)
  }

  /// C++ method: <span style='color: green;'>```bool QMenuBar::isDefaultUp() const```</span>
  ///
  ///
  pub fn is_default_up(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_isDefaultUp(self as *const ::menu_bar::MenuBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QMenuBar::isNativeMenuBar() const```</span>
  ///
  ///
  pub fn is_native_menu_bar(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_isNativeMenuBar(self as *const ::menu_bar::MenuBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMenuBar::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_metaObject(self as *const ::menu_bar::MenuBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QMenuBar::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMenuBar_minimumSizeHint_to_output(self as *const ::menu_bar::MenuBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMenuBar::QMenuBar()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::menu_bar::MenuBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMenuBar::QMenuBar(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::menu_bar::MenuBar> {
    let ffi_result = ::ffi::qt_widgets_c_QMenuBar_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QMenuBar::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMenuBar_qt_metacall(self as *mut ::menu_bar::MenuBar,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMenuBar::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMenuBar_qt_metacast(self as *mut ::menu_bar::MenuBar, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QMenuBar::setActiveAction(QAction* action)```</span>
  ///
  ///
  pub unsafe fn set_active_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QMenuBar_setActiveAction(self as *mut ::menu_bar::MenuBar, action)
  }

  /// C++ method: <span style='color: green;'>```QMenuBar::setCornerWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_corner_widget(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMenuBar::setCornerWidget(QWidget* w)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_corner_widget(&mut self, (*mut ::widget::Widget, &::qt_core::qt::Corner)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMenuBar::setCornerWidget(QWidget* w, Qt::Corner corner = ?)```</span>
  ///
  ///
  pub unsafe fn set_corner_widget<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MenuBarSetCornerWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QMenuBar::setDefaultUp(bool arg1)```</span>
  ///
  ///
  pub fn set_default_up(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_setDefaultUp(self as *mut ::menu_bar::MenuBar, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QMenuBar::setNativeMenuBar(bool nativeMenuBar)```</span>
  ///
  ///
  pub fn set_native_menu_bar(&mut self, native_menu_bar: bool) {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_setNativeMenuBar(self as *mut ::menu_bar::MenuBar, native_menu_bar) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QMenuBar::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QMenuBar_setVisible(self as *mut ::menu_bar::MenuBar, visible) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QMenuBar::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMenuBar_sizeHint_to_output(self as *const ::menu_bar::MenuBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMenuBar::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMenuBar_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMenuBar::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMenuBar_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::menu_bar::MenuBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMenuBar_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MenuBar`.
  pub struct Signals<'a>(&'a ::menu_bar::MenuBar);
  /// Represents a built-in Qt signal `QMenuBar::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::menu_bar::MenuBar);
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
  /// Represents a built-in Qt signal `QMenuBar::hovered`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.signals().hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Hovered<'a>(&'a ::menu_bar::MenuBar);
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
  /// Represents a built-in Qt signal `QMenuBar::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct WindowIconTextChanged<'a>(&'a ::menu_bar::MenuBar);
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
  /// Represents a built-in Qt signal `QMenuBar::triggered`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Triggered<'a>(&'a ::menu_bar::MenuBar);
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
  /// Represents a built-in Qt signal `QMenuBar::windowTitleChanged`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct WindowTitleChanged<'a>(&'a ::menu_bar::MenuBar);
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
  /// Represents a built-in Qt signal `QMenuBar::windowIconChanged`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct WindowIconChanged<'a>(&'a ::menu_bar::MenuBar);
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
    /// Returns an object representing a built-in Qt signal `QMenuBar::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenuBar::hovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hovered(&self) -> Hovered {
      Hovered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenuBar::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenuBar::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenuBar::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMenuBar::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MenuBar`.
  pub struct Slots<'a>(&'a ::menu_bar::MenuBar);
  /// Represents a built-in Qt slot `QMenuBar::showMaximized`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct ShowMaximized<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::showNormal`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct ShowNormal<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::raise`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Raise<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setStyleSheet`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetStyleSheet<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::lower`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Lower<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::showFullScreen`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct ShowFullScreen<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setFocus`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetFocus<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::updateMicroFocus`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct UpdateMicroFocus<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::show`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Show<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setWindowTitle`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetWindowTitle<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::hide`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Hide<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::showMinimized`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct ShowMinimized<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setDisabled`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetDisabled<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setVisible`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetVisible<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setEnabled`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetEnabled<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::close`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Close<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setHidden`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetHidden<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::setWindowModified`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct SetWindowModified<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::repaint`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Repaint<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QMenuBar::update`.
  ///
  /// An object of this type can be created from `MenuBar` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MenuBar` object.
  pub struct Update<'a>(&'a ::menu_bar::MenuBar);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMenuBar::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMenuBar::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::menu_bar::MenuBar {
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

impl ::cpp_utils::DynamicCast<::menu_bar::MenuBar> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::menu_bar::MenuBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_G_dynamic_cast_QMenuBar_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::menu_bar::MenuBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_G_dynamic_cast_QMenuBar_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::menu_bar::MenuBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QObject_ptr(self as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QObject_ptr(self as *const ::menu_bar::MenuBar as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::menu_bar::MenuBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QPaintDevice_ptr(self as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QPaintDevice_ptr(self as *const ::menu_bar::MenuBar as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::menu_bar::MenuBar {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QWidget_ptr(self as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QWidget_ptr(self as *const ::menu_bar::MenuBar as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::menu_bar::MenuBar> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::menu_bar::MenuBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::menu_bar::MenuBar {
    let ffi_result = ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::menu_bar::MenuBar> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::menu_bar::MenuBar {
    let ffi_result = ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::menu_bar::MenuBar {
    let ffi_result = ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::menu_bar::MenuBar> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::menu_bar::MenuBar {
    let ffi_result = ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::menu_bar::MenuBar {
    let ffi_result = ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QMenuBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::menu_bar::MenuBar {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QWidget_ptr(self as *const ::menu_bar::MenuBar as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::menu_bar::MenuBar {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMenuBar_G_static_cast_QWidget_ptr(self as *mut ::menu_bar::MenuBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MenuBar::add_menu](../struct.MenuBar.html#method.add_menu) method.
  pub trait MenuBarAddMenuArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::menu_bar::MenuBar) -> *mut ::menu::Menu;
  }
  impl<'largs> MenuBarAddMenuArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::menu_bar::MenuBar) -> *mut ::menu::Menu {
      let icon = self.0;
      let title = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMenuBar_addMenu_icon_title(original_self as *mut ::menu_bar::MenuBar,
                                                        icon as *const ::qt_gui::icon::Icon,
                                                        title as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> MenuBarAddMenuArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::menu_bar::MenuBar) -> *mut ::menu::Menu {
      let title = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenuBar_addMenu_title(original_self as *mut ::menu_bar::MenuBar,
                                                   title as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MenuBar::corner_widget](../struct.MenuBar.html#method.corner_widget) method.
  pub trait MenuBarCornerWidgetArgs<'largs> {
    fn exec(self, original_self: &'largs ::menu_bar::MenuBar) -> *mut ::widget::Widget;
  }
  impl<'largs> MenuBarCornerWidgetArgs<'largs> for &'largs ::qt_core::qt::Corner {
    fn exec(self, original_self: &'largs ::menu_bar::MenuBar) -> *mut ::widget::Widget {
      let corner = self;
      unsafe {
        ::ffi::qt_widgets_c_QMenuBar_cornerWidget_corner(original_self as *const ::menu_bar::MenuBar,
                                                         corner as *const ::qt_core::qt::Corner)
      }
    }
  }
  impl<'largs> MenuBarCornerWidgetArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::menu_bar::MenuBar) -> *mut ::widget::Widget {

      unsafe { ::ffi::qt_widgets_c_QMenuBar_cornerWidget_no_args(original_self as *const ::menu_bar::MenuBar) }
    }
  }
  /// This trait represents a set of arguments accepted by [MenuBar::set_corner_widget](../struct.MenuBar.html#method.set_corner_widget) method.
  pub trait MenuBarSetCornerWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::menu_bar::MenuBar) -> ();
  }
  impl<'largs> MenuBarSetCornerWidgetArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::menu_bar::MenuBar) -> () {
      let w = self;
      ::ffi::qt_widgets_c_QMenuBar_setCornerWidget_w(original_self as *mut ::menu_bar::MenuBar, w)
    }
  }
  impl<'largs> MenuBarSetCornerWidgetArgs<'largs> for (*mut ::widget::Widget, &'largs ::qt_core::qt::Corner) {
    unsafe fn exec(self, original_self: &'largs mut ::menu_bar::MenuBar) -> () {
      let w = self.0;
      let corner = self.1;
      ::ffi::qt_widgets_c_QMenuBar_setCornerWidget_w_corner(original_self as *mut ::menu_bar::MenuBar,
                                                            w,
                                                            corner as *const ::qt_core::qt::Corner)
    }
  }
}
