/// C++ type: <span style='color: green;'>```QGraphicsItem::CacheMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CacheMode {
  /// C++ enum variant: <span style='color: green;'>```NoCache = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ItemCoordinateCache = 1```</span>
  ItemCoordinate = 1,
  /// C++ enum variant: <span style='color: green;'>```DeviceCoordinateCache = 2```</span>
  DeviceCoordinate = 2,
}

/// C++ type: <span style='color: green;'>```QGraphicsItem::Extension```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Extension {
  /// C++ enum variant: <span style='color: green;'>```UserExtension = -2147483648```</span>
  UserExtension = -2147483648,
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
}

/// C++ type: <span style='color: green;'>```QGraphicsItem```</span>
#[repr(C)]
pub struct GraphicsItem(u8);

impl GraphicsItem {
  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::acceptDrops() const```</span>
  ///
  ///
  pub fn accept_drops(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_acceptDrops(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::acceptHoverEvents() const```</span>
  ///
  ///
  pub fn accept_hover_events(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_acceptHoverEvents(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::acceptTouchEvents() const```</span>
  ///
  ///
  pub fn accept_touch_events(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_acceptTouchEvents(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsItem::advance(int phase)```</span>
  ///
  ///
  pub fn advance(&mut self, phase: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_advance(self as *mut ::graphics_item::GraphicsItem, phase) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QRectF QGraphicsItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_boundingRect_to_output(self as *const ::graphics_item::GraphicsItem,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QGraphicsItem::boundingRegion(const QTransform& itemToDeviceTransform) const```</span>
  ///
  ///
  pub fn bounding_region(&self,
                         item_to_device_transform: &::qt_gui::transform::Transform)
                         -> ::cpp_utils::CppBox<::qt_gui::region::Region> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItem_boundingRegion_as_ptr(self as *const ::graphics_item::GraphicsItem, item_to_device_transform as *const ::qt_gui::transform::Transform) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::boundingRegionGranularity() const```</span>
  ///
  ///
  pub fn bounding_region_granularity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_boundingRegionGranularity(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::CacheMode QGraphicsItem::cacheMode() const```</span>
  ///
  ///
  pub fn cache_mode(&self) -> ::graphics_item::CacheMode {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_cacheMode(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsItem::childItems() const```</span>
  ///
  ///
  pub fn child_items(&self) -> ::list::ListGraphicsItemMutPtr {
    {
      let mut object: ::list::ListGraphicsItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_childItems_to_output(self as *const ::graphics_item::GraphicsItem,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::childrenBoundingRect() const```</span>
  ///
  ///
  pub fn children_bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_childrenBoundingRect_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::clearFocus()```</span>
  ///
  ///
  pub fn clear_focus(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_clearFocus(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::clipPath() const```</span>
  ///
  ///
  pub fn clip_path(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_clipPath_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::collidesWithItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn collides_with_item(&self, *const ::graphics_item::GraphicsItem) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItem::collidesWithItem(const QGraphicsItem* other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn collides_with_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::qt::ItemSelectionMode)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItem::collidesWithItem(const QGraphicsItem* other, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  pub unsafe fn collides_with_item<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::GraphicsItemCollidesWithItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::collidesWithPath```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn collides_with_path(&self, &::qt_gui::painter_path::PainterPath) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItem::collidesWithPath(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn collides_with_path(&self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItem::collidesWithPath(const QPainterPath& path, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  pub fn collides_with_path<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::GraphicsItemCollidesWithPathArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::collidingItems```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn colliding_items(&self, ()) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsItem::collidingItems() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn colliding_items(&self, &::qt_core::qt::ItemSelectionMode) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsItem::collidingItems(Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  pub fn colliding_items<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsItemMutPtr
    where Args: overloading::GraphicsItemCollidingItemsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::commonAncestorItem(const QGraphicsItem* other) const```</span>
  ///
  ///
  pub unsafe fn common_ancestor_item(&self,
                                     other: *const ::graphics_item::GraphicsItem)
                                     -> *mut ::graphics_item::GraphicsItem {
    ::ffi::qt_widgets_c_QGraphicsItem_commonAncestorItem(self as *const ::graphics_item::GraphicsItem, other)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_contains(self as *const ::graphics_item::GraphicsItem,
                                                 point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QCursor QGraphicsItem::cursor() const```</span>
  ///
  ///
  pub fn cursor(&self) -> ::cpp_utils::CppBox<::qt_gui::cursor::Cursor> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_cursor_as_ptr(self as *const ::graphics_item::GraphicsItem) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QGraphicsItem::data(int key) const```</span>
  ///
  ///
  pub fn data(&self, key: ::libc::c_int) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_data_to_output(self as *const ::graphics_item::GraphicsItem,
                                                         key,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QGraphicsItem::deviceTransform(const QTransform& viewportTransform) const```</span>
  ///
  ///
  pub fn device_transform(&self,
                          viewport_transform: &::qt_gui::transform::Transform)
                          -> ::qt_gui::transform::Transform {
    {
      let mut object: ::qt_gui::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_deviceTransform_to_output(self as *const ::graphics_item::GraphicsItem, viewport_transform as *const ::qt_gui::transform::Transform, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::effectiveOpacity() const```</span>
  ///
  ///
  pub fn effective_opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_effectiveOpacity(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::ensureVisible```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible(const QRectF& rect = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible(const QRectF& rect = ?, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible(const QRectF& rect = ?, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible(double x, double y, double w, double h)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible(double x, double y, double w, double h, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ensureVisible(double x, double y, double w, double h, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  pub fn ensure_visible<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemEnsureVisibleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::filtersChildEvents() const```</span>
  ///
  ///
  pub fn filters_child_events(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_filtersChildEvents(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QGraphicsItem::GraphicsItemFlag> QGraphicsItem::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::qt_core::flags::Flags<::graphics_item::GraphicsItemFlag> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItem_flags(self as *const ::graphics_item::GraphicsItem) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::focusItem() const```</span>
  ///
  ///
  pub fn focus_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_focusItem(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::focusProxy() const```</span>
  ///
  ///
  pub fn focus_proxy(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_focusProxy(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::focusScopeItem() const```</span>
  ///
  ///
  pub fn focus_scope_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_focusScopeItem(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::grabKeyboard()```</span>
  ///
  ///
  pub fn grab_keyboard(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_grabKeyboard(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::grabMouse()```</span>
  ///
  ///
  pub fn grab_mouse(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_grabMouse(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsEffect* QGraphicsItem::graphicsEffect() const```</span>
  ///
  ///
  pub fn graphics_effect(&self) -> *mut ::graphics_effect::GraphicsEffect {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_graphicsEffect(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItemGroup* QGraphicsItem::group() const```</span>
  ///
  ///
  pub fn group(&self) -> *mut ::graphics_item_group::GraphicsItemGroup {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_group(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::handlesChildEvents() const```</span>
  ///
  ///
  pub fn handles_child_events(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_handlesChildEvents(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::hasCursor() const```</span>
  ///
  ///
  pub fn has_cursor(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_hasCursor(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::hasFocus() const```</span>
  ///
  ///
  pub fn has_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_hasFocus(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::hide()```</span>
  ///
  ///
  pub fn hide(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_hide(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::installSceneEventFilter(QGraphicsItem* filterItem)```</span>
  ///
  ///
  pub unsafe fn install_scene_event_filter(&mut self, filter_item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItem_installSceneEventFilter(self as *mut ::graphics_item::GraphicsItem, filter_item)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isActive(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isAncestorOf(const QGraphicsItem* child) const```</span>
  ///
  ///
  pub unsafe fn is_ancestor_of(&self, child: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsItem_isAncestorOf(self as *const ::graphics_item::GraphicsItem, child)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isBlockedByModalPanel() const```</span>
  ///
  ///
  pub fn is_blocked_by_modal_panel(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_isBlockedByModalPanel_no_args(self as *const ::graphics_item::GraphicsItem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem** blockingPanel = ?) const```</span>
  ///
  ///
  pub unsafe fn is_blocked_by_modal_panel_unsafe(&self,
                                                 blocking_panel: *mut *mut ::graphics_item::GraphicsItem)
                                                 -> bool {
    ::ffi::qt_widgets_c_QGraphicsItem_isBlockedByModalPanel_blockingPanel(self as *const ::graphics_item::GraphicsItem, blocking_panel)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isClipped() const```</span>
  ///
  ///
  pub fn is_clipped(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isClipped(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isEnabled(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::isObscured```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_obscured(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isObscured() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_obscured(&self, &::qt_core::rect_f::RectF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isObscured(const QRectF& rect = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn is_obscured(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isObscured(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn is_obscured<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::GraphicsItemIsObscuredArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsItem_isObscuredBy(self as *const ::graphics_item::GraphicsItem, item)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isPanel() const```</span>
  ///
  ///
  pub fn is_panel(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isPanel(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isSelected() const```</span>
  ///
  ///
  pub fn is_selected(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isSelected(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isUnderMouse() const```</span>
  ///
  ///
  pub fn is_under_mouse(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isUnderMouse(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isVisible(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isVisibleTo(const QGraphicsItem* parent) const```</span>
  ///
  ///
  pub unsafe fn is_visible_to(&self, parent: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsItem_isVisibleTo(self as *const ::graphics_item::GraphicsItem, parent)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isWidget() const```</span>
  ///
  ///
  pub fn is_widget(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isWidget(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsItem::isWindow() const```</span>
  ///
  ///
  pub fn is_window(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_isWindow(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::itemTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_transform(&self, *const ::graphics_item::GraphicsItem) -> ::qt_gui::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform QGraphicsItem::itemTransform(const QGraphicsItem* other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_transform(&self, (*const ::graphics_item::GraphicsItem, *mut bool)) -> ::qt_gui::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform QGraphicsItem::itemTransform(const QGraphicsItem* other, bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn item_transform<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::transform::Transform
    where Args: overloading::GraphicsItemItemTransformArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapFromItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_from_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_gui::painter_path::PainterPath)) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem* item, const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_from_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::point_f::PointF)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapFromItem(const QGraphicsItem* item, const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_from_item(&self, (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapFromItem(const QGraphicsItem* item, double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_from_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_gui::polygon_f::PolygonF)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem* item, const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_from_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::rect_f::RectF)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem* item, const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_from_item(&self, (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem* item, double x, double y, double w, double h) const```</span>
  ///
  ///
  pub unsafe fn map_from_item<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsItemMapFromItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapFromParent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_from_parent(&self, &::qt_gui::painter_path::PainterPath) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::mapFromParent(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_from_parent(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapFromParent(const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_from_parent(&self, (::libc::c_double, ::libc::c_double)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapFromParent(double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_from_parent(&self, &::qt_gui::polygon_f::PolygonF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromParent(const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_from_parent(&self, &::qt_core::rect_f::RectF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromParent(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_from_parent(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromParent(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_from_parent<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsItemMapFromParentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapFromScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_gui::painter_path::PainterPath) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::mapFromScene(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapFromScene(const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_from_scene(&self, (::libc::c_double, ::libc::c_double)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapFromScene(double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_gui::polygon_f::PolygonF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromScene(const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_core::rect_f::RectF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromScene(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_from_scene(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapFromScene(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_from_scene<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsItemMapFromSceneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapRectFromItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect_from_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::rect_f::RectF)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem* item, const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect_from_item(&self, (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem* item, double x, double y, double w, double h) const```</span>
  ///
  ///
  pub unsafe fn map_rect_from_item<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::GraphicsItemMapRectFromItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapRectFromParent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect_from_parent(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectFromParent(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect_from_parent(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectFromParent(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_rect_from_parent<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::GraphicsItemMapRectFromParentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapRectFromScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect_from_scene(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectFromScene(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect_from_scene(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectFromScene(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_rect_from_scene<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::GraphicsItemMapRectFromSceneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapRectToItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect_to_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::rect_f::RectF)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem* item, const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect_to_item(&self, (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem* item, double x, double y, double w, double h) const```</span>
  ///
  ///
  pub unsafe fn map_rect_to_item<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::GraphicsItemMapRectToItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapRectToParent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect_to_parent(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectToParent(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect_to_parent(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectToParent(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_rect_to_parent<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::GraphicsItemMapRectToParentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapRectToScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect_to_scene(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectToScene(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect_to_scene(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::mapRectToScene(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_rect_to_scene<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::GraphicsItemMapRectToSceneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapToItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_to_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_gui::painter_path::PainterPath)) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem* item, const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_to_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::point_f::PointF)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapToItem(const QGraphicsItem* item, const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_to_item(&self, (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapToItem(const QGraphicsItem* item, double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_to_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_gui::polygon_f::PolygonF)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem* item, const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_to_item(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::rect_f::RectF)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem* item, const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_to_item(&self, (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem* item, double x, double y, double w, double h) const```</span>
  ///
  ///
  pub unsafe fn map_to_item<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsItemMapToItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapToParent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_to_parent(&self, &::qt_gui::painter_path::PainterPath) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::mapToParent(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_to_parent(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapToParent(const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_to_parent(&self, (::libc::c_double, ::libc::c_double)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapToParent(double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_to_parent(&self, &::qt_gui::polygon_f::PolygonF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToParent(const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_to_parent(&self, &::qt_core::rect_f::RectF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToParent(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_to_parent(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToParent(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_to_parent<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsItemMapToParentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::mapToScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_gui::painter_path::PainterPath) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsItem::mapToScene(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapToScene(const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_to_scene(&self, (::libc::c_double, ::libc::c_double)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::mapToScene(double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_gui::polygon_f::PolygonF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToScene(const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_core::rect_f::RectF) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToScene(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_to_scene(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsItem::mapToScene(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_to_scene<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsItemMapToSceneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix QGraphicsItem::matrix() const```</span>
  ///
  ///
  pub fn matrix(&self) -> ::qt_gui::matrix::Matrix {
    {
      let mut object: ::qt_gui::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_matrix_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::moveBy(double dx, double dy)```</span>
  ///
  ///
  pub fn move_by(&mut self, dx: ::libc::c_double, dy: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_moveBy(self as *mut ::graphics_item::GraphicsItem, dx, dy) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::opacity() const```</span>
  ///
  ///
  pub fn opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_opacity(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_opaqueArea_to_output(self as *const ::graphics_item::GraphicsItem,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QGraphicsItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QGraphicsItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::panel() const```</span>
  ///
  ///
  pub fn panel(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_panel(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::PanelModality QGraphicsItem::panelModality() const```</span>
  ///
  ///
  pub fn panel_modality(&self) -> ::graphics_item::PanelModality {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_panelModality(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::parentItem() const```</span>
  ///
  ///
  pub fn parent_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_parentItem(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsObject* QGraphicsItem::parentObject() const```</span>
  ///
  ///
  pub fn parent_object(&self) -> *mut ::graphics_object::GraphicsObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_parentObject(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QGraphicsItem::parentWidget() const```</span>
  ///
  ///
  pub fn parent_widget(&self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_parentWidget(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_pos_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::removeSceneEventFilter(QGraphicsItem* filterItem)```</span>
  ///
  ///
  pub unsafe fn remove_scene_event_filter(&mut self, filter_item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItem_removeSceneEventFilter(self as *mut ::graphics_item::GraphicsItem, filter_item)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::resetMatrix()```</span>
  ///
  ///
  pub fn reset_matrix(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_resetMatrix(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::resetTransform()```</span>
  ///
  ///
  pub fn reset_transform(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_resetTransform(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::rotation() const```</span>
  ///
  ///
  pub fn rotation(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_rotation(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::scale() const```</span>
  ///
  ///
  pub fn scale(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_scale(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene* QGraphicsItem::scene() const```</span>
  ///
  ///
  pub fn scene(&self) -> *mut ::graphics_scene::GraphicsScene {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_scene(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsItem::sceneBoundingRect() const```</span>
  ///
  ///
  pub fn scene_bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_sceneBoundingRect_to_output(self as *const ::graphics_item::GraphicsItem,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QGraphicsItem::sceneMatrix() const```</span>
  ///
  ///
  pub fn scene_matrix(&self) -> ::qt_gui::matrix::Matrix {
    {
      let mut object: ::qt_gui::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_sceneMatrix_to_output(self as *const ::graphics_item::GraphicsItem,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_scenePos_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QGraphicsItem::sceneTransform() const```</span>
  ///
  ///
  pub fn scene_transform(&self) -> ::qt_gui::transform::Transform {
    {
      let mut object: ::qt_gui::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_sceneTransform_to_output(self as *const ::graphics_item::GraphicsItem,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::scroll```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::scroll(double dx, double dy)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll(&mut self, (::libc::c_double, ::libc::c_double, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::scroll(double dx, double dy, const QRectF& rect = ?)```</span>
  ///
  ///
  pub fn scroll<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemScrollArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setAcceptDrops(bool on)```</span>
  ///
  ///
  pub fn set_accept_drops(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setAcceptDrops(self as *mut ::graphics_item::GraphicsItem, on) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setAcceptHoverEvents(bool enabled)```</span>
  ///
  ///
  pub fn set_accept_hover_events(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setAcceptHoverEvents(self as *mut ::graphics_item::GraphicsItem, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setAcceptTouchEvents(bool enabled)```</span>
  ///
  ///
  pub fn set_accept_touch_events(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setAcceptTouchEvents(self as *mut ::graphics_item::GraphicsItem, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setActive(bool active)```</span>
  ///
  ///
  pub fn set_active(&mut self, active: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setActive(self as *mut ::graphics_item::GraphicsItem, active) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setBoundingRegionGranularity(double granularity)```</span>
  ///
  ///
  pub fn set_bounding_region_granularity(&mut self, granularity: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setBoundingRegionGranularity(self as *mut ::graphics_item::GraphicsItem,
                                                                     granularity)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::setCacheMode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_cache_mode(&mut self, ::graphics_item::CacheMode) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setCacheMode(QGraphicsItem::CacheMode mode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_cache_mode(&mut self, (::graphics_item::CacheMode, &::qt_core::size::Size)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setCacheMode(QGraphicsItem::CacheMode mode, const QSize& cacheSize = ?)```</span>
  ///
  ///
  pub fn set_cache_mode<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetCacheModeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setCursor(const QCursor& cursor)```</span>
  ///
  ///
  pub fn set_cursor(&mut self, cursor: &::qt_gui::cursor::Cursor) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setCursor(self as *mut ::graphics_item::GraphicsItem,
                                                  cursor as *const ::qt_gui::cursor::Cursor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setData(int key, const QVariant& value)```</span>
  ///
  ///
  pub fn set_data(&mut self, key: ::libc::c_int, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setData(self as *mut ::graphics_item::GraphicsItem,
                                                key,
                                                value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setEnabled(self as *mut ::graphics_item::GraphicsItem, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFiltersChildEvents(bool enabled)```</span>
  ///
  ///
  pub fn set_filters_child_events(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setFiltersChildEvents(self as *mut ::graphics_item::GraphicsItem, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::setFlag```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_flag(&mut self, ::graphics_item::GraphicsItemFlag) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFlag(QGraphicsItem::GraphicsItemFlag flag)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_flag(&mut self, (::graphics_item::GraphicsItemFlag, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFlag(QGraphicsItem::GraphicsItemFlag flag, bool enabled = ?)```</span>
  ///
  ///
  pub fn set_flag<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetFlagArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFlags(QFlags<QGraphicsItem::GraphicsItemFlag> flags)```</span>
  ///
  ///
  pub fn set_flags(&mut self, flags: ::qt_core::flags::Flags<::graphics_item::GraphicsItemFlag>) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setFlags(self as *mut ::graphics_item::GraphicsItem,
                                                 flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::setFocus```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_focus(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFocus()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_focus(&mut self, &::qt_core::qt::FocusReason) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFocus(Qt::FocusReason focusReason = ?)```</span>
  ///
  ///
  pub fn set_focus<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetFocusArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setFocusProxy(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn set_focus_proxy(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItem_setFocusProxy(self as *mut ::graphics_item::GraphicsItem, item)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setGraphicsEffect(QGraphicsEffect* effect)```</span>
  ///
  ///
  pub unsafe fn set_graphics_effect(&mut self, effect: *mut ::graphics_effect::GraphicsEffect) {
    ::ffi::qt_widgets_c_QGraphicsItem_setGraphicsEffect(self as *mut ::graphics_item::GraphicsItem, effect)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setGroup(QGraphicsItemGroup* group)```</span>
  ///
  ///
  pub unsafe fn set_group(&mut self, group: *mut ::graphics_item_group::GraphicsItemGroup) {
    ::ffi::qt_widgets_c_QGraphicsItem_setGroup(self as *mut ::graphics_item::GraphicsItem, group)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setHandlesChildEvents(bool enabled)```</span>
  ///
  ///
  pub fn set_handles_child_events(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setHandlesChildEvents(self as *mut ::graphics_item::GraphicsItem, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::setMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_matrix(&mut self, &::qt_gui::matrix::Matrix) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setMatrix(const QMatrix& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_matrix(&mut self, (&::qt_gui::matrix::Matrix, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setMatrix(const QMatrix& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_matrix<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetMatrixArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setOpacity(double opacity)```</span>
  ///
  ///
  pub fn set_opacity(&mut self, opacity: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setOpacity(self as *mut ::graphics_item::GraphicsItem, opacity) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setPanelModality(QGraphicsItem::PanelModality panelModality)```</span>
  ///
  ///
  pub fn set_panel_modality(&mut self, panel_modality: ::graphics_item::PanelModality) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setPanelModality(self as *mut ::graphics_item::GraphicsItem, panel_modality)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setParentItem(QGraphicsItem* parent)```</span>
  ///
  ///
  pub unsafe fn set_parent_item(&mut self, parent: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItem_setParentItem(self as *mut ::graphics_item::GraphicsItem, parent)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::setPos```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pos(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setPos(const QPointF& pos)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pos(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setPos(double x, double y)```</span>
  ///
  ///
  pub fn set_pos<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetPosArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setRotation(double angle)```</span>
  ///
  ///
  pub fn set_rotation(&mut self, angle: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setRotation(self as *mut ::graphics_item::GraphicsItem, angle) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setScale(double scale)```</span>
  ///
  ///
  pub fn set_scale(&mut self, scale: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setScale(self as *mut ::graphics_item::GraphicsItem, scale) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setSelected(bool selected)```</span>
  ///
  ///
  pub fn set_selected(&mut self, selected: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setSelected(self as *mut ::graphics_item::GraphicsItem, selected) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setToolTip(const QString& toolTip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tool_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsItem_setToolTip(self as *mut ::graphics_item::GraphicsItem,
                                                   tool_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::setTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_transform(&mut self, &::qt_gui::transform::Transform) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setTransform(const QTransform& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_transform(&mut self, (&::qt_gui::transform::Transform, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setTransform(const QTransform& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_transform<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetTransformArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem::setTransformOriginPoint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_transform_origin_point(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setTransformOriginPoint(const QPointF& origin)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_transform_origin_point(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setTransformOriginPoint(double ax, double ay)```</span>
  ///
  ///
  pub fn set_transform_origin_point<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemSetTransformOriginPointArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setTransformations(const QList<QGraphicsTransform*>& transformations)```</span>
  ///
  ///
  pub fn set_transformations(&mut self, transformations: &::list::ListGraphicsTransformMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setTransformations(self as *mut ::graphics_item::GraphicsItem, transformations as *const ::list::ListGraphicsTransformMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setVisible(self as *mut ::graphics_item::GraphicsItem, visible) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setX(double x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setX(self as *mut ::graphics_item::GraphicsItem, x) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setY(double y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setY(self as *mut ::graphics_item::GraphicsItem, y) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::setZValue(double z)```</span>
  ///
  ///
  pub fn set_z_value(&mut self, z: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setZValue(self as *mut ::graphics_item::GraphicsItem, z) }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_shape_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::show()```</span>
  ///
  ///
  pub fn show(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_show(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::stackBefore(const QGraphicsItem* sibling)```</span>
  ///
  ///
  pub unsafe fn stack_before(&mut self, sibling: *const ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItem_stackBefore(self as *mut ::graphics_item::GraphicsItem, sibling)
  }

  /// C++ method: <span style='color: green;'>```const QGraphicsObject* QGraphicsItem::toGraphicsObject() const```</span>
  ///
  ///
  pub fn to_graphics_object(&self) -> *const ::graphics_object::GraphicsObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_toGraphicsObject_const(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsObject* QGraphicsItem::toGraphicsObject()```</span>
  ///
  ///
  pub fn to_graphics_object_mut(&mut self) -> *mut ::graphics_object::GraphicsObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_toGraphicsObject(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QGraphicsItem::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_toolTip_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsItem::topLevelItem() const```</span>
  ///
  ///
  pub fn top_level_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_topLevelItem(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QGraphicsItem::topLevelWidget() const```</span>
  ///
  ///
  pub fn top_level_widget(&self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_topLevelWidget(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QTransform QGraphicsItem::transform() const```</span>
  ///
  ///
  pub fn transform(&self) -> ::qt_gui::transform::Transform {
    {
      let mut object: ::qt_gui::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_transform_to_output(self as *const ::graphics_item::GraphicsItem,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsItem::transformOriginPoint() const```</span>
  ///
  ///
  pub fn transform_origin_point(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_transformOriginPoint_to_output(self as *const ::graphics_item::GraphicsItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*> QGraphicsItem::transformations() const```</span>
  ///
  ///
  pub fn transformations(&self) -> ::list::ListGraphicsTransformMutPtr {
    {
      let mut object: ::list::ListGraphicsTransformMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_transformations_to_output(self as *const ::graphics_item::GraphicsItem,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_type(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ungrabKeyboard()```</span>
  ///
  ///
  pub fn ungrab_keyboard(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_ungrabKeyboard(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::ungrabMouse()```</span>
  ///
  ///
  pub fn ungrab_mouse(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_ungrabMouse(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsItem::unsetCursor()```</span>
  ///
  ///
  pub fn unset_cursor(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_unsetCursor(self as *mut ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem::update```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn update(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::update()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn update(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::update(const QRectF& rect = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn update(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsItem::update(double x, double y, double width, double height)```</span>
  ///
  ///
  pub fn update<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemUpdateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QGraphicsItem::window() const```</span>
  ///
  ///
  pub fn window(&self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_window(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_x(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_y(self as *const ::graphics_item::GraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsItem::zValue() const```</span>
  ///
  ///
  pub fn z_value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItem_zValue(self as *const ::graphics_item::GraphicsItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_item::GraphicsItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsItem_delete
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsItem::GraphicsItemChange```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GraphicsItemChange {
  /// C++ enum variant: <span style='color: green;'>```ItemPositionChange = 0```</span>
  PositionChange = 0,
  /// C++ enum variant: <span style='color: green;'>```ItemMatrixChange = 1```</span>
  MatrixChange = 1,
  /// C++ enum variant: <span style='color: green;'>```ItemVisibleChange = 2```</span>
  VisibleChange = 2,
  /// C++ enum variant: <span style='color: green;'>```ItemEnabledChange = 3```</span>
  EnabledChange = 3,
  /// C++ enum variant: <span style='color: green;'>```ItemSelectedChange = 4```</span>
  SelectedChange = 4,
  /// C++ enum variant: <span style='color: green;'>```ItemParentChange = 5```</span>
  ParentChange = 5,
  /// C++ enum variant: <span style='color: green;'>```ItemChildAddedChange = 6```</span>
  ChildAddedChange = 6,
  /// C++ enum variant: <span style='color: green;'>```ItemChildRemovedChange = 7```</span>
  ChildRemovedChange = 7,
  /// C++ enum variant: <span style='color: green;'>```ItemTransformChange = 8```</span>
  TransformChange = 8,
  /// C++ enum variant: <span style='color: green;'>```ItemPositionHasChanged = 9```</span>
  PositionHasChanged = 9,
  /// C++ enum variant: <span style='color: green;'>```ItemTransformHasChanged = 10```</span>
  TransformHasChanged = 10,
  /// C++ enum variant: <span style='color: green;'>```ItemSceneChange = 11```</span>
  SceneChange = 11,
  /// C++ enum variant: <span style='color: green;'>```ItemVisibleHasChanged = 12```</span>
  VisibleHasChanged = 12,
  /// C++ enum variant: <span style='color: green;'>```ItemEnabledHasChanged = 13```</span>
  EnabledHasChanged = 13,
  /// C++ enum variant: <span style='color: green;'>```ItemSelectedHasChanged = 14```</span>
  SelectedHasChanged = 14,
  /// C++ enum variant: <span style='color: green;'>```ItemParentHasChanged = 15```</span>
  ParentHasChanged = 15,
  /// C++ enum variant: <span style='color: green;'>```ItemSceneHasChanged = 16```</span>
  SceneHasChanged = 16,
  /// C++ enum variant: <span style='color: green;'>```ItemCursorChange = 17```</span>
  CursorChange = 17,
  /// C++ enum variant: <span style='color: green;'>```ItemCursorHasChanged = 18```</span>
  CursorHasChanged = 18,
  /// C++ enum variant: <span style='color: green;'>```ItemToolTipChange = 19```</span>
  ToolTipChange = 19,
  /// C++ enum variant: <span style='color: green;'>```ItemToolTipHasChanged = 20```</span>
  ToolTipHasChanged = 20,
  /// C++ enum variant: <span style='color: green;'>```ItemFlagsChange = 21```</span>
  FlagsChange = 21,
  /// C++ enum variant: <span style='color: green;'>```ItemFlagsHaveChanged = 22```</span>
  FlagsHaveChanged = 22,
  /// C++ enum variant: <span style='color: green;'>```ItemZValueChange = 23```</span>
  ZValueChange = 23,
  /// C++ enum variant: <span style='color: green;'>```ItemZValueHasChanged = 24```</span>
  ZValueHasChanged = 24,
  /// C++ enum variant: <span style='color: green;'>```ItemOpacityChange = 25```</span>
  OpacityChange = 25,
  /// C++ enum variant: <span style='color: green;'>```ItemOpacityHasChanged = 26```</span>
  OpacityHasChanged = 26,
  /// C++ enum variant: <span style='color: green;'>```ItemScenePositionHasChanged = 27```</span>
  ScenePositionHasChanged = 27,
  /// C++ enum variant: <span style='color: green;'>```ItemRotationChange = 28```</span>
  RotationChange = 28,
  /// C++ enum variant: <span style='color: green;'>```ItemRotationHasChanged = 29```</span>
  RotationHasChanged = 29,
  /// C++ enum variant: <span style='color: green;'>```ItemScaleChange = 30```</span>
  ScaleChange = 30,
  /// C++ enum variant: <span style='color: green;'>```ItemScaleHasChanged = 31```</span>
  ScaleHasChanged = 31,
  /// C++ enum variant: <span style='color: green;'>```ItemTransformOriginPointChange = 32```</span>
  TransformOriginPointChange = 32,
  /// C++ enum variant: <span style='color: green;'>```ItemTransformOriginPointHasChanged = 33```</span>
  TransformOriginPointHasChanged = 33,
}

/// C++ type: <span style='color: green;'>```QGraphicsItem::GraphicsItemFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GraphicsItemFlag {
  /// C++ enum variant: <span style='color: green;'>```ItemIsMovable = 1```</span>
  IsMovable = 1,
  /// C++ enum variant: <span style='color: green;'>```ItemIsSelectable = 2```</span>
  IsSelectable = 2,
  /// C++ enum variant: <span style='color: green;'>```ItemIsFocusable = 4```</span>
  IsFocusable = 4,
  /// C++ enum variant: <span style='color: green;'>```ItemClipsToShape = 8```</span>
  ClipsToShape = 8,
  /// C++ enum variant: <span style='color: green;'>```ItemClipsChildrenToShape = 16```</span>
  ClipsChildrenToShape = 16,
  /// C++ enum variant: <span style='color: green;'>```ItemIgnoresTransformations = 32```</span>
  IgnoresTransformations = 32,
  /// C++ enum variant: <span style='color: green;'>```ItemIgnoresParentOpacity = 64```</span>
  IgnoresParentOpacity = 64,
  /// C++ enum variant: <span style='color: green;'>```ItemDoesntPropagateOpacityToChildren = 128```</span>
  DoesntPropagateOpacityToChildren = 128,
  /// C++ enum variant: <span style='color: green;'>```ItemStacksBehindParent = 256```</span>
  StacksBehindParent = 256,
  /// C++ enum variant: <span style='color: green;'>```ItemUsesExtendedStyleOption = 512```</span>
  UsesExtendedStyleOption = 512,
  /// C++ enum variant: <span style='color: green;'>```ItemHasNoContents = 1024```</span>
  HasNoContents = 1024,
  /// C++ enum variant: <span style='color: green;'>```ItemSendsGeometryChanges = 2048```</span>
  SendsGeometryChanges = 2048,
  /// C++ enum variant: <span style='color: green;'>```ItemAcceptsInputMethod = 4096```</span>
  AcceptsInputMethod = 4096,
  /// C++ enum variant: <span style='color: green;'>```ItemNegativeZStacksBehindParent = 8192```</span>
  NegativeZStacksBehindParent = 8192,
  /// C++ enum variant: <span style='color: green;'>```ItemIsPanel = 16384```</span>
  IsPanel = 16384,
  /// C++ enum variant: <span style='color: green;'>```ItemIsFocusScope = 32768```</span>
  IsFocusScope = 32768,
  /// C++ enum variant: <span style='color: green;'>```ItemSendsScenePositionChanges = 65536```</span>
  SendsScenePositionChanges = 65536,
  /// C++ enum variant: <span style='color: green;'>```ItemStopsClickFocusPropagation = 131072```</span>
  StopsClickFocusPropagation = 131072,
  /// C++ enum variant: <span style='color: green;'>```ItemStopsFocusHandling = 262144```</span>
  StopsFocusHandling = 262144,
  /// C++ enum variant: <span style='color: green;'>```ItemContainsChildrenInShape = 524288```</span>
  ContainsChildrenInShape = 524288,
}

impl ::qt_core::flags::FlaggableEnum for GraphicsItemFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "GraphicsItemFlag"
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsItem::PanelModality```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PanelModality {
  /// C++ enum variant: <span style='color: green;'>```NonModal = 0```</span>
  Non = 0,
  /// C++ enum variant: <span style='color: green;'>```PanelModal = 1```</span>
  Panel = 1,
  /// C++ enum variant: <span style='color: green;'>```SceneModal = 2```</span>
  Scene = 2,
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::graphics_item::GraphicsItemChange)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsItem::GraphicsItemChange change)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::graphics_item::GraphicsItemFlag)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsItem::GraphicsItemFlag flag)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> ::qt_core::debug::Debug
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl_unsafe((&::qt_core::debug::Debug, *mut ::graphics_item::GraphicsItem)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsItem* item)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl_unsafe((&::qt_core::debug::Debug, *mut ::graphics_object::GraphicsObject)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QGraphicsObject* item)```</span>
///
///
pub unsafe fn op_shl_unsafe<Args>(args: Args) -> ::qt_core::debug::Debug
  where Args: overloading::OpShlUnsafeArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsItem::collides_with_item](../struct.GraphicsItem.html#method.collides_with_item) method.
  pub trait GraphicsItemCollidesWithItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool;
  }
  impl<'largs> GraphicsItemCollidesWithItemArgs<'largs> for *const ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {
      let other = self;
      ::ffi::qt_widgets_c_QGraphicsItem_collidesWithItem_other(original_self as *const ::graphics_item::GraphicsItem,
                                                               other)
    }
  }
  impl<'largs> GraphicsItemCollidesWithItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::qt::ItemSelectionMode) {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {
      let other = self.0;
      let mode = self.1;
      ::ffi::qt_widgets_c_QGraphicsItem_collidesWithItem_other_mode(original_self as *const ::graphics_item::GraphicsItem, other, mode as *const ::qt_core::qt::ItemSelectionMode)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::collides_with_path](../struct.GraphicsItem.html#method.collides_with_path) method.
  pub trait GraphicsItemCollidesWithPathArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool;
  }
  impl<'largs> GraphicsItemCollidesWithPathArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {
      let path = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_collidesWithPath_path(original_self as *const ::graphics_item::GraphicsItem,
                                                                path as *const ::qt_gui::painter_path::PainterPath)
      }
    }
  }
  impl<'largs> GraphicsItemCollidesWithPathArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {
      let path = self.0;
      let mode = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_collidesWithPath_path_mode(original_self as *const ::graphics_item::GraphicsItem, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::colliding_items](../struct.GraphicsItem.html#method.colliding_items) method.
  pub trait GraphicsItemCollidingItemsArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::list::ListGraphicsItemMutPtr;
  }
  impl<'largs> GraphicsItemCollidingItemsArgs<'largs> for &'largs ::qt_core::qt::ItemSelectionMode {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::list::ListGraphicsItemMutPtr {
      let mode = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_collidingItems_to_output_mode(original_self as *const ::graphics_item::GraphicsItem, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemCollidingItemsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::list::ListGraphicsItemMutPtr {

      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_collidingItems_to_output_no_args(original_self as *const ::graphics_item::GraphicsItem, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::ensure_visible](../struct.GraphicsItem.html#method.ensure_visible) method.
  pub trait GraphicsItemEnsureVisibleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_no_args(original_self as *mut ::graphics_item::GraphicsItem)
      }
    }
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_rect(original_self as *mut ::graphics_item::GraphicsItem,
                                                             rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let rect = self.0;
      let xmargin = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_rect_xmargin(original_self as *mut ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, xmargin) }
    }
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let rect = self.0;
      let xmargin = self.1;
      let ymargin = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_rect_xmargin_ymargin(original_self as *mut ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, xmargin, ymargin) }
    }
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_x_y_w_h(original_self as *mut ::graphics_item::GraphicsItem,
                                                                x,
                                                                y,
                                                                w,
                                                                h)
      }
    }
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let xmargin = self.4;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_x_y_w_h_xmargin(original_self as *mut ::graphics_item::GraphicsItem, x, y, w, h, xmargin) }
    }
  }
  impl<'largs> GraphicsItemEnsureVisibleArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let xmargin = self.4;
      let ymargin = self.5;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_ensureVisible_x_y_w_h_xmargin_ymargin(original_self as *mut ::graphics_item::GraphicsItem, x, y, w, h, xmargin, ymargin) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::is_obscured](../struct.GraphicsItem.html#method.is_obscured) method.
  pub trait GraphicsItemIsObscuredArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool;
  }
  impl<'largs> GraphicsItemIsObscuredArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {

      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_isObscured_no_args(original_self as *const ::graphics_item::GraphicsItem)
      }
    }
  }
  impl<'largs> GraphicsItemIsObscuredArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_isObscured_rect(original_self as *const ::graphics_item::GraphicsItem,
                                                          rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsItemIsObscuredArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> bool {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_isObscured_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem,
                                                             x,
                                                             y,
                                                             w,
                                                             h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::item_transform](../struct.GraphicsItem.html#method.item_transform) method.
  pub trait GraphicsItemItemTransformArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::transform::Transform;
  }
  impl<'largs> GraphicsItemItemTransformArgs<'largs> for *const ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::transform::Transform {
      let other = self;
      {
        let mut object: ::qt_gui::transform::Transform =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_itemTransform_to_output_other(original_self as *const ::graphics_item::GraphicsItem, other, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemItemTransformArgs<'largs> for (*const ::graphics_item::GraphicsItem, *mut bool) {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::transform::Transform {
      let other = self.0;
      let ok = self.1;
      {
        let mut object: ::qt_gui::transform::Transform =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_itemTransform_to_output_other_ok(original_self as *const ::graphics_item::GraphicsItem, other, ok, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_from_item](../struct.GraphicsItem.html#method.map_from_item) method.
  pub trait GraphicsItemMapFromItemArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> Self::ReturnType;
  }
  impl<'largs> GraphicsItemMapFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_gui::painter_path::PainterPath) {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::painter_path::PainterPath {
      let item = self.0;
      let path = self.1;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_path(original_self as *const ::graphics_item::GraphicsItem, item, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::point_f::PointF) {
    type ReturnType = ::qt_core::point_f::PointF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let item = self.0;
      let point = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_point(original_self as *const ::graphics_item::GraphicsItem, item, point as *const ::qt_core::point_f::PointF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_gui::polygon_f::PolygonF) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let item = self.0;
      let polygon = self.1;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_polygon(original_self as *const ::graphics_item::GraphicsItem, item, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::rect_f::RectF) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let item = self.0;
      let rect = self.1;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_rect(original_self as *const ::graphics_item::GraphicsItem, item, rect as *const ::qt_core::rect_f::RectF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point_f::PointF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let item = self.0;
      let x = self.1;
      let y = self.2;
      {
        let mut object: ::qt_core::point_f::PointF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_x_y(original_self as *const ::graphics_item::GraphicsItem, item, x, y, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let item = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapFromItem_to_output_item_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, item, x, y, w, h, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_from_parent](../struct.GraphicsItem.html#method.map_from_parent) method.
  pub trait GraphicsItemMapFromParentArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> Self::ReturnType;
  }
  impl<'largs> GraphicsItemMapFromParentArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::painter_path::PainterPath {
      let path = self;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromParent_to_output_path(original_self as *const ::graphics_item::GraphicsItem, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromParentArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let point = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromParent_to_output_point(original_self as *const ::graphics_item::GraphicsItem, point as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromParentArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let polygon = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromParent_to_output_polygon(original_self as *const ::graphics_item::GraphicsItem, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromParentArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let rect = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromParent_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromParentArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromParent_to_output_x_y(original_self as *const ::graphics_item::GraphicsItem, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromParentArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromParent_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_from_scene](../struct.GraphicsItem.html#method.map_from_scene) method.
  pub trait GraphicsItemMapFromSceneArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> Self::ReturnType;
  }
  impl<'largs> GraphicsItemMapFromSceneArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::painter_path::PainterPath {
      let path = self;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromScene_to_output_path(original_self as *const ::graphics_item::GraphicsItem, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromSceneArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let point = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromScene_to_output_point(original_self as *const ::graphics_item::GraphicsItem, point as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromSceneArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let polygon = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromScene_to_output_polygon(original_self as *const ::graphics_item::GraphicsItem, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromSceneArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let rect = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromScene_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromSceneArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromScene_to_output_x_y(original_self as *const ::graphics_item::GraphicsItem, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapFromSceneArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapFromScene_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_rect_from_item](../struct.GraphicsItem.html#method.map_rect_from_item) method.
  pub trait GraphicsItemMapRectFromItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> GraphicsItemMapRectFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let item = self.0;
      let rect = self.1;
      {
        let mut object: ::qt_core::rect_f::RectF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapRectFromItem_to_output_item_rect(original_self as *const ::graphics_item::GraphicsItem, item, rect as *const ::qt_core::rect_f::RectF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapRectFromItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let item = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      {
        let mut object: ::qt_core::rect_f::RectF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapRectFromItem_to_output_item_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, item, x, y, w, h, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_rect_from_parent](../struct.GraphicsItem.html#method.map_rect_from_parent) method.
  pub trait GraphicsItemMapRectFromParentArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> GraphicsItemMapRectFromParentArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let rect = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectFromParent_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapRectFromParentArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectFromParent_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_rect_from_scene](../struct.GraphicsItem.html#method.map_rect_from_scene) method.
  pub trait GraphicsItemMapRectFromSceneArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> GraphicsItemMapRectFromSceneArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let rect = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectFromScene_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapRectFromSceneArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectFromScene_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_rect_to_item](../struct.GraphicsItem.html#method.map_rect_to_item) method.
  pub trait GraphicsItemMapRectToItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> GraphicsItemMapRectToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let item = self.0;
      let rect = self.1;
      {
        let mut object: ::qt_core::rect_f::RectF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapRectToItem_to_output_item_rect(original_self as *const ::graphics_item::GraphicsItem, item, rect as *const ::qt_core::rect_f::RectF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapRectToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let item = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      {
        let mut object: ::qt_core::rect_f::RectF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapRectToItem_to_output_item_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, item, x, y, w, h, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_rect_to_parent](../struct.GraphicsItem.html#method.map_rect_to_parent) method.
  pub trait GraphicsItemMapRectToParentArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> GraphicsItemMapRectToParentArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let rect = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectToParent_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapRectToParentArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectToParent_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_rect_to_scene](../struct.GraphicsItem.html#method.map_rect_to_scene) method.
  pub trait GraphicsItemMapRectToSceneArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> GraphicsItemMapRectToSceneArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let rect = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectToScene_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapRectToSceneArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::rect_f::RectF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapRectToScene_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_to_item](../struct.GraphicsItem.html#method.map_to_item) method.
  pub trait GraphicsItemMapToItemArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> Self::ReturnType;
  }
  impl<'largs> GraphicsItemMapToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_gui::painter_path::PainterPath) {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::painter_path::PainterPath {
      let item = self.0;
      let path = self.1;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_path(original_self as *const ::graphics_item::GraphicsItem, item, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::point_f::PointF) {
    type ReturnType = ::qt_core::point_f::PointF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let item = self.0;
      let point = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_point(original_self as *const ::graphics_item::GraphicsItem, item, point as *const ::qt_core::point_f::PointF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_gui::polygon_f::PolygonF) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let item = self.0;
      let polygon = self.1;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_polygon(original_self as *const ::graphics_item::GraphicsItem, item, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::rect_f::RectF) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let item = self.0;
      let rect = self.1;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_rect(original_self as *const ::graphics_item::GraphicsItem, item, rect as *const ::qt_core::rect_f::RectF, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point_f::PointF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let item = self.0;
      let x = self.1;
      let y = self.2;
      {
        let mut object: ::qt_core::point_f::PointF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_x_y(original_self as *const ::graphics_item::GraphicsItem, item, x, y, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToItemArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    unsafe fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let item = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_mapToItem_to_output_item_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, item, x, y, w, h, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_to_parent](../struct.GraphicsItem.html#method.map_to_parent) method.
  pub trait GraphicsItemMapToParentArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> Self::ReturnType;
  }
  impl<'largs> GraphicsItemMapToParentArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::painter_path::PainterPath {
      let path = self;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToParent_to_output_path(original_self as *const ::graphics_item::GraphicsItem, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToParentArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let point = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToParent_to_output_point(original_self as *const ::graphics_item::GraphicsItem, point as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToParentArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let polygon = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToParent_to_output_polygon(original_self as *const ::graphics_item::GraphicsItem, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToParentArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let rect = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToParent_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToParentArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToParent_to_output_x_y(original_self as *const ::graphics_item::GraphicsItem, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToParentArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToParent_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::map_to_scene](../struct.GraphicsItem.html#method.map_to_scene) method.
  pub trait GraphicsItemMapToSceneArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> Self::ReturnType;
  }
  impl<'largs> GraphicsItemMapToSceneArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::painter_path::PainterPath {
      let path = self;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToScene_to_output_path(original_self as *const ::graphics_item::GraphicsItem, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToSceneArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let point = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToScene_to_output_point(original_self as *const ::graphics_item::GraphicsItem, point as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToSceneArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let polygon = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToScene_to_output_polygon(original_self as *const ::graphics_item::GraphicsItem, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToSceneArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let rect = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToScene_to_output_rect(original_self as *const ::graphics_item::GraphicsItem, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToSceneArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_core::point_f::PointF {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToScene_to_output_x_y(original_self as *const ::graphics_item::GraphicsItem, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsItemMapToSceneArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_item::GraphicsItem) -> ::qt_gui::polygon_f::PolygonF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_mapToScene_to_output_x_y_w_h(original_self as *const ::graphics_item::GraphicsItem, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::paint](../struct.GraphicsItem.html#method.paint) method.
  pub trait GraphicsItemPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsItem_paint_painter_option(original_self as *mut ::graphics_item::GraphicsItem,
                                                             painter,
                                                             option)
    }
  }
  impl<'largs> GraphicsItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                      *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                      *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsItem_paint_painter_option_widget(original_self as *mut ::graphics_item::GraphicsItem, painter, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::scroll](../struct.GraphicsItem.html#method.scroll) method.
  pub trait GraphicsItemScrollArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemScrollArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_scroll_dx_dy(original_self as *mut ::graphics_item::GraphicsItem, dx, dy)
      }
    }
  }
  impl<'largs> GraphicsItemScrollArgs<'largs> for (::libc::c_double, ::libc::c_double, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let dx = self.0;
      let dy = self.1;
      let rect = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_scroll_dx_dy_rect(original_self as *mut ::graphics_item::GraphicsItem,
                                                            dx,
                                                            dy,
                                                            rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_cache_mode](../struct.GraphicsItem.html#method.set_cache_mode) method.
  pub trait GraphicsItemSetCacheModeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetCacheModeArgs<'largs> for ::graphics_item::CacheMode {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let mode = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setCacheMode_mode(original_self as *mut ::graphics_item::GraphicsItem, mode)
      }
    }
  }
  impl<'largs> GraphicsItemSetCacheModeArgs<'largs> for (::graphics_item::CacheMode, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let mode = self.0;
      let cache_size = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setCacheMode_mode_cacheSize(original_self as *mut ::graphics_item::GraphicsItem, mode, cache_size as *const ::qt_core::size::Size) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_flag](../struct.GraphicsItem.html#method.set_flag) method.
  pub trait GraphicsItemSetFlagArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetFlagArgs<'largs> for ::graphics_item::GraphicsItemFlag {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let flag = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setFlag_flag(original_self as *mut ::graphics_item::GraphicsItem, flag)
      }
    }
  }
  impl<'largs> GraphicsItemSetFlagArgs<'largs> for (::graphics_item::GraphicsItemFlag, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let flag = self.0;
      let enabled = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setFlag_flag_enabled(original_self as *mut ::graphics_item::GraphicsItem,
                                                               flag,
                                                               enabled)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_focus](../struct.GraphicsItem.html#method.set_focus) method.
  pub trait GraphicsItemSetFocusArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetFocusArgs<'largs> for &'largs ::qt_core::qt::FocusReason {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let focus_reason = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setFocus_focusReason(original_self as *mut ::graphics_item::GraphicsItem,
                                                               focus_reason as *const ::qt_core::qt::FocusReason)
      }
    }
  }
  impl<'largs> GraphicsItemSetFocusArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {

      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setFocus_no_args(original_self as *mut ::graphics_item::GraphicsItem) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_matrix](../struct.GraphicsItem.html#method.set_matrix) method.
  pub trait GraphicsItemSetMatrixArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetMatrixArgs<'largs> for &'largs ::qt_gui::matrix::Matrix {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setMatrix_matrix(original_self as *mut ::graphics_item::GraphicsItem,
                                                           matrix as *const ::qt_gui::matrix::Matrix)
      }
    }
  }
  impl<'largs> GraphicsItemSetMatrixArgs<'largs> for (&'largs ::qt_gui::matrix::Matrix, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setMatrix_matrix_combine(original_self as *mut ::graphics_item::GraphicsItem, matrix as *const ::qt_gui::matrix::Matrix, combine)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_pos](../struct.GraphicsItem.html#method.set_pos) method.
  pub trait GraphicsItemSetPosArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetPosArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setPos_pos(original_self as *mut ::graphics_item::GraphicsItem,
                                                     pos as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> GraphicsItemSetPosArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setPos_x_y(original_self as *mut ::graphics_item::GraphicsItem, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_transform](../struct.GraphicsItem.html#method.set_transform) method.
  pub trait GraphicsItemSetTransformArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetTransformArgs<'largs> for &'largs ::qt_gui::transform::Transform {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_setTransform_matrix(original_self as *mut ::graphics_item::GraphicsItem,
                                                              matrix as *const ::qt_gui::transform::Transform)
      }
    }
  }
  impl<'largs> GraphicsItemSetTransformArgs<'largs> for (&'largs ::qt_gui::transform::Transform, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setTransform_matrix_combine(original_self as *mut ::graphics_item::GraphicsItem, matrix as *const ::qt_gui::transform::Transform, combine) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::set_transform_origin_point](../struct.GraphicsItem.html#method.set_transform_origin_point) method.
  pub trait GraphicsItemSetTransformOriginPointArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemSetTransformOriginPointArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let ax = self.0;
      let ay = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setTransformOriginPoint_ax_ay(original_self as *mut ::graphics_item::GraphicsItem, ax, ay) }
    }
  }
  impl<'largs> GraphicsItemSetTransformOriginPointArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let origin = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_setTransformOriginPoint_origin(original_self as *mut ::graphics_item::GraphicsItem, origin as *const ::qt_core::point_f::PointF) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsItem::update](../struct.GraphicsItem.html#method.update) method.
  pub trait GraphicsItemUpdateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> ();
  }
  impl<'largs> GraphicsItemUpdateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {

      unsafe { ::ffi::qt_widgets_c_QGraphicsItem_update_no_args(original_self as *mut ::graphics_item::GraphicsItem) }
    }
  }
  impl<'largs> GraphicsItemUpdateArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_update_rect(original_self as *mut ::graphics_item::GraphicsItem,
                                                      rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsItemUpdateArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_item::GraphicsItem) -> () {
      let x = self.0;
      let y = self.1;
      let width = self.2;
      let height = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItem_update_x_y_width_height(original_self as *mut ::graphics_item::GraphicsItem,
                                                                  x,
                                                                  y,
                                                                  width,
                                                                  height)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    fn exec(self) -> ::qt_core::debug::Debug;
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::graphics_item::GraphicsItemChange) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let change = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsItem_GraphicsItemChange(debug as *const ::qt_core::debug::Debug, change as *const ::graphics_item::GraphicsItemChange, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::graphics_item::GraphicsItemFlag) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let flag = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsItem_GraphicsItemFlag(debug as *const ::qt_core::debug::Debug, flag as *const ::graphics_item::GraphicsItemFlag, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl_unsafe](../fn.op_shl_unsafe.html) method.
  pub trait OpShlUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::debug::Debug;
  }
  impl<'a> OpShlUnsafeArgs for (&'a ::qt_core::debug::Debug, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let item = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsItem(debug as *const ::qt_core::debug::Debug, item, &mut object);
        object
      }
    }
  }
  impl<'a> OpShlUnsafeArgs for (&'a ::qt_core::debug::Debug, *mut ::graphics_object::GraphicsObject) {
    unsafe fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let item = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsItem_G_operator_shl_to_output_QDebug_QGraphicsObject(debug as *const ::qt_core::debug::Debug, item, &mut object);
        object
      }
    }
  }
}
