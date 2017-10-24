/// C++ type: <span style='color: green;'>```QToolBar```</span>
#[repr(C)]
pub struct ToolBar(u8);

impl ToolBar {
  /// C++ method: <span style='color: green;'>```QToolBar::actionAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn action_at(&self, &::qt_core::point::Point) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::actionAt(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn action_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::actionAt(int x, int y) const```</span>
  ///
  ///
  pub fn action_at<'largs, Args>(&'largs self, args: Args) -> *mut ::action::Action
    where Args: overloading::ToolBarActionAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QToolBar::actionGeometry(QAction* action) const```</span>
  ///
  ///
  pub unsafe fn action_geometry(&self, action: *mut ::action::Action) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolBar_actionGeometry_to_output(self as *const ::tool_bar::ToolBar, action, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QToolBar::addAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_action(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::addAction(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_action(&mut self, &::qt_core::string::String) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::addAction(const QString& text)```</span>
  ///
  ///
  pub fn add_action<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::ToolBarAddActionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QToolBar::addAction```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_action_unsafe(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::addAction(const QIcon& icon, const QString& text, const QObject* receiver, const char* member)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_action_unsafe(&mut self, (&::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char)) -> *mut ::action::Action```<br>
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::addAction(const QString& text, const QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn add_action_unsafe<'largs, Args>(&'largs mut self, args: Args) -> *mut ::action::Action
    where Args: overloading::ToolBarAddActionUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QToolBar::addSeparator()```</span>
  ///
  ///
  pub fn add_separator(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QToolBar_addSeparator(self as *mut ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QToolBar::addWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn add_widget(&mut self, widget: *mut ::widget::Widget) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QToolBar_addWidget(self as *mut ::tool_bar::ToolBar, widget)
  }

  /// C++ method: <span style='color: green;'>```void QToolBar::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QToolBar_clear(self as *mut ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```QSize QToolBar::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolBar_iconSize_to_output(self as *const ::tool_bar::ToolBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* QToolBar::insertSeparator(QAction* before)```</span>
  ///
  ///
  pub unsafe fn insert_separator(&mut self, before: *mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QToolBar_insertSeparator(self as *mut ::tool_bar::ToolBar, before)
  }

  /// C++ method: <span style='color: green;'>```QAction* QToolBar::insertWidget(QAction* before, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn insert_widget(&mut self,
                              before: *mut ::action::Action,
                              widget: *mut ::widget::Widget)
                              -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QToolBar_insertWidget(self as *mut ::tool_bar::ToolBar, before, widget)
  }

  /// C++ method: <span style='color: green;'>```bool QToolBar::isAreaAllowed(Qt::ToolBarArea area) const```</span>
  ///
  ///
  pub fn is_area_allowed(&self, area: &::qt_core::qt::ToolBarArea) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QToolBar_isAreaAllowed(self as *const ::tool_bar::ToolBar,
                                                 area as *const ::qt_core::qt::ToolBarArea)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QToolBar::isFloatable() const```</span>
  ///
  ///
  pub fn is_floatable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QToolBar_isFloatable(self as *const ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QToolBar::isFloating() const```</span>
  ///
  ///
  pub fn is_floating(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QToolBar_isFloating(self as *const ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QToolBar::isMovable() const```</span>
  ///
  ///
  pub fn is_movable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QToolBar_isMovable(self as *const ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QToolBar::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QToolBar_metaObject(self as *const ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```QToolBar::QToolBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QToolBar::QToolBar()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QToolBar::QToolBar(const QString& title)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>
    where Args: overloading::ToolBarNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QToolBar::QToolBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QToolBar::QToolBar(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QToolBar::QToolBar(const QString& title, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>
    where Args: overloading::ToolBarNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QToolBar::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QToolBar_qt_metacall(self as *mut ::tool_bar::ToolBar,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QToolBar::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QToolBar_qt_metacast(self as *mut ::tool_bar::ToolBar, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QToolBar::setFloatable(bool floatable)```</span>
  ///
  ///
  pub fn set_floatable(&mut self, floatable: bool) {
    unsafe { ::ffi::qt_widgets_c_QToolBar_setFloatable(self as *mut ::tool_bar::ToolBar, floatable) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolBar::setIconSize(const QSize& iconSize)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, icon_size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QToolBar_setIconSize(self as *mut ::tool_bar::ToolBar,
                                               icon_size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QToolBar::setMovable(bool movable)```</span>
  ///
  ///
  pub fn set_movable(&mut self, movable: bool) {
    unsafe { ::ffi::qt_widgets_c_QToolBar_setMovable(self as *mut ::tool_bar::ToolBar, movable) }
  }

  /// C++ method: <span style='color: green;'>```void QToolBar::setOrientation(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, orientation: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QToolBar_setOrientation(self as *mut ::tool_bar::ToolBar,
                                                  orientation as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolBar::setToolButtonStyle(Qt::ToolButtonStyle toolButtonStyle)```</span>
  ///
  ///
  pub fn set_tool_button_style(&mut self, tool_button_style: &::qt_core::qt::ToolButtonStyle) {
    unsafe {
      ::ffi::qt_widgets_c_QToolBar_setToolButtonStyle(self as *mut ::tool_bar::ToolBar,
                                                      tool_button_style as *const ::qt_core::qt::ToolButtonStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* QToolBar::toggleViewAction() const```</span>
  ///
  ///
  pub fn toggle_view_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QToolBar_toggleViewAction(self as *const ::tool_bar::ToolBar) }
  }

  /// C++ method: <span style='color: green;'>```static QString QToolBar::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolBar_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QToolBar::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolBar_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QToolBar::widgetForAction(QAction* action) const```</span>
  ///
  ///
  pub unsafe fn widget_for_action(&self, action: *mut ::action::Action) -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QToolBar_widgetForAction(self as *const ::tool_bar::ToolBar, action)
  }
}

impl ::cpp_utils::CppDeletable for ::tool_bar::ToolBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QToolBar_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ToolBar`.
  pub struct Signals<'a>(&'a ::tool_bar::ToolBar);
  /// Represents a built-in Qt signal `QToolBar::visibilityChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().visibility_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct VisibilityChanged<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for VisibilityChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibilityChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibilityChanged<'a> {}
  /// Represents a built-in Qt signal `QToolBar::windowTitleChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct WindowTitleChanged<'a>(&'a ::tool_bar::ToolBar);
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
  /// Represents a built-in Qt signal `QToolBar::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::tool_bar::ToolBar);
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
  /// Represents a built-in Qt signal `QToolBar::movableChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().movable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct MovableChanged<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for MovableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2movableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MovableChanged<'a> {}
  /// Represents a built-in Qt signal `QToolBar::iconSizeChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct IconSizeChanged<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for IconSizeChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2iconSizeChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IconSizeChanged<'a> {}
  /// Represents a built-in Qt signal `QToolBar::topLevelChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().top_level_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct TopLevelChanged<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for TopLevelChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2topLevelChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TopLevelChanged<'a> {}
  /// Represents a built-in Qt signal `QToolBar::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct WindowIconTextChanged<'a>(&'a ::tool_bar::ToolBar);
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
  /// Represents a built-in Qt signal `QToolBar::orientationChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().orientation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct OrientationChanged<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for OrientationChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2orientationChanged(Qt::Orientation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OrientationChanged<'a> {}
  /// Represents a built-in Qt signal `QToolBar::windowIconChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct WindowIconChanged<'a>(&'a ::tool_bar::ToolBar);
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
  /// Represents a built-in Qt signal `QToolBar::actionTriggered`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().action_triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct ActionTriggered<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for ActionTriggered<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2actionTriggered(QAction*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActionTriggered<'a> {}
  /// Represents a built-in Qt signal `QToolBar::toolButtonStyleChanged`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.signals().tool_button_style_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct ToolButtonStyleChanged<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for ToolButtonStyleChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::ToolButtonStyle,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2toolButtonStyleChanged(Qt::ToolButtonStyle)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ToolButtonStyleChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QToolBar::visibilityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visibility_changed(&self) -> VisibilityChanged {
      VisibilityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::movableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn movable_changed(&self) -> MovableChanged {
      MovableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::topLevelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_level_changed(&self) -> TopLevelChanged {
      TopLevelChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::orientationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn orientation_changed(&self) -> OrientationChanged {
      OrientationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::actionTriggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn action_triggered(&self) -> ActionTriggered {
      ActionTriggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolBar::toolButtonStyleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tool_button_style_changed(&self) -> ToolButtonStyleChanged {
      ToolButtonStyleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ToolBar`.
  pub struct Slots<'a>(&'a ::tool_bar::ToolBar);
  /// Represents a built-in Qt slot `QToolBar::setFocus`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetFocus<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::update`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Update<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setToolButtonStyle`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_tool_button_style()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetToolButtonStyle<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetToolButtonStyle<'a> {
    type Arguments = (&'static ::qt_core::qt::ToolButtonStyle,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setToolButtonStyle(Qt::ToolButtonStyle)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::showMinimized`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct ShowMinimized<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::close`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Close<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setEnabled`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetEnabled<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::showNormal`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct ShowNormal<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setVisible`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetVisible<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setWindowModified`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetWindowModified<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setIconSize`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_icon_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetIconSize<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetIconSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIconSize(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::hide`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Hide<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setHidden`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetHidden<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::show`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Show<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::showMaximized`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct ShowMaximized<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::lower`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Lower<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::updateMicroFocus`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct UpdateMicroFocus<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::showFullScreen`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct ShowFullScreen<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::repaint`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Repaint<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setWindowTitle`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetWindowTitle<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setStyleSheet`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetStyleSheet<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::raise`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct Raise<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolBar::setDisabled`.
  ///
  /// An object of this type can be created from `ToolBar` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolBar` object.
  pub struct SetDisabled<'a>(&'a ::tool_bar::ToolBar);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QToolBar::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setToolButtonStyle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_tool_button_style(&self) -> SetToolButtonStyle {
      SetToolButtonStyle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setIconSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_icon_size(&self) -> SetIconSize {
      SetIconSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolBar::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
  }
  impl ::tool_bar::ToolBar {
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

impl ::cpp_utils::DynamicCast<::tool_bar::ToolBar> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tool_bar::ToolBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_G_dynamic_cast_QToolBar_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tool_bar::ToolBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_G_dynamic_cast_QToolBar_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tool_bar::ToolBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QObject_ptr(self as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QObject_ptr(self as *const ::tool_bar::ToolBar as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tool_bar::ToolBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QPaintDevice_ptr(self as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QPaintDevice_ptr(self as *const ::tool_bar::ToolBar as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tool_bar::ToolBar {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QWidget_ptr(self as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QWidget_ptr(self as *const ::tool_bar::ToolBar as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_bar::ToolBar> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_bar::ToolBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_bar::ToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_bar::ToolBar> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_bar::ToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_bar::ToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_bar::ToolBar> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_bar::ToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_bar::ToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QToolBar_G_static_cast_QToolBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tool_bar::ToolBar {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QWidget_ptr(self as *const ::tool_bar::ToolBar as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tool_bar::ToolBar {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolBar_G_static_cast_QWidget_ptr(self as *mut ::tool_bar::ToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ToolBar::action_at](../struct.ToolBar.html#method.action_at) method.
  pub trait ToolBarActionAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::tool_bar::ToolBar) -> *mut ::action::Action;
  }
  impl<'largs> ToolBarActionAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::tool_bar::ToolBar) -> *mut ::action::Action {
      let p = self;
      unsafe {
        ::ffi::qt_widgets_c_QToolBar_actionAt_p(original_self as *const ::tool_bar::ToolBar,
                                                p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> ToolBarActionAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::tool_bar::ToolBar) -> *mut ::action::Action {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QToolBar_actionAt_x_y(original_self as *const ::tool_bar::ToolBar, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [ToolBar::add_action](../struct.ToolBar.html#method.add_action) method.
  pub trait ToolBarAddActionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::tool_bar::ToolBar) -> *mut ::action::Action;
  }
  impl<'largs> ToolBarAddActionArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::tool_bar::ToolBar) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QToolBar_addAction_icon_text(original_self as *mut ::tool_bar::ToolBar,
                                                         icon as *const ::qt_gui::icon::Icon,
                                                         text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ToolBarAddActionArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::tool_bar::ToolBar) -> *mut ::action::Action {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QToolBar_addAction_text(original_self as *mut ::tool_bar::ToolBar,
                                                    text as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ToolBar::add_action_unsafe](../struct.ToolBar.html#method.add_action_unsafe) method.
  pub trait ToolBarAddActionUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tool_bar::ToolBar) -> *mut ::action::Action;
  }
  impl<'largs> ToolBarAddActionUnsafeArgs<'largs>
    for (&'largs ::qt_gui::icon::Icon,
                                                           &'largs ::qt_core::string::String,
                                                           *const ::qt_core::object::Object,
                                                           *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::tool_bar::ToolBar) -> *mut ::action::Action {
      let icon = self.0;
      let text = self.1;
      let receiver = self.2;
      let member = self.3;
      ::ffi::qt_widgets_c_QToolBar_addAction_icon_text_receiver_member(original_self as *mut ::tool_bar::ToolBar,
                                                                       icon as *const ::qt_gui::icon::Icon,
                                                                       text as *const ::qt_core::string::String,
                                                                       receiver,
                                                                       member)
    }
  }
  impl<'largs> ToolBarAddActionUnsafeArgs<'largs>
    for (&'largs ::qt_core::string::String, *const ::qt_core::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::tool_bar::ToolBar) -> *mut ::action::Action {
      let text = self.0;
      let receiver = self.1;
      let member = self.2;
      ::ffi::qt_widgets_c_QToolBar_addAction_text_receiver_member(original_self as *mut ::tool_bar::ToolBar,
                                                                  text as *const ::qt_core::string::String,
                                                                  receiver,
                                                                  member)
    }
  }
  /// This trait represents a set of arguments accepted by [ToolBar::new](../struct.ToolBar.html#method.new) method.
  pub trait ToolBarNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>;
  }
  impl ToolBarNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::tool_bar::ToolBar> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ToolBarNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::tool_bar::ToolBar> {
      let title = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolBar_new_title(title as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ToolBar::new_unsafe](../struct.ToolBar.html#method.new_unsafe) method.
  pub trait ToolBarNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tool_bar::ToolBar>;
  }
  impl ToolBarNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tool_bar::ToolBar> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QToolBar_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> ToolBarNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::tool_bar::ToolBar> {
      let title = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QToolBar_new_title_parent(title as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
