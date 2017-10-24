/// C++ type: <span style='color: green;'>```QMdiArea::AreaOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AreaOption {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```DontMaximizeSubWindowOnActivation = 1```</span>
  DontMaximizeSubWindowOnActivation = 1,
}

/// C++ type: <span style='color: green;'>```QMdiArea```</span>
#[repr(C)]
pub struct MdiArea(u8);

impl MdiArea {
  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::activateNextSubWindow()```</span>
  ///
  ///
  pub fn activate_next_sub_window(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_activateNextSubWindow(self as *mut ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::activatePreviousSubWindow()```</span>
  ///
  ///
  pub fn activate_previous_sub_window(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_activatePreviousSubWindow(self as *mut ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```QMdiArea::WindowOrder QMdiArea::activationOrder() const```</span>
  ///
  ///
  pub fn activation_order(&self) -> ::mdi_area::WindowOrder {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_activationOrder(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QMdiArea::activeSubWindow() const```</span>
  ///
  ///
  pub fn active_sub_window(&self) -> *mut ::mdi_sub_window::MdiSubWindow {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_activeSubWindow(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```QBrush QMdiArea::background() const```</span>
  ///
  ///
  pub fn background(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMdiArea_background_to_output(self as *const ::mdi_area::MdiArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::cascadeSubWindows()```</span>
  ///
  ///
  pub fn cascade_sub_windows(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_cascadeSubWindows(self as *mut ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::closeActiveSubWindow()```</span>
  ///
  ///
  pub fn close_active_sub_window(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_closeActiveSubWindow(self as *mut ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::closeAllSubWindows()```</span>
  ///
  ///
  pub fn close_all_sub_windows(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_closeAllSubWindows(self as *mut ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QMdiArea::currentSubWindow() const```</span>
  ///
  ///
  pub fn current_sub_window(&self) -> *mut ::mdi_sub_window::MdiSubWindow {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_currentSubWindow(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```bool QMdiArea::documentMode() const```</span>
  ///
  ///
  pub fn document_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_documentMode(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMdiArea::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_metaObject(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QMdiArea::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMdiArea_minimumSizeHint_to_output(self as *const ::mdi_area::MdiArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMdiArea::QMdiArea()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::mdi_area::MdiArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMdiArea::QMdiArea(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::mdi_area::MdiArea> {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QMdiArea::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMdiArea_qt_metacall(self as *mut ::mdi_area::MdiArea,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMdiArea::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMdiArea_qt_metacast(self as *mut ::mdi_area::MdiArea, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::removeSubWindow(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn remove_sub_window(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QMdiArea_removeSubWindow(self as *mut ::mdi_area::MdiArea, widget)
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setActivationOrder(QMdiArea::WindowOrder order)```</span>
  ///
  ///
  pub fn set_activation_order(&mut self, order: ::mdi_area::WindowOrder) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_setActivationOrder(self as *mut ::mdi_area::MdiArea, order) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::setActiveSubWindow(QMdiSubWindow* window)```</span>
  ///
  ///
  pub unsafe fn set_active_sub_window(&mut self, window: *mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QMdiArea_setActiveSubWindow(self as *mut ::mdi_area::MdiArea, window)
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setBackground(const QBrush& background)```</span>
  ///
  ///
  pub fn set_background(&mut self, background: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QMdiArea_setBackground(self as *mut ::mdi_area::MdiArea,
                                                 background as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setDocumentMode(bool enabled)```</span>
  ///
  ///
  pub fn set_document_mode(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_setDocumentMode(self as *mut ::mdi_area::MdiArea, enabled) }
  }

  /// C++ method: <span style='color: green;'>```QMdiArea::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::mdi_area::AreaOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMdiArea::setOption(QMdiArea::AreaOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::mdi_area::AreaOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMdiArea::setOption(QMdiArea::AreaOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MdiAreaSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QMdiArea::setTabPosition(QTabWidget::TabPosition position)```</span>
  ///
  ///
  pub fn set_tab_position(&mut self, position: &::tab_widget::TabPosition) {
    unsafe {
      ::ffi::qt_widgets_c_QMdiArea_setTabPosition(self as *mut ::mdi_area::MdiArea,
                                                  position as *const ::tab_widget::TabPosition)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setTabShape(QTabWidget::TabShape shape)```</span>
  ///
  ///
  pub fn set_tab_shape(&mut self, shape: &::tab_widget::TabShape) {
    unsafe {
      ::ffi::qt_widgets_c_QMdiArea_setTabShape(self as *mut ::mdi_area::MdiArea,
                                               shape as *const ::tab_widget::TabShape)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setTabsClosable(bool closable)```</span>
  ///
  ///
  pub fn set_tabs_closable(&mut self, closable: bool) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_setTabsClosable(self as *mut ::mdi_area::MdiArea, closable) }
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setTabsMovable(bool movable)```</span>
  ///
  ///
  pub fn set_tabs_movable(&mut self, movable: bool) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_setTabsMovable(self as *mut ::mdi_area::MdiArea, movable) }
  }

  /// C++ method: <span style='color: green;'>```void QMdiArea::setViewMode(QMdiArea::ViewMode mode)```</span>
  ///
  ///
  pub fn set_view_mode(&mut self, mode: ::mdi_area::ViewMode) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_setViewMode(self as *mut ::mdi_area::MdiArea, mode) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QMdiArea::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMdiArea_sizeHint_to_output(self as *const ::mdi_area::MdiArea, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMdiArea::subWindowList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sub_window_list(&self, ()) -> ::list::ListMdiSubWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*> QMdiArea::subWindowList() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sub_window_list(&self, ::mdi_area::WindowOrder) -> ::list::ListMdiSubWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*> QMdiArea::subWindowList(QMdiArea::WindowOrder order = ?) const```</span>
  ///
  ///
  pub fn sub_window_list<'largs, Args>(&'largs self, args: Args) -> ::list::ListMdiSubWindowMutPtr
    where Args: overloading::MdiAreaSubWindowListArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMdiArea::tabsClosable() const```</span>
  ///
  ///
  pub fn tabs_closable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_tabsClosable(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```bool QMdiArea::tabsMovable() const```</span>
  ///
  ///
  pub fn tabs_movable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_tabsMovable(self as *const ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```bool QMdiArea::testOption(QMdiArea::AreaOption opton) const```</span>
  ///
  ///
  pub fn test_option(&self, opton: ::mdi_area::AreaOption) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_testOption(self as *const ::mdi_area::MdiArea, opton) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiArea::tileSubWindows()```</span>
  ///
  ///
  pub fn tile_sub_windows(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_tileSubWindows(self as *mut ::mdi_area::MdiArea) }
  }

  /// C++ method: <span style='color: green;'>```static QString QMdiArea::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMdiArea_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMdiArea::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMdiArea_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMdiArea::ViewMode QMdiArea::viewMode() const```</span>
  ///
  ///
  pub fn view_mode(&self) -> ::mdi_area::ViewMode {
    unsafe { ::ffi::qt_widgets_c_QMdiArea_viewMode(self as *const ::mdi_area::MdiArea) }
  }
}

impl ::cpp_utils::CppDeletable for ::mdi_area::MdiArea {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMdiArea_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MdiArea`.
  pub struct Signals<'a>(&'a ::mdi_area::MdiArea);
  /// Represents a built-in Qt signal `QMdiArea::subWindowActivated`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.signals().sub_window_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct SubWindowActivated<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for SubWindowActivated<'a> {
    type Arguments = (*mut ::mdi_sub_window::MdiSubWindow,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2subWindowActivated(QMdiSubWindow*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SubWindowActivated<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QMdiArea::subWindowActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sub_window_activated(&self) -> SubWindowActivated {
      SubWindowActivated(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MdiArea`.
  pub struct Slots<'a>(&'a ::mdi_area::MdiArea);
  /// Represents a built-in Qt slot `QMdiArea::closeAllSubWindows`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().close_all_sub_windows()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct CloseAllSubWindows<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for CloseAllSubWindows<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeAllSubWindows()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::activateNextSubWindow`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().activate_next_sub_window()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct ActivateNextSubWindow<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for ActivateNextSubWindow<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1activateNextSubWindow()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::cascadeSubWindows`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().cascade_sub_windows()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct CascadeSubWindows<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for CascadeSubWindows<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1cascadeSubWindows()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::setupViewport`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().setup_viewport()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct SetupViewport<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for SetupViewport<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setupViewport(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::activatePreviousSubWindow`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().activate_previous_sub_window()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct ActivatePreviousSubWindow<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for ActivatePreviousSubWindow<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1activatePreviousSubWindow()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::setActiveSubWindow`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().set_active_sub_window()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct SetActiveSubWindow<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for SetActiveSubWindow<'a> {
    type Arguments = (*mut ::mdi_sub_window::MdiSubWindow,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setActiveSubWindow(QMdiSubWindow*)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::closeActiveSubWindow`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().close_active_sub_window()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct CloseActiveSubWindow<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for CloseActiveSubWindow<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1closeActiveSubWindow()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiArea::tileSubWindows`.
  ///
  /// An object of this type can be created from `MdiArea` with `object.slots().tile_sub_windows()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiArea` object.
  pub struct TileSubWindows<'a>(&'a ::mdi_area::MdiArea);
  impl<'a> ::qt_core::connection::Receiver for TileSubWindows<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1tileSubWindows()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMdiArea::closeAllSubWindows`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_all_sub_windows(&self) -> CloseAllSubWindows {
      CloseAllSubWindows(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::activateNextSubWindow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activate_next_sub_window(&self) -> ActivateNextSubWindow {
      ActivateNextSubWindow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::cascadeSubWindows`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cascade_sub_windows(&self) -> CascadeSubWindows {
      CascadeSubWindows(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::setupViewport`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn setup_viewport(&self) -> SetupViewport {
      SetupViewport(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::activatePreviousSubWindow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activate_previous_sub_window(&self) -> ActivatePreviousSubWindow {
      ActivatePreviousSubWindow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::setActiveSubWindow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_active_sub_window(&self) -> SetActiveSubWindow {
      SetActiveSubWindow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::closeActiveSubWindow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close_active_sub_window(&self) -> CloseActiveSubWindow {
      CloseActiveSubWindow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiArea::tileSubWindows`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tile_sub_windows(&self) -> TileSubWindows {
      TileSubWindows(self.0)
    }
  }
  impl ::mdi_area::MdiArea {
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

/// C++ type: <span style='color: green;'>```QMdiArea::ViewMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewMode {
  /// C++ enum variant: <span style='color: green;'>```SubWindowView = 0```</span>
  SubWindow = 0,
  /// C++ enum variant: <span style='color: green;'>```TabbedView = 1```</span>
  Tabbed = 1,
}

/// C++ type: <span style='color: green;'>```QMdiArea::WindowOrder```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WindowOrder {
  /// C++ enum variant: <span style='color: green;'>```CreationOrder = 0```</span>
  Creation = 0,
  /// C++ enum variant: <span style='color: green;'>```StackingOrder = 1```</span>
  Stacking = 1,
  /// C++ enum variant: <span style='color: green;'>```ActivationHistoryOrder = 2```</span>
  ActivationHistory = 2,
}

impl ::cpp_utils::DynamicCast<::mdi_area::MdiArea> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mdi_area::MdiArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mdi_area::MdiArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::mdi_area::MdiArea> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mdi_area::MdiArea> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mdi_area::MdiArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::mdi_area::MdiArea> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mdi_area::MdiArea> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mdi_area::MdiArea> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_dynamic_cast_QMdiArea_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mdi_area::MdiArea {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QObject_ptr(self as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QObject_ptr(self as *const ::mdi_area::MdiArea as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::mdi_area::MdiArea {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QPaintDevice_ptr(self as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QPaintDevice_ptr(self as *const ::mdi_area::MdiArea as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::mdi_area::MdiArea {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QAbstractScrollArea_ptr(self as *const ::mdi_area::MdiArea as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::mdi_area::MdiArea {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QFrame_ptr(self as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QFrame_ptr(self as *const ::mdi_area::MdiArea as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::mdi_area::MdiArea {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QWidget_ptr(self as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QWidget_ptr(self as *const ::mdi_area::MdiArea as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_area::MdiArea> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_area::MdiArea> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_area::MdiArea> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_area::MdiArea {
    let ffi_result =
      ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_area::MdiArea> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_area::MdiArea> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_area::MdiArea {
    let ffi_result = ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QMdiArea_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mdi_area::MdiArea {
  type Target = ::abstract_scroll_area::AbstractScrollArea;
  fn deref(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QAbstractScrollArea_ptr(self as *const ::mdi_area::MdiArea as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mdi_area::MdiArea {
  fn deref_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiArea_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::mdi_area::MdiArea) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MdiArea::set_option](../struct.MdiArea.html#method.set_option) method.
  pub trait MdiAreaSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::mdi_area::MdiArea) -> ();
  }
  impl<'largs> MdiAreaSetOptionArgs<'largs> for ::mdi_area::AreaOption {
    fn exec(self, original_self: &'largs mut ::mdi_area::MdiArea) -> () {
      let option = self;
      unsafe { ::ffi::qt_widgets_c_QMdiArea_setOption_option(original_self as *mut ::mdi_area::MdiArea, option) }
    }
  }
  impl<'largs> MdiAreaSetOptionArgs<'largs> for (::mdi_area::AreaOption, bool) {
    fn exec(self, original_self: &'largs mut ::mdi_area::MdiArea) -> () {
      let option = self.0;
      let on = self.1;
      unsafe { ::ffi::qt_widgets_c_QMdiArea_setOption_option_on(original_self as *mut ::mdi_area::MdiArea, option, on) }
    }
  }
  /// This trait represents a set of arguments accepted by [MdiArea::sub_window_list](../struct.MdiArea.html#method.sub_window_list) method.
  pub trait MdiAreaSubWindowListArgs<'largs> {
    fn exec(self, original_self: &'largs ::mdi_area::MdiArea) -> ::list::ListMdiSubWindowMutPtr;
  }
  impl<'largs> MdiAreaSubWindowListArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::mdi_area::MdiArea) -> ::list::ListMdiSubWindowMutPtr {

      {
        let mut object: ::list::ListMdiSubWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMdiArea_subWindowList_to_output_no_args(original_self as *const ::mdi_area::MdiArea,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MdiAreaSubWindowListArgs<'largs> for ::mdi_area::WindowOrder {
    fn exec(self, original_self: &'largs ::mdi_area::MdiArea) -> ::list::ListMdiSubWindowMutPtr {
      let order = self;
      {
        let mut object: ::list::ListMdiSubWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QMdiArea_subWindowList_to_output_order(original_self as *const ::mdi_area::MdiArea,
                                                                     order,
                                                                     &mut object);
        }
        object
      }
    }
  }
}
