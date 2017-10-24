/// C++ type: <span style='color: green;'>```QGraphicsScene```</span>
#[repr(C)]
pub struct GraphicsScene(u8);

impl GraphicsScene {
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsScene::activePanel() const```</span>
  ///
  ///
  pub fn active_panel(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_activePanel(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QGraphicsScene::activeWindow() const```</span>
  ///
  ///
  pub fn active_window(&self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_activeWindow(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::addEllipse```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, &::qt_core::rect_f::RectF) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem* QGraphicsScene::addEllipse(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (&::qt_core::rect_f::RectF, &::qt_gui::pen::Pen)) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem* QGraphicsScene::addEllipse(const QRectF& rect, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (&::qt_core::rect_f::RectF, &::qt_gui::pen::Pen, &::qt_gui::brush::Brush)) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem* QGraphicsScene::addEllipse(const QRectF& rect, const QPen& pen = ?, const QBrush& brush = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem* QGraphicsScene::addEllipse(double x, double y, double w, double h)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_gui::pen::Pen)) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem* QGraphicsScene::addEllipse(double x, double y, double w, double h, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_gui::pen::Pen, &::qt_gui::brush::Brush)) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem* QGraphicsScene::addEllipse(double x, double y, double w, double h, const QPen& pen = ?, const QBrush& brush = ?)```</span>
  ///
  ///
  pub fn add_ellipse<'largs, Args>(&'largs mut self, args: Args) -> *mut ::graphics_ellipse_item::GraphicsEllipseItem
    where Args: overloading::GraphicsSceneAddEllipseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::addItem(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn add_item(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsScene_addItem(self as *mut ::graphics_scene::GraphicsScene, item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::addLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_line(&mut self, &::qt_core::line_f::LineF) -> *mut ::graphics_line_item::GraphicsLineItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsLineItem* QGraphicsScene::addLine(const QLineF& line)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_line(&mut self, (&::qt_core::line_f::LineF, &::qt_gui::pen::Pen)) -> *mut ::graphics_line_item::GraphicsLineItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsLineItem* QGraphicsScene::addLine(const QLineF& line, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_line(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> *mut ::graphics_line_item::GraphicsLineItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsLineItem* QGraphicsScene::addLine(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_line(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_gui::pen::Pen)) -> *mut ::graphics_line_item::GraphicsLineItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsLineItem* QGraphicsScene::addLine(double x1, double y1, double x2, double y2, const QPen& pen = ?)```</span>
  ///
  ///
  pub fn add_line<'largs, Args>(&'largs mut self, args: Args) -> *mut ::graphics_line_item::GraphicsLineItem
    where Args: overloading::GraphicsSceneAddLineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::addPath```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_path(&mut self, &::qt_gui::painter_path::PainterPath) -> *mut ::graphics_path_item::GraphicsPathItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsPathItem* QGraphicsScene::addPath(const QPainterPath& path)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_path(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_gui::pen::Pen)) -> *mut ::graphics_path_item::GraphicsPathItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsPathItem* QGraphicsScene::addPath(const QPainterPath& path, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_path(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_gui::pen::Pen, &::qt_gui::brush::Brush)) -> *mut ::graphics_path_item::GraphicsPathItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsPathItem* QGraphicsScene::addPath(const QPainterPath& path, const QPen& pen = ?, const QBrush& brush = ?)```</span>
  ///
  ///
  pub fn add_path<'largs, Args>(&'largs mut self, args: Args) -> *mut ::graphics_path_item::GraphicsPathItem
    where Args: overloading::GraphicsSceneAddPathArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsPixmapItem* QGraphicsScene::addPixmap(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn add_pixmap(&mut self, pixmap: &::qt_gui::pixmap::Pixmap) -> *mut ::graphics_pixmap_item::GraphicsPixmapItem {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_addPixmap(self as *mut ::graphics_scene::GraphicsScene,
                                                   pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::addPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_polygon(&mut self, &::qt_gui::polygon_f::PolygonF) -> *mut ::graphics_polygon_item::GraphicsPolygonItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsPolygonItem* QGraphicsScene::addPolygon(const QPolygonF& polygon)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_polygon(&mut self, (&::qt_gui::polygon_f::PolygonF, &::qt_gui::pen::Pen)) -> *mut ::graphics_polygon_item::GraphicsPolygonItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsPolygonItem* QGraphicsScene::addPolygon(const QPolygonF& polygon, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_polygon(&mut self, (&::qt_gui::polygon_f::PolygonF, &::qt_gui::pen::Pen, &::qt_gui::brush::Brush)) -> *mut ::graphics_polygon_item::GraphicsPolygonItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsPolygonItem* QGraphicsScene::addPolygon(const QPolygonF& polygon, const QPen& pen = ?, const QBrush& brush = ?)```</span>
  ///
  ///
  pub fn add_polygon<'largs, Args>(&'largs mut self, args: Args) -> *mut ::graphics_polygon_item::GraphicsPolygonItem
    where Args: overloading::GraphicsSceneAddPolygonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::addRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_rect(&mut self, &::qt_core::rect_f::RectF) -> *mut ::graphics_rect_item::GraphicsRectItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem* QGraphicsScene::addRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_gui::pen::Pen)) -> *mut ::graphics_rect_item::GraphicsRectItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem* QGraphicsScene::addRect(const QRectF& rect, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_gui::pen::Pen, &::qt_gui::brush::Brush)) -> *mut ::graphics_rect_item::GraphicsRectItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem* QGraphicsScene::addRect(const QRectF& rect, const QPen& pen = ?, const QBrush& brush = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> *mut ::graphics_rect_item::GraphicsRectItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem* QGraphicsScene::addRect(double x, double y, double w, double h)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn add_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_gui::pen::Pen)) -> *mut ::graphics_rect_item::GraphicsRectItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem* QGraphicsScene::addRect(double x, double y, double w, double h, const QPen& pen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn add_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_gui::pen::Pen, &::qt_gui::brush::Brush)) -> *mut ::graphics_rect_item::GraphicsRectItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem* QGraphicsScene::addRect(double x, double y, double w, double h, const QPen& pen = ?, const QBrush& brush = ?)```</span>
  ///
  ///
  pub fn add_rect<'largs, Args>(&'largs mut self, args: Args) -> *mut ::graphics_rect_item::GraphicsRectItem
    where Args: overloading::GraphicsSceneAddRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::addSimpleText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_simple_text(&mut self, &::qt_core::string::String) -> *mut ::graphics_simple_text_item::GraphicsSimpleTextItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsSimpleTextItem* QGraphicsScene::addSimpleText(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_simple_text(&mut self, (&::qt_core::string::String, &::qt_gui::font::Font)) -> *mut ::graphics_simple_text_item::GraphicsSimpleTextItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsSimpleTextItem* QGraphicsScene::addSimpleText(const QString& text, const QFont& font = ?)```</span>
  ///
  ///
  pub fn add_simple_text<'largs, Args>(&'largs mut self,
                                       args: Args)
                                       -> *mut ::graphics_simple_text_item::GraphicsSimpleTextItem
    where Args: overloading::GraphicsSceneAddSimpleTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::addText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_text(&mut self, &::qt_core::string::String) -> *mut ::graphics_text_item::GraphicsTextItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsTextItem* QGraphicsScene::addText(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_text(&mut self, (&::qt_core::string::String, &::qt_gui::font::Font)) -> *mut ::graphics_text_item::GraphicsTextItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsTextItem* QGraphicsScene::addText(const QString& text, const QFont& font = ?)```</span>
  ///
  ///
  pub fn add_text<'largs, Args>(&'largs mut self, args: Args) -> *mut ::graphics_text_item::GraphicsTextItem
    where Args: overloading::GraphicsSceneAddTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::advance()```</span>
  ///
  ///
  pub fn advance(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_advance(self as *mut ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QBrush QGraphicsScene::backgroundBrush() const```</span>
  ///
  ///
  pub fn background_brush(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_backgroundBrush_to_output(self as *const ::graphics_scene::GraphicsScene,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsScene::bspTreeDepth() const```</span>
  ///
  ///
  pub fn bsp_tree_depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_bspTreeDepth(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_clear(self as *mut ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::clearFocus()```</span>
  ///
  ///
  pub fn clear_focus(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_clearFocus(self as *mut ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::clearSelection()```</span>
  ///
  ///
  pub fn clear_selection(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_clearSelection(self as *mut ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::collidingItems```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn colliding_items(&self, *const ::graphics_item::GraphicsItem) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::collidingItems(const QGraphicsItem* item) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn colliding_items(&self, (*const ::graphics_item::GraphicsItem, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::collidingItems(const QGraphicsItem* item, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  pub unsafe fn colliding_items<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsItemMutPtr
    where Args: overloading::GraphicsSceneCollidingItemsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItemGroup* QGraphicsScene::createItemGroup(const QList<QGraphicsItem*>& items)```</span>
  ///
  ///
  pub fn create_item_group(&mut self,
                           items: &::list::ListGraphicsItemMutPtr)
                           -> *mut ::graphics_item_group::GraphicsItemGroup {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_createItemGroup(self as *mut ::graphics_scene::GraphicsScene,
                                                         items as *const ::list::ListGraphicsItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup* group)```</span>
  ///
  ///
  pub unsafe fn destroy_item_group(&mut self, group: *mut ::graphics_item_group::GraphicsItemGroup) {
    ::ffi::qt_widgets_c_QGraphicsScene_destroyItemGroup(self as *mut ::graphics_scene::GraphicsScene, group)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsScene::focusItem() const```</span>
  ///
  ///
  pub fn focus_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_focusItem(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QFont QGraphicsScene::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_font_to_output(self as *const ::graphics_scene::GraphicsScene, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QGraphicsScene::foregroundBrush() const```</span>
  ///
  ///
  pub fn foreground_brush(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_foregroundBrush_to_output(self as *const ::graphics_scene::GraphicsScene,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsScene::hasFocus() const```</span>
  ///
  ///
  pub fn has_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_hasFocus(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsScene::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_height(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QGraphicsScene::inputMethodQuery(Qt::InputMethodQuery query) const```</span>
  ///
  ///
  pub fn input_method_query(&self, query: &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_inputMethodQuery_to_output(self as *const ::graphics_scene::GraphicsScene, query as *const ::qt_core::qt::InputMethodQuery, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::invalidate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn invalidate(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::invalidate()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn invalidate(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::invalidate(const QRectF& rect = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn invalidate(&mut self, (&::qt_core::rect_f::RectF, ::qt_core::flags::Flags<::graphics_scene::SceneLayer>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::invalidate(const QRectF& rect = ?, QFlags<QGraphicsScene::SceneLayer> layers = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn invalidate(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::invalidate(double x, double y, double w, double h)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn invalidate(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::qt_core::flags::Flags<::graphics_scene::SceneLayer>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::invalidate(double x, double y, double w, double h, QFlags<QGraphicsScene::SceneLayer> layers = ?)```</span>
  ///
  ///
  pub fn invalidate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneInvalidateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QGraphicsScene::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_isActive(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsScene::isSortCacheEnabled() const```</span>
  ///
  ///
  pub fn is_sort_cache_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_isSortCacheEnabled(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, (&::qt_core::point_f::PointF, &::qt_gui::transform::Transform)) -> *mut ::graphics_item::GraphicsItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsScene::itemAt(const QPointF& pos, const QTransform& deviceTransform) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_double, ::libc::c_double, &::qt_gui::transform::Transform)) -> *mut ::graphics_item::GraphicsItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsScene::itemAt(double x, double y, const QTransform& deviceTransform) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::graphics_item::GraphicsItem
    where Args: overloading::GraphicsSceneItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::ItemIndexMethod QGraphicsScene::itemIndexMethod() const```</span>
  ///
  ///
  pub fn item_index_method(&self) -> ::graphics_scene::ItemIndexMethod {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_itemIndexMethod(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::items```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn items(&self, ()) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn items(&self, &::qt_core::qt::SortOrder) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(Qt::SortOrder order = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn items(&self, &::qt_gui::painter_path::PainterPath) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPainterPath& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPainterPath& path, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPainterPath& path, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder, &::qt_gui::transform::Transform)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPainterPath& path, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?, const QTransform& deviceTransform = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn items(&self, &::qt_core::point_f::PointF) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPointF& pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::point_f::PointF, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPointF& pos, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::point_f::PointF, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPointF& pos, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::point_f::PointF, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder, &::qt_gui::transform::Transform)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPointF& pos, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?, const QTransform& deviceTransform = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn items(&self, &::qt_gui::polygon_f::PolygonF) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPolygonF& polygon) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::polygon_f::PolygonF, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPolygonF& polygon, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::polygon_f::PolygonF, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPolygonF& polygon, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_gui::polygon_f::PolygonF, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder, &::qt_gui::transform::Transform)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QPolygonF& polygon, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?, const QTransform& deviceTransform = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn items(&self, &::qt_core::rect_f::RectF) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QRectF& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::rect_f::RectF, &::qt_core::qt::ItemSelectionMode)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QRectF& rect, Qt::ItemSelectionMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::rect_f::RectF, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QRectF& rect, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn items(&self, (&::qt_core::rect_f::RectF, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder, &::qt_gui::transform::Transform)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(const QRectF& rect, Qt::ItemSelectionMode mode = ?, Qt::SortOrder order = ?, const QTransform& deviceTransform = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn items(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(double x, double y, double w, double h, Qt::ItemSelectionMode mode, Qt::SortOrder order) const```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn items(&self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_core::qt::ItemSelectionMode, &::qt_core::qt::SortOrder, &::qt_gui::transform::Transform)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::items(double x, double y, double w, double h, Qt::ItemSelectionMode mode, Qt::SortOrder order, const QTransform& deviceTransform = ?) const```</span>
  ///
  ///
  pub fn items<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsItemMutPtr
    where Args: overloading::GraphicsSceneItemsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsScene::itemsBoundingRect() const```</span>
  ///
  ///
  pub fn items_bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_itemsBoundingRect_to_output(self as *const ::graphics_scene::GraphicsScene, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsScene::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_metaObject(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsScene::minimumRenderSize() const```</span>
  ///
  ///
  pub fn minimum_render_size(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_minimumRenderSize(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QGraphicsScene::mouseGrabberItem() const```</span>
  ///
  ///
  pub fn mouse_grabber_item(&self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_mouseGrabberItem(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::QGraphicsScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScene::QGraphicsScene()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect_f::RectF) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScene::QGraphicsScene(const QRectF& sceneRect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScene::QGraphicsScene(double x, double y, double width, double height)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>
    where Args: overloading::GraphicsSceneNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::QGraphicsScene```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScene::QGraphicsScene(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::rect_f::RectF, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScene::QGraphicsScene(const QRectF& sceneRect, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScene::QGraphicsScene(double x, double y, double width, double height, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>
    where Args: overloading::GraphicsSceneNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPalette QGraphicsScene::palette() const```</span>
  ///
  ///
  pub fn palette(&self) -> ::qt_gui::palette::Palette {
    {
      let mut object: ::qt_gui::palette::Palette =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_palette_to_output(self as *const ::graphics_scene::GraphicsScene,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsScene::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsScene_qt_metacall(self as *mut ::graphics_scene::GraphicsScene,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsScene::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsScene_qt_metacast(self as *mut ::graphics_scene::GraphicsScene, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::removeItem(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn remove_item(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsScene_removeItem(self as *mut ::graphics_scene::GraphicsScene, item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::render```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn render(&mut self, *mut ::qt_gui::painter::Painter) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::render(QPainter* painter)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::render(QPainter* painter, const QRectF& target = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect_f::RectF, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::render(QPainter* painter, const QRectF& target = ?, const QRectF& source = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn render(&mut self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect_f::RectF, &::qt_core::rect_f::RectF, &::qt_core::qt::AspectRatioMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::render(QPainter* painter, const QRectF& target = ?, const QRectF& source = ?, Qt::AspectRatioMode aspectRatioMode = ?)```</span>
  ///
  ///
  pub unsafe fn render<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneRenderArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsScene::sceneRect() const```</span>
  ///
  ///
  pub fn scene_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_sceneRect_to_output(self as *const ::graphics_scene::GraphicsScene,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QGraphicsScene::selectedItems() const```</span>
  ///
  ///
  pub fn selected_items(&self) -> ::list::ListGraphicsItemMutPtr {
    {
      let mut object: ::list::ListGraphicsItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_selectedItems_to_output(self as *const ::graphics_scene::GraphicsScene,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsScene::selectionArea() const```</span>
  ///
  ///
  pub fn selection_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_selectionArea_to_output(self as *const ::graphics_scene::GraphicsScene,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsScene::sendEvent(QGraphicsItem* item, QEvent* event)```</span>
  ///
  ///
  pub unsafe fn send_event(&mut self,
                           item: *mut ::graphics_item::GraphicsItem,
                           event: *mut ::qt_core::event::Event)
                           -> bool {
    ::ffi::qt_widgets_c_QGraphicsScene_sendEvent(self as *mut ::graphics_scene::GraphicsScene, item, event)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setActivePanel(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn set_active_panel(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsScene_setActivePanel(self as *mut ::graphics_scene::GraphicsScene, item)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setActiveWindow(QGraphicsWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_active_window(&mut self, widget: *mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QGraphicsScene_setActiveWindow(self as *mut ::graphics_scene::GraphicsScene, widget)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setBackgroundBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background_brush(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setBackgroundBrush(self as *mut ::graphics_scene::GraphicsScene,
                                                            brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setBspTreeDepth(int depth)```</span>
  ///
  ///
  pub fn set_bsp_tree_depth(&mut self, depth: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setBspTreeDepth(self as *mut ::graphics_scene::GraphicsScene, depth) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::setFocus```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_focus(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setFocus()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_focus(&mut self, &::qt_core::qt::FocusReason) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setFocus(Qt::FocusReason focusReason = ?)```</span>
  ///
  ///
  pub fn set_focus<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneSetFocusArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::setFocusItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_focus_item(&mut self, *mut ::graphics_item::GraphicsItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setFocusItem(QGraphicsItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_focus_item(&mut self, (*mut ::graphics_item::GraphicsItem, &::qt_core::qt::FocusReason)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setFocusItem(QGraphicsItem* item, Qt::FocusReason focusReason = ?)```</span>
  ///
  ///
  pub unsafe fn set_focus_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneSetFocusItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setFont(self as *mut ::graphics_scene::GraphicsScene,
                                                 font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setForegroundBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground_brush(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setForegroundBrush(self as *mut ::graphics_scene::GraphicsScene,
                                                            brush as *const ::qt_gui::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setItemIndexMethod(QGraphicsScene::ItemIndexMethod method)```</span>
  ///
  ///
  pub fn set_item_index_method(&mut self, method: ::graphics_scene::ItemIndexMethod) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setItemIndexMethod(self as *mut ::graphics_scene::GraphicsScene, method)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setMinimumRenderSize(double minSize)```</span>
  ///
  ///
  pub fn set_minimum_render_size(&mut self, min_size: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setMinimumRenderSize(self as *mut ::graphics_scene::GraphicsScene, min_size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setPalette(const QPalette& palette)```</span>
  ///
  ///
  pub fn set_palette(&mut self, palette: &::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setPalette(self as *mut ::graphics_scene::GraphicsScene,
                                                    palette as *const ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::setSceneRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_scene_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSceneRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_scene_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSceneRect(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn set_scene_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneSetSceneRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsScene::setSelectionArea```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, &::qt_gui::painter_path::PainterPath) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path, Qt::ItemSelectionMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionMode, &::qt_gui::transform::Transform)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path, Qt::ItemSelectionMode mode = ?, const QTransform& deviceTransform = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionOperation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path, Qt::ItemSelectionOperation selectionOperation)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionOperation, &::qt_core::qt::ItemSelectionMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path, Qt::ItemSelectionOperation selectionOperation, Qt::ItemSelectionMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_core::qt::ItemSelectionOperation, &::qt_core::qt::ItemSelectionMode, &::qt_gui::transform::Transform)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path, Qt::ItemSelectionOperation selectionOperation, Qt::ItemSelectionMode mode = ?, const QTransform& deviceTransform = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_selection_area(&mut self, (&::qt_gui::painter_path::PainterPath, &::qt_gui::transform::Transform)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSelectionArea(const QPainterPath& path, const QTransform& deviceTransform)```</span>
  ///
  ///
  pub fn set_selection_area<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneSetSelectionAreaArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setSortCacheEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_sort_cache_enabled(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScene_setSortCacheEnabled(self as *mut ::graphics_scene::GraphicsScene, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setStickyFocus(bool enabled)```</span>
  ///
  ///
  pub fn set_sticky_focus(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setStickyFocus(self as *mut ::graphics_scene::GraphicsScene, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScene::setStyle(QStyle* style)```</span>
  ///
  ///
  pub unsafe fn set_style(&mut self, style: *mut ::style::Style) {
    ::ffi::qt_widgets_c_QGraphicsScene_setStyle(self as *mut ::graphics_scene::GraphicsScene, style)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsScene::stickyFocus() const```</span>
  ///
  ///
  pub fn sticky_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_stickyFocus(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```QStyle* QGraphicsScene::style() const```</span>
  ///
  ///
  pub fn style(&self) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_style(self as *const ::graphics_scene::GraphicsScene) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsScene::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsScene_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsScene::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsScene_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsScene::update```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn update(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::update()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn update(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsScene::update(const QRectF& rect = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn update(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsScene::update(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn update<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsSceneUpdateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*> QGraphicsScene::views() const```</span>
  ///
  ///
  pub fn views(&self) -> ::list::ListGraphicsViewMutPtr {
    {
      let mut object: ::list::ListGraphicsViewMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_views_to_output(self as *const ::graphics_scene::GraphicsScene, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsScene::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScene_width(self as *const ::graphics_scene::GraphicsScene) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene::GraphicsScene {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsScene_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsScene`.
  pub struct Signals<'a>(&'a ::graphics_scene::GraphicsScene);
  /// Represents a built-in Qt signal `QGraphicsScene::focusItemChanged`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.signals().focus_item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct FocusItemChanged<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for FocusItemChanged<'a> {
    type Arguments = (*mut ::graphics_item::GraphicsItem,
     *mut ::graphics_item::GraphicsItem,
     &'static ::qt_core::qt::FocusReason);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusItemChanged(QGraphicsItem*,QGraphicsItem*,Qt::FocusReason)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusItemChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScene::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_scene::GraphicsScene);
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
  /// Represents a built-in Qt signal `QGraphicsScene::sceneRectChanged`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.signals().scene_rect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct SceneRectChanged<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for SceneRectChanged<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sceneRectChanged(const QRectF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SceneRectChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScene::selectionChanged`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct SelectionChanged<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectionChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScene::changed`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct Changed<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for Changed<'a> {
    type Arguments = (&'static ::list::ListQtCoreRectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2changed(const QList< QRectF >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Changed<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsScene::focusItemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_item_changed(&self) -> FocusItemChanged {
      FocusItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScene::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScene::sceneRectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scene_rect_changed(&self) -> SceneRectChanged {
      SceneRectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScene::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScene::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsScene`.
  pub struct Slots<'a>(&'a ::graphics_scene::GraphicsScene);
  /// Represents a built-in Qt slot `QGraphicsScene::clear`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct Clear<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsScene::advance`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.slots().advance()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct Advance<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for Advance<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1advance()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsScene::focusNextPrevChild`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.slots().focus_next_prev_child()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct FocusNextPrevChild<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for FocusNextPrevChild<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1focusNextPrevChild(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsScene::update`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct Update<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update(const QRectF&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsScene::clearSelection`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.slots().clear_selection()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct ClearSelection<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for ClearSelection<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearSelection()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsScene::invalidate`.
  ///
  /// An object of this type can be created from `GraphicsScene` with `object.slots().invalidate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScene` object.
  pub struct Invalidate<'a>(&'a ::graphics_scene::GraphicsScene);
  impl<'a> ::qt_core::connection::Receiver for Invalidate<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF, ::qt_core::flags::Flags<::graphics_scene::SceneLayer>);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1invalidate(const QRectF&,QFlags< QGraphicsScene::SceneLayer >)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsScene::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsScene::advance`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn advance(&self) -> Advance {
      Advance(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsScene::focusNextPrevChild`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_next_prev_child(&self) -> FocusNextPrevChild {
      FocusNextPrevChild(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsScene::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsScene::clearSelection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_selection(&self) -> ClearSelection {
      ClearSelection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsScene::invalidate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn invalidate(&self) -> Invalidate {
      Invalidate(self.0)
    }
  }
  impl ::graphics_scene::GraphicsScene {
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

/// C++ type: <span style='color: green;'>```QGraphicsScene::ItemIndexMethod```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemIndexMethod {
  /// C++ enum variant: <span style='color: green;'>```NoIndex = -1```</span>
  No = -1,
  /// C++ enum variant: <span style='color: green;'>```BspTreeIndex = 0```</span>
  BspTree = 0,
}

/// C++ type: <span style='color: green;'>```QGraphicsScene::SceneLayer```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SceneLayer {
  /// C++ enum variant: <span style='color: green;'>```ItemLayer = 1```</span>
  ItemLayer = 1,
  /// C++ enum variant: <span style='color: green;'>```BackgroundLayer = 2```</span>
  BackgroundLayer = 2,
  /// C++ enum variant: <span style='color: green;'>```ForegroundLayer = 4```</span>
  ForegroundLayer = 4,
  /// C++ enum variant: <span style='color: green;'>```AllLayers = 65535```</span>
  AllLayers = 65535,
}

impl ::qt_core::flags::FlaggableEnum for SceneLayer {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "SceneLayer"
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_scene::GraphicsScene {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_G_static_cast_QObject_ptr(self as *mut ::graphics_scene::GraphicsScene)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScene_G_static_cast_QObject_ptr(self as *const ::graphics_scene::GraphicsScene as *mut ::graphics_scene::GraphicsScene) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene::GraphicsScene> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene::GraphicsScene {
    let ffi_result =
      ::ffi::qt_widgets_c_QGraphicsScene_G_static_cast_QGraphicsScene_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_scene::GraphicsScene {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsScene_G_static_cast_QGraphicsScene_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_scene::GraphicsScene {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScene_G_static_cast_QObject_ptr(self as *const ::graphics_scene::GraphicsScene as *mut ::graphics_scene::GraphicsScene) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene::GraphicsScene {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_G_static_cast_QObject_ptr(self as *mut ::graphics_scene::GraphicsScene)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_ellipse](../struct.GraphicsScene.html#method.add_ellipse) method.
  pub trait GraphicsSceneAddEllipseArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem;
  }
  impl<'largs> GraphicsSceneAddEllipseArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addEllipse_rect(original_self as *mut ::graphics_scene::GraphicsScene,
                                                           rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsSceneAddEllipseArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem {
      let rect = self.0;
      let pen = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addEllipse_rect_pen(original_self as *mut ::graphics_scene::GraphicsScene,
                                                               rect as *const ::qt_core::rect_f::RectF,
                                                               pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddEllipseArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_gui::pen::Pen, &'largs ::qt_gui::brush::Brush) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem {
      let rect = self.0;
      let pen = self.1;
      let brush = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_addEllipse_rect_pen_brush(original_self as *mut ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, pen as *const ::qt_gui::pen::Pen, brush as *const ::qt_gui::brush::Brush) }
    }
  }
  impl<'largs> GraphicsSceneAddEllipseArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addEllipse_x_y_w_h(original_self as *mut ::graphics_scene::GraphicsScene,
                                                              x,
                                                              y,
                                                              w,
                                                              h)
      }
    }
  }
  impl<'largs> GraphicsSceneAddEllipseArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let pen = self.4;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addEllipse_x_y_w_h_pen(original_self as *mut ::graphics_scene::GraphicsScene, x, y, w, h, pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddEllipseArgs<'largs>
    for (::libc::c_double,
                                                            ::libc::c_double,
                                                            ::libc::c_double,
                                                            ::libc::c_double,
                                                            &'largs ::qt_gui::pen::Pen,
                                                            &'largs ::qt_gui::brush::Brush) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_ellipse_item::GraphicsEllipseItem {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let pen = self.4;
      let brush = self.5;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_addEllipse_x_y_w_h_pen_brush(original_self as *mut ::graphics_scene::GraphicsScene, x, y, w, h, pen as *const ::qt_gui::pen::Pen, brush as *const ::qt_gui::brush::Brush) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_line](../struct.GraphicsScene.html#method.add_line) method.
  pub trait GraphicsSceneAddLineArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_line_item::GraphicsLineItem;
  }
  impl<'largs> GraphicsSceneAddLineArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_line_item::GraphicsLineItem {
      let line = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addLine_line(original_self as *mut ::graphics_scene::GraphicsScene,
                                                        line as *const ::qt_core::line_f::LineF)
      }
    }
  }
  impl<'largs> GraphicsSceneAddLineArgs<'largs> for (&'largs ::qt_core::line_f::LineF, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_line_item::GraphicsLineItem {
      let line = self.0;
      let pen = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addLine_line_pen(original_self as *mut ::graphics_scene::GraphicsScene,
                                                            line as *const ::qt_core::line_f::LineF,
                                                            pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddLineArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_line_item::GraphicsLineItem {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addLine_x1_y1_x2_y2(original_self as *mut ::graphics_scene::GraphicsScene,
                                                               x1,
                                                               y1,
                                                               x2,
                                                               y2)
      }
    }
  }
  impl<'largs> GraphicsSceneAddLineArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_line_item::GraphicsLineItem {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      let pen = self.4;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_addLine_x1_y1_x2_y2_pen(original_self as *mut ::graphics_scene::GraphicsScene, x1, y1, x2, y2, pen as *const ::qt_gui::pen::Pen) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_path](../struct.GraphicsScene.html#method.add_path) method.
  pub trait GraphicsSceneAddPathArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_path_item::GraphicsPathItem;
  }
  impl<'largs> GraphicsSceneAddPathArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_path_item::GraphicsPathItem {
      let path = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addPath_path(original_self as *mut ::graphics_scene::GraphicsScene,
                                                        path as *const ::qt_gui::painter_path::PainterPath)
      }
    }
  }
  impl<'largs> GraphicsSceneAddPathArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_path_item::GraphicsPathItem {
      let path = self.0;
      let pen = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addPath_path_pen(original_self as *mut ::graphics_scene::GraphicsScene,
                                                            path as *const ::qt_gui::painter_path::PainterPath,
                                                            pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddPathArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_gui::pen::Pen, &'largs ::qt_gui::brush::Brush) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_path_item::GraphicsPathItem {
      let path = self.0;
      let pen = self.1;
      let brush = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addPath_path_pen_brush(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, pen as *const ::qt_gui::pen::Pen, brush as *const ::qt_gui::brush::Brush)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_polygon](../struct.GraphicsScene.html#method.add_polygon) method.
  pub trait GraphicsSceneAddPolygonArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_polygon_item::GraphicsPolygonItem;
  }
  impl<'largs> GraphicsSceneAddPolygonArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_polygon_item::GraphicsPolygonItem {
      let polygon = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addPolygon_polygon(original_self as *mut ::graphics_scene::GraphicsScene,
                                                              polygon as *const ::qt_gui::polygon_f::PolygonF)
      }
    }
  }
  impl<'largs> GraphicsSceneAddPolygonArgs<'largs>
    for (&'largs ::qt_gui::polygon_f::PolygonF, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_polygon_item::GraphicsPolygonItem {
      let polygon = self.0;
      let pen = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addPolygon_polygon_pen(original_self as *mut ::graphics_scene::GraphicsScene, polygon as *const ::qt_gui::polygon_f::PolygonF, pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddPolygonArgs<'largs>
    for (&'largs ::qt_gui::polygon_f::PolygonF, &'largs ::qt_gui::pen::Pen, &'largs ::qt_gui::brush::Brush) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_polygon_item::GraphicsPolygonItem {
      let polygon = self.0;
      let pen = self.1;
      let brush = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_addPolygon_polygon_pen_brush(original_self as *mut ::graphics_scene::GraphicsScene, polygon as *const ::qt_gui::polygon_f::PolygonF, pen as *const ::qt_gui::pen::Pen, brush as *const ::qt_gui::brush::Brush) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_rect](../struct.GraphicsScene.html#method.add_rect) method.
  pub trait GraphicsSceneAddRectArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem;
  }
  impl<'largs> GraphicsSceneAddRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addRect_rect(original_self as *mut ::graphics_scene::GraphicsScene,
                                                        rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsSceneAddRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem {
      let rect = self.0;
      let pen = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addRect_rect_pen(original_self as *mut ::graphics_scene::GraphicsScene,
                                                            rect as *const ::qt_core::rect_f::RectF,
                                                            pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_gui::pen::Pen, &'largs ::qt_gui::brush::Brush) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem {
      let rect = self.0;
      let pen = self.1;
      let brush = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addRect_rect_pen_brush(original_self as *mut ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, pen as *const ::qt_gui::pen::Pen, brush as *const ::qt_gui::brush::Brush)
      }
    }
  }
  impl<'largs> GraphicsSceneAddRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addRect_x_y_w_h(original_self as *mut ::graphics_scene::GraphicsScene,
                                                           x,
                                                           y,
                                                           w,
                                                           h)
      }
    }
  }
  impl<'largs> GraphicsSceneAddRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &'largs ::qt_gui::pen::Pen) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let pen = self.4;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addRect_x_y_w_h_pen(original_self as *mut ::graphics_scene::GraphicsScene,
                                                               x,
                                                               y,
                                                               w,
                                                               h,
                                                               pen as *const ::qt_gui::pen::Pen)
      }
    }
  }
  impl<'largs> GraphicsSceneAddRectArgs<'largs>
    for (::libc::c_double,
                                                         ::libc::c_double,
                                                         ::libc::c_double,
                                                         ::libc::c_double,
                                                         &'largs ::qt_gui::pen::Pen,
                                                         &'largs ::qt_gui::brush::Brush) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_rect_item::GraphicsRectItem {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let pen = self.4;
      let brush = self.5;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_addRect_x_y_w_h_pen_brush(original_self as *mut ::graphics_scene::GraphicsScene, x, y, w, h, pen as *const ::qt_gui::pen::Pen, brush as *const ::qt_gui::brush::Brush) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_simple_text](../struct.GraphicsScene.html#method.add_simple_text) method.
  pub trait GraphicsSceneAddSimpleTextArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_simple_text_item::GraphicsSimpleTextItem;
  }
  impl<'largs> GraphicsSceneAddSimpleTextArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_simple_text_item::GraphicsSimpleTextItem {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addSimpleText_text(original_self as *mut ::graphics_scene::GraphicsScene,
                                                              text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> GraphicsSceneAddSimpleTextArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_gui::font::Font) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_simple_text_item::GraphicsSimpleTextItem {
      let text = self.0;
      let font = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_addSimpleText_text_font(original_self as *mut ::graphics_scene::GraphicsScene, text as *const ::qt_core::string::String, font as *const ::qt_gui::font::Font) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::add_text](../struct.GraphicsScene.html#method.add_text) method.
  pub trait GraphicsSceneAddTextArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_text_item::GraphicsTextItem;
  }
  impl<'largs> GraphicsSceneAddTextArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_text_item::GraphicsTextItem {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addText_text(original_self as *mut ::graphics_scene::GraphicsScene,
                                                        text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> GraphicsSceneAddTextArgs<'largs> for (&'largs ::qt_core::string::String, &'largs ::qt_gui::font::Font) {
    fn exec(self,
            original_self: &'largs mut ::graphics_scene::GraphicsScene)
            -> *mut ::graphics_text_item::GraphicsTextItem {
      let text = self.0;
      let font = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_addText_text_font(original_self as *mut ::graphics_scene::GraphicsScene,
                                                             text as *const ::qt_core::string::String,
                                                             font as *const ::qt_gui::font::Font)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::colliding_items](../struct.GraphicsScene.html#method.colliding_items) method.
  pub trait GraphicsSceneCollidingItemsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr;
  }
  impl<'largs> GraphicsSceneCollidingItemsArgs<'largs> for *const ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let item = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsScene_collidingItems_to_output_item(original_self as *const ::graphics_scene::GraphicsScene, item, &mut object);
        object
      }
    }
  }
  impl<'largs> GraphicsSceneCollidingItemsArgs<'largs>
    for (*const ::graphics_item::GraphicsItem, &'largs ::qt_core::qt::ItemSelectionMode) {
    unsafe fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let item = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGraphicsScene_collidingItems_to_output_item_mode(original_self as *const ::graphics_scene::GraphicsScene, item, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::invalidate](../struct.GraphicsScene.html#method.invalidate) method.
  pub trait GraphicsSceneInvalidateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneInvalidateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_invalidate_no_args(original_self as *mut ::graphics_scene::GraphicsScene)
      }
    }
  }
  impl<'largs> GraphicsSceneInvalidateArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_invalidate_rect(original_self as *mut ::graphics_scene::GraphicsScene,
                                                           rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsSceneInvalidateArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::qt_core::flags::Flags<::graphics_scene::SceneLayer>) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let rect = self.0;
      let layers = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_invalidate_rect_layers(original_self as *mut ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, layers.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> GraphicsSceneInvalidateArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_invalidate_x_y_w_h(original_self as *mut ::graphics_scene::GraphicsScene,
                                                              x,
                                                              y,
                                                              w,
                                                              h)
      }
    }
  }
  impl<'largs> GraphicsSceneInvalidateArgs<'largs>
    for (::libc::c_double,
                                                            ::libc::c_double,
                                                            ::libc::c_double,
                                                            ::libc::c_double,
                                                            ::qt_core::flags::Flags<::graphics_scene::SceneLayer>) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let layers = self.4;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_invalidate_x_y_w_h_layers(original_self as *mut ::graphics_scene::GraphicsScene, x, y, w, h, layers.to_int() as ::libc::c_uint) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::item_at](../struct.GraphicsScene.html#method.item_at) method.
  pub trait GraphicsSceneItemAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> *mut ::graphics_item::GraphicsItem;
  }
  impl<'largs> GraphicsSceneItemAtArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> *mut ::graphics_item::GraphicsItem {
      let pos = self.0;
      let device_transform = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_itemAt_pos_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, pos as *const ::qt_core::point_f::PointF, device_transform as *const ::qt_gui::transform::Transform) }
    }
  }
  impl<'largs> GraphicsSceneItemAtArgs<'largs>
    for (::libc::c_double, ::libc::c_double, &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> *mut ::graphics_item::GraphicsItem {
      let x = self.0;
      let y = self.1;
      let device_transform = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_itemAt_x_y_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, x, y, device_transform as *const ::qt_gui::transform::Transform) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::items](../struct.GraphicsScene.html#method.items) method.
  pub trait GraphicsSceneItemsArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr;
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {

      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_no_args(original_self as *const ::graphics_scene::GraphicsScene, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs> for &'largs ::qt_core::qt::SortOrder {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let order = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_order(original_self as *const ::graphics_scene::GraphicsScene, order as *const ::qt_core::qt::SortOrder, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let path = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_path(original_self as *const ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let path = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_path_mode(original_self as *const ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let path = self.0;
      let mode = self.1;
      let order = self.2;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_path_mode_order(original_self as *const ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder,
                                                       &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let path = self.0;
      let mode = self.1;
      let order = self.2;
      let device_transform = self.3;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_path_mode_order_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, device_transform as *const ::qt_gui::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs> for &'largs ::qt_gui::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let polygon = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_polygon(original_self as *const ::graphics_scene::GraphicsScene, polygon as *const ::qt_gui::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_gui::polygon_f::PolygonF, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let polygon = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_polygon_mode(original_self as *const ::graphics_scene::GraphicsScene, polygon as *const ::qt_gui::polygon_f::PolygonF, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_gui::polygon_f::PolygonF,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let polygon = self.0;
      let mode = self.1;
      let order = self.2;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_polygon_mode_order(original_self as *const ::graphics_scene::GraphicsScene, polygon as *const ::qt_gui::polygon_f::PolygonF, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_gui::polygon_f::PolygonF,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder,
                                                       &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let polygon = self.0;
      let mode = self.1;
      let order = self.2;
      let device_transform = self.3;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_polygon_mode_order_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, polygon as *const ::qt_gui::polygon_f::PolygonF, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, device_transform as *const ::qt_gui::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_pos(original_self as *const ::graphics_scene::GraphicsScene, pos as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let pos = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_pos_mode(original_self as *const ::graphics_scene::GraphicsScene, pos as *const ::qt_core::point_f::PointF, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let pos = self.0;
      let mode = self.1;
      let order = self.2;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_pos_mode_order(original_self as *const ::graphics_scene::GraphicsScene, pos as *const ::qt_core::point_f::PointF, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder,
                                                       &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let pos = self.0;
      let mode = self.1;
      let order = self.2;
      let device_transform = self.3;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_pos_mode_order_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, pos as *const ::qt_core::point_f::PointF, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, device_transform as *const ::qt_gui::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let rect = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_rect(original_self as *const ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let rect = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_rect_mode(original_self as *const ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, mode as *const ::qt_core::qt::ItemSelectionMode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::qt::ItemSelectionMode, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let rect = self.0;
      let mode = self.1;
      let order = self.2;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_rect_mode_order(original_self as *const ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder,
                                                       &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let rect = self.0;
      let mode = self.1;
      let order = self.2;
      let device_transform = self.3;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_rect_mode_order_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, rect as *const ::qt_core::rect_f::RectF, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, device_transform as *const ::qt_gui::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (::libc::c_double,
                                                       ::libc::c_double,
                                                       ::libc::c_double,
                                                       ::libc::c_double,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let mode = self.4;
      let order = self.5;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_x_y_w_h_mode_order(original_self as *const ::graphics_scene::GraphicsScene, x, y, w, h, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsSceneItemsArgs<'largs>
    for (::libc::c_double,
                                                       ::libc::c_double,
                                                       ::libc::c_double,
                                                       ::libc::c_double,
                                                       &'largs ::qt_core::qt::ItemSelectionMode,
                                                       &'largs ::qt_core::qt::SortOrder,
                                                       &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs ::graphics_scene::GraphicsScene) -> ::list::ListGraphicsItemMutPtr {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let mode = self.4;
      let order = self.5;
      let device_transform = self.6;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsScene_items_to_output_x_y_w_h_mode_order_deviceTransform(original_self as *const ::graphics_scene::GraphicsScene, x, y, w, h, mode as *const ::qt_core::qt::ItemSelectionMode, order as *const ::qt_core::qt::SortOrder, device_transform as *const ::qt_gui::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::new](../struct.GraphicsScene.html#method.new) method.
  pub trait GraphicsSceneNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>;
  }
  impl GraphicsSceneNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScene_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsSceneNewArgs for &'a ::qt_core::rect_f::RectF {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene> {
      let scene_rect = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsScene_new_sceneRect(scene_rect as *const ::qt_core::rect_f::RectF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsSceneNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene> {
      let x = self.0;
      let y = self.1;
      let width = self.2;
      let height = self.3;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScene_new_x_y_width_height(x, y, width, height) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::new_unsafe](../struct.GraphicsScene.html#method.new_unsafe) method.
  pub trait GraphicsSceneNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene>;
  }
  impl GraphicsSceneNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsScene_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsSceneNewUnsafeArgs for (&'a ::qt_core::rect_f::RectF, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene> {
      let scene_rect = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsScene_new_sceneRect_parent(scene_rect as *const ::qt_core::rect_f::RectF, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsSceneNewUnsafeArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene::GraphicsScene> {
      let x = self.0;
      let y = self.1;
      let width = self.2;
      let height = self.3;
      let parent = self.4;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsScene_new_x_y_width_height_parent(x, y, width, height, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::render](../struct.GraphicsScene.html#method.render) method.
  pub trait GraphicsSceneRenderArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneRenderArgs<'largs> for *mut ::qt_gui::painter::Painter {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let painter = self;
      ::ffi::qt_widgets_c_QGraphicsScene_render_painter(original_self as *mut ::graphics_scene::GraphicsScene,
                                                        painter)
    }
  }
  impl<'largs> GraphicsSceneRenderArgs<'largs> for (*mut ::qt_gui::painter::Painter, &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let painter = self.0;
      let target = self.1;
      ::ffi::qt_widgets_c_QGraphicsScene_render_painter_target(original_self as *mut ::graphics_scene::GraphicsScene,
                                                               painter,
                                                               target as *const ::qt_core::rect_f::RectF)
    }
  }
  impl<'largs> GraphicsSceneRenderArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, &'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let painter = self.0;
      let target = self.1;
      let source = self.2;
      ::ffi::qt_widgets_c_QGraphicsScene_render_painter_target_source(original_self as *mut ::graphics_scene::GraphicsScene, painter, target as *const ::qt_core::rect_f::RectF, source as *const ::qt_core::rect_f::RectF)
    }
  }
  impl<'largs> GraphicsSceneRenderArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                        &'largs ::qt_core::rect_f::RectF,
                                                        &'largs ::qt_core::rect_f::RectF,
                                                        &'largs ::qt_core::qt::AspectRatioMode) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let painter = self.0;
      let target = self.1;
      let source = self.2;
      let aspect_ratio_mode = self.3;
      ::ffi::qt_widgets_c_QGraphicsScene_render_painter_target_source_aspectRatioMode(original_self as *mut ::graphics_scene::GraphicsScene, painter, target as *const ::qt_core::rect_f::RectF, source as *const ::qt_core::rect_f::RectF, aspect_ratio_mode as *const ::qt_core::qt::AspectRatioMode)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::set_focus](../struct.GraphicsScene.html#method.set_focus) method.
  pub trait GraphicsSceneSetFocusArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneSetFocusArgs<'largs> for &'largs ::qt_core::qt::FocusReason {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let focus_reason = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_setFocus_focusReason(original_self as *mut ::graphics_scene::GraphicsScene,
                                                                focus_reason as *const ::qt_core::qt::FocusReason)
      }
    }
  }
  impl<'largs> GraphicsSceneSetFocusArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_setFocus_no_args(original_self as *mut ::graphics_scene::GraphicsScene)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::set_focus_item](../struct.GraphicsScene.html#method.set_focus_item) method.
  pub trait GraphicsSceneSetFocusItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneSetFocusItemArgs<'largs> for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QGraphicsScene_setFocusItem_item(original_self as *mut ::graphics_scene::GraphicsScene, item)
    }
  }
  impl<'largs> GraphicsSceneSetFocusItemArgs<'largs>
    for (*mut ::graphics_item::GraphicsItem, &'largs ::qt_core::qt::FocusReason) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let item = self.0;
      let focus_reason = self.1;
      ::ffi::qt_widgets_c_QGraphicsScene_setFocusItem_item_focusReason(original_self as *mut ::graphics_scene::GraphicsScene, item, focus_reason as *const ::qt_core::qt::FocusReason)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::set_scene_rect](../struct.GraphicsScene.html#method.set_scene_rect) method.
  pub trait GraphicsSceneSetSceneRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneSetSceneRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_setSceneRect_rect(original_self as *mut ::graphics_scene::GraphicsScene,
                                                             rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsSceneSetSceneRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_setSceneRect_x_y_w_h(original_self as *mut ::graphics_scene::GraphicsScene,
                                                                x,
                                                                y,
                                                                w,
                                                                h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::set_selection_area](../struct.GraphicsScene.html#method.set_selection_area) method.
  pub trait GraphicsSceneSetSelectionAreaArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs> for &'largs ::qt_gui::painter_path::PainterPath {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath)
      }
    }
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self.0;
      let device_transform = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path_deviceTransform(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, device_transform as *const ::qt_gui::transform::Transform) }
    }
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self.0;
      let mode = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path_mode(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode) }
    }
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath,
                                                                  &'largs ::qt_core::qt::ItemSelectionMode,
                                                                  &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self.0;
      let mode = self.1;
      let device_transform = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path_mode_deviceTransform(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, mode as *const ::qt_core::qt::ItemSelectionMode, device_transform as *const ::qt_gui::transform::Transform) }
    }
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath, &'largs ::qt_core::qt::ItemSelectionOperation) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self.0;
      let selection_operation = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path_selectionOperation(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, selection_operation as *const ::qt_core::qt::ItemSelectionOperation) }
    }
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath,
                                                                  &'largs ::qt_core::qt::ItemSelectionOperation,
                                                                  &'largs ::qt_core::qt::ItemSelectionMode) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self.0;
      let selection_operation = self.1;
      let mode = self.2;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path_selectionOperation_mode(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, selection_operation as *const ::qt_core::qt::ItemSelectionOperation, mode as *const ::qt_core::qt::ItemSelectionMode) }
    }
  }
  impl<'largs> GraphicsSceneSetSelectionAreaArgs<'largs>
    for (&'largs ::qt_gui::painter_path::PainterPath,
                                                                  &'largs ::qt_core::qt::ItemSelectionOperation,
                                                                  &'largs ::qt_core::qt::ItemSelectionMode,
                                                                  &'largs ::qt_gui::transform::Transform) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let path = self.0;
      let selection_operation = self.1;
      let mode = self.2;
      let device_transform = self.3;
      unsafe { ::ffi::qt_widgets_c_QGraphicsScene_setSelectionArea_path_selectionOperation_mode_deviceTransform(original_self as *mut ::graphics_scene::GraphicsScene, path as *const ::qt_gui::painter_path::PainterPath, selection_operation as *const ::qt_core::qt::ItemSelectionOperation, mode as *const ::qt_core::qt::ItemSelectionMode, device_transform as *const ::qt_gui::transform::Transform) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsScene::update](../struct.GraphicsScene.html#method.update) method.
  pub trait GraphicsSceneUpdateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> ();
  }
  impl<'largs> GraphicsSceneUpdateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_update_no_args(original_self as *mut ::graphics_scene::GraphicsScene)
      }
    }
  }
  impl<'largs> GraphicsSceneUpdateArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_update_rect(original_self as *mut ::graphics_scene::GraphicsScene,
                                                       rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsSceneUpdateArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_scene::GraphicsScene) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScene_update_x_y_w_h(original_self as *mut ::graphics_scene::GraphicsScene,
                                                          x,
                                                          y,
                                                          w,
                                                          h)
      }
    }
  }
}
