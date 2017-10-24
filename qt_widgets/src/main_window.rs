/// C++ type: <span style='color: green;'>```QMainWindow::DockOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DockOption {
  /// C++ enum variant: <span style='color: green;'>```AnimatedDocks = 1```</span>
  AnimatedDocks = 1,
  /// C++ enum variant: <span style='color: green;'>```AllowNestedDocks = 2```</span>
  AllowNestedDocks = 2,
  /// C++ enum variant: <span style='color: green;'>```AllowTabbedDocks = 4```</span>
  AllowTabbedDocks = 4,
  /// C++ enum variant: <span style='color: green;'>```ForceTabbedDocks = 8```</span>
  ForceTabbedDocks = 8,
  /// C++ enum variant: <span style='color: green;'>```VerticalTabs = 16```</span>
  VerticalTabs = 16,
  /// C++ enum variant: <span style='color: green;'>```GroupedDragging = 32```</span>
  GroupedDragging = 32,
}

impl ::qt_core::flags::FlaggableEnum for DockOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "DockOption"
  }
}

/// C++ type: <span style='color: green;'>```QMainWindow```</span>
#[repr(C)]
pub struct MainWindow(u8);

impl MainWindow {
  /// C++ method: <span style='color: green;'>```QMainWindow::addDockWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_dock_widget(&mut self, (&::qt_core::qt::DockWidgetArea, *mut ::dock_widget::DockWidget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMainWindow::addDockWidget(Qt::DockWidgetArea area, QDockWidget* dockwidget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_dock_widget(&mut self, (&::qt_core::qt::DockWidgetArea, *mut ::dock_widget::DockWidget, &::qt_core::qt::Orientation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMainWindow::addDockWidget(Qt::DockWidgetArea area, QDockWidget* dockwidget, Qt::Orientation orientation)```</span>
  ///
  ///
  pub unsafe fn add_dock_widget<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MainWindowAddDockWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QToolBar* QMainWindow::addToolBar(const QString& title)```</span>
  ///
  ///
  pub fn add_tool_bar(&mut self, title: &::qt_core::string::String) -> *mut ::tool_bar::ToolBar {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_addToolBar_title(self as *mut ::main_window::MainWindow,
                                                       title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QMainWindow::addToolBarBreak```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_tool_bar_break(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMainWindow::addToolBarBreak()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_tool_bar_break(&mut self, &::qt_core::qt::ToolBarArea) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMainWindow::addToolBarBreak(Qt::ToolBarArea area = ?)```</span>
  ///
  ///
  pub fn add_tool_bar_break<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MainWindowAddToolBarBreakArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMainWindow::addToolBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_tool_bar_unsafe(&mut self, *mut ::tool_bar::ToolBar) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMainWindow::addToolBar(QToolBar* toolbar)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_tool_bar_unsafe(&mut self, (&::qt_core::qt::ToolBarArea, *mut ::tool_bar::ToolBar)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMainWindow::addToolBar(Qt::ToolBarArea area, QToolBar* toolbar)```</span>
  ///
  ///
  pub unsafe fn add_tool_bar_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MainWindowAddToolBarUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget* QMainWindow::centralWidget() const```</span>
  ///
  ///
  pub fn central_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_centralWidget(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual QMenu* QMainWindow::createPopupMenu()```</span>
  ///
  ///
  pub fn create_popup_menu(&mut self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_createPopupMenu(self as *mut ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QMainWindow::DockOption> QMainWindow::dockOptions() const```</span>
  ///
  ///
  pub fn dock_options(&self) -> ::qt_core::flags::Flags<::main_window::DockOption> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMainWindow_dockOptions(self as *const ::main_window::MainWindow) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::documentMode() const```</span>
  ///
  ///
  pub fn document_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_documentMode(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```QSize QMainWindow::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMainWindow_iconSize_to_output(self as *const ::main_window::MainWindow, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::insertToolBar(QToolBar* before, QToolBar* toolbar)```</span>
  ///
  ///
  pub unsafe fn insert_tool_bar(&mut self, before: *mut ::tool_bar::ToolBar, toolbar: *mut ::tool_bar::ToolBar) {
    ::ffi::qt_widgets_c_QMainWindow_insertToolBar(self as *mut ::main_window::MainWindow, before, toolbar)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::insertToolBarBreak(QToolBar* before)```</span>
  ///
  ///
  pub unsafe fn insert_tool_bar_break(&mut self, before: *mut ::tool_bar::ToolBar) {
    ::ffi::qt_widgets_c_QMainWindow_insertToolBarBreak(self as *mut ::main_window::MainWindow, before)
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::isAnimated() const```</span>
  ///
  ///
  pub fn is_animated(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_isAnimated(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::isDockNestingEnabled() const```</span>
  ///
  ///
  pub fn is_dock_nesting_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_isDockNestingEnabled(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::isSeparator(const QPoint& pos) const```</span>
  ///
  ///
  pub fn is_separator(&self, pos: &::qt_core::point::Point) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_isSeparator(self as *const ::main_window::MainWindow,
                                                  pos as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QMenuBar* QMainWindow::menuBar() const```</span>
  ///
  ///
  pub fn menu_bar(&self) -> *mut ::menu_bar::MenuBar {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_menuBar(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMainWindow::menuWidget() const```</span>
  ///
  ///
  pub fn menu_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_menuWidget(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMainWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_metaObject(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QMainWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMainWindow_qt_metacall(self as *mut ::main_window::MainWindow,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMainWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMainWindow_qt_metacast(self as *mut ::main_window::MainWindow, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::removeDockWidget(QDockWidget* dockwidget)```</span>
  ///
  ///
  pub unsafe fn remove_dock_widget(&mut self, dockwidget: *mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QMainWindow_removeDockWidget(self as *mut ::main_window::MainWindow, dockwidget)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::removeToolBar(QToolBar* toolbar)```</span>
  ///
  ///
  pub unsafe fn remove_tool_bar(&mut self, toolbar: *mut ::tool_bar::ToolBar) {
    ::ffi::qt_widgets_c_QMainWindow_removeToolBar(self as *mut ::main_window::MainWindow, toolbar)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::removeToolBarBreak(QToolBar* before)```</span>
  ///
  ///
  pub unsafe fn remove_tool_bar_break(&mut self, before: *mut ::tool_bar::ToolBar) {
    ::ffi::qt_widgets_c_QMainWindow_removeToolBarBreak(self as *mut ::main_window::MainWindow, before)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::resizeDocks(const QList<QDockWidget*>& docks, const QList<int>& sizes, Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn resize_docks(&mut self,
                      docks: &::list::ListDockWidgetMutPtr,
                      sizes: &::qt_core::list::ListCInt,
                      orientation: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_resizeDocks(self as *mut ::main_window::MainWindow,
                                                  docks as *const ::list::ListDockWidgetMutPtr,
                                                  sizes as *const ::qt_core::list::ListCInt,
                                                  orientation as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::restoreDockWidget(QDockWidget* dockwidget)```</span>
  ///
  ///
  pub unsafe fn restore_dock_widget(&mut self, dockwidget: *mut ::dock_widget::DockWidget) -> bool {
    ::ffi::qt_widgets_c_QMainWindow_restoreDockWidget(self as *mut ::main_window::MainWindow, dockwidget)
  }

  /// C++ method: <span style='color: green;'>```QMainWindow::restoreState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn restore_state(&mut self, &::qt_core::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMainWindow::restoreState(const QByteArray& state)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn restore_state(&mut self, (&::qt_core::byte_array::ByteArray, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMainWindow::restoreState(const QByteArray& state, int version = ?)```</span>
  ///
  ///
  pub fn restore_state<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::MainWindowRestoreStateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMainWindow::saveState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn save_state(&self, ()) -> ::qt_core::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QMainWindow::saveState() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn save_state(&self, ::libc::c_int) -> ::qt_core::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QMainWindow::saveState(int version = ?) const```</span>
  ///
  ///
  pub fn save_state<'largs, Args>(&'largs self, args: Args) -> ::qt_core::byte_array::ByteArray
    where Args: overloading::MainWindowSaveStateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QMainWindow::setAnimated(bool enabled)```</span>
  ///
  ///
  pub fn set_animated(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_setAnimated(self as *mut ::main_window::MainWindow, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setCentralWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_central_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QMainWindow_setCentralWidget(self as *mut ::main_window::MainWindow, widget)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setCorner(Qt::Corner corner, Qt::DockWidgetArea area)```</span>
  ///
  ///
  pub fn set_corner(&mut self, corner: &::qt_core::qt::Corner, area: &::qt_core::qt::DockWidgetArea) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_setCorner(self as *mut ::main_window::MainWindow,
                                                corner as *const ::qt_core::qt::Corner,
                                                area as *const ::qt_core::qt::DockWidgetArea)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMainWindow::setDockNestingEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_dock_nesting_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_setDockNestingEnabled(self as *mut ::main_window::MainWindow, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setDockOptions(QFlags<QMainWindow::DockOption> options)```</span>
  ///
  ///
  pub fn set_dock_options(&mut self, options: ::qt_core::flags::Flags<::main_window::DockOption>) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_setDockOptions(self as *mut ::main_window::MainWindow,
                                                     options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setDocumentMode(bool enabled)```</span>
  ///
  ///
  pub fn set_document_mode(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_setDocumentMode(self as *mut ::main_window::MainWindow, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setIconSize(const QSize& iconSize)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, icon_size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_setIconSize(self as *mut ::main_window::MainWindow,
                                                  icon_size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setMenuBar(QMenuBar* menubar)```</span>
  ///
  ///
  pub unsafe fn set_menu_bar(&mut self, menubar: *mut ::menu_bar::MenuBar) {
    ::ffi::qt_widgets_c_QMainWindow_setMenuBar(self as *mut ::main_window::MainWindow, menubar)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setMenuWidget(QWidget* menubar)```</span>
  ///
  ///
  pub unsafe fn set_menu_widget(&mut self, menubar: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QMainWindow_setMenuWidget(self as *mut ::main_window::MainWindow, menubar)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setStatusBar(QStatusBar* statusbar)```</span>
  ///
  ///
  pub unsafe fn set_status_bar(&mut self, statusbar: *mut ::status_bar::StatusBar) {
    ::ffi::qt_widgets_c_QMainWindow_setStatusBar(self as *mut ::main_window::MainWindow, statusbar)
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setTabShape(QTabWidget::TabShape tabShape)```</span>
  ///
  ///
  pub fn set_tab_shape(&mut self, tab_shape: &::tab_widget::TabShape) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_setTabShape(self as *mut ::main_window::MainWindow,
                                                  tab_shape as *const ::tab_widget::TabShape)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::setToolButtonStyle(Qt::ToolButtonStyle toolButtonStyle)```</span>
  ///
  ///
  pub fn set_tool_button_style(&mut self, tool_button_style: &::qt_core::qt::ToolButtonStyle) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_setToolButtonStyle(self as *mut ::main_window::MainWindow,
                                                         tool_button_style as *const ::qt_core::qt::ToolButtonStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set)```</span>
  ///
  ///
  pub fn set_unified_title_and_tool_bar_on_mac(&mut self, set: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QMainWindow_setUnifiedTitleAndToolBarOnMac(self as *mut ::main_window::MainWindow, set)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::splitDockWidget(QDockWidget* after, QDockWidget* dockwidget, Qt::Orientation orientation)```</span>
  ///
  ///
  pub unsafe fn split_dock_widget(&mut self,
                                  after: *mut ::dock_widget::DockWidget,
                                  dockwidget: *mut ::dock_widget::DockWidget,
                                  orientation: &::qt_core::qt::Orientation) {
    ::ffi::qt_widgets_c_QMainWindow_splitDockWidget(self as *mut ::main_window::MainWindow,
                                                    after,
                                                    dockwidget,
                                                    orientation as *const ::qt_core::qt::Orientation)
  }

  /// C++ method: <span style='color: green;'>```QStatusBar* QMainWindow::statusBar() const```</span>
  ///
  ///
  pub fn status_bar(&self) -> *mut ::status_bar::StatusBar {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_statusBar(self as *const ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*> QMainWindow::tabifiedDockWidgets(QDockWidget* dockwidget) const```</span>
  ///
  ///
  pub unsafe fn tabified_dock_widgets(&self,
                                      dockwidget: *mut ::dock_widget::DockWidget)
                                      -> ::list::ListDockWidgetMutPtr {
    {
      let mut object: ::list::ListDockWidgetMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMainWindow_tabifiedDockWidgets_to_output(self as *const ::main_window::MainWindow,
                                                                    dockwidget,
                                                                    &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMainWindow::tabifyDockWidget(QDockWidget* first, QDockWidget* second)```</span>
  ///
  ///
  pub unsafe fn tabify_dock_widget(&mut self,
                                   first: *mut ::dock_widget::DockWidget,
                                   second: *mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QMainWindow_tabifyDockWidget(self as *mut ::main_window::MainWindow, first, second)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMainWindow::takeCentralWidget()```</span>
  ///
  ///
  pub fn take_central_widget(&mut self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_takeCentralWidget(self as *mut ::main_window::MainWindow) }
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::toolBarBreak(QToolBar* toolbar) const```</span>
  ///
  ///
  pub unsafe fn tool_bar_break(&self, toolbar: *mut ::tool_bar::ToolBar) -> bool {
    ::ffi::qt_widgets_c_QMainWindow_toolBarBreak(self as *const ::main_window::MainWindow, toolbar)
  }

  /// C++ method: <span style='color: green;'>```static QString QMainWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMainWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMainWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMainWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMainWindow::unifiedTitleAndToolBarOnMac() const```</span>
  ///
  ///
  pub fn unified_title_and_tool_bar_on_mac(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMainWindow_unifiedTitleAndToolBarOnMac(self as *const ::main_window::MainWindow) }
  }
}

impl ::cpp_utils::CppDeletable for ::main_window::MainWindow {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMainWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MainWindow`.
  pub struct Signals<'a>(&'a ::main_window::MainWindow);
  /// Represents a built-in Qt signal `QMainWindow::windowTitleChanged`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct WindowTitleChanged<'a>(&'a ::main_window::MainWindow);
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
  /// Represents a built-in Qt signal `QMainWindow::tabifiedDockWidgetActivated`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().tabified_dock_widget_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct TabifiedDockWidgetActivated<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for TabifiedDockWidgetActivated<'a> {
    type Arguments = (*mut ::dock_widget::DockWidget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabifiedDockWidgetActivated(QDockWidget*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabifiedDockWidgetActivated<'a> {}
  /// Represents a built-in Qt signal `QMainWindow::iconSizeChanged`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().icon_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct IconSizeChanged<'a>(&'a ::main_window::MainWindow);
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
  /// Represents a built-in Qt signal `QMainWindow::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct WindowIconTextChanged<'a>(&'a ::main_window::MainWindow);
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
  /// Represents a built-in Qt signal `QMainWindow::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::main_window::MainWindow);
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
  /// Represents a built-in Qt signal `QMainWindow::windowIconChanged`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct WindowIconChanged<'a>(&'a ::main_window::MainWindow);
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
  /// Represents a built-in Qt signal `QMainWindow::toolButtonStyleChanged`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.signals().tool_button_style_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct ToolButtonStyleChanged<'a>(&'a ::main_window::MainWindow);
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
    /// Returns an object representing a built-in Qt signal `QMainWindow::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMainWindow::tabifiedDockWidgetActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tabified_dock_widget_activated(&self) -> TabifiedDockWidgetActivated {
      TabifiedDockWidgetActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMainWindow::iconSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn icon_size_changed(&self) -> IconSizeChanged {
      IconSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMainWindow::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMainWindow::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMainWindow::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMainWindow::toolButtonStyleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tool_button_style_changed(&self) -> ToolButtonStyleChanged {
      ToolButtonStyleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MainWindow`.
  pub struct Slots<'a>(&'a ::main_window::MainWindow);
  /// Represents a built-in Qt slot `QMainWindow::setUnifiedTitleAndToolBarOnMac`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_unified_title_and_tool_bar_on_mac()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetUnifiedTitleAndToolBarOnMac<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetUnifiedTitleAndToolBarOnMac<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setUnifiedTitleAndToolBarOnMac(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::showMaximized`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct ShowMaximized<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setEnabled`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetEnabled<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::lower`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Lower<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::close`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Close<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setHidden`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetHidden<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::raise`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Raise<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::updateMicroFocus`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct UpdateMicroFocus<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::hide`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Hide<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setDockNestingEnabled`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_dock_nesting_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetDockNestingEnabled<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetDockNestingEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDockNestingEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::update`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Update<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setDisabled`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetDisabled<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setFocus`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetFocus<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setWindowModified`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetWindowModified<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::showNormal`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct ShowNormal<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setWindowTitle`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetWindowTitle<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::showMinimized`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct ShowMinimized<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setVisible`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetVisible<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setAnimated`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_animated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetAnimated<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetAnimated<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAnimated(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::setStyleSheet`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct SetStyleSheet<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::show`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Show<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::showFullScreen`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct ShowFullScreen<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QMainWindow::repaint`.
  ///
  /// An object of this type can be created from `MainWindow` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MainWindow` object.
  pub struct Repaint<'a>(&'a ::main_window::MainWindow);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMainWindow::setUnifiedTitleAndToolBarOnMac`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_unified_title_and_tool_bar_on_mac(&self) -> SetUnifiedTitleAndToolBarOnMac {
      SetUnifiedTitleAndToolBarOnMac(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setDockNestingEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_dock_nesting_enabled(&self) -> SetDockNestingEnabled {
      SetDockNestingEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setAnimated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_animated(&self) -> SetAnimated {
      SetAnimated(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMainWindow::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
  }
  impl ::main_window::MainWindow {
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

impl ::cpp_utils::DynamicCast<::main_window::MainWindow> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::main_window::MainWindow> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMainWindow_G_dynamic_cast_QMainWindow_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::main_window::MainWindow> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMainWindow_G_dynamic_cast_QMainWindow_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::main_window::MainWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QObject_ptr(self as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QObject_ptr(self as *const ::main_window::MainWindow as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::main_window::MainWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QPaintDevice_ptr(self as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QPaintDevice_ptr(self as *const ::main_window::MainWindow as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::main_window::MainWindow {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QWidget_ptr(self as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QWidget_ptr(self as *const ::main_window::MainWindow as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::main_window::MainWindow> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::main_window::MainWindow {
    let ffi_result =
      ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::main_window::MainWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::main_window::MainWindow> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::main_window::MainWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::main_window::MainWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::main_window::MainWindow> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::main_window::MainWindow {
    let ffi_result =
      ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::main_window::MainWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QMainWindow_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::main_window::MainWindow {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QWidget_ptr(self as *const ::main_window::MainWindow as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::main_window::MainWindow {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMainWindow_G_static_cast_QWidget_ptr(self as *mut ::main_window::MainWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MainWindow::add_dock_widget](../struct.MainWindow.html#method.add_dock_widget) method.
  pub trait MainWindowAddDockWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> ();
  }
  impl<'largs> MainWindowAddDockWidgetArgs<'largs>
    for (&'largs ::qt_core::qt::DockWidgetArea, *mut ::dock_widget::DockWidget) {
    unsafe fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> () {
      let area = self.0;
      let dockwidget = self.1;
      ::ffi::qt_widgets_c_QMainWindow_addDockWidget_area_dockwidget(original_self as *mut ::main_window::MainWindow,
                                                                    area as *const ::qt_core::qt::DockWidgetArea,
                                                                    dockwidget)
    }
  }
  impl<'largs> MainWindowAddDockWidgetArgs<'largs>
    for (&'largs ::qt_core::qt::DockWidgetArea, *mut ::dock_widget::DockWidget, &'largs ::qt_core::qt::Orientation) {
    unsafe fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> () {
      let area = self.0;
      let dockwidget = self.1;
      let orientation = self.2;
      ::ffi::qt_widgets_c_QMainWindow_addDockWidget_area_dockwidget_orientation(original_self as *mut ::main_window::MainWindow, area as *const ::qt_core::qt::DockWidgetArea, dockwidget, orientation as *const ::qt_core::qt::Orientation)
    }
  }
  /// This trait represents a set of arguments accepted by [MainWindow::add_tool_bar_break](../struct.MainWindow.html#method.add_tool_bar_break) method.
  pub trait MainWindowAddToolBarBreakArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> ();
  }
  impl<'largs> MainWindowAddToolBarBreakArgs<'largs> for &'largs ::qt_core::qt::ToolBarArea {
    fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> () {
      let area = self;
      unsafe {
        ::ffi::qt_widgets_c_QMainWindow_addToolBarBreak_area(original_self as *mut ::main_window::MainWindow,
                                                             area as *const ::qt_core::qt::ToolBarArea)
      }
    }
  }
  impl<'largs> MainWindowAddToolBarBreakArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QMainWindow_addToolBarBreak_no_args(original_self as *mut ::main_window::MainWindow)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MainWindow::add_tool_bar_unsafe](../struct.MainWindow.html#method.add_tool_bar_unsafe) method.
  pub trait MainWindowAddToolBarUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> ();
  }
  impl<'largs> MainWindowAddToolBarUnsafeArgs<'largs> for (&'largs ::qt_core::qt::ToolBarArea, *mut ::tool_bar::ToolBar) {
    unsafe fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> () {
      let area = self.0;
      let toolbar = self.1;
      ::ffi::qt_widgets_c_QMainWindow_addToolBar_area_toolbar(original_self as *mut ::main_window::MainWindow,
                                                              area as *const ::qt_core::qt::ToolBarArea,
                                                              toolbar)
    }
  }
  impl<'largs> MainWindowAddToolBarUnsafeArgs<'largs> for *mut ::tool_bar::ToolBar {
    unsafe fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> () {
      let toolbar = self;
      ::ffi::qt_widgets_c_QMainWindow_addToolBar_toolbar(original_self as *mut ::main_window::MainWindow, toolbar)
    }
  }
  /// This trait represents a set of arguments accepted by [MainWindow::restore_state](../struct.MainWindow.html#method.restore_state) method.
  pub trait MainWindowRestoreStateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> bool;
  }
  impl<'largs> MainWindowRestoreStateArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> bool {
      let state = self;
      unsafe {
        ::ffi::qt_widgets_c_QMainWindow_restoreState_state(original_self as *mut ::main_window::MainWindow,
                                                           state as *const ::qt_core::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> MainWindowRestoreStateArgs<'largs> for (&'largs ::qt_core::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::main_window::MainWindow) -> bool {
      let state = self.0;
      let version = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMainWindow_restoreState_state_version(original_self as *mut ::main_window::MainWindow,
                                                                   state as *const ::qt_core::byte_array::ByteArray,
                                                                   version)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MainWindow::save_state](../struct.MainWindow.html#method.save_state) method.
  pub trait MainWindowSaveStateArgs<'largs> {
    fn exec(self, original_self: &'largs ::main_window::MainWindow) -> ::qt_core::byte_array::ByteArray;
  }
  impl<'largs> MainWindowSaveStateArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::main_window::MainWindow) -> ::qt_core::byte_array::ByteArray {

      {
        let mut object: ::qt_core::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMainWindow_saveState_to_output_no_args(original_self as *const ::main_window::MainWindow, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MainWindowSaveStateArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::main_window::MainWindow) -> ::qt_core::byte_array::ByteArray {
      let version = self;
      {
        let mut object: ::qt_core::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMainWindow_saveState_to_output_version(original_self as *const ::main_window::MainWindow, version, &mut object);
        }
        object
      }
    }
  }
}
