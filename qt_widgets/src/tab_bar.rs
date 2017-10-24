/// C++ type: <span style='color: green;'>```QTabBar::ButtonPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ButtonPosition {
  /// C++ enum variant: <span style='color: green;'>```LeftSide = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```RightSide = 1```</span>
  Right = 1,
}

/// C++ type: <span style='color: green;'>```QTabBar::SelectionBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectionBehavior {
  /// C++ enum variant: <span style='color: green;'>```SelectLeftTab = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```SelectRightTab = 1```</span>
  Right = 1,
  /// C++ enum variant: <span style='color: green;'>```SelectPreviousTab = 2```</span>
  Previous = 2,
}

/// C++ type: <span style='color: green;'>```QTabBar::Shape```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Shape {
  /// C++ enum variant: <span style='color: green;'>```RoundedNorth = 0```</span>
  RoundedNorth = 0,
  /// C++ enum variant: <span style='color: green;'>```RoundedSouth = 1```</span>
  RoundedSouth = 1,
  /// C++ enum variant: <span style='color: green;'>```RoundedWest = 2```</span>
  RoundedWest = 2,
  /// C++ enum variant: <span style='color: green;'>```RoundedEast = 3```</span>
  RoundedEast = 3,
  /// C++ enum variant: <span style='color: green;'>```TriangularNorth = 4```</span>
  TriangularNorth = 4,
  /// C++ enum variant: <span style='color: green;'>```TriangularSouth = 5```</span>
  TriangularSouth = 5,
  /// C++ enum variant: <span style='color: green;'>```TriangularWest = 6```</span>
  TriangularWest = 6,
  /// C++ enum variant: <span style='color: green;'>```TriangularEast = 7```</span>
  TriangularEast = 7,
}

/// C++ type: <span style='color: green;'>```QTabBar```</span>
#[repr(C)]
pub struct TabBar(u8);

impl TabBar {
  /// C++ method: <span style='color: green;'>```QString QTabBar::accessibleTabName(int index) const```</span>
  ///
  ///
  pub fn accessible_tab_name(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_accessibleTabName_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTabBar::addTab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_tab(&mut self, (&::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabBar::addTab(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_tab(&mut self, &::qt_core::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabBar::addTab(const QString& text)```</span>
  ///
  ///
  pub fn add_tab<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::TabBarAddTabArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTabBar::autoHide() const```</span>
  ///
  ///
  pub fn auto_hide(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_autoHide(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::changeCurrentOnDrag() const```</span>
  ///
  ///
  pub fn change_current_on_drag(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_changeCurrentOnDrag(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```int QTabBar::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTabBar_count(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```int QTabBar::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTabBar_currentIndex(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::documentMode() const```</span>
  ///
  ///
  pub fn document_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_documentMode(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::drawBase() const```</span>
  ///
  ///
  pub fn draw_base(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_drawBase(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::expanding() const```</span>
  ///
  ///
  pub fn expanding(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_expanding(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```QSize QTabBar::iconSize() const```</span>
  ///
  ///
  pub fn icon_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_iconSize_to_output(self as *const ::tab_bar::TabBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTabBar::insertTab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_tab(&mut self, (::libc::c_int, &::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabBar::insertTab(int index, const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_tab(&mut self, (::libc::c_int, &::qt_core::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTabBar::insertTab(int index, const QString& text)```</span>
  ///
  ///
  pub fn insert_tab<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::TabBarInsertTabArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTabBar::isMovable() const```</span>
  ///
  ///
  pub fn is_movable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_isMovable(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::isTabEnabled(int index) const```</span>
  ///
  ///
  pub fn is_tab_enabled(&self, index: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_isTabEnabled(self as *const ::tab_bar::TabBar, index) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTabBar::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTabBar_metaObject(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QTabBar::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_minimumSizeHint_to_output(self as *const ::tab_bar::TabBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::moveTab(int from, int to)```</span>
  ///
  ///
  pub fn move_tab(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_moveTab(self as *mut ::tab_bar::TabBar, from, to) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTabBar::QTabBar()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::tab_bar::TabBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTabBar::QTabBar(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::tab_bar::TabBar> {
    let ffi_result = ::ffi::qt_widgets_c_QTabBar_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTabBar::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTabBar_qt_metacall(self as *mut ::tab_bar::TabBar,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTabBar::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTabBar_qt_metacast(self as *mut ::tab_bar::TabBar, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::removeTab(int index)```</span>
  ///
  ///
  pub fn remove_tab(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_removeTab(self as *mut ::tab_bar::TabBar, index) }
  }

  /// C++ method: <span style='color: green;'>```QTabBar::SelectionBehavior QTabBar::selectionBehaviorOnRemove() const```</span>
  ///
  ///
  pub fn selection_behavior_on_remove(&self) -> ::tab_bar::SelectionBehavior {
    unsafe { ::ffi::qt_widgets_c_QTabBar_selectionBehaviorOnRemove(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setAccessibleTabName(int index, const QString& name)```</span>
  ///
  ///
  pub fn set_accessible_tab_name(&mut self, index: ::libc::c_int, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setAccessibleTabName(self as *mut ::tab_bar::TabBar,
                                                       index,
                                                       name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setAutoHide(bool hide)```</span>
  ///
  ///
  pub fn set_auto_hide(&mut self, hide: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setAutoHide(self as *mut ::tab_bar::TabBar, hide) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setChangeCurrentOnDrag(bool change)```</span>
  ///
  ///
  pub fn set_change_current_on_drag(&mut self, change: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setChangeCurrentOnDrag(self as *mut ::tab_bar::TabBar, change) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTabBar::setCurrentIndex(int index)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setCurrentIndex(self as *mut ::tab_bar::TabBar, index) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setDocumentMode(bool set)```</span>
  ///
  ///
  pub fn set_document_mode(&mut self, set: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setDocumentMode(self as *mut ::tab_bar::TabBar, set) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setDrawBase(bool drawTheBase)```</span>
  ///
  ///
  pub fn set_draw_base(&mut self, draw_the_base: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setDrawBase(self as *mut ::tab_bar::TabBar, draw_the_base) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setElideMode(Qt::TextElideMode arg1)```</span>
  ///
  ///
  pub fn set_elide_mode(&mut self, arg1: &::qt_core::qt::TextElideMode) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setElideMode(self as *mut ::tab_bar::TabBar,
                                               arg1 as *const ::qt_core::qt::TextElideMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setExpanding(bool enabled)```</span>
  ///
  ///
  pub fn set_expanding(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setExpanding(self as *mut ::tab_bar::TabBar, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setIconSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setIconSize(self as *mut ::tab_bar::TabBar,
                                              size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setMovable(bool movable)```</span>
  ///
  ///
  pub fn set_movable(&mut self, movable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setMovable(self as *mut ::tab_bar::TabBar, movable) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setSelectionBehaviorOnRemove(QTabBar::SelectionBehavior behavior)```</span>
  ///
  ///
  pub fn set_selection_behavior_on_remove(&mut self, behavior: ::tab_bar::SelectionBehavior) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setSelectionBehaviorOnRemove(self as *mut ::tab_bar::TabBar, behavior) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setShape(QTabBar::Shape shape)```</span>
  ///
  ///
  pub fn set_shape(&mut self, shape: ::tab_bar::Shape) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setShape(self as *mut ::tab_bar::TabBar, shape) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabButton(int index, QTabBar::ButtonPosition position, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_tab_button(&mut self,
                               index: ::libc::c_int,
                               position: ::tab_bar::ButtonPosition,
                               widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QTabBar_setTabButton(self as *mut ::tab_bar::TabBar, index, position, widget)
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabData(int index, const QVariant& data)```</span>
  ///
  ///
  pub fn set_tab_data(&mut self, index: ::libc::c_int, data: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setTabData(self as *mut ::tab_bar::TabBar,
                                             index,
                                             data as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabEnabled(int index, bool arg2)```</span>
  ///
  ///
  pub fn set_tab_enabled(&mut self, index: ::libc::c_int, arg2: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setTabEnabled(self as *mut ::tab_bar::TabBar, index, arg2) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabIcon(int index, const QIcon& icon)```</span>
  ///
  ///
  pub fn set_tab_icon(&mut self, index: ::libc::c_int, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setTabIcon(self as *mut ::tab_bar::TabBar,
                                             index,
                                             icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabText(int index, const QString& text)```</span>
  ///
  ///
  pub fn set_tab_text(&mut self, index: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setTabText(self as *mut ::tab_bar::TabBar,
                                             index,
                                             text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabTextColor(int index, const QColor& color)```</span>
  ///
  ///
  pub fn set_tab_text_color(&mut self, index: ::libc::c_int, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setTabTextColor(self as *mut ::tab_bar::TabBar,
                                                  index,
                                                  color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabToolTip(int index, const QString& tip)```</span>
  ///
  ///
  pub fn set_tab_tool_tip(&mut self, index: ::libc::c_int, tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setTabToolTip(self as *mut ::tab_bar::TabBar,
                                                index,
                                                tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabWhatsThis(int index, const QString& text)```</span>
  ///
  ///
  pub fn set_tab_whats_this(&mut self, index: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_setTabWhatsThis(self as *mut ::tab_bar::TabBar,
                                                  index,
                                                  text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setTabsClosable(bool closable)```</span>
  ///
  ///
  pub fn set_tabs_closable(&mut self, closable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setTabsClosable(self as *mut ::tab_bar::TabBar, closable) }
  }

  /// C++ method: <span style='color: green;'>```void QTabBar::setUsesScrollButtons(bool useButtons)```</span>
  ///
  ///
  pub fn set_uses_scroll_buttons(&mut self, use_buttons: bool) {
    unsafe { ::ffi::qt_widgets_c_QTabBar_setUsesScrollButtons(self as *mut ::tab_bar::TabBar, use_buttons) }
  }

  /// C++ method: <span style='color: green;'>```QTabBar::Shape QTabBar::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::tab_bar::Shape {
    unsafe { ::ffi::qt_widgets_c_QTabBar_shape(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QTabBar::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_sizeHint_to_output(self as *const ::tab_bar::TabBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTabBar::tabAt(const QPoint& pos) const```</span>
  ///
  ///
  pub fn tab_at(&self, pos: &::qt_core::point::Point) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QTabBar_tabAt(self as *const ::tab_bar::TabBar,
                                        pos as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QTabBar::tabButton(int index, QTabBar::ButtonPosition position) const```</span>
  ///
  ///
  pub fn tab_button(&self, index: ::libc::c_int, position: ::tab_bar::ButtonPosition) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QTabBar_tabButton(self as *const ::tab_bar::TabBar, index, position) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QTabBar::tabData(int index) const```</span>
  ///
  ///
  pub fn tab_data(&self, index: ::libc::c_int) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabData_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon QTabBar::tabIcon(int index) const```</span>
  ///
  ///
  pub fn tab_icon(&self, index: ::libc::c_int) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabIcon_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QTabBar::tabRect(int index) const```</span>
  ///
  ///
  pub fn tab_rect(&self, index: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabRect_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTabBar::tabText(int index) const```</span>
  ///
  ///
  pub fn tab_text(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabText_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QTabBar::tabTextColor(int index) const```</span>
  ///
  ///
  pub fn tab_text_color(&self, index: ::libc::c_int) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabTextColor_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTabBar::tabToolTip(int index) const```</span>
  ///
  ///
  pub fn tab_tool_tip(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabToolTip_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTabBar::tabWhatsThis(int index) const```</span>
  ///
  ///
  pub fn tab_whats_this(&self, index: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_tabWhatsThis_to_output(self as *const ::tab_bar::TabBar, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::tabsClosable() const```</span>
  ///
  ///
  pub fn tabs_closable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_tabsClosable(self as *const ::tab_bar::TabBar) }
  }

  /// C++ method: <span style='color: green;'>```static QString QTabBar::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTabBar_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTabBar::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTabBar_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTabBar::usesScrollButtons() const```</span>
  ///
  ///
  pub fn uses_scroll_buttons(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTabBar_usesScrollButtons(self as *const ::tab_bar::TabBar) }
  }
}

impl ::cpp_utils::CppDeletable for ::tab_bar::TabBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTabBar_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TabBar`.
  pub struct Signals<'a>(&'a ::tab_bar::TabBar);
  /// Represents a built-in Qt signal `QTabBar::tabMoved`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().tab_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct TabMoved<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for TabMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabMoved(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabMoved<'a> {}
  /// Represents a built-in Qt signal `QTabBar::tabBarDoubleClicked`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().tab_bar_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct TabBarDoubleClicked<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::windowTitleChanged`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct WindowTitleChanged<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::currentChanged`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct CurrentChanged<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::windowIconChanged`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct WindowIconChanged<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct WindowIconTextChanged<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::tabCloseRequested`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().tab_close_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct TabCloseRequested<'a>(&'a ::tab_bar::TabBar);
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
  /// Represents a built-in Qt signal `QTabBar::tabBarClicked`.
  ///
  /// An object of this type can be created from `TabBar` with `object.signals().tab_bar_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct TabBarClicked<'a>(&'a ::tab_bar::TabBar);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTabBar::tabMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_moved(&self) -> TabMoved {
      TabMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::tabBarDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_bar_double_clicked(&self) -> TabBarDoubleClicked {
      TabBarDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::tabCloseRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_close_requested(&self) -> TabCloseRequested {
      TabCloseRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTabBar::tabBarClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_bar_clicked(&self) -> TabBarClicked {
      TabBarClicked(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TabBar`.
  pub struct Slots<'a>(&'a ::tab_bar::TabBar);
  /// Represents a built-in Qt slot `QTabBar::showMaximized`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct ShowMaximized<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::hide`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Hide<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::showFullScreen`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct ShowFullScreen<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setVisible`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetVisible<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setEnabled`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetEnabled<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setCurrentIndex`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetCurrentIndex<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setWindowTitle`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetWindowTitle<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setHidden`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetHidden<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::updateMicroFocus`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct UpdateMicroFocus<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::showNormal`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct ShowNormal<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setDisabled`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetDisabled<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setWindowModified`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetWindowModified<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::close`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Close<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::lower`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Lower<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::repaint`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Repaint<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::show`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Show<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::raise`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Raise<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::showMinimized`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct ShowMinimized<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setStyleSheet`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetStyleSheet<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::update`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct Update<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QTabBar::setFocus`.
  ///
  /// An object of this type can be created from `TabBar` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TabBar` object.
  pub struct SetFocus<'a>(&'a ::tab_bar::TabBar);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTabBar::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTabBar::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
  }
  impl ::tab_bar::TabBar {
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

impl ::cpp_utils::DynamicCast<::tab_bar::TabBar> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tab_bar::TabBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_dynamic_cast_QTabBar_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tab_bar::TabBar> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_G_dynamic_cast_QTabBar_ptr(self as *const ::widget::Widget as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tab_bar::TabBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QObject_ptr(self as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QObject_ptr(self as *const ::tab_bar::TabBar as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tab_bar::TabBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QPaintDevice_ptr(self as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QPaintDevice_ptr(self as *const ::tab_bar::TabBar as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tab_bar::TabBar {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QWidget_ptr(self as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QWidget_ptr(self as *const ::tab_bar::TabBar as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tab_bar::TabBar> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tab_bar::TabBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tab_bar::TabBar {
    let ffi_result = ::ffi::qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tab_bar::TabBar> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tab_bar::TabBar {
    let ffi_result = ::ffi::qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tab_bar::TabBar {
    let ffi_result = ::ffi::qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tab_bar::TabBar> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tab_bar::TabBar {
    let ffi_result = ::ffi::qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tab_bar::TabBar {
    let ffi_result = ::ffi::qt_widgets_c_QTabBar_G_static_cast_QTabBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tab_bar::TabBar {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QWidget_ptr(self as *const ::tab_bar::TabBar as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tab_bar::TabBar {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTabBar_G_static_cast_QWidget_ptr(self as *mut ::tab_bar::TabBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TabBar::add_tab](../struct.TabBar.html#method.add_tab) method.
  pub trait TabBarAddTabArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::tab_bar::TabBar) -> ::libc::c_int;
  }
  impl<'largs> TabBarAddTabArgs<'largs> for (&'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::tab_bar::TabBar) -> ::libc::c_int {
      let icon = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_addTab_icon_text(original_self as *mut ::tab_bar::TabBar,
                                                     icon as *const ::qt_gui::icon::Icon,
                                                     text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> TabBarAddTabArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::tab_bar::TabBar) -> ::libc::c_int {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_addTab_text(original_self as *mut ::tab_bar::TabBar,
                                                text as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TabBar::insert_tab](../struct.TabBar.html#method.insert_tab) method.
  pub trait TabBarInsertTabArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::tab_bar::TabBar) -> ::libc::c_int;
  }
  impl<'largs> TabBarInsertTabArgs<'largs>
    for (::libc::c_int, &'largs ::qt_gui::icon::Icon, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::tab_bar::TabBar) -> ::libc::c_int {
      let index = self.0;
      let icon = self.1;
      let text = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_insertTab_index_icon_text(original_self as *mut ::tab_bar::TabBar,
                                                              index,
                                                              icon as *const ::qt_gui::icon::Icon,
                                                              text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> TabBarInsertTabArgs<'largs> for (::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::tab_bar::TabBar) -> ::libc::c_int {
      let index = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTabBar_insertTab_index_text(original_self as *mut ::tab_bar::TabBar,
                                                         index,
                                                         text as *const ::qt_core::string::String)
      }
    }
  }
}
