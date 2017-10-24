/// C++ type: <span style='color: green;'>```QWidget::RenderFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RenderFlag {
  /// C++ enum variant: <span style='color: green;'>```DrawWindowBackground = 1```</span>
  DrawWindowBackground = 1,
  /// C++ enum variant: <span style='color: green;'>```DrawChildren = 2```</span>
  DrawChildren = 2,
  /// C++ enum variant: <span style='color: green;'>```IgnoreMask = 4```</span>
  IgnoreMask = 4,
}

impl ::qt_core::flags::FlaggableEnum for RenderFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "RenderFlag"
  }
}

/// C++ type: <span style='color: green;'>```QWidget```</span>
#[repr(C)]
pub struct Widget(u8);

impl Widget {
  /// C++ method: <span style='color: green;'>```bool QWidget::acceptDrops() const```</span>
  ///
  ///
  pub fn accept_drops(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_acceptDrops(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::accessibleDescription() const```</span>
  ///
  ///
  pub fn accessible_description(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_accessibleDescription_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::accessibleName() const```</span>
  ///
  ///
  pub fn accessible_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_accessibleName_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*> QWidget::actions() const```</span>
  ///
  ///
  pub fn actions(&self) -> ::list::ListActionMutPtr {
    {
      let mut object: ::list::ListActionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_actions_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::activateWindow()```</span>
  ///
  ///
  pub fn activate_window(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_activateWindow(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::addAction(QAction* action)```</span>
  ///
  ///
  pub unsafe fn add_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QWidget_addAction(self as *mut ::widget::Widget, action)
  }

  /// C++ method: <span style='color: green;'>```void QWidget::addActions(QList<QAction*> actions)```</span>
  ///
  ///
  pub fn add_actions(&mut self, actions: &::list::ListActionMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_addActions(self as *mut ::widget::Widget,
                                             actions as *const ::list::ListActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::adjustSize()```</span>
  ///
  ///
  pub fn adjust_size(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_adjustSize(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::autoFillBackground() const```</span>
  ///
  ///
  pub fn auto_fill_background(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_autoFillBackground(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QBackingStore* QWidget::backingStore() const```</span>
  ///
  ///
  pub fn backing_store(&self) -> *mut ::qt_gui::backing_store::BackingStore {
    unsafe { ::ffi::qt_widgets_c_QWidget_backingStore(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWidget::baseSize() const```</span>
  ///
  ///
  pub fn base_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_baseSize_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::childAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn child_at(&self, &::qt_core::point::Point) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QWidget::childAt(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn child_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QWidget::childAt(int x, int y) const```</span>
  ///
  ///
  pub fn child_at<'largs, Args>(&'largs self, args: Args) -> *mut ::widget::Widget
    where Args: overloading::WidgetChildAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QWidget::childrenRect() const```</span>
  ///
  ///
  pub fn children_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_childrenRect_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QWidget::childrenRegion() const```</span>
  ///
  ///
  pub fn children_region(&self) -> ::cpp_utils::CppBox<::qt_gui::region::Region> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_childrenRegion_as_ptr(self as *const ::widget::Widget) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::clearFocus()```</span>
  ///
  ///
  pub fn clear_focus(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_clearFocus(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::clearMask()```</span>
  ///
  ///
  pub fn clear_mask(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_clearMask(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] bool QWidget::close()```</span>
  ///
  ///
  pub fn close(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_close(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QMargins QWidget::contentsMargins() const```</span>
  ///
  ///
  pub fn contents_margins(&self) -> ::qt_core::margins::Margins {
    {
      let mut object: ::qt_core::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_contentsMargins_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QWidget::contentsRect() const```</span>
  ///
  ///
  pub fn contents_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_contentsRect_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::createWinId()```</span>
  ///
  ///
  pub fn create_win_id(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_createWinId(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QCursor QWidget::cursor() const```</span>
  ///
  ///
  pub fn cursor(&self) -> ::cpp_utils::CppBox<::qt_gui::cursor::Cursor> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_cursor_as_ptr(self as *const ::widget::Widget) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWidget::devType() const```</span>
  ///
  ///
  pub fn dev_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_devType(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```unsigned long long QWidget::effectiveWinId() const```</span>
  ///
  ///
  pub fn effective_win_id(&self) -> ::libc::c_ulonglong {
    unsafe { ::ffi::qt_widgets_c_QWidget_effectiveWinId(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::ensurePolished() const```</span>
  ///
  ///
  pub fn ensure_polished(&self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_ensurePolished(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QWidget::find(unsigned long long arg1)```</span>
  ///
  ///
  pub fn find(arg1: ::libc::c_ulonglong) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_find(arg1) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::focusProxy() const```</span>
  ///
  ///
  pub fn focus_proxy(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_focusProxy(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::focusWidget() const```</span>
  ///
  ///
  pub fn focus_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_focusWidget(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```const QFont& QWidget::font() const```</span>
  ///
  ///
  pub fn font<'l0>(&'l0 self) -> &'l0 ::qt_gui::font::Font {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_font(self as *const ::widget::Widget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontInfo QWidget::fontInfo() const```</span>
  ///
  ///
  pub fn font_info(&self) -> ::qt_gui::font_info::FontInfo {
    {
      let mut object: ::qt_gui::font_info::FontInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_fontInfo_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics QWidget::fontMetrics() const```</span>
  ///
  ///
  pub fn font_metrics(&self) -> ::qt_gui::font_metrics::FontMetrics {
    {
      let mut object: ::qt_gui::font_metrics::FontMetrics =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_fontMetrics_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QWidget::frameGeometry() const```</span>
  ///
  ///
  pub fn frame_geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_frameGeometry_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QWidget::frameSize() const```</span>
  ///
  ///
  pub fn frame_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_frameSize_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QRect& QWidget::geometry() const```</span>
  ///
  ///
  pub fn geometry<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_geometry(self as *const ::widget::Widget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QWidget::getContentsMargins(int* left, int* top, int* right, int* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_contents_margins(&self,
                                     left: *mut ::libc::c_int,
                                     top: *mut ::libc::c_int,
                                     right: *mut ::libc::c_int,
                                     bottom: *mut ::libc::c_int) {
    ::ffi::qt_widgets_c_QWidget_getContentsMargins(self as *const ::widget::Widget, left, top, right, bottom)
  }

  /// C++ method: <span style='color: green;'>```QWidget::grab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab(&mut self, ()) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QWidget::grab()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab(&mut self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QWidget::grab(const QRect& rectangle = ?)```</span>
  ///
  ///
  pub fn grab<'largs, Args>(&'largs mut self, args: Args) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>
    where Args: overloading::WidgetGrabArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::grabKeyboard()```</span>
  ///
  ///
  pub fn grab_keyboard(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_grabKeyboard(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget::grabMouse```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab_mouse(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::grabMouse()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab_mouse(&mut self, &::qt_gui::cursor::Cursor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::grabMouse(const QCursor& arg1)```</span>
  ///
  ///
  pub fn grab_mouse<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetGrabMouseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::grabShortcut```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab_shortcut(&mut self, &::qt_gui::key_sequence::KeySequence) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QWidget::grabShortcut(const QKeySequence& key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab_shortcut(&mut self, (&::qt_gui::key_sequence::KeySequence, &::qt_core::qt::ShortcutContext)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QWidget::grabShortcut(const QKeySequence& key, Qt::ShortcutContext context = ?)```</span>
  ///
  ///
  pub fn grab_shortcut<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::WidgetGrabShortcutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsEffect* QWidget::graphicsEffect() const```</span>
  ///
  ///
  pub fn graphics_effect(&self) -> *mut ::graphics_effect::GraphicsEffect {
    unsafe { ::ffi::qt_widgets_c_QWidget_graphicsEffect(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsProxyWidget* QWidget::graphicsProxyWidget() const```</span>
  ///
  ///
  pub fn graphics_proxy_widget(&self) -> *mut ::graphics_proxy_widget::GraphicsProxyWidget {
    unsafe { ::ffi::qt_widgets_c_QWidget_graphicsProxyWidget(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::hasFocus() const```</span>
  ///
  ///
  pub fn has_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_hasFocus(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QWidget::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_hasHeightForWidth(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::hasMouseTracking() const```</span>
  ///
  ///
  pub fn has_mouse_tracking(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_hasMouseTracking(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::hasTabletTracking() const```</span>
  ///
  ///
  pub fn has_tablet_tracking(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_hasTabletTracking(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_height(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWidget::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_heightForWidth(self as *const ::widget::Widget, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::hide()```</span>
  ///
  ///
  pub fn hide(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_hide(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QWidget::inputMethodQuery(Qt::InputMethodQuery arg1) const```</span>
  ///
  ///
  pub fn input_method_query(&self, arg1: &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_inputMethodQuery_to_output(self as *const ::widget::Widget,
                                                               arg1 as *const ::qt_core::qt::InputMethodQuery,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::insertAction(QAction* before, QAction* action)```</span>
  ///
  ///
  pub unsafe fn insert_action(&mut self, before: *mut ::action::Action, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QWidget_insertAction(self as *mut ::widget::Widget, before, action)
  }

  /// C++ method: <span style='color: green;'>```void QWidget::insertActions(QAction* before, QList<QAction*> actions)```</span>
  ///
  ///
  pub unsafe fn insert_actions(&mut self, before: *mut ::action::Action, actions: &::list::ListActionMutPtr) {
    ::ffi::qt_widgets_c_QWidget_insertActions(self as *mut ::widget::Widget,
                                              before,
                                              actions as *const ::list::ListActionMutPtr)
  }

  /// C++ method: <span style='color: green;'>```unsigned long long QWidget::internalWinId() const```</span>
  ///
  ///
  pub fn internal_win_id(&self) -> ::libc::c_ulonglong {
    unsafe { ::ffi::qt_widgets_c_QWidget_internalWinId(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isActiveWindow() const```</span>
  ///
  ///
  pub fn is_active_window(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isActiveWindow(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isAncestorOf(const QWidget* child) const```</span>
  ///
  ///
  pub unsafe fn is_ancestor_of(&self, child: *const ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QWidget_isAncestorOf(self as *const ::widget::Widget, child)
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isEnabled(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isEnabledTo(const QWidget* arg1) const```</span>
  ///
  ///
  pub unsafe fn is_enabled_to(&self, arg1: *const ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QWidget_isEnabledTo(self as *const ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isEnabledToTLW() const```</span>
  ///
  ///
  pub fn is_enabled_to_t_l_w(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isEnabledToTLW(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isFullScreen() const```</span>
  ///
  ///
  pub fn is_full_screen(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isFullScreen(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isHidden() const```</span>
  ///
  ///
  pub fn is_hidden(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isHidden(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isLeftToRight() const```</span>
  ///
  ///
  pub fn is_left_to_right(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isLeftToRight(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isMaximized() const```</span>
  ///
  ///
  pub fn is_maximized(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isMaximized(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isMinimized() const```</span>
  ///
  ///
  pub fn is_minimized(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isMinimized(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isModal() const```</span>
  ///
  ///
  pub fn is_modal(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isModal(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isRightToLeft() const```</span>
  ///
  ///
  pub fn is_right_to_left(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isRightToLeft(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isTopLevel() const```</span>
  ///
  ///
  pub fn is_top_level(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isTopLevel(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isVisible(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isVisibleTo(const QWidget* arg1) const```</span>
  ///
  ///
  pub unsafe fn is_visible_to(&self, arg1: *const ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QWidget_isVisibleTo(self as *const ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isWindow() const```</span>
  ///
  ///
  pub fn is_window(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isWindow(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::isWindowModified() const```</span>
  ///
  ///
  pub fn is_window_modified(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_isWindowModified(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QWidget::keyboardGrabber()```</span>
  ///
  ///
  pub fn keyboard_grabber() -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_keyboardGrabber() }
  }

  /// C++ method: <span style='color: green;'>```QLayout* QWidget::layout() const```</span>
  ///
  ///
  pub fn layout(&self) -> *mut ::layout::Layout {
    unsafe { ::ffi::qt_widgets_c_QWidget_layout(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QLocale QWidget::locale() const```</span>
  ///
  ///
  pub fn locale(&self) -> ::qt_core::locale::Locale {
    {
      let mut object: ::qt_core::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_locale_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::lower()```</span>
  ///
  ///
  pub fn lower(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_lower(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::mapFrom(const QWidget* arg1, const QPoint& arg2) const```</span>
  ///
  ///
  pub unsafe fn map_from(&self,
                         arg1: *const ::widget::Widget,
                         arg2: &::qt_core::point::Point)
                         -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWidget_mapFrom_to_output(self as *const ::widget::Widget,
                                                    arg1,
                                                    arg2 as *const ::qt_core::point::Point,
                                                    &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::mapFromGlobal(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn map_from_global(&self, arg1: &::qt_core::point::Point) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_mapFromGlobal_to_output(self as *const ::widget::Widget,
                                                            arg1 as *const ::qt_core::point::Point,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::mapFromParent(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn map_from_parent(&self, arg1: &::qt_core::point::Point) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_mapFromParent_to_output(self as *const ::widget::Widget,
                                                            arg1 as *const ::qt_core::point::Point,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::mapTo(const QWidget* arg1, const QPoint& arg2) const```</span>
  ///
  ///
  pub unsafe fn map_to(&self,
                       arg1: *const ::widget::Widget,
                       arg2: &::qt_core::point::Point)
                       -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWidget_mapTo_to_output(self as *const ::widget::Widget,
                                                  arg1,
                                                  arg2 as *const ::qt_core::point::Point,
                                                  &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::mapToGlobal(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn map_to_global(&self, arg1: &::qt_core::point::Point) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_mapToGlobal_to_output(self as *const ::widget::Widget,
                                                          arg1 as *const ::qt_core::point::Point,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::mapToParent(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn map_to_parent(&self, arg1: &::qt_core::point::Point) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_mapToParent_to_output(self as *const ::widget::Widget,
                                                          arg1 as *const ::qt_core::point::Point,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QWidget::mask() const```</span>
  ///
  ///
  pub fn mask(&self) -> ::cpp_utils::CppBox<::qt_gui::region::Region> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_mask_as_ptr(self as *const ::widget::Widget) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::maximumHeight() const```</span>
  ///
  ///
  pub fn maximum_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_maximumHeight(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWidget::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_maximumSize_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::maximumWidth() const```</span>
  ///
  ///
  pub fn maximum_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_maximumWidth(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QWidget_metaObject(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::minimumHeight() const```</span>
  ///
  ///
  pub fn minimum_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_minimumHeight(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWidget::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_minimumSize_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWidget::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_minimumSizeHint_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::minimumWidth() const```</span>
  ///
  ///
  pub fn minimum_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_minimumWidth(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```static QWidget* QWidget::mouseGrabber()```</span>
  ///
  ///
  pub fn mouse_grabber() -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_mouseGrabber() }
  }

  /// C++ method: <span style='color: green;'>```QWidget::move```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::move(const QPoint& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::move(int x, int y)```</span>
  ///
  ///
  pub fn move_<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetMoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget* QWidget::nativeParentWidget() const```</span>
  ///
  ///
  pub fn native_parent_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_nativeParentWidget(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::nextInFocusChain() const```</span>
  ///
  ///
  pub fn next_in_focus_chain(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_nextInFocusChain(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QRect QWidget::normalGeometry() const```</span>
  ///
  ///
  pub fn normal_geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_normalGeometry_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPaintEngine* QWidget::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::qt_gui::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_widgets_c_QWidget_paintEngine(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```const QPalette& QWidget::palette() const```</span>
  ///
  ///
  pub fn palette<'l0>(&'l0 self) -> &'l0 ::qt_gui::palette::Palette {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_palette(self as *const ::widget::Widget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::parentWidget() const```</span>
  ///
  ///
  pub fn parent_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_parentWidget(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWidget::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_pos_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::previousInFocusChain() const```</span>
  ///
  ///
  pub fn previous_in_focus_chain(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_previousInFocusChain(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QWidget_qt_metacall(self as *mut ::widget::Widget,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QWidget_qt_metacast(self as *mut ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::raise()```</span>
  ///
  ///
  pub fn raise(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_raise(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QRect QWidget::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_rect_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::releaseKeyboard()```</span>
  ///
  ///
  pub fn release_keyboard(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_releaseKeyboard(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::releaseMouse()```</span>
  ///
  ///
  pub fn release_mouse(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_releaseMouse(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::releaseShortcut(int id)```</span>
  ///
  ///
  pub fn release_shortcut(&mut self, id: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_releaseShortcut(self as *mut ::widget::Widget, id) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::removeAction(QAction* action)```</span>
  ///
  ///
  pub unsafe fn remove_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QWidget_removeAction(self as *mut ::widget::Widget, action)
  }

  /// C++ method: <span style='color: green;'>```QWidget::render```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn render(&mut self, *mut ::qt_gui::paint_device::PaintDevice) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPaintDevice* target)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::paint_device::PaintDevice, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPaintDevice* target, const QPoint& targetOffset = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::paint_device::PaintDevice, &::qt_core::point::Point, &::qt_gui::region::Region)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPaintDevice* target, const QPoint& targetOffset = ?, const QRegion& sourceRegion = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::paint_device::PaintDevice, &::qt_core::point::Point, &::qt_gui::region::Region, ::qt_core::flags::Flags<::widget::RenderFlag>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPaintDevice* target, const QPoint& targetOffset = ?, const QRegion& sourceRegion = ?, QFlags<QWidget::RenderFlag> renderFlags = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn render(&mut self, *mut ::qt_gui::painter::Painter) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPainter* painter)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPainter* painter, const QPoint& targetOffset = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::point::Point, &::qt_gui::region::Region)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPainter* painter, const QPoint& targetOffset = ?, const QRegion& sourceRegion = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::point::Point, &::qt_gui::region::Region, ::qt_core::flags::Flags<::widget::RenderFlag>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::render(QPainter* painter, const QPoint& targetOffset = ?, const QRegion& sourceRegion = ?, QFlags<QWidget::RenderFlag> renderFlags = ?)```</span>
  ///
  ///
  pub unsafe fn render<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetRenderArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::repaint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn repaint(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QWidget::repaint()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn repaint(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::repaint(const QRect& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn repaint(&mut self, &::qt_gui::region::Region) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::repaint(const QRegion& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn repaint(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::repaint(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn repaint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetRepaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::resize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resize(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::resize(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resize(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::resize(int w, int h)```</span>
  ///
  ///
  pub fn resize<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetResizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QWidget::restoreGeometry(const QByteArray& geometry)```</span>
  ///
  ///
  pub fn restore_geometry(&mut self, geometry: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_restoreGeometry(self as *mut ::widget::Widget,
                                                  geometry as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QWidget::saveGeometry() const```</span>
  ///
  ///
  pub fn save_geometry(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_saveGeometry_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::scroll```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::scroll(int dx, int dy)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::scroll(int dx, int dy, const QRect& arg3)```</span>
  ///
  ///
  pub fn scroll<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetScrollArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setAcceptDrops(bool on)```</span>
  ///
  ///
  pub fn set_accept_drops(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setAcceptDrops(self as *mut ::widget::Widget, on) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setAccessibleDescription(const QString& description)```</span>
  ///
  ///
  pub fn set_accessible_description(&mut self, description: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setAccessibleDescription(self as *mut ::widget::Widget,
                                                           description as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setAccessibleName(const QString& name)```</span>
  ///
  ///
  pub fn set_accessible_name(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setAccessibleName(self as *mut ::widget::Widget,
                                                    name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute(&mut self, &::qt_core::qt::WidgetAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setAttribute(Qt::WidgetAttribute arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute(&mut self, (&::qt_core::qt::WidgetAttribute, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setAttribute(Qt::WidgetAttribute arg1, bool on = ?)```</span>
  ///
  ///
  pub fn set_attribute<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetAttributeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setAutoFillBackground(bool enabled)```</span>
  ///
  ///
  pub fn set_auto_fill_background(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setAutoFillBackground(self as *mut ::widget::Widget, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setBackgroundRole(QPalette::ColorRole arg1)```</span>
  ///
  ///
  pub fn set_background_role(&mut self, arg1: &::qt_gui::palette::ColorRole) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setBackgroundRole(self as *mut ::widget::Widget,
                                                    arg1 as *const ::qt_gui::palette::ColorRole)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setBaseSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_base_size(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setBaseSize(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_base_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setBaseSize(int basew, int baseh)```</span>
  ///
  ///
  pub fn set_base_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetBaseSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::setContentsMargins```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_contents_margins(&mut self, &::qt_core::margins::Margins) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setContentsMargins(const QMargins& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_contents_margins(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setContentsMargins(int left, int top, int right, int bottom)```</span>
  ///
  ///
  pub fn set_contents_margins<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetContentsMarginsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setContextMenuPolicy(Qt::ContextMenuPolicy policy)```</span>
  ///
  ///
  pub fn set_context_menu_policy(&mut self, policy: &::qt_core::qt::ContextMenuPolicy) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setContextMenuPolicy(self as *mut ::widget::Widget,
                                                       policy as *const ::qt_core::qt::ContextMenuPolicy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setCursor(const QCursor& arg1)```</span>
  ///
  ///
  pub fn set_cursor(&mut self, arg1: &::qt_gui::cursor::Cursor) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setCursor(self as *mut ::widget::Widget,
                                            arg1 as *const ::qt_gui::cursor::Cursor)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setDisabled(bool arg1)```</span>
  ///
  ///
  pub fn set_disabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setDisabled(self as *mut ::widget::Widget, arg1) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setEnabled(self as *mut ::widget::Widget, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setFixedHeight(int h)```</span>
  ///
  ///
  pub fn set_fixed_height(&mut self, h: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setFixedHeight(self as *mut ::widget::Widget, h) }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setFixedSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_fixed_size(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setFixedSize(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_fixed_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setFixedSize(int w, int h)```</span>
  ///
  ///
  pub fn set_fixed_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetFixedSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setFixedWidth(int w)```</span>
  ///
  ///
  pub fn set_fixed_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setFixedWidth(self as *mut ::widget::Widget, w) }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setFocus```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_focus(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setFocus()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_focus(&mut self, &::qt_core::qt::FocusReason) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setFocus(Qt::FocusReason reason)```</span>
  ///
  ///
  pub fn set_focus<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetFocusArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setFocusPolicy(Qt::FocusPolicy policy)```</span>
  ///
  ///
  pub fn set_focus_policy(&mut self, policy: &::qt_core::qt::FocusPolicy) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setFocusPolicy(self as *mut ::widget::Widget,
                                                 policy as *const ::qt_core::qt::FocusPolicy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setFocusProxy(QWidget* arg1)```</span>
  ///
  ///
  pub unsafe fn set_focus_proxy(&mut self, arg1: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWidget_setFocusProxy(self as *mut ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setFont(const QFont& arg1)```</span>
  ///
  ///
  pub fn set_font(&mut self, arg1: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setFont(self as *mut ::widget::Widget,
                                          arg1 as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setForegroundRole(QPalette::ColorRole arg1)```</span>
  ///
  ///
  pub fn set_foreground_role(&mut self, arg1: &::qt_gui::palette::ColorRole) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setForegroundRole(self as *mut ::widget::Widget,
                                                    arg1 as *const ::qt_gui::palette::ColorRole)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setGeometry```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_geometry(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_geometry(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setGeometry(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn set_geometry<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetGeometryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setGraphicsEffect(QGraphicsEffect* effect)```</span>
  ///
  ///
  pub unsafe fn set_graphics_effect(&mut self, effect: *mut ::graphics_effect::GraphicsEffect) {
    ::ffi::qt_widgets_c_QWidget_setGraphicsEffect(self as *mut ::widget::Widget, effect)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setHidden(bool hidden)```</span>
  ///
  ///
  pub fn set_hidden(&mut self, hidden: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setHidden(self as *mut ::widget::Widget, hidden) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setLayout(QLayout* arg1)```</span>
  ///
  ///
  pub unsafe fn set_layout(&mut self, arg1: *mut ::layout::Layout) {
    ::ffi::qt_widgets_c_QWidget_setLayout(self as *mut ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setLayoutDirection(Qt::LayoutDirection direction)```</span>
  ///
  ///
  pub fn set_layout_direction(&mut self, direction: &::qt_core::qt::LayoutDirection) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setLayoutDirection(self as *mut ::widget::Widget,
                                                     direction as *const ::qt_core::qt::LayoutDirection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setLocale(const QLocale& locale)```</span>
  ///
  ///
  pub fn set_locale(&mut self, locale: &::qt_core::locale::Locale) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setLocale(self as *mut ::widget::Widget,
                                            locale as *const ::qt_core::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setMask```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_mask(&mut self, &::qt_gui::bitmap::Bitmap) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setMask(const QBitmap& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_mask(&mut self, &::qt_gui::region::Region) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setMask(const QRegion& arg1)```</span>
  ///
  ///
  pub fn set_mask<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetMaskArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setMaximumHeight(int maxh)```</span>
  ///
  ///
  pub fn set_maximum_height(&mut self, maxh: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setMaximumHeight(self as *mut ::widget::Widget, maxh) }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setMaximumSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_maximum_size(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setMaximumSize(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_maximum_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setMaximumSize(int maxw, int maxh)```</span>
  ///
  ///
  pub fn set_maximum_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetMaximumSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setMaximumWidth(int maxw)```</span>
  ///
  ///
  pub fn set_maximum_width(&mut self, maxw: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setMaximumWidth(self as *mut ::widget::Widget, maxw) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setMinimumHeight(int minh)```</span>
  ///
  ///
  pub fn set_minimum_height(&mut self, minh: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setMinimumHeight(self as *mut ::widget::Widget, minh) }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setMinimumSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_minimum_size(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setMinimumSize(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_minimum_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setMinimumSize(int minw, int minh)```</span>
  ///
  ///
  pub fn set_minimum_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetMinimumSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setMinimumWidth(int minw)```</span>
  ///
  ///
  pub fn set_minimum_width(&mut self, minw: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setMinimumWidth(self as *mut ::widget::Widget, minw) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setMouseTracking(bool enable)```</span>
  ///
  ///
  pub fn set_mouse_tracking(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setMouseTracking(self as *mut ::widget::Widget, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setPalette(const QPalette& arg1)```</span>
  ///
  ///
  pub fn set_palette(&mut self, arg1: &::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setPalette(self as *mut ::widget::Widget,
                                             arg1 as *const ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setParent(QWidget* parent)```</span>
  ///
  ///
  pub unsafe fn set_parent(&mut self, parent: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWidget_setParent(self as *mut ::widget::Widget, parent)
  }

  /// C++ method: <span style='color: green;'>```QWidget::setShortcutAutoRepeat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_shortcut_auto_repeat(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setShortcutAutoRepeat(int id)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_shortcut_auto_repeat(&mut self, (::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setShortcutAutoRepeat(int id, bool enable = ?)```</span>
  ///
  ///
  pub fn set_shortcut_auto_repeat<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetShortcutAutoRepeatArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::setShortcutEnabled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_shortcut_enabled(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setShortcutEnabled(int id)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_shortcut_enabled(&mut self, (::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setShortcutEnabled(int id, bool enable = ?)```</span>
  ///
  ///
  pub fn set_shortcut_enabled<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetShortcutEnabledArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::setSizeIncrement```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_size_increment(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setSizeIncrement(const QSize& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_size_increment(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setSizeIncrement(int w, int h)```</span>
  ///
  ///
  pub fn set_size_increment<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetSizeIncrementArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget::setSizePolicy```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_size_policy(&mut self, &::size_policy::SizePolicy) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setSizePolicy(QSizePolicy arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_size_policy(&mut self, (&::size_policy::Policy, &::size_policy::Policy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setSizePolicy(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical)```</span>
  ///
  ///
  pub fn set_size_policy<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetSizePolicyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setStatusTip(const QString& arg1)```</span>
  ///
  ///
  pub fn set_status_tip(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setStatusTip(self as *mut ::widget::Widget,
                                               arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setStyle(QStyle* arg1)```</span>
  ///
  ///
  pub unsafe fn set_style(&mut self, arg1: *mut ::style::Style) {
    ::ffi::qt_widgets_c_QWidget_setStyle(self as *mut ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setStyleSheet(const QString& styleSheet)```</span>
  ///
  ///
  pub fn set_style_sheet(&mut self, style_sheet: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setStyleSheet(self as *mut ::widget::Widget,
                                                style_sheet as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QWidget::setTabOrder(QWidget* arg1, QWidget* arg2)```</span>
  ///
  ///
  pub unsafe fn set_tab_order(arg1: *mut ::widget::Widget, arg2: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWidget_setTabOrder(arg1, arg2)
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setTabletTracking(bool enable)```</span>
  ///
  ///
  pub fn set_tablet_tracking(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setTabletTracking(self as *mut ::widget::Widget, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setToolTip(const QString& arg1)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setToolTip(self as *mut ::widget::Widget,
                                             arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setToolTipDuration(int msec)```</span>
  ///
  ///
  pub fn set_tool_tip_duration(&mut self, msec: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setToolTipDuration(self as *mut ::widget::Widget, msec) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setUpdatesEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_updates_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setUpdatesEnabled(self as *mut ::widget::Widget, enable) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QWidget::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setVisible(self as *mut ::widget::Widget, visible) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setWhatsThis(const QString& arg1)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWhatsThis(self as *mut ::widget::Widget,
                                               arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setWindowFilePath(const QString& filePath)```</span>
  ///
  ///
  pub fn set_window_file_path(&mut self, file_path: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWindowFilePath(self as *mut ::widget::Widget,
                                                    file_path as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget::setWindowFlag```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_window_flag(&mut self, &::qt_core::qt::WindowType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setWindowFlag(Qt::WindowType arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_window_flag(&mut self, (&::qt_core::qt::WindowType, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::setWindowFlag(Qt::WindowType arg1, bool on = ?)```</span>
  ///
  ///
  pub fn set_window_flag<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetSetWindowFlagArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::setWindowIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_window_icon(&mut self, icon: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWindowIcon(self as *mut ::widget::Widget,
                                                icon as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setWindowIconText(const QString& arg1)```</span>
  ///
  ///
  pub fn set_window_icon_text(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWindowIconText(self as *mut ::widget::Widget,
                                                    arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setWindowModality(Qt::WindowModality windowModality)```</span>
  ///
  ///
  pub fn set_window_modality(&mut self, window_modality: &::qt_core::qt::WindowModality) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWindowModality(self as *mut ::widget::Widget,
                                                    window_modality as *const ::qt_core::qt::WindowModality)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setWindowModified(bool arg1)```</span>
  ///
  ///
  pub fn set_window_modified(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setWindowModified(self as *mut ::widget::Widget, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setWindowOpacity(double level)```</span>
  ///
  ///
  pub fn set_window_opacity(&mut self, level: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QWidget_setWindowOpacity(self as *mut ::widget::Widget, level) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::setWindowRole(const QString& arg1)```</span>
  ///
  ///
  pub fn set_window_role(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWindowRole(self as *mut ::widget::Widget,
                                                arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::setWindowTitle(const QString& arg1)```</span>
  ///
  ///
  pub fn set_window_title(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_setWindowTitle(self as *mut ::widget::Widget,
                                                 arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::show()```</span>
  ///
  ///
  pub fn show(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_show(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::showFullScreen()```</span>
  ///
  ///
  pub fn show_full_screen(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_showFullScreen(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::showMaximized()```</span>
  ///
  ///
  pub fn show_maximized(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_showMaximized(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::showMinimized()```</span>
  ///
  ///
  pub fn show_minimized(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_showMinimized(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWidget::showNormal()```</span>
  ///
  ///
  pub fn show_normal(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_showNormal(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWidget::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_size_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWidget::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_sizeHint_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QWidget::sizeIncrement() const```</span>
  ///
  ///
  pub fn size_increment(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_sizeIncrement_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizePolicy QWidget::sizePolicy() const```</span>
  ///
  ///
  pub fn size_policy(&self) -> ::size_policy::SizePolicy {
    {
      let mut object: ::size_policy::SizePolicy =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_sizePolicy_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::stackUnder(QWidget* arg1)```</span>
  ///
  ///
  pub unsafe fn stack_under(&mut self, arg1: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QWidget_stackUnder(self as *mut ::widget::Widget, arg1)
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::statusTip() const```</span>
  ///
  ///
  pub fn status_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_statusTip_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStyle* QWidget::style() const```</span>
  ///
  ///
  pub fn style(&self) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QWidget_style(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::styleSheet() const```</span>
  ///
  ///
  pub fn style_sheet(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_styleSheet_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::testAttribute(Qt::WidgetAttribute arg1) const```</span>
  ///
  ///
  pub fn test_attribute(&self, arg1: &::qt_core::qt::WidgetAttribute) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_testAttribute(self as *const ::widget::Widget,
                                                arg1 as *const ::qt_core::qt::WidgetAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_toolTip_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::toolTipDuration() const```</span>
  ///
  ///
  pub fn tool_tip_duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_toolTipDuration(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::topLevelWidget() const```</span>
  ///
  ///
  pub fn top_level_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_topLevelWidget(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```static QString QWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::underMouse() const```</span>
  ///
  ///
  pub fn under_mouse(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_underMouse(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::ungrabGesture(Qt::GestureType type)```</span>
  ///
  ///
  pub fn ungrab_gesture(&mut self, type_: &::qt_core::qt::GestureType) {
    unsafe {
      ::ffi::qt_widgets_c_QWidget_ungrabGesture(self as *mut ::widget::Widget,
                                                type_ as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::unsetCursor()```</span>
  ///
  ///
  pub fn unset_cursor(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_unsetCursor(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::unsetLayoutDirection()```</span>
  ///
  ///
  pub fn unset_layout_direction(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_unsetLayoutDirection(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```void QWidget::unsetLocale()```</span>
  ///
  ///
  pub fn unset_locale(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_unsetLocale(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget::update```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn update(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QWidget::update()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn update(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::update(const QRect& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn update(&mut self, &::qt_gui::region::Region) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::update(const QRegion& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn update(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWidget::update(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn update<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WidgetUpdateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWidget::updateGeometry()```</span>
  ///
  ///
  pub fn update_geometry(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QWidget_updateGeometry(self as *mut ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```bool QWidget::updatesEnabled() const```</span>
  ///
  ///
  pub fn updates_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidget_updatesEnabled(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QRegion QWidget::visibleRegion() const```</span>
  ///
  ///
  pub fn visible_region(&self) -> ::cpp_utils::CppBox<::qt_gui::region::Region> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_visibleRegion_as_ptr(self as *const ::widget::Widget) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::whatsThis() const```</span>
  ///
  ///
  pub fn whats_this(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_whatsThis_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_width(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```unsigned long long QWidget::winId() const```</span>
  ///
  ///
  pub fn win_id(&self) -> ::libc::c_ulonglong {
    unsafe { ::ffi::qt_widgets_c_QWidget_winId(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QWidget::window() const```</span>
  ///
  ///
  pub fn window(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidget_window(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::windowFilePath() const```</span>
  ///
  ///
  pub fn window_file_path(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_windowFilePath_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QWidget::windowHandle() const```</span>
  ///
  ///
  pub fn window_handle(&self) -> *mut ::qt_gui::window::Window {
    unsafe { ::ffi::qt_widgets_c_QWidget_windowHandle(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QWidget::windowIcon() const```</span>
  ///
  ///
  pub fn window_icon(&self) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_windowIcon_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::windowIconText() const```</span>
  ///
  ///
  pub fn window_icon_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_windowIconText_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QWidget::windowOpacity() const```</span>
  ///
  ///
  pub fn window_opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QWidget_windowOpacity(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::windowRole() const```</span>
  ///
  ///
  pub fn window_role(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_windowRole_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QWidget::windowTitle() const```</span>
  ///
  ///
  pub fn window_title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidget_windowTitle_to_output(self as *const ::widget::Widget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_x(self as *const ::widget::Widget) }
  }

  /// C++ method: <span style='color: green;'>```int QWidget::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidget_y(self as *const ::widget::Widget) }
  }
}

impl ::cpp_utils::CppDeletable for ::widget::Widget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Widget`.
  pub struct Signals<'a>(&'a ::widget::Widget);
  /// Represents a built-in Qt signal `QWidget::windowTitleChanged`.
  ///
  /// An object of this type can be created from `Widget` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct WindowTitleChanged<'a>(&'a ::widget::Widget);
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
  /// Represents a built-in Qt signal `QWidget::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `Widget` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct WindowIconTextChanged<'a>(&'a ::widget::Widget);
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
  /// Represents a built-in Qt signal `QWidget::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `Widget` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::widget::Widget);
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
  /// Represents a built-in Qt signal `QWidget::objectNameChanged`.
  ///
  /// An object of this type can be created from `Widget` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct ObjectNameChanged<'a>(&'a ::widget::Widget);
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
  /// Represents a built-in Qt signal `QWidget::windowIconChanged`.
  ///
  /// An object of this type can be created from `Widget` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct WindowIconChanged<'a>(&'a ::widget::Widget);
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
    /// Returns an object representing a built-in Qt signal `QWidget::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidget::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidget::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidget::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWidget::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Widget`.
  pub struct Slots<'a>(&'a ::widget::Widget);
  /// Represents a built-in Qt slot `QWidget::raise`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Raise<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setWindowTitle`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetWindowTitle<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::showNormal`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct ShowNormal<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setStyleSheet`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetStyleSheet<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::show`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Show<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setHidden`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetHidden<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setVisible`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetVisible<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::showFullScreen`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct ShowFullScreen<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setWindowModified`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetWindowModified<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::hide`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Hide<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setFocus`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetFocus<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::showMaximized`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct ShowMaximized<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::showMinimized`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct ShowMinimized<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::close`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Close<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setDisabled`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetDisabled<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::lower`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Lower<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::update`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Update<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::setEnabled`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct SetEnabled<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::repaint`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct Repaint<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `Widget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Widget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::widget::Widget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QWidget::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
  }
  impl ::widget::Widget {
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

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QWidget* arg2)```</span>
///
///
pub unsafe fn op_shl(arg1: &::qt_core::debug::Debug, arg2: *const ::widget::Widget) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_widgets_c_QWidget_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug, arg2, &mut object);
    object
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::widget::Widget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_G_static_cast_QObject_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QWidget_G_static_cast_QObject_ptr(self as *const ::widget::Widget as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::widget::Widget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidget_G_static_cast_QPaintDevice_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::widget::Widget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      ::ffi::qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = ::ffi::qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::widget::Widget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = ::ffi::qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = ::ffi::qt_widgets_c_QWidget_G_static_cast_QWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Widget::child_at](../struct.Widget.html#method.child_at) method.
  pub trait WidgetChildAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::widget::Widget) -> *mut ::widget::Widget;
  }
  impl<'largs> WidgetChildAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::widget::Widget) -> *mut ::widget::Widget {
      let p = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_childAt_p(original_self as *const ::widget::Widget,
                                              p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> WidgetChildAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::widget::Widget) -> *mut ::widget::Widget {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QWidget_childAt_x_y(original_self as *const ::widget::Widget, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::grab](../struct.Widget.html#method.grab) method.
  pub trait WidgetGrabArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>;
  }
  impl<'largs> WidgetGrabArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {

      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QWidget_grab_as_ptr_no_args(original_self as *mut ::widget::Widget) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> WidgetGrabArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let rectangle = self;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QWidget_grab_as_ptr_rectangle(original_self as *mut ::widget::Widget,
                                                          rectangle as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::grab_mouse](../struct.Widget.html#method.grab_mouse) method.
  pub trait WidgetGrabMouseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetGrabMouseArgs<'largs> for &'largs ::qt_gui::cursor::Cursor {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_grabMouse_arg1(original_self as *mut ::widget::Widget,
                                                   arg1 as *const ::qt_gui::cursor::Cursor)
      }
    }
  }
  impl<'largs> WidgetGrabMouseArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {

      unsafe { ::ffi::qt_widgets_c_QWidget_grabMouse_no_args(original_self as *mut ::widget::Widget) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::grab_shortcut](../struct.Widget.html#method.grab_shortcut) method.
  pub trait WidgetGrabShortcutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ::libc::c_int;
  }
  impl<'largs> WidgetGrabShortcutArgs<'largs> for &'largs ::qt_gui::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ::libc::c_int {
      let key = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_grabShortcut_key(original_self as *mut ::widget::Widget,
                                                     key as *const ::qt_gui::key_sequence::KeySequence)
      }
    }
  }
  impl<'largs> WidgetGrabShortcutArgs<'largs>
    for (&'largs ::qt_gui::key_sequence::KeySequence, &'largs ::qt_core::qt::ShortcutContext) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ::libc::c_int {
      let key = self.0;
      let context = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_grabShortcut_key_context(original_self as *mut ::widget::Widget,
                                                             key as *const ::qt_gui::key_sequence::KeySequence,
                                                             context as *const ::qt_core::qt::ShortcutContext)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::move_](../struct.Widget.html#method.move_) method.
  pub trait WidgetMoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetMoveArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_move_arg1(original_self as *mut ::widget::Widget,
                                              arg1 as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> WidgetMoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QWidget_move_x_y(original_self as *mut ::widget::Widget, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::render](../struct.Widget.html#method.render) method.
  pub trait WidgetRenderArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetRenderArgs<'largs> for *mut ::qt_gui::painter::Painter {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let painter = self;
      ::ffi::qt_widgets_c_QWidget_render_painter(original_self as *mut ::widget::Widget, painter)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs> for (*mut ::qt_gui::painter::Painter, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let painter = self.0;
      let target_offset = self.1;
      ::ffi::qt_widgets_c_QWidget_render_painter_targetOffset(original_self as *mut ::widget::Widget,
                                                              painter,
                                                              target_offset as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, &'largs ::qt_core::point::Point, &'largs ::qt_gui::region::Region) {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let painter = self.0;
      let target_offset = self.1;
      let source_region = self.2;
      ::ffi::qt_widgets_c_QWidget_render_painter_targetOffset_sourceRegion(original_self as *mut ::widget::Widget, painter, target_offset as *const ::qt_core::point::Point, source_region as *const ::qt_gui::region::Region)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                 &'largs ::qt_core::point::Point,
                                                 &'largs ::qt_gui::region::Region,
                                                 ::qt_core::flags::Flags<::widget::RenderFlag>) {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let painter = self.0;
      let target_offset = self.1;
      let source_region = self.2;
      let render_flags = self.3;
      ::ffi::qt_widgets_c_QWidget_render_painter_targetOffset_sourceRegion_renderFlags(original_self as *mut ::widget::Widget, painter, target_offset as *const ::qt_core::point::Point, source_region as *const ::qt_gui::region::Region, render_flags.to_int() as ::libc::c_uint)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs> for *mut ::qt_gui::paint_device::PaintDevice {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let target = self;
      ::ffi::qt_widgets_c_QWidget_render_target(original_self as *mut ::widget::Widget, target)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs> for (*mut ::qt_gui::paint_device::PaintDevice, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let target = self.0;
      let target_offset = self.1;
      ::ffi::qt_widgets_c_QWidget_render_target_targetOffset(original_self as *mut ::widget::Widget,
                                                             target,
                                                             target_offset as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs>
    for (*mut ::qt_gui::paint_device::PaintDevice, &'largs ::qt_core::point::Point, &'largs ::qt_gui::region::Region) {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let target = self.0;
      let target_offset = self.1;
      let source_region = self.2;
      ::ffi::qt_widgets_c_QWidget_render_target_targetOffset_sourceRegion(original_self as *mut ::widget::Widget, target, target_offset as *const ::qt_core::point::Point, source_region as *const ::qt_gui::region::Region)
    }
  }
  impl<'largs> WidgetRenderArgs<'largs>
    for (*mut ::qt_gui::paint_device::PaintDevice,
                                                 &'largs ::qt_core::point::Point,
                                                 &'largs ::qt_gui::region::Region,
                                                 ::qt_core::flags::Flags<::widget::RenderFlag>) {
    unsafe fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let target = self.0;
      let target_offset = self.1;
      let source_region = self.2;
      let render_flags = self.3;
      ::ffi::qt_widgets_c_QWidget_render_target_targetOffset_sourceRegion_renderFlags(original_self as *mut ::widget::Widget, target, target_offset as *const ::qt_core::point::Point, source_region as *const ::qt_gui::region::Region, render_flags.to_int() as ::libc::c_uint)
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::repaint](../struct.Widget.html#method.repaint) method.
  pub trait WidgetRepaintArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetRepaintArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_repaint_QRect(original_self as *mut ::widget::Widget,
                                                  arg1 as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> WidgetRepaintArgs<'largs> for &'largs ::qt_gui::region::Region {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_repaint_QRegion(original_self as *mut ::widget::Widget,
                                                    arg1 as *const ::qt_gui::region::Region)
      }
    }
  }
  impl<'largs> WidgetRepaintArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_widgets_c_QWidget_repaint_int_int_int_int(original_self as *mut ::widget::Widget, x, y, w, h) }
    }
  }
  impl<'largs> WidgetRepaintArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {

      unsafe { ::ffi::qt_widgets_c_QWidget_repaint_no_args(original_self as *mut ::widget::Widget) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::resize](../struct.Widget.html#method.resize) method.
  pub trait WidgetResizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetResizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_resize_arg1(original_self as *mut ::widget::Widget,
                                                arg1 as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WidgetResizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QWidget_resize_w_h(original_self as *mut ::widget::Widget, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::scroll](../struct.Widget.html#method.scroll) method.
  pub trait WidgetScrollArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetScrollArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_widgets_c_QWidget_scroll_dx_dy(original_self as *mut ::widget::Widget, dx, dy) }
    }
  }
  impl<'largs> WidgetScrollArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let dx = self.0;
      let dy = self.1;
      let arg3 = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_scroll_dx_dy_arg3(original_self as *mut ::widget::Widget,
                                                      dx,
                                                      dy,
                                                      arg3 as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_attribute](../struct.Widget.html#method.set_attribute) method.
  pub trait WidgetSetAttributeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetAttributeArgs<'largs> for &'largs ::qt_core::qt::WidgetAttribute {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setAttribute_arg1(original_self as *mut ::widget::Widget,
                                                      arg1 as *const ::qt_core::qt::WidgetAttribute)
      }
    }
  }
  impl<'largs> WidgetSetAttributeArgs<'largs> for (&'largs ::qt_core::qt::WidgetAttribute, bool) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setAttribute_arg1_on(original_self as *mut ::widget::Widget,
                                                         arg1 as *const ::qt_core::qt::WidgetAttribute,
                                                         on)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_base_size](../struct.Widget.html#method.set_base_size) method.
  pub trait WidgetSetBaseSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetBaseSizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setBaseSize_arg1(original_self as *mut ::widget::Widget,
                                                     arg1 as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WidgetSetBaseSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let basew = self.0;
      let baseh = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setBaseSize_basew_baseh(original_self as *mut ::widget::Widget, basew, baseh)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_contents_margins](../struct.Widget.html#method.set_contents_margins) method.
  pub trait WidgetSetContentsMarginsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetContentsMarginsArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let left = self.0;
      let top = self.1;
      let right = self.2;
      let bottom = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setContentsMargins_left_top_right_bottom(original_self as *mut ::widget::Widget,
                                                                             left,
                                                                             top,
                                                                             right,
                                                                             bottom)
      }
    }
  }
  impl<'largs> WidgetSetContentsMarginsArgs<'largs> for &'largs ::qt_core::margins::Margins {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let margins = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setContentsMargins_margins(original_self as *mut ::widget::Widget,
                                                               margins as *const ::qt_core::margins::Margins)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_fixed_size](../struct.Widget.html#method.set_fixed_size) method.
  pub trait WidgetSetFixedSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetFixedSizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setFixedSize_arg1(original_self as *mut ::widget::Widget,
                                                      arg1 as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WidgetSetFixedSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QWidget_setFixedSize_w_h(original_self as *mut ::widget::Widget, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_focus](../struct.Widget.html#method.set_focus) method.
  pub trait WidgetSetFocusArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetFocusArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {

      unsafe { ::ffi::qt_widgets_c_QWidget_setFocus_no_args(original_self as *mut ::widget::Widget) }
    }
  }
  impl<'largs> WidgetSetFocusArgs<'largs> for &'largs ::qt_core::qt::FocusReason {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let reason = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setFocus_reason(original_self as *mut ::widget::Widget,
                                                    reason as *const ::qt_core::qt::FocusReason)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_geometry](../struct.Widget.html#method.set_geometry) method.
  pub trait WidgetSetGeometryArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetGeometryArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setGeometry_arg1(original_self as *mut ::widget::Widget,
                                                     arg1 as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> WidgetSetGeometryArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_widgets_c_QWidget_setGeometry_x_y_w_h(original_self as *mut ::widget::Widget, x, y, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_mask](../struct.Widget.html#method.set_mask) method.
  pub trait WidgetSetMaskArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetMaskArgs<'largs> for &'largs ::qt_gui::bitmap::Bitmap {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setMask_QBitmap(original_self as *mut ::widget::Widget,
                                                    arg1 as *const ::qt_gui::bitmap::Bitmap)
      }
    }
  }
  impl<'largs> WidgetSetMaskArgs<'largs> for &'largs ::qt_gui::region::Region {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setMask_QRegion(original_self as *mut ::widget::Widget,
                                                    arg1 as *const ::qt_gui::region::Region)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_maximum_size](../struct.Widget.html#method.set_maximum_size) method.
  pub trait WidgetSetMaximumSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetMaximumSizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setMaximumSize_arg1(original_self as *mut ::widget::Widget,
                                                        arg1 as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WidgetSetMaximumSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let maxw = self.0;
      let maxh = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setMaximumSize_maxw_maxh(original_self as *mut ::widget::Widget, maxw, maxh)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_minimum_size](../struct.Widget.html#method.set_minimum_size) method.
  pub trait WidgetSetMinimumSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetMinimumSizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setMinimumSize_arg1(original_self as *mut ::widget::Widget,
                                                        arg1 as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WidgetSetMinimumSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let minw = self.0;
      let minh = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setMinimumSize_minw_minh(original_self as *mut ::widget::Widget, minw, minh)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_shortcut_auto_repeat](../struct.Widget.html#method.set_shortcut_auto_repeat) method.
  pub trait WidgetSetShortcutAutoRepeatArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetShortcutAutoRepeatArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let id = self;
      unsafe { ::ffi::qt_widgets_c_QWidget_setShortcutAutoRepeat_id(original_self as *mut ::widget::Widget, id) }
    }
  }
  impl<'largs> WidgetSetShortcutAutoRepeatArgs<'largs> for (::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let id = self.0;
      let enable = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setShortcutAutoRepeat_id_enable(original_self as *mut ::widget::Widget, id, enable)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_shortcut_enabled](../struct.Widget.html#method.set_shortcut_enabled) method.
  pub trait WidgetSetShortcutEnabledArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetShortcutEnabledArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let id = self;
      unsafe { ::ffi::qt_widgets_c_QWidget_setShortcutEnabled_id(original_self as *mut ::widget::Widget, id) }
    }
  }
  impl<'largs> WidgetSetShortcutEnabledArgs<'largs> for (::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let id = self.0;
      let enable = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setShortcutEnabled_id_enable(original_self as *mut ::widget::Widget, id, enable)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_size_increment](../struct.Widget.html#method.set_size_increment) method.
  pub trait WidgetSetSizeIncrementArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetSizeIncrementArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setSizeIncrement_arg1(original_self as *mut ::widget::Widget,
                                                          arg1 as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WidgetSetSizeIncrementArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_widgets_c_QWidget_setSizeIncrement_w_h(original_self as *mut ::widget::Widget, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_size_policy](../struct.Widget.html#method.set_size_policy) method.
  pub trait WidgetSetSizePolicyArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetSizePolicyArgs<'largs> for &'largs ::size_policy::SizePolicy {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setSizePolicy_arg1(original_self as *mut ::widget::Widget,
                                                       arg1 as *const ::size_policy::SizePolicy)
      }
    }
  }
  impl<'largs> WidgetSetSizePolicyArgs<'largs> for (&'largs ::size_policy::Policy, &'largs ::size_policy::Policy) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let horizontal = self.0;
      let vertical = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setSizePolicy_horizontal_vertical(original_self as *mut ::widget::Widget,
                                                                      horizontal as *const ::size_policy::Policy,
                                                                      vertical as *const ::size_policy::Policy)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::set_window_flag](../struct.Widget.html#method.set_window_flag) method.
  pub trait WidgetSetWindowFlagArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetSetWindowFlagArgs<'largs> for &'largs ::qt_core::qt::WindowType {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setWindowFlag_arg1(original_self as *mut ::widget::Widget,
                                                       arg1 as *const ::qt_core::qt::WindowType)
      }
    }
  }
  impl<'largs> WidgetSetWindowFlagArgs<'largs> for (&'largs ::qt_core::qt::WindowType, bool) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_setWindowFlag_arg1_on(original_self as *mut ::widget::Widget,
                                                          arg1 as *const ::qt_core::qt::WindowType,
                                                          on)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Widget::update](../struct.Widget.html#method.update) method.
  pub trait WidgetUpdateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> ();
  }
  impl<'largs> WidgetUpdateArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_update_QRect(original_self as *mut ::widget::Widget,
                                                 arg1 as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> WidgetUpdateArgs<'largs> for &'largs ::qt_gui::region::Region {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QWidget_update_QRegion(original_self as *mut ::widget::Widget,
                                                   arg1 as *const ::qt_gui::region::Region)
      }
    }
  }
  impl<'largs> WidgetUpdateArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_widgets_c_QWidget_update_int_int_int_int(original_self as *mut ::widget::Widget, x, y, w, h) }
    }
  }
  impl<'largs> WidgetUpdateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::widget::Widget) -> () {

      unsafe { ::ffi::qt_widgets_c_QWidget_update_no_args(original_self as *mut ::widget::Widget) }
    }
  }
}
