/// C++ type: <span style='color: green;'>```QGraphicsView::CacheModeFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CacheModeFlag {
  /// C++ enum variant: <span style='color: green;'>```CacheNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```CacheBackground = 1```</span>
  Background = 1,
}

impl ::qt_core::flags::FlaggableEnum for CacheModeFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "CacheModeFlag"
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsView::DragMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DragMode {
  /// C++ enum variant: <span style='color: green;'>```NoDrag = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ScrollHandDrag = 1```</span>
  ScrollHand = 1,
  /// C++ enum variant: <span style='color: green;'>```RubberBandDrag = 2```</span>
  RubberBand = 2,
}

/// C++ type: <span style='color: green;'>```QGraphicsView```</span>
#[repr(C)]
pub struct GraphicsView(u8);

impl GraphicsView {
  /// C++ method: <span style='color: green;'>```QBrush QGraphicsView::backgroundBrush() const```</span>
  ///
  ///
  pub fn background_brush(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_backgroundBrush_to_output(self as *const ::graphics_view::GraphicsView,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QGraphicsView::CacheModeFlag> QGraphicsView::cacheMode() const```</span>
  ///
  ///
  pub fn cache_mode(&self) -> ::qt_core::flags::Flags<::graphics_view::CacheModeFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_cacheMode(self as *const ::graphics_view::GraphicsView) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::centerOn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn center_on(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::centerOn(const QPointF& pos)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn center_on(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::centerOn(double x, double y)```</span>
  ///
  ///
  pub fn center_on<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewCenterOnArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsView::centerOn(const QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn center_on_unsafe(&mut self, item: *const ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsView_centerOn_item(self as *mut ::graphics_view::GraphicsView, item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::DragMode QGraphicsView::dragMode() const```</span>
  ///
  ///
  pub fn drag_mode(&self) -> ::graphics_view::DragMode {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_dragMode(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::ensureVisible```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(const QRectF& rect, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(const QRectF& rect, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(double x, double y, double w, double h)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(double x, double y, double w, double h, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(double x, double y, double w, double h, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  pub fn ensure_visible<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewEnsureVisibleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::ensureVisible```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ensure_visible_unsafe(&mut self, *const ::graphics_item::GraphicsItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(const QGraphicsItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ensure_visible_unsafe(&mut self, (*const ::graphics_item::GraphicsItem, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(const QGraphicsItem* item, int xmargin = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ensure_visible_unsafe(&mut self, (*const ::graphics_item::GraphicsItem, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::ensureVisible(const QGraphicsItem* item, int xmargin = ?, int ymargin = ?)```</span>
  ///
  ///
  pub unsafe fn ensure_visible_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewEnsureVisibleUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::fitInView```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fit_in_view(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::fitInView(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fit_in_view(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::fitInView(const QRectF& rect, Qt::AspectRatioMode aspectRadioMode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn fit_in_view(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::fitInView(double x, double y, double w, double h)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn fit_in_view(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_core::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::fitInView(double x, double y, double w, double h, Qt::AspectRatioMode aspectRadioMode = ?)```</span>
  ///
  ///
  pub fn fit_in_view<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewFitInViewArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::fitInView```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fit_in_view_unsafe(&mut self, *const ::graphics_item::GraphicsItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::fitInView(const QGraphicsItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fit_in_view_unsafe(&mut self, (*const ::graphics_item::GraphicsItem, &::qt_core::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::fitInView(const QGraphicsItem* item, Qt::AspectRatioMode aspectRadioMode = ?)```</span>
  ///
  ///
  pub unsafe fn fit_in_view_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewFitInViewUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QBrush QGraphicsView::foregroundBrush() const```</span>
  ///
  ///
  pub fn foreground_brush(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_foregroundBrush_to_output(self as *const ::graphics_view::GraphicsView,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QGraphicsView::inputMethodQuery(Qt::InputMethodQuery query) const```</span>
  ///
  ///
  pub fn input_method_query(&self, query: &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_inputMethodQuery_to_output(self as *const ::graphics_view::GraphicsView,
                                                                     query as *const ::qt_core::qt::InputMethodQuery,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsView::isInteractive() const```</span>
  ///
  ///
  pub fn is_interactive(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_isInteractive(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsView::isTransformed() const```</span>
  ///
  ///
  pub fn is_transformed(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_isTransformed(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, &::qt_core::point::Point) -> *mut ::graphics_item::GraphicsItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsView::itemAt(const QPoint& pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::graphics_item::GraphicsItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsView::itemAt(int x, int y) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::graphics_item::GraphicsItem
    where Args: overloading::GraphicsViewItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::items```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn items(&self, ()) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn items(&self, &::qt_gui::painter_path::PainterPath) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QPainterPath& path, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn items(&self, &::qt_core::point::Point) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QPoint& pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn items(&self, &::qt_gui::polygon::Polygon) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QPolygon& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::polygon::Polygon, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QPolygon& polygon, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn items(&self, &::qt_core::rect::Rect) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QRect& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::rect::Rect, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(const QRect& rect, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn items(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(int x, int y) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn items(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(int x, int y, int w, int h) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn items(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsView::items(int x, int y, int w, int h, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  pub fn items<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsItemMutPtr
    where Args: overloading::GraphicsViewItemsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::mapFromScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_gui::painter_path::PainterPath) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsView::mapFromScene(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_core::point_f::PointF) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QGraphicsView::mapFromScene(const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_from_scene(&self, (::libc::c_double, ::libc::c_double)) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QGraphicsView::mapFromScene(double x, double y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_gui::polygon_f::PolygonF) -> ::qt_gui::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QGraphicsView::mapFromScene(const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_from_scene(&self, &::qt_core::rect_f::RectF) -> ::qt_gui::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QGraphicsView::mapFromScene(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_from_scene(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::qt_gui::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QGraphicsView::mapFromScene(double x, double y, double w, double h) const```</span>
  ///
  ///
  pub fn map_from_scene<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsViewMapFromSceneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::mapToScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_gui::painter_path::PainterPath) -> ::qt_gui::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsView::mapToScene(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_core::point::Point) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsView::mapToScene(const QPoint& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map_to_scene(&self, (::libc::c_int, ::libc::c_int)) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsView::mapToScene(int x, int y) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_gui::polygon::Polygon) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsView::mapToScene(const QPolygon& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map_to_scene(&self, &::qt_core::rect::Rect) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsView::mapToScene(const QRect& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map_to_scene(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::qt_gui::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h) const```</span>
  ///
  ///
  pub fn map_to_scene<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::GraphicsViewMapToSceneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix QGraphicsView::matrix() const```</span>
  ///
  ///
  pub fn matrix(&self) -> ::qt_gui::matrix::Matrix {
    {
      let mut object: ::qt_gui::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_matrix_to_output(self as *const ::graphics_view::GraphicsView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsView::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_metaObject(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsView::QGraphicsView()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_view::GraphicsView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::QGraphicsView```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_scene::GraphicsScene) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsView::QGraphicsView(QGraphicsScene* scene)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::graphics_scene::GraphicsScene, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsView::QGraphicsView(QGraphicsScene* scene, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsView::QGraphicsView(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView>
    where Args: overloading::GraphicsViewNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFlags<QGraphicsView::OptimizationFlag> QGraphicsView::optimizationFlags() const```</span>
  ///
  ///
  pub fn optimization_flags(&self) -> ::qt_core::flags::Flags<::graphics_view::OptimizationFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_optimizationFlags(self as *const ::graphics_view::GraphicsView) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsView::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsView_qt_metacall(self as *mut ::graphics_view::GraphicsView,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsView::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsView_qt_metacast(self as *mut ::graphics_view::GraphicsView, arg1)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::render```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn render(&mut self, *mut ::qt_gui::painter::Painter) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::render(QPainter* painter)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::render(QPainter* painter, const QRectF& target = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect_f::RectF, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::render(QPainter* painter, const QRectF& target = ?, const QRect& source = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect_f::RectF, &::qt_core::rect::Rect, &::qt_core::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::render(QPainter* painter, const QRectF& target = ?, const QRect& source = ?, Qt::AspectRatioMode aspectRatioMode = ?)```</span>
  ///
  ///
  pub unsafe fn render<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewRenderArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsView::resetCachedContent()```</span>
  ///
  ///
  pub fn reset_cached_content(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_resetCachedContent(self as *mut ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::resetMatrix()```</span>
  ///
  ///
  pub fn reset_matrix(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_resetMatrix(self as *mut ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::resetTransform()```</span>
  ///
  ///
  pub fn reset_transform(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_resetTransform(self as *mut ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::ViewportAnchor QGraphicsView::resizeAnchor() const```</span>
  ///
  ///
  pub fn resize_anchor(&self) -> ::graphics_view::ViewportAnchor {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_resizeAnchor(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::rotate(double angle)```</span>
  ///
  ///
  pub fn rotate(&mut self, angle: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_rotate(self as *mut ::graphics_view::GraphicsView, angle) }
  }

  /// C++ method: <span style='color: green;'>```QRect QGraphicsView::rubberBandRect() const```</span>
  ///
  ///
  pub fn rubber_band_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_rubberBandRect_to_output(self as *const ::graphics_view::GraphicsView,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::scale(double sx, double sy)```</span>
  ///
  ///
  pub fn scale(&mut self, sx: ::libc::c_double, sy: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_scale(self as *mut ::graphics_view::GraphicsView, sx, sy) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene* QGraphicsView::scene() const```</span>
  ///
  ///
  pub fn scene(&self) -> *mut ::graphics_scene::GraphicsScene {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_scene(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsView::sceneRect() const```</span>
  ///
  ///
  pub fn scene_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_sceneRect_to_output(self as *const ::graphics_view::GraphicsView,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setBackgroundBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background_brush(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_setBackgroundBrush(self as *mut ::graphics_view::GraphicsView,
                                                           brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setCacheMode(QFlags<QGraphicsView::CacheModeFlag> mode)```</span>
  ///
  ///
  pub fn set_cache_mode(&mut self, mode: ::qt_core::flags::Flags<::graphics_view::CacheModeFlag>) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_setCacheMode(self as *mut ::graphics_view::GraphicsView,
                                                     mode.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setDragMode(QGraphicsView::DragMode mode)```</span>
  ///
  ///
  pub fn set_drag_mode(&mut self, mode: ::graphics_view::DragMode) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_setDragMode(self as *mut ::graphics_view::GraphicsView, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setForegroundBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground_brush(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_setForegroundBrush(self as *mut ::graphics_view::GraphicsView,
                                                           brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setInteractive(bool allowed)```</span>
  ///
  ///
  pub fn set_interactive(&mut self, allowed: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_setInteractive(self as *mut ::graphics_view::GraphicsView, allowed) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::setMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_matrix(&mut self, &::qt_gui::matrix::Matrix) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setMatrix(const QMatrix& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_matrix(&mut self, (&::qt_gui::matrix::Matrix, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setMatrix(const QMatrix& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_matrix<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewSetMatrixArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::setOptimizationFlag```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_optimization_flag(&mut self, ::graphics_view::OptimizationFlag) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setOptimizationFlag(QGraphicsView::OptimizationFlag flag)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_optimization_flag(&mut self, (::graphics_view::OptimizationFlag, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setOptimizationFlag(QGraphicsView::OptimizationFlag flag, bool enabled = ?)```</span>
  ///
  ///
  pub fn set_optimization_flag<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewSetOptimizationFlagArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setOptimizationFlags(QFlags<QGraphicsView::OptimizationFlag> flags)```</span>
  ///
  ///
  pub fn set_optimization_flags(&mut self, flags: ::qt_core::flags::Flags<::graphics_view::OptimizationFlag>) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_setOptimizationFlags(self as *mut ::graphics_view::GraphicsView,
                                                             flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::setRenderHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_render_hint(&mut self, &::qt_gui::painter::RenderHint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setRenderHint(QPainter::RenderHint hint)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_render_hint(&mut self, (&::qt_gui::painter::RenderHint, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setRenderHint(QPainter::RenderHint hint, bool enabled = ?)```</span>
  ///
  ///
  pub fn set_render_hint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewSetRenderHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setResizeAnchor(QGraphicsView::ViewportAnchor anchor)```</span>
  ///
  ///
  pub fn set_resize_anchor(&mut self, anchor: ::graphics_view::ViewportAnchor) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_setResizeAnchor(self as *mut ::graphics_view::GraphicsView, anchor) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setRubberBandSelectionMode(Qt::ItemSelectionMode mode)```</span>
  ///
  ///
  pub fn set_rubber_band_selection_mode(&mut self, mode: &::qt_core::qt::ItemSelectionMode) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_setRubberBandSelectionMode(self as *mut ::graphics_view::GraphicsView,
                                                                   mode as *const ::qt_core::qt::ItemSelectionMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setScene(QGraphicsScene* scene)```</span>
  ///
  ///
  pub unsafe fn set_scene(&mut self, scene: *mut ::graphics_scene::GraphicsScene) {
    ::ffi::qt_widgets_c_QGraphicsView_setScene(self as *mut ::graphics_view::GraphicsView, scene)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::setSceneRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_scene_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setSceneRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_scene_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setSceneRect(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn set_scene_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewSetSceneRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView::setTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_transform(&mut self, &::qt_gui::transform::Transform) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setTransform(const QTransform& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_transform(&mut self, (&::qt_gui::transform::Transform, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setTransform(const QTransform& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_transform<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsViewSetTransformArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsView::setTransformationAnchor(QGraphicsView::ViewportAnchor anchor)```</span>
  ///
  ///
  pub fn set_transformation_anchor(&mut self, anchor: ::graphics_view::ViewportAnchor) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_setTransformationAnchor(self as *mut ::graphics_view::GraphicsView, anchor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::setViewportUpdateMode(QGraphicsView::ViewportUpdateMode mode)```</span>
  ///
  ///
  pub fn set_viewport_update_mode(&mut self, mode: ::graphics_view::ViewportUpdateMode) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_setViewportUpdateMode(self as *mut ::graphics_view::GraphicsView, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::shear(double sh, double sv)```</span>
  ///
  ///
  pub fn shear(&mut self, sh: ::libc::c_double, sv: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_shear(self as *mut ::graphics_view::GraphicsView, sh, sv) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QGraphicsView::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_sizeHint_to_output(self as *const ::graphics_view::GraphicsView, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsView::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsView_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsView::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsView_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QGraphicsView::transform() const```</span>
  ///
  ///
  pub fn transform(&self) -> ::qt_gui::transform::Transform {
    {
      let mut object: ::qt_gui::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_transform_to_output(self as *const ::graphics_view::GraphicsView,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::ViewportAnchor QGraphicsView::transformationAnchor() const```</span>
  ///
  ///
  pub fn transformation_anchor(&self) -> ::graphics_view::ViewportAnchor {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_transformationAnchor(self as *const ::graphics_view::GraphicsView) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsView::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate(&mut self, dx: ::libc::c_double, dy: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_translate(self as *mut ::graphics_view::GraphicsView, dx, dy) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsView::updateScene(const QList<QRectF>& rects)```</span>
  ///
  ///
  pub fn update_scene(&mut self, rects: &::list::ListQtCoreRectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_updateScene(self as *mut ::graphics_view::GraphicsView,
                                                    rects as *const ::list::ListQtCoreRectF)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsView::updateSceneRect(const QRectF& rect)```</span>
  ///
  ///
  pub fn update_scene_rect(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsView_updateSceneRect(self as *mut ::graphics_view::GraphicsView,
                                                        rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QGraphicsView::viewportTransform() const```</span>
  ///
  ///
  pub fn viewport_transform(&self) -> ::qt_gui::transform::Transform {
    {
      let mut object: ::qt_gui::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_viewportTransform_to_output(self as *const ::graphics_view::GraphicsView,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView::ViewportUpdateMode QGraphicsView::viewportUpdateMode() const```</span>
  ///
  ///
  pub fn viewport_update_mode(&self) -> ::graphics_view::ViewportUpdateMode {
    unsafe { ::ffi::qt_widgets_c_QGraphicsView_viewportUpdateMode(self as *const ::graphics_view::GraphicsView) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_view::GraphicsView {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsView_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsView`.
  pub struct Signals<'a>(&'a ::graphics_view::GraphicsView);
  /// Represents a built-in Qt signal `QGraphicsView::rubberBandChanged`.
  ///
  /// An object of this type can be created from `GraphicsView` with `object.signals().rubber_band_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsView` object.
  pub struct RubberBandChanged<'a>(&'a ::graphics_view::GraphicsView);
  impl<'a> ::qt_core::connection::Receiver for RubberBandChanged<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect,
     &'static ::qt_core::point_f::PointF,
     &'static ::qt_core::point_f::PointF);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rubberBandChanged(QRect,QPointF,QPointF)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RubberBandChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsView::rubberBandChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rubber_band_changed(&self) -> RubberBandChanged {
      RubberBandChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsView`.
  pub struct Slots<'a>(&'a ::graphics_view::GraphicsView);
  /// Represents a built-in Qt slot `QGraphicsView::updateScene`.
  ///
  /// An object of this type can be created from `GraphicsView` with `object.slots().update_scene()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsView` object.
  pub struct UpdateScene<'a>(&'a ::graphics_view::GraphicsView);
  impl<'a> ::qt_core::connection::Receiver for UpdateScene<'a> {
    type Arguments = (&'static ::list::ListQtCoreRectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateScene(const QList< QRectF >&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsView::updateSceneRect`.
  ///
  /// An object of this type can be created from `GraphicsView` with `object.slots().update_scene_rect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsView` object.
  pub struct UpdateSceneRect<'a>(&'a ::graphics_view::GraphicsView);
  impl<'a> ::qt_core::connection::Receiver for UpdateSceneRect<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateSceneRect(const QRectF&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsView::setupViewport`.
  ///
  /// An object of this type can be created from `GraphicsView` with `object.slots().setup_viewport()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsView` object.
  pub struct SetupViewport<'a>(&'a ::graphics_view::GraphicsView);
  impl<'a> ::qt_core::connection::Receiver for SetupViewport<'a> {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setupViewport(QWidget*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsView::updateScene`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_scene(&self) -> UpdateScene {
      UpdateScene(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsView::updateSceneRect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_scene_rect(&self) -> UpdateSceneRect {
      UpdateSceneRect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsView::setupViewport`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn setup_viewport(&self) -> SetupViewport {
      SetupViewport(self.0)
    }
  }
  impl ::graphics_view::GraphicsView {
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

/// C++ type: <span style='color: green;'>```QGraphicsView::OptimizationFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OptimizationFlag {
  /// C++ enum variant: <span style='color: green;'>```DontClipPainter = 1```</span>
  DontClipPainter = 1,
  /// C++ enum variant: <span style='color: green;'>```DontSavePainterState = 2```</span>
  DontSavePainterState = 2,
  /// C++ enum variant: <span style='color: green;'>```DontAdjustForAntialiasing = 4```</span>
  DontAdjustForAntialiasing = 4,
  /// C++ enum variant: <span style='color: green;'>```IndirectPainting = 8```</span>
  IndirectPainting = 8,
}

impl ::qt_core::flags::FlaggableEnum for OptimizationFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "OptimizationFlag"
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsView::ViewportAnchor```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewportAnchor {
  /// C++ enum variant: <span style='color: green;'>```NoAnchor = 0```</span>
  NoAnchor = 0,
  /// C++ enum variant: <span style='color: green;'>```AnchorViewCenter = 1```</span>
  AnchorViewCenter = 1,
  /// C++ enum variant: <span style='color: green;'>```AnchorUnderMouse = 2```</span>
  AnchorUnderMouse = 2,
}

/// C++ type: <span style='color: green;'>```QGraphicsView::ViewportUpdateMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewportUpdateMode {
  /// C++ enum variant: <span style='color: green;'>```FullViewportUpdate = 0```</span>
  Full = 0,
  /// C++ enum variant: <span style='color: green;'>```MinimalViewportUpdate = 1```</span>
  Minimal = 1,
  /// C++ enum variant: <span style='color: green;'>```SmartViewportUpdate = 2```</span>
  Smart = 2,
  /// C++ enum variant: <span style='color: green;'>```NoViewportUpdate = 3```</span>
  No = 3,
  /// C++ enum variant: <span style='color: green;'>```BoundingRectViewportUpdate = 4```</span>
  BoundingRect = 4,
}

impl ::cpp_utils::DynamicCast<::graphics_view::GraphicsView> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_view::GraphicsView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_view::GraphicsView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_view::GraphicsView> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_view::GraphicsView> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_view::GraphicsView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_view::GraphicsView> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_view::GraphicsView> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_view::GraphicsView> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_dynamic_cast_QGraphicsView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_view::GraphicsView {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QObject_ptr(self as *mut ::graphics_view::GraphicsView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QObject_ptr(self as *const ::graphics_view::GraphicsView as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::graphics_view::GraphicsView {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QPaintDevice_ptr(self as *mut ::graphics_view::GraphicsView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QPaintDevice_ptr(self as *const ::graphics_view::GraphicsView as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::graphics_view::GraphicsView {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::graphics_view::GraphicsView as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::graphics_view::GraphicsView {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QFrame_ptr(self as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QFrame_ptr(self as *const ::graphics_view::GraphicsView as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::graphics_view::GraphicsView {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QWidget_ptr(self as *mut ::graphics_view::GraphicsView)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QWidget_ptr(self as *const ::graphics_view::GraphicsView as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_view::GraphicsView> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_view::GraphicsView> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_view::GraphicsView {
    let ffi_result =
      ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_view::GraphicsView> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_view::GraphicsView> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_view::GraphicsView> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_view::GraphicsView {
    let ffi_result =
      ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_view::GraphicsView {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QGraphicsView_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_view::GraphicsView {
  type Target = ::abstract_scroll_area::AbstractScrollArea;
  fn deref(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QAbstractScrollArea_ptr(self as *const ::graphics_view::GraphicsView as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_view::GraphicsView {
  fn deref_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsView_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::graphics_view::GraphicsView) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsView::center_on](../struct.GraphicsView.html#method.center_on) method.
  pub trait GraphicsViewCenterOnArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewCenterOnArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_centerOn_pos(original_self as *mut ::graphics_view::GraphicsView,
                                                       pos as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> GraphicsViewCenterOnArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_centerOn_x_y(original_self as *mut ::graphics_view::GraphicsView, x, y)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::ensure_visible](../struct.GraphicsView.html#method.ensure_visible) method.
  pub trait GraphicsViewEnsureVisibleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewEnsureVisibleArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_rect(original_self as *mut ::graphics_view::GraphicsView,
                                                             rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let rect = self.0;
      let xmargin = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_rect_xmargin(original_self as *mut ::graphics_view::GraphicsView, rect as *const ::qt_core::rect_f::RectF, xmargin) }
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let rect = self.0;
      let xmargin = self.1;
      let ymargin = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_rect_xmargin_ymargin(original_self as *mut ::graphics_view::GraphicsView, rect as *const ::qt_core::rect_f::RectF, xmargin, ymargin) }
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h(original_self as *mut ::graphics_view::GraphicsView,
                                                                x,
                                                                y,
                                                                w,
                                                                h)
      }
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let xmargin = self.4;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h_xmargin(original_self as *mut ::graphics_view::GraphicsView, x, y, w, h, xmargin) }
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let xmargin = self.4;
      let ymargin = self.5;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_x_y_w_h_xmargin_ymargin(original_self as *mut ::graphics_view::GraphicsView, x, y, w, h, xmargin, ymargin) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::ensure_visible_unsafe](../struct.GraphicsView.html#method.ensure_visible_unsafe) method.
  pub trait GraphicsViewEnsureVisibleUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewEnsureVisibleUnsafeArgs<'largs> for *const ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_item(original_self as *mut ::graphics_view::GraphicsView, item)
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleUnsafeArgs<'largs> for (*const ::graphics_item::GraphicsItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let item = self.0;
      let xmargin = self.1;
      ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_item_xmargin(original_self as *mut ::graphics_view::GraphicsView, item, xmargin)
    }
  }
  impl<'largs> GraphicsViewEnsureVisibleUnsafeArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let item = self.0;
      let xmargin = self.1;
      let ymargin = self.2;
      ::ffi::qt_widgets_c_QGraphicsView_ensureVisible_item_xmargin_ymargin(original_self as *mut ::graphics_view::GraphicsView, item, xmargin, ymargin)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::fit_in_view](../struct.GraphicsView.html#method.fit_in_view) method.
  pub trait GraphicsViewFitInViewArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewFitInViewArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_fitInView_rect(original_self as *mut ::graphics_view::GraphicsView,
                                                         rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsViewFitInViewArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let rect = self.0;
      let aspect_radio_mode = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_fitInView_rect_aspectRadioMode(original_self as *mut ::graphics_view::GraphicsView, rect as *const ::qt_core::rect_f::RectF, aspect_radio_mode as *const ::qt_core::qt::AspectRatioMode) }
    }
  }
  impl<'largs> GraphicsViewFitInViewArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_fitInView_x_y_w_h(original_self as *mut ::graphics_view::GraphicsView,
                                                            x,
                                                            y,
                                                            w,
                                                            h)
      }
    }
  }
  impl<'largs> GraphicsViewFitInViewArgs<'largs>
    for (::libc::c_double,
                                                          ::libc::c_double,
                                                          ::libc::c_double,
                                                          ::libc::c_double,
                                                          &'largs ::qt_core::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let aspect_radio_mode = self.4;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_fitInView_x_y_w_h_aspectRadioMode(original_self as *mut ::graphics_view::GraphicsView, x, y, w, h, aspect_radio_mode as *const ::qt_core::qt::AspectRatioMode) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::fit_in_view_unsafe](../struct.GraphicsView.html#method.fit_in_view_unsafe) method.
  pub trait GraphicsViewFitInViewUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewFitInViewUnsafeArgs<'largs> for *const ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QGraphicsView_fitInView_item(original_self as *mut ::graphics_view::GraphicsView, item)
    }
  }
  impl<'largs> GraphicsViewFitInViewUnsafeArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::qt::AspectRatioMode) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let item = self.0;
      let aspect_radio_mode = self.1;
      ::ffi::qt_widgets_c_QGraphicsView_fitInView_item_aspectRadioMode(original_self as *mut ::graphics_view::GraphicsView, item, aspect_radio_mode as *const ::qt_core::qt::AspectRatioMode)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::item_at](../struct.GraphicsView.html#method.item_at) method.
  pub trait GraphicsViewItemAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> *mut ::graphics_item::GraphicsItem;
  }
  impl<'largs> GraphicsViewItemAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> *mut ::graphics_item::GraphicsItem {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_itemAt_pos(original_self as *const ::graphics_view::GraphicsView,
                                                     pos as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> GraphicsViewItemAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> *mut ::graphics_item::GraphicsItem {
      let x = self.0;
      let y = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_itemAt_x_y(original_self as *const ::graphics_view::GraphicsView, x, y)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::items](../struct.GraphicsView.html#method.items) method.
  pub trait GraphicsViewItemsArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr;
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {

      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_no_args(original_self as *const ::graphics_view::GraphicsView, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let path = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_path(original_self as *const ::graphics_view::GraphicsView, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let path = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_path_mode(original_self as *const ::graphics_view::GraphicsView, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for &'largs ::qt_gui::polygon::Polygon {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let polygon = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_polygon(original_self as *const ::graphics_view::GraphicsView, polygon as *const ::qt_gui::polygon::Polygon, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs>
    for (&'largs ::qt_gui::polygon::Polygon, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let polygon = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_polygon_mode(original_self as *const ::graphics_view::GraphicsView, polygon as *const ::qt_gui::polygon::Polygon, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_pos(original_self as *const ::graphics_view::GraphicsView, pos as *const ::qt_core::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let rect = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_rect(original_self as *const ::graphics_view::GraphicsView, rect as *const ::qt_core::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let rect = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_rect_mode(original_self as *const ::graphics_view::GraphicsView, rect as *const ::qt_core::rect::Rect, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_x_y(original_self as *const ::graphics_view::GraphicsView, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_x_y_w_h(original_self as *const ::graphics_view::GraphicsView, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewItemsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::list::ListGraphicsItemMutPtr {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let mode = self.4;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_items_to_output_x_y_w_h_mode(original_self as *const ::graphics_view::GraphicsView, x, y, w, h, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::map_from_scene](../struct.GraphicsView.html#method.map_from_scene) method.
  pub trait GraphicsViewMapFromSceneArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> Self::ReturnType;
  }
  impl<'largs> GraphicsViewMapFromSceneArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::painter_path::PainterPath {
      let path = self;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapFromScene_to_output_path(original_self as *const ::graphics_view::GraphicsView, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapFromSceneArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_core::point::Point {
      let point = self;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapFromScene_to_output_point(original_self as *const ::graphics_view::GraphicsView, point as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapFromSceneArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    type ReturnType = ::qt_gui::polygon::Polygon;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::polygon::Polygon {
      let polygon = self;
      {
        let mut object: ::qt_gui::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapFromScene_to_output_polygon(original_self as *const ::graphics_view::GraphicsView, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapFromSceneArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_gui::polygon::Polygon;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::polygon::Polygon {
      let rect = self;
      {
        let mut object: ::qt_gui::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapFromScene_to_output_rect(original_self as *const ::graphics_view::GraphicsView, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapFromSceneArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_core::point::Point {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapFromScene_to_output_x_y(original_self as *const ::graphics_view::GraphicsView, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapFromSceneArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    type ReturnType = ::qt_gui::polygon::Polygon;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::polygon::Polygon {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_gui::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapFromScene_to_output_x_y_w_h(original_self as *const ::graphics_view::GraphicsView, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::map_to_scene](../struct.GraphicsView.html#method.map_to_scene) method.
  pub trait GraphicsViewMapToSceneArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> Self::ReturnType;
  }
  impl<'largs> GraphicsViewMapToSceneArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    type ReturnType = ::qt_gui::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::painter_path::PainterPath {
      let path = self;
      {
        let mut object: ::qt_gui::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapToScene_to_output_path(original_self as *const ::graphics_view::GraphicsView, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapToSceneArgs<'largs> for &'largs ::qt_core::point::Point {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_core::point_f::PointF {
      let point = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapToScene_to_output_point(original_self as *const ::graphics_view::GraphicsView, point as *const ::qt_core::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapToSceneArgs<'largs> for &'largs ::qt_gui::polygon::Polygon {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::polygon_f::PolygonF {
      let polygon = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapToScene_to_output_polygon(original_self as *const ::graphics_view::GraphicsView, polygon as *const ::qt_gui::polygon::Polygon, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapToSceneArgs<'largs> for &'largs ::qt_core::rect::Rect {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::polygon_f::PolygonF {
      let rect = self;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapToScene_to_output_rect(original_self as *const ::graphics_view::GraphicsView, rect as *const ::qt_core::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapToSceneArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_core::point_f::PointF {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapToScene_to_output_x_y(original_self as *const ::graphics_view::GraphicsView, x, y, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsViewMapToSceneArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    type ReturnType = ::qt_gui::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::graphics_view::GraphicsView) -> ::qt_gui::polygon_f::PolygonF {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      {
        let mut object: ::qt_gui::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsView_mapToScene_to_output_x_y_w_h(original_self as *const ::graphics_view::GraphicsView, x, y, w, h, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::new_unsafe](../struct.GraphicsView.html#method.new_unsafe) method.
  pub trait GraphicsViewNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView>;
  }
  impl GraphicsViewNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsViewNewUnsafeArgs for *mut ::graphics_scene::GraphicsScene {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView> {
      let scene = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_new_scene(scene);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsViewNewUnsafeArgs for (*mut ::graphics_scene::GraphicsScene, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_view::GraphicsView> {
      let scene = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsView_new_scene_parent(scene, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::render](../struct.GraphicsView.html#method.render) method.
  pub trait GraphicsViewRenderArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewRenderArgs<'largs> for *mut ::qt_gui::painter::Painter {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let painter = self;
      ::ffi::qt_widgets_c_QGraphicsView_render_painter(original_self as *mut ::graphics_view::GraphicsView, painter)
    }
  }
  impl<'largs> GraphicsViewRenderArgs<'largs> for (*mut ::qt_gui::painter::Painter, &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let painter = self.0;
      let target = self.1;
      ::ffi::qt_widgets_c_QGraphicsView_render_painter_target(original_self as *mut ::graphics_view::GraphicsView,
                                                              painter,
                                                              target as *const ::qt_core::rect_f::RectF)
    }
  }
  impl<'largs> GraphicsViewRenderArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, &'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::rect::Rect) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let painter = self.0;
      let target = self.1;
      let source = self.2;
      ::ffi::qt_widgets_c_QGraphicsView_render_painter_target_source(original_self as *mut ::graphics_view::GraphicsView, painter, target as *const ::qt_core::rect_f::RectF, source as *const ::qt_core::rect::Rect)
    }
  }
  impl<'largs> GraphicsViewRenderArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                       &'largs ::qt_core::rect_f::RectF,
                                                       &'largs ::qt_core::rect::Rect,
                                                       &'largs ::qt_core::qt::AspectRatioMode) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let painter = self.0;
      let target = self.1;
      let source = self.2;
      let aspect_ratio_mode = self.3;
      ::ffi::qt_widgets_c_QGraphicsView_render_painter_target_source_aspectRatioMode(original_self as *mut ::graphics_view::GraphicsView, painter, target as *const ::qt_core::rect_f::RectF, source as *const ::qt_core::rect::Rect, aspect_ratio_mode as *const ::qt_core::qt::AspectRatioMode)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::set_matrix](../struct.GraphicsView.html#method.set_matrix) method.
  pub trait GraphicsViewSetMatrixArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewSetMatrixArgs<'largs> for &'largs ::qt_gui::matrix::Matrix {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setMatrix_matrix(original_self as *mut ::graphics_view::GraphicsView,
                                                           matrix as *const ::qt_gui::matrix::Matrix)
      }
    }
  }
  impl<'largs> GraphicsViewSetMatrixArgs<'largs> for (&'largs ::qt_gui::matrix::Matrix, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setMatrix_matrix_combine(original_self as *mut ::graphics_view::GraphicsView, matrix as *const ::qt_gui::matrix::Matrix, combine)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::set_optimization_flag](../struct.GraphicsView.html#method.set_optimization_flag) method.
  pub trait GraphicsViewSetOptimizationFlagArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewSetOptimizationFlagArgs<'largs> for ::graphics_view::OptimizationFlag {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let flag = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setOptimizationFlag_flag(original_self as *mut ::graphics_view::GraphicsView, flag)
      }
    }
  }
  impl<'largs> GraphicsViewSetOptimizationFlagArgs<'largs> for (::graphics_view::OptimizationFlag, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let flag = self.0;
      let enabled = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_setOptimizationFlag_flag_enabled(original_self as *mut ::graphics_view::GraphicsView, flag, enabled) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::set_render_hint](../struct.GraphicsView.html#method.set_render_hint) method.
  pub trait GraphicsViewSetRenderHintArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewSetRenderHintArgs<'largs> for &'largs ::qt_gui::painter::RenderHint {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let hint = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setRenderHint_hint(original_self as *mut ::graphics_view::GraphicsView,
                                                             hint as *const ::qt_gui::painter::RenderHint)
      }
    }
  }
  impl<'largs> GraphicsViewSetRenderHintArgs<'largs> for (&'largs ::qt_gui::painter::RenderHint, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let hint = self.0;
      let enabled = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_setRenderHint_hint_enabled(original_self as *mut ::graphics_view::GraphicsView, hint as *const ::qt_gui::painter::RenderHint, enabled) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::set_scene_rect](../struct.GraphicsView.html#method.set_scene_rect) method.
  pub trait GraphicsViewSetSceneRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewSetSceneRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setSceneRect_rect(original_self as *mut ::graphics_view::GraphicsView,
                                                            rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsViewSetSceneRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setSceneRect_x_y_w_h(original_self as *mut ::graphics_view::GraphicsView,
                                                               x,
                                                               y,
                                                               w,
                                                               h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsView::set_transform](../struct.GraphicsView.html#method.set_transform) method.
  pub trait GraphicsViewSetTransformArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> ();
  }
  impl<'largs> GraphicsViewSetTransformArgs<'largs> for &'largs ::qt_gui::transform::Transform {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsView_setTransform_matrix(original_self as *mut ::graphics_view::GraphicsView,
                                                              matrix as *const ::qt_gui::transform::Transform)
      }
    }
  }
  impl<'largs> GraphicsViewSetTransformArgs<'largs> for (&'largs ::qt_gui::transform::Transform, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_view::GraphicsView) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsView_setTransform_matrix_combine(original_self as *mut ::graphics_view::GraphicsView, matrix as *const ::qt_gui::transform::Transform, combine) }
    }
  }
}
