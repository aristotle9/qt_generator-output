/// C++ type: <span style='color: green;'>```QTabWidget::TabPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabPosition {
  /// C++ enum variant: <span style='color: green;'>```North = 0```</span>
  North = 0,
  /// C++ enum variant: <span style='color: green;'>```South = 1```</span>
  South = 1,
  /// C++ enum variant: <span style='color: green;'>```West = 2```</span>
  West = 2,
  /// C++ enum variant: <span style='color: green;'>```East = 3```</span>
  East = 3,
}

/// C++ type: <span style='color: green;'>```QTabWidget::TabShape```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabShape {
  /// C++ enum variant: <span style='color: green;'>```Rounded = 0```</span>
  Rounded = 0,
  /// C++ enum variant: <span style='color: green;'>```Triangular = 1```</span>
  Triangular = 1,
}

/// C++ type: <span style='color: green;'>```QTabWidget```</span>
#[repr(C)]
pub struct TabWidget(u8);

impl TabWidget {
  /// C++ method: <span style='color: green;'>```QTabWidget::addTab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_tab(&mut self, (*mut ::widget::Widget, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabWidget::addTab(QWidget* widget, const QIcon& icon, const QString& label)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_tab(&mut self, (*mut ::widget::Widget, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabWidget::addTab(QWidget* widget, const QString& arg2)```</span>
  ///
  ///
  pub unsafe fn add_tab<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::TabWidgetAddTabArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTabWidget::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_clear(self as *mut ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTabWidget::cornerWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn corner_widget(&self, ()) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QTabWidget::cornerWidget() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn corner_widget(&self, &::qt_core::qt::Corner) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QTabWidget::cornerWidget(Qt::Corner corner = ?) const```</span>
  ///
  ///
  pub fn corner_widget<'largs, Args>(&'largs self, args: Args) -> *mut ::widget::Widget
    where Args: overloading::TabWidgetCornerWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QTabWidget::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_count(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QTabWidget::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_currentIndex(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QTabWidget::currentWidget() const```</span>
  ///
  ///
  pub fn current_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_currentWidget(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabWidget::documentMode() const```</span>
  ///
  ///
  pub fn document_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_documentMode(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QTabWidget::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_hasHeightForWidth(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QTabWidget::heightForWidth(int width) const```</span>
  ///
  ///
  pub fn height_for_width(&self, width: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_heightForWidth(self as *const ::tab_widget::TabWidget, width) }
  }

  /// C++ method: <span style='color: green;'>```QSize QTabWidget::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_iconSize_to_output(self as *const ::tab_widget::TabWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTabWidget::indexOf(QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn index_of(&self, widget: *mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTabWidget_indexOf(self as *const ::tab_widget::TabWidget, widget)
  }

  /// C++ method: <span style='color: green;'>```QTabWidget::insertTab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_tab(&mut self, (::libc::c_int, *mut ::widget::Widget, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabWidget::insertTab(int index, QWidget* widget, const QIcon& icon, const QString& label)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_tab(&mut self, (::libc::c_int, *mut ::widget::Widget, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabWidget::insertTab(int index, QWidget* widget, const QString& arg3)```</span>
  ///
  ///
  pub unsafe fn insert_tab<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::TabWidgetInsertTabArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTabWidget::isMovable() const```</span>
  ///
  ///
  pub fn is_movable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_isMovable(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabWidget::isTabEnabled(int index) const```</span>
  ///
  ///
  pub fn is_tab_enabled(&self, index: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_isTabEnabled(self as *const ::tab_widget::TabWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTabWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_metaObject(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QTabWidget::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_minimumSizeHint_to_output(self as *const ::tab_widget::TabWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTabWidget::QTabWidget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::tab_widget::TabWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabWidget_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTabWidget::QTabWidget(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::tab_widget::TabWidget> {
    let ffi_result = ::ffi::qt_widgets_c_QTabWidget_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTabWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTabWidget_qt_metacall(self as *mut ::tab_widget::TabWidget,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTabWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTabWidget_qt_metacast(self as *mut ::tab_widget::TabWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::removeTab(int index)```</span>
  ///
  ///
  pub fn remove_tab(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_removeTab(self as *mut ::tab_widget::TabWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```QTabWidget::setCornerWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_corner_widget(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTabWidget::setCornerWidget(QWidget* w)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_corner_widget(&mut self, (*mut ::widget::Widget, &::qt_core::qt::Corner)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTabWidget::setCornerWidget(QWidget* w, Qt::Corner corner = ?)```</span>
  ///
  ///
  pub unsafe fn set_corner_widget<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TabWidgetSetCornerWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QTabWidget::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setCurrentIndex(self as *mut ::tab_widget::TabWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTabWidget::setCurrentWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_current_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QTabWidget_setCurrentWidget(self as *mut ::tab_widget::TabWidget, widget)
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setDocumentMode(bool set)```</span>
  ///
  ///
  pub fn set_document_mode(&mut self, set: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setDocumentMode(self as *mut ::tab_widget::TabWidget, set) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setElideMode(Qt::TextElideMode arg1)```</span>
  ///
  ///
  pub fn set_elide_mode(&mut self, arg1: &::qt_core::qt::TextElideMode) {
    unsafe {
      ::ffi::qt_widgets_c_QTabWidget_setElideMode(self as *mut ::tab_widget::TabWidget,
                                                  arg1 as *const ::qt_core::qt::TextElideMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setIconSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QTabWidget_setIconSize(self as *mut ::tab_widget::TabWidget,
                                                 size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setMovable(bool movable)```</span>
  ///
  ///
  pub fn set_movable(&mut self, movable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setMovable(self as *mut ::tab_widget::TabWidget, movable) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabBarAutoHide(bool enabled)```</span>
  ///
  ///
  pub fn set_tab_bar_auto_hide(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setTabBarAutoHide(self as *mut ::tab_widget::TabWidget, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabEnabled(int index, bool arg2)```</span>
  ///
  ///
  pub fn set_tab_enabled(&mut self, index: ::libc::c_int, arg2: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setTabEnabled(self as *mut ::tab_widget::TabWidget, index, arg2) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabIcon(int index, const QIcon& icon)```</span>
  ///
  ///
  pub fn set_tab_icon(&mut self, index: ::libc::c_int, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QTabWidget_setTabIcon(self as *mut ::tab_widget::TabWidget,
                                                index,
                                                icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabPosition(QTabWidget::TabPosition arg1)```</span>
  ///
  ///
  pub fn set_tab_position(&mut self, arg1: ::tab_widget::TabPosition) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setTabPosition(self as *mut ::tab_widget::TabWidget, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabShape(QTabWidget::TabShape s)```</span>
  ///
  ///
  pub fn set_tab_shape(&mut self, s: ::tab_widget::TabShape) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setTabShape(self as *mut ::tab_widget::TabWidget, s) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabText(int index, const QString& arg2)```</span>
  ///
  ///
  pub fn set_tab_text(&mut self, index: ::libc::c_int, arg2: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabWidget_setTabText(self as *mut ::tab_widget::TabWidget,
                                                index,
                                                arg2 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabToolTip(int index, const QString& tip)```</span>
  ///
  ///
  pub fn set_tab_tool_tip(&mut self, index: ::libc::c_int, tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabWidget_setTabToolTip(self as *mut ::tab_widget::TabWidget,
                                                   index,
                                                   tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabWhatsThis(int index, const QString& text)```</span>
  ///
  ///
  pub fn set_tab_whats_this(&mut self, index: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabWidget_setTabWhatsThis(self as *mut ::tab_widget::TabWidget,
                                                     index,
                                                     text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setTabsClosable(bool closeable)```</span>
  ///
  ///
  pub fn set_tabs_closable(&mut self, closeable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setTabsClosable(self as *mut ::tab_widget::TabWidget, closeable) }
  }

  /// C++ method: <span style='color: green;'>```void QTabWidget::setUsesScrollButtons(bool useButtons)```</span>
  ///
  ///
  pub fn set_uses_scroll_buttons(&mut self, use_buttons: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_setUsesScrollButtons(self as *mut ::tab_widget::TabWidget, use_buttons) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QTabWidget::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_sizeHint_to_output(self as *const ::tab_widget::TabWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTabBar* QTabWidget::tabBar() const```</span>
  ///
  ///
  pub fn tab_bar(&self) -> *mut ::tab_bar::TabBar {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_tabBar(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabWidget::tabBarAutoHide() const```</span>
  ///
  ///
  pub fn tab_bar_auto_hide(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_tabBarAutoHide(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QTabWidget::tabIcon(int index) const```</span>
  ///
  ///
  pub fn tab_icon(&self, index: ::libc::c_int) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_tabIcon_to_output(self as *const ::tab_widget::TabWidget, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTabWidget::TabPosition QTabWidget::tabPosition() const```</span>
  ///
  ///
  pub fn tab_position(&self) -> ::tab_widget::TabPosition {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_tabPosition(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTabWidget::TabShape QTabWidget::tabShape() const```</span>
  ///
  ///
  pub fn tab_shape(&self) -> ::tab_widget::TabShape {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_tabShape(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```QString QTabWidget::tabText(int index) const```</span>
  ///
  ///
  pub fn tab_text(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_tabText_to_output(self as *const ::tab_widget::TabWidget, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTabWidget::tabToolTip(int index) const```</span>
  ///
  ///
  pub fn tab_tool_tip(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_tabToolTip_to_output(self as *const ::tab_widget::TabWidget, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTabWidget::tabWhatsThis(int index) const```</span>
  ///
  ///
  pub fn tab_whats_this(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_tabWhatsThis_to_output(self as *const ::tab_widget::TabWidget,
                                                              index,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTabWidget::tabsClosable() const```</span>
  ///
  ///
  pub fn tabs_closable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_tabsClosable(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```static QString QTabWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTabWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTabWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTabWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTabWidget::usesScrollButtons() const```</span>
  ///
  ///
  pub fn uses_scroll_buttons(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_usesScrollButtons(self as *const ::tab_widget::TabWidget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QTabWidget::widget(int index) const```</span>
  ///
  ///
  pub fn widget(&self, index: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QTabWidget_widget(self as *const ::tab_widget::TabWidget, index) }
  }
}

impl ::cpp_utils::CppDeletable for ::tab_widget::TabWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTabWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TabWidget`.
  pub struct Signals<'a>(&'a ::tab_widget::TabWidget);
  /// Represents a built-in Qt signal `QTabWidget::tabBarClicked`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().tab_bar_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct TabBarClicked<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for TabBarClicked<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabBarClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabBarClicked<'a> {}
  /// Represents a built-in Qt signal `QTabWidget::windowIconChanged`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct WindowIconChanged<'a>(&'a ::tab_widget::TabWidget);
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
  /// Represents a built-in Qt signal `QTabWidget::currentChanged`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct CurrentChanged<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentChanged<'a> {}
  /// Represents a built-in Qt signal `QTabWidget::tabCloseRequested`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().tab_close_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct TabCloseRequested<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for TabCloseRequested<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabCloseRequested(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabCloseRequested<'a> {}
  /// Represents a built-in Qt signal `QTabWidget::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct WindowIconTextChanged<'a>(&'a ::tab_widget::TabWidget);
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
  /// Represents a built-in Qt signal `QTabWidget::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::tab_widget::TabWidget);
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
  /// Represents a built-in Qt signal `QTabWidget::tabBarDoubleClicked`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().tab_bar_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct TabBarDoubleClicked<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for TabBarDoubleClicked<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabBarDoubleClicked(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabBarDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QTabWidget::windowTitleChanged`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct WindowTitleChanged<'a>(&'a ::tab_widget::TabWidget);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTabWidget::tabBarClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_bar_clicked(&self) -> TabBarClicked {
      TabBarClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::tabCloseRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_close_requested(&self) -> TabCloseRequested {
      TabCloseRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::tabBarDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_bar_double_clicked(&self) -> TabBarDoubleClicked {
      TabBarDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabWidget::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TabWidget`.
  pub struct Slots<'a>(&'a ::tab_widget::TabWidget);
  /// Represents a built-in Qt slot `QTabWidget::setEnabled`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetEnabled<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::update`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Update<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setWindowTitle`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetWindowTitle<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::showMaximized`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct ShowMaximized<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setStyleSheet`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetStyleSheet<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::showMinimized`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct ShowMinimized<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setHidden`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetHidden<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::hide`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Hide<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::showFullScreen`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct ShowFullScreen<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::repaint`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Repaint<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setWindowModified`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetWindowModified<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setFocus`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetFocus<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::showNormal`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct ShowNormal<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::raise`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Raise<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setDisabled`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetDisabled<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::close`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Close<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setCurrentIndex`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetCurrentIndex<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setCurrentWidget`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_current_widget()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetCurrentWidget<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentWidget<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentWidget(QWidget*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::lower`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Lower<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::setVisible`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct SetVisible<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabWidget::show`.
  ///
  /// An object of this type can be created from `TabWidget` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabWidget` object.
  pub struct Show<'a>(&'a ::tab_widget::TabWidget);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTabWidget::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setCurrentWidget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_widget(&self) -> SetCurrentWidget {
      SetCurrentWidget(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabWidget::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
  }
  impl ::tab_widget::TabWidget {
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

impl ::cpp_utils::DynamicCast<::tab_widget::TabWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tab_widget::TabWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTabWidget_G_dynamic_cast_QTabWidget_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tab_widget::TabWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabWidget_G_dynamic_cast_QTabWidget_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tab_widget::TabWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QObject_ptr(self as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QObject_ptr(self as *const ::tab_widget::TabWidget as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tab_widget::TabWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QPaintDevice_ptr(self as *const ::tab_widget::TabWidget as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tab_widget::TabWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QWidget_ptr(self as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QWidget_ptr(self as *const ::tab_widget::TabWidget as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tab_widget::TabWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tab_widget::TabWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tab_widget::TabWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tab_widget::TabWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tab_widget::TabWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tab_widget::TabWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tab_widget::TabWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tab_widget::TabWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tab_widget::TabWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QTabWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tab_widget::TabWidget {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QWidget_ptr(self as *const ::tab_widget::TabWidget as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tab_widget::TabWidget {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTabWidget_G_static_cast_QWidget_ptr(self as *mut ::tab_widget::TabWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TabWidget::add_tab](../struct.TabWidget.html#method.add_tab) method.
  pub trait TabWidgetAddTabArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ::libc::c_int;
  }
  impl<'largs> TabWidgetAddTabArgs<'largs> for (*mut ::widget::Widget, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ::libc::c_int {
      let widget = self.0;
      let arg2 = self.1;
      ::ffi::qt_widgets_c_QTabWidget_addTab_widget_arg2(original_self as *mut ::tab_widget::TabWidget,
                                                        widget,
                                                        arg2 as *const ::qt_core::string::String)
    }
  }
  impl<'largs> TabWidgetAddTabArgs<'largs>
    for (*mut ::widget::Widget, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ::libc::c_int {
      let widget = self.0;
      let icon = self.1;
      let label = self.2;
      ::ffi::qt_widgets_c_QTabWidget_addTab_widget_icon_label(original_self as *mut ::tab_widget::TabWidget,
                                                              widget,
                                                              icon as *const ::qt_gui::icon::Icon,
                                                              label as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [TabWidget::corner_widget](../struct.TabWidget.html#method.corner_widget) method.
  pub trait TabWidgetCornerWidgetArgs<'largs> {
    fn exec(self, original_self: &'largs ::tab_widget::TabWidget) -> *mut ::widget::Widget;
  }
  impl<'largs> TabWidgetCornerWidgetArgs<'largs> for &'largs ::qt_core::qt::Corner {
    fn exec(self, original_self: &'largs ::tab_widget::TabWidget) -> *mut ::widget::Widget {
      let corner = self;
      unsafe {
        ::ffi::qt_widgets_c_QTabWidget_cornerWidget_corner(original_self as *const ::tab_widget::TabWidget,
                                                           corner as *const ::qt_core::qt::Corner)
      }
    }
  }
  impl<'largs> TabWidgetCornerWidgetArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::tab_widget::TabWidget) -> *mut ::widget::Widget {

      unsafe { ::ffi::qt_widgets_c_QTabWidget_cornerWidget_no_args(original_self as *const ::tab_widget::TabWidget) }
    }
  }
  /// This trait represents a set of arguments accepted by [TabWidget::insert_tab](../struct.TabWidget.html#method.insert_tab) method.
  pub trait TabWidgetInsertTabArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ::libc::c_int;
  }
  impl<'largs> TabWidgetInsertTabArgs<'largs>
    for (::libc::c_int, *mut ::widget::Widget, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      let arg3 = self.2;
      ::ffi::qt_widgets_c_QTabWidget_insertTab_index_widget_arg3(original_self as *mut ::tab_widget::TabWidget,
                                                                 index,
                                                                 widget,
                                                                 arg3 as *const ::qt_core::string::String)
    }
  }
  impl<'largs> TabWidgetInsertTabArgs<'largs>
    for (::libc::c_int, *mut ::widget::Widget, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      let icon = self.2;
      let label = self.3;
      ::ffi::qt_widgets_c_QTabWidget_insertTab_index_widget_icon_label(original_self as *mut ::tab_widget::TabWidget,
                                                                       index,
                                                                       widget,
                                                                       icon as *const ::qt_gui::icon::Icon,
                                                                       label as *const ::qt_core::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [TabWidget::set_corner_widget](../struct.TabWidget.html#method.set_corner_widget) method.
  pub trait TabWidgetSetCornerWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> ();
  }
  impl<'largs> TabWidgetSetCornerWidgetArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> () {
      let w = self;
      ::ffi::qt_widgets_c_QTabWidget_setCornerWidget_w(original_self as *mut ::tab_widget::TabWidget, w)
    }
  }
  impl<'largs> TabWidgetSetCornerWidgetArgs<'largs> for (*mut ::widget::Widget, &'largs ::qt_core::qt::Corner) {
    unsafe fn exec(self, original_self: &'largs mut ::tab_widget::TabWidget) -> () {
      let w = self.0;
      let corner = self.1;
      ::ffi::qt_widgets_c_QTabWidget_setCornerWidget_w_corner(original_self as *mut ::tab_widget::TabWidget,
                                                              w,
                                                              corner as *const ::qt_core::qt::Corner)
    }
  }
}
