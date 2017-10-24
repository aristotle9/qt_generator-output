/// C++ type: <span style='color: green;'>```QTileRules```</span>
#[repr(C)]
pub struct TileRules(u8);

impl TileRules {
  /// C++ method: <span style='color: green;'>```const Qt::TileRule& QTileRules::horizontal() const```</span>
  ///
  ///
  pub fn horizontal<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::TileRule {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_horizontal(self as *const ::tile_rules::TileRules) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::TileRule& QTileRules::horizontal_mut()```</span>
  ///
  ///
  pub fn horizontal_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::TileRule {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_horizontal_mut(self as *mut ::tile_rules::TileRules) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTileRules::QTileRules```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::tile_rules::TileRules>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTileRules::QTileRules()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::qt::TileRule, &::qt_core::qt::TileRule)) -> ::cpp_utils::CppBox<::tile_rules::TileRules>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTileRules::QTileRules(Qt::TileRule horizontalRule, Qt::TileRule verticalRule)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::TileRule) -> ::cpp_utils::CppBox<::tile_rules::TileRules>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTileRules::QTileRules(Qt::TileRule rule = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::tile_rules::TileRules>
    where Args: overloading::TileRulesNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QTileRules::set_horizontal(Qt::TileRule value)```</span>
  ///
  ///
  pub fn set_horizontal(&mut self, value: &::qt_core::qt::TileRule) {
    unsafe {
      ::ffi::qt_widgets_c_QTileRules_set_horizontal(self as *mut ::tile_rules::TileRules,
                                                    value as *const ::qt_core::qt::TileRule)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTileRules::set_vertical(Qt::TileRule value)```</span>
  ///
  ///
  pub fn set_vertical(&mut self, value: &::qt_core::qt::TileRule) {
    unsafe {
      ::ffi::qt_widgets_c_QTileRules_set_vertical(self as *mut ::tile_rules::TileRules,
                                                  value as *const ::qt_core::qt::TileRule)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt::TileRule& QTileRules::vertical() const```</span>
  ///
  ///
  pub fn vertical<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::TileRule {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_vertical(self as *const ::tile_rules::TileRules) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::TileRule& QTileRules::vertical_mut()```</span>
  ///
  ///
  pub fn vertical_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::TileRule {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_vertical_mut(self as *mut ::tile_rules::TileRules) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::tile_rules::TileRules {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTileRules_delete
  }
}

/// C++ method: <span style='color: green;'>```void qDrawBorderPixmap(QPainter* painter, const QRect& target, const QMargins& margins, const QPixmap& pixmap)```</span>
///
///
pub unsafe fn draw_border_pixmap(painter: *mut ::qt_gui::painter::Painter,
                                 target: &::qt_core::rect::Rect,
                                 margins: &::qt_core::margins::Margins,
                                 pixmap: &::qt_gui::pixmap::Pixmap) {
  ::ffi::qt_widgets_c_QTileRules_G_qDrawBorderPixmap(painter,
                                                     target as *const ::qt_core::rect::Rect,
                                                     margins as *const ::qt_core::margins::Margins,
                                                     pixmap as *const ::qt_gui::pixmap::Pixmap)
}

/// C++ method: <span style='color: green;'>```qDrawPlainRect```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn draw_plain_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::color::Color)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawPlainRect(QPainter* p, const QRect& r, const QColor& arg3)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn draw_plain_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::color::Color, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawPlainRect(QPainter* p, const QRect& r, const QColor& arg3, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn draw_plain_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::color::Color, ::libc::c_int, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawPlainRect(QPainter* p, const QRect& r, const QColor& arg3, int lineWidth = ?, const QBrush* fill = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn draw_plain_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::color::Color)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawPlainRect(QPainter* p, int x, int y, int w, int h, const QColor& arg6)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn draw_plain_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::color::Color, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawPlainRect(QPainter* p, int x, int y, int w, int h, const QColor& arg6, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn draw_plain_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::color::Color, ::libc::c_int, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawPlainRect(QPainter* p, int x, int y, int w, int h, const QColor& arg6, int lineWidth = ?, const QBrush* fill = ?)```</span>
///
///
pub unsafe fn draw_plain_rect<Args>(args: Args) -> ()
  where Args: overloading::DrawPlainRectArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qDrawShadeLine```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, &::qt_core::point::Point, &::qt_core::point::Point, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, const QPoint& p1, const QPoint& p2, const QPalette& pal)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, &::qt_core::point::Point, &::qt_core::point::Point, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, const QPoint& p1, const QPoint& p2, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, &::qt_core::point::Point, &::qt_core::point::Point, &::qt_gui::palette::Palette, bool, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, const QPoint& p1, const QPoint& p2, const QPalette& pal, bool sunken = ?, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, &::qt_core::point::Point, &::qt_core::point::Point, &::qt_gui::palette::Palette, bool, ::libc::c_int, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, const QPoint& p1, const QPoint& p2, const QPalette& pal, bool sunken = ?, int lineWidth = ?, int midLineWidth = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, int x1, int y1, int x2, int y2, const QPalette& pal)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, int x1, int y1, int x2, int y2, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, int x1, int y1, int x2, int y2, const QPalette& pal, bool sunken = ?, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn draw_shade_line((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeLine(QPainter* p, int x1, int y1, int x2, int y2, const QPalette& pal, bool sunken = ?, int lineWidth = ?, int midLineWidth = ?)```</span>
///
///
pub unsafe fn draw_shade_line<Args>(args: Args) -> ()
  where Args: overloading::DrawShadeLineArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qDrawShadePanel```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, const QRect& r, const QPalette& pal)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, ::libc::c_int, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, int lineWidth = ?, const QBrush* fill = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn draw_shade_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadePanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, int lineWidth = ?, const QBrush* fill = ?)```</span>
///
///
pub unsafe fn draw_shade_panel<Args>(args: Args) -> ()
  where Args: overloading::DrawShadePanelArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qDrawShadeRect```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, const QRect& r, const QPalette& pal)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, ::libc::c_int, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, int lineWidth = ?, int midLineWidth = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, ::libc::c_int, ::libc::c_int, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, int lineWidth = ?, int midLineWidth = ?, const QBrush* fill = ?)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, int x, int y, int w, int h, const QPalette& pal)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, int lineWidth = ?)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int, ::libc::c_int)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, int lineWidth = ?, int midLineWidth = ?)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn draw_shade_rect((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, ::libc::c_int, ::libc::c_int, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawShadeRect(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, int lineWidth = ?, int midLineWidth = ?, const QBrush* fill = ?)```</span>
///
///
pub unsafe fn draw_shade_rect<Args>(args: Args) -> ()
  where Args: overloading::DrawShadeRectArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qDrawWinButton```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn draw_win_button((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinButton(QPainter* p, const QRect& r, const QPalette& pal)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn draw_win_button((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinButton(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn draw_win_button((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinButton(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, const QBrush* fill = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn draw_win_button((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinButton(QPainter* p, int x, int y, int w, int h, const QPalette& pal)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn draw_win_button((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinButton(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn draw_win_button((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinButton(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, const QBrush* fill = ?)```</span>
///
///
pub unsafe fn draw_win_button<Args>(args: Args) -> ()
  where Args: overloading::DrawWinButtonArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qDrawWinPanel```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn draw_win_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinPanel(QPainter* p, const QRect& r, const QPalette& pal)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn draw_win_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinPanel(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn draw_win_panel((*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, &::qt_gui::palette::Palette, bool, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinPanel(QPainter* p, const QRect& r, const QPalette& pal, bool sunken = ?, const QBrush* fill = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn draw_win_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinPanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn draw_win_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinPanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn draw_win_panel((*mut ::qt_gui::painter::Painter, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_gui::palette::Palette, bool, *const ::qt_gui::brush::Brush)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void qDrawWinPanel(QPainter* p, int x, int y, int w, int h, const QPalette& pal, bool sunken = ?, const QBrush* fill = ?)```</span>
///
///
pub unsafe fn draw_win_panel<Args>(args: Args) -> ()
  where Args: overloading::DrawWinPanelArgs
{
  args.exec()
}
pub mod draw_border_pixmap {
  /// C++ type: <span style='color: green;'>```QDrawBorderPixmap::DrawingHint```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum DrawingHint {
    /// C++ enum variant: <span style='color: green;'>```OpaqueTopLeft = 1```</span>
    TopLeft = 1,
    /// C++ enum variant: <span style='color: green;'>```OpaqueTop = 2```</span>
    Top = 2,
    /// C++ enum variant: <span style='color: green;'>```OpaqueTopRight = 4```</span>
    TopRight = 4,
    /// C++ enum variant: <span style='color: green;'>```OpaqueLeft = 8```</span>
    Left = 8,
    /// C++ enum variant: <span style='color: green;'>```OpaqueCenter = 16```</span>
    Center = 16,
    /// C++ enum variant: <span style='color: green;'>```OpaqueRight = 32```</span>
    Right = 32,
    /// C++ enum variant: <span style='color: green;'>```OpaqueBottomLeft = 64```</span>
    BottomLeft = 64,
    /// C++ enum variant: <span style='color: green;'>```OpaqueBottom = 128```</span>
    Bottom = 128,
    /// C++ enum variant: <span style='color: green;'>```OpaqueEdges = 170```</span>
    Edges = 170,
    /// C++ enum variant: <span style='color: green;'>```OpaqueBottomRight = 256```</span>
    BottomRight = 256,
    /// C++ enum variant: <span style='color: green;'>```OpaqueCorners = 325```</span>
    Corners = 325,
    /// C++ enum variant: <span style='color: green;'>```OpaqueFrame = 495```</span>
    Frame = 495,
    /// C++ enum variant: <span style='color: green;'>```OpaqueAll = 511```</span>
    All = 511,
  }

}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TileRules::new](../struct.TileRules.html#method.new) method.
  pub trait TileRulesNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::tile_rules::TileRules>;
  }
  impl<'a> TileRulesNewArgs for (&'a ::qt_core::qt::TileRule, &'a ::qt_core::qt::TileRule) {
    fn exec(self) -> ::cpp_utils::CppBox<::tile_rules::TileRules> {
      let horizontal_rule = self.0;
      let vertical_rule = self.1;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_new_horizontalRule_verticalRule(horizontal_rule as *const ::qt_core::qt::TileRule, vertical_rule as *const ::qt_core::qt::TileRule) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TileRulesNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::tile_rules::TileRules> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TileRulesNewArgs for &'a ::qt_core::qt::TileRule {
    fn exec(self) -> ::cpp_utils::CppBox<::tile_rules::TileRules> {
      let rule = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTileRules_new_rule(rule as *const ::qt_core::qt::TileRule) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [draw_plain_rect](../fn.draw_plain_rect.html) method.
  pub trait DrawPlainRectArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> DrawPlainRectArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::color::Color) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let arg3 = self.2;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawPlainRect_p_r_arg3(p,
                                                               r as *const ::qt_core::rect::Rect,
                                                               arg3 as *const ::qt_gui::color::Color)
    }
  }
  impl<'a> DrawPlainRectArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::color::Color, ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let arg3 = self.2;
      let line_width = self.3;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawPlainRect_p_r_arg3_lineWidth(p,
                                                                         r as *const ::qt_core::rect::Rect,
                                                                         arg3 as *const ::qt_gui::color::Color,
                                                                         line_width)
    }
  }
  impl<'a> DrawPlainRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::rect::Rect,
                                      &'a ::qt_gui::color::Color,
                                      ::libc::c_int,
                                      *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let arg3 = self.2;
      let line_width = self.3;
      let fill = self.4;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawPlainRect_p_r_arg3_lineWidth_fill(p,
                                                                              r as *const ::qt_core::rect::Rect,
                                                                              arg3 as *const ::qt_gui::color::Color,
                                                                              line_width,
                                                                              fill)
    }
  }
  impl<'a> DrawPlainRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::color::Color) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let arg6 = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawPlainRect_p_x_y_w_h_arg6(p,
                                                                     x,
                                                                     y,
                                                                     w,
                                                                     h,
                                                                     arg6 as *const ::qt_gui::color::Color)
    }
  }
  impl<'a> DrawPlainRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::color::Color,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let arg6 = self.5;
      let line_width = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawPlainRect_p_x_y_w_h_arg6_lineWidth(p,
                                                                               x,
                                                                               y,
                                                                               w,
                                                                               h,
                                                                               arg6 as *const ::qt_gui::color::Color,
                                                                               line_width)
    }
  }
  impl<'a> DrawPlainRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::color::Color,
                                      ::libc::c_int,
                                      *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let arg6 = self.5;
      let line_width = self.6;
      let fill = self.7;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawPlainRect_p_x_y_w_h_arg6_lineWidth_fill(p, x, y, w, h, arg6 as *const ::qt_gui::color::Color, line_width, fill)
    }
  }
  /// This trait represents a set of arguments accepted by [draw_shade_line](../fn.draw_shade_line.html) method.
  pub trait DrawShadeLineArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let p1 = self.1;
      let p2 = self.2;
      let pal = self.3;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_p1_p2_pal(p,
                                                                  p1 as *const ::qt_core::point::Point,
                                                                  p2 as *const ::qt_core::point::Point,
                                                                  pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_gui::palette::Palette,
                                      bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let p1 = self.1;
      let p2 = self.2;
      let pal = self.3;
      let sunken = self.4;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_p1_p2_pal_sunken(p,
                                                                         p1 as *const ::qt_core::point::Point,
                                                                         p2 as *const ::qt_core::point::Point,
                                                                         pal as *const ::qt_gui::palette::Palette,
                                                                         sunken)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let p1 = self.1;
      let p2 = self.2;
      let pal = self.3;
      let sunken = self.4;
      let line_width = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_p1_p2_pal_sunken_lineWidth(p, p1 as *const ::qt_core::point::Point, p2 as *const ::qt_core::point::Point, pal as *const ::qt_gui::palette::Palette, sunken, line_width)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_core::point::Point,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let p1 = self.1;
      let p2 = self.2;
      let pal = self.3;
      let sunken = self.4;
      let line_width = self.5;
      let mid_line_width = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_p1_p2_pal_sunken_lineWidth_midLineWidth(p, p1 as *const ::qt_core::point::Point, p2 as *const ::qt_core::point::Point, pal as *const ::qt_gui::palette::Palette, sunken, line_width, mid_line_width)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x1 = self.1;
      let y1 = self.2;
      let x2 = self.3;
      let y2 = self.4;
      let pal = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_x1_y1_x2_y2_pal(p,
                                                                        x1,
                                                                        y1,
                                                                        x2,
                                                                        y2,
                                                                        pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x1 = self.1;
      let y1 = self.2;
      let x2 = self.3;
      let y2 = self.4;
      let pal = self.5;
      let sunken = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_x1_y1_x2_y2_pal_sunken(p, x1, y1, x2, y2, pal as *const ::qt_gui::palette::Palette, sunken)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x1 = self.1;
      let y1 = self.2;
      let x2 = self.3;
      let y2 = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_x1_y1_x2_y2_pal_sunken_lineWidth(p, x1, y1, x2, y2, pal as *const ::qt_gui::palette::Palette, sunken, line_width)
    }
  }
  impl<'a> DrawShadeLineArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x1 = self.1;
      let y1 = self.2;
      let x2 = self.3;
      let y2 = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      let mid_line_width = self.8;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeLine_p_x1_y1_x2_y2_pal_sunken_lineWidth_midLineWidth(p, x1, y1, x2, y2, pal as *const ::qt_gui::palette::Palette, sunken, line_width, mid_line_width)
    }
  }
  /// This trait represents a set of arguments accepted by [draw_shade_panel](../fn.draw_shade_panel.html) method.
  pub trait DrawShadePanelArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_r_pal(p,
                                                               r as *const ::qt_core::rect::Rect,
                                                               pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette, bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_r_pal_sunken(p,
                                                                      r as *const ::qt_core::rect::Rect,
                                                                      pal as *const ::qt_gui::palette::Palette,
                                                                      sunken)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                       &'a ::qt_core::rect::Rect,
                                       &'a ::qt_gui::palette::Palette,
                                       bool,
                                       ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let line_width = self.4;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_r_pal_sunken_lineWidth(p, r as *const ::qt_core::rect::Rect, pal as *const ::qt_gui::palette::Palette, sunken, line_width)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                       &'a ::qt_core::rect::Rect,
                                       &'a ::qt_gui::palette::Palette,
                                       bool,
                                       ::libc::c_int,
                                       *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let line_width = self.4;
      let fill = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_r_pal_sunken_lineWidth_fill(p, r as *const ::qt_core::rect::Rect, pal as *const ::qt_gui::palette::Palette, sunken, line_width, fill)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_x_y_w_h_pal(p,
                                                                     x,
                                                                     y,
                                                                     w,
                                                                     h,
                                                                     pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       &'a ::qt_gui::palette::Palette,
                                       bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_x_y_w_h_pal_sunken(p,
                                                                            x,
                                                                            y,
                                                                            w,
                                                                            h,
                                                                            pal as *const ::qt_gui::palette::Palette,
                                                                            sunken)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       &'a ::qt_gui::palette::Palette,
                                       bool,
                                       ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_x_y_w_h_pal_sunken_lineWidth(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, line_width)
    }
  }
  impl<'a> DrawShadePanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       ::libc::c_int,
                                       &'a ::qt_gui::palette::Palette,
                                       bool,
                                       ::libc::c_int,
                                       *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      let fill = self.8;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadePanel_p_x_y_w_h_pal_sunken_lineWidth_fill(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, line_width, fill)
    }
  }
  /// This trait represents a set of arguments accepted by [draw_shade_rect](../fn.draw_shade_rect.html) method.
  pub trait DrawShadeRectArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_r_pal(p,
                                                              r as *const ::qt_core::rect::Rect,
                                                              pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette, bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_r_pal_sunken(p,
                                                                     r as *const ::qt_core::rect::Rect,
                                                                     pal as *const ::qt_gui::palette::Palette,
                                                                     sunken)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::rect::Rect,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let line_width = self.4;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_r_pal_sunken_lineWidth(p, r as *const ::qt_core::rect::Rect, pal as *const ::qt_gui::palette::Palette, sunken, line_width)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::rect::Rect,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let line_width = self.4;
      let mid_line_width = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_r_pal_sunken_lineWidth_midLineWidth(p, r as *const ::qt_core::rect::Rect, pal as *const ::qt_gui::palette::Palette, sunken, line_width, mid_line_width)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::rect::Rect,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let line_width = self.4;
      let mid_line_width = self.5;
      let fill = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_r_pal_sunken_lineWidth_midLineWidth_fill(p, r as *const ::qt_core::rect::Rect, pal as *const ::qt_gui::palette::Palette, sunken, line_width, mid_line_width, fill)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_x_y_w_h_pal(p,
                                                                    x,
                                                                    y,
                                                                    w,
                                                                    h,
                                                                    pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_x_y_w_h_pal_sunken(p,
                                                                           x,
                                                                           y,
                                                                           w,
                                                                           h,
                                                                           pal as *const ::qt_gui::palette::Palette,
                                                                           sunken)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_x_y_w_h_pal_sunken_lineWidth(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, line_width)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int,
                                      ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      let mid_line_width = self.8;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_x_y_w_h_pal_sunken_lineWidth_midLineWidth(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, line_width, mid_line_width)
    }
  }
  impl<'a> DrawShadeRectArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let line_width = self.7;
      let mid_line_width = self.8;
      let fill = self.9;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawShadeRect_p_x_y_w_h_pal_sunken_lineWidth_midLineWidth_fill(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, line_width, mid_line_width, fill)
    }
  }
  /// This trait represents a set of arguments accepted by [draw_win_button](../fn.draw_win_button.html) method.
  pub trait DrawWinButtonArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> DrawWinButtonArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinButton_p_r_pal(p,
                                                              r as *const ::qt_core::rect::Rect,
                                                              pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawWinButtonArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette, bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinButton_p_r_pal_sunken(p,
                                                                     r as *const ::qt_core::rect::Rect,
                                                                     pal as *const ::qt_gui::palette::Palette,
                                                                     sunken)
    }
  }
  impl<'a> DrawWinButtonArgs
    for (*mut ::qt_gui::painter::Painter,
                                      &'a ::qt_core::rect::Rect,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let fill = self.4;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinButton_p_r_pal_sunken_fill(p,
                                                                          r as *const ::qt_core::rect::Rect,
                                                                          pal as *const ::qt_gui::palette::Palette,
                                                                          sunken,
                                                                          fill)
    }
  }
  impl<'a> DrawWinButtonArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinButton_p_x_y_w_h_pal(p,
                                                                    x,
                                                                    y,
                                                                    w,
                                                                    h,
                                                                    pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawWinButtonArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinButton_p_x_y_w_h_pal_sunken(p,
                                                                           x,
                                                                           y,
                                                                           w,
                                                                           h,
                                                                           pal as *const ::qt_gui::palette::Palette,
                                                                           sunken)
    }
  }
  impl<'a> DrawWinButtonArgs
    for (*mut ::qt_gui::painter::Painter,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      &'a ::qt_gui::palette::Palette,
                                      bool,
                                      *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let fill = self.7;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinButton_p_x_y_w_h_pal_sunken_fill(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, fill)
    }
  }
  /// This trait represents a set of arguments accepted by [draw_win_panel](../fn.draw_win_panel.html) method.
  pub trait DrawWinPanelArgs {
    unsafe fn exec(self) -> ();
  }
  impl<'a> DrawWinPanelArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinPanel_p_r_pal(p,
                                                             r as *const ::qt_core::rect::Rect,
                                                             pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawWinPanelArgs
    for (*mut ::qt_gui::painter::Painter, &'a ::qt_core::rect::Rect, &'a ::qt_gui::palette::Palette, bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinPanel_p_r_pal_sunken(p,
                                                                    r as *const ::qt_core::rect::Rect,
                                                                    pal as *const ::qt_gui::palette::Palette,
                                                                    sunken)
    }
  }
  impl<'a> DrawWinPanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                     &'a ::qt_core::rect::Rect,
                                     &'a ::qt_gui::palette::Palette,
                                     bool,
                                     *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let r = self.1;
      let pal = self.2;
      let sunken = self.3;
      let fill = self.4;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinPanel_p_r_pal_sunken_fill(p,
                                                                         r as *const ::qt_core::rect::Rect,
                                                                         pal as *const ::qt_gui::palette::Palette,
                                                                         sunken,
                                                                         fill)
    }
  }
  impl<'a> DrawWinPanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     &'a ::qt_gui::palette::Palette) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinPanel_p_x_y_w_h_pal(p,
                                                                   x,
                                                                   y,
                                                                   w,
                                                                   h,
                                                                   pal as *const ::qt_gui::palette::Palette)
    }
  }
  impl<'a> DrawWinPanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     &'a ::qt_gui::palette::Palette,
                                     bool) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinPanel_p_x_y_w_h_pal_sunken(p,
                                                                          x,
                                                                          y,
                                                                          w,
                                                                          h,
                                                                          pal as *const ::qt_gui::palette::Palette,
                                                                          sunken)
    }
  }
  impl<'a> DrawWinPanelArgs
    for (*mut ::qt_gui::painter::Painter,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     ::libc::c_int,
                                     &'a ::qt_gui::palette::Palette,
                                     bool,
                                     *const ::qt_gui::brush::Brush) {
    unsafe fn exec(self) -> () {
      let p = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let pal = self.5;
      let sunken = self.6;
      let fill = self.7;
      ::ffi::qt_widgets_c_QTileRules_G_qDrawWinPanel_p_x_y_w_h_pal_sunken_fill(p, x, y, w, h, pal as *const ::qt_gui::palette::Palette, sunken, fill)
    }
  }
}
