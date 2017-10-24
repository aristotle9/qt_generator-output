/// C++ type: <span style='color: green;'>```QProxyStyle```</span>
#[repr(C)]
pub struct ProxyStyle(u8);

impl ProxyStyle {
  /// C++ method: <span style='color: green;'>```QStyle* QProxyStyle::baseStyle() const```</span>
  ///
  ///
  pub fn base_style(&self) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QProxyStyle_baseStyle(self as *const ::proxy_style::ProxyStyle) }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::drawComplexControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawComplexControl(QStyle::ComplexControl control, const QStyleOptionComplex* option, QPainter* painter) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawComplexControl(QStyle::ComplexControl control, const QStyleOptionComplex* option, QPainter* painter, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_complex_control<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ProxyStyleDrawComplexControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProxyStyle::drawControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_control(&self, (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawControl(QStyle::ControlElement element, const QStyleOption* option, QPainter* painter) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_control(&self, (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawControl(QStyle::ControlElement element, const QStyleOption* option, QPainter* painter, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_control<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ProxyStyleDrawControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawItemPixmap(QPainter* painter, const QRect& rect, int alignment, const QPixmap& pixmap) const```</span>
  ///
  ///
  pub unsafe fn draw_item_pixmap(&self,
                                 painter: *mut ::qt_gui::painter::Painter,
                                 rect: &::qt_core::rect::Rect,
                                 alignment: ::libc::c_int,
                                 pixmap: &::qt_gui::pixmap::Pixmap) {
    ::ffi::qt_widgets_c_QProxyStyle_drawItemPixmap(self as *const ::proxy_style::ProxyStyle,
                                                   painter,
                                                   rect as *const ::qt_core::rect::Rect,
                                                   alignment,
                                                   pixmap as *const ::qt_gui::pixmap::Pixmap)
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::drawItemText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_item_text(&self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, ::libc::c_int, &::qt_gui::palette::Palette, bool, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawItemText(QPainter* painter, const QRect& rect, int flags, const QPalette& pal, bool enabled, const QString& text) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_item_text(&self, (*mut ::qt_gui::painter::Painter, &::qt_core::rect::Rect, ::libc::c_int, &::qt_gui::palette::Palette, bool, &::qt_core::string::String, &::qt_gui::palette::ColorRole)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawItemText(QPainter* painter, const QRect& rect, int flags, const QPalette& pal, bool enabled, const QString& text, QPalette::ColorRole textRole = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_item_text<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ProxyStyleDrawItemTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProxyStyle::drawPrimitive```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_primitive(&self, (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawPrimitive(QStyle::PrimitiveElement element, const QStyleOption* option, QPainter* painter) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_primitive(&self, (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::drawPrimitive(QStyle::PrimitiveElement element, const QStyleOption* option, QPainter* painter, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_primitive<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ProxyStyleDrawPrimitiveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QPixmap QProxyStyle::generatedIconPixmap(QIcon::Mode iconMode, const QPixmap& pixmap, const QStyleOption* opt) const```</span>
  ///
  ///
  pub unsafe fn generated_icon_pixmap(&self,
                                      icon_mode: &::qt_gui::icon::Mode,
                                      pixmap: &::qt_gui::pixmap::Pixmap,
                                      opt: *const ::style_option::StyleOption)
                                      -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result =
      ::ffi::qt_widgets_c_QProxyStyle_generatedIconPixmap_as_ptr(self as *const ::proxy_style::ProxyStyle,
                                                                 icon_mode as *const ::qt_gui::icon::Mode,
                                                                 pixmap as *const ::qt_gui::pixmap::Pixmap,
                                                                 opt);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::hitTestComplexControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn hit_test_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &::qt_core::point::Point)) -> ::style::SubControl```<br>
  /// C++ method: <span style='color: green;'>```virtual QStyle::SubControl QProxyStyle::hitTestComplexControl(QStyle::ComplexControl control, const QStyleOptionComplex* option, const QPoint& pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn hit_test_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &::qt_core::point::Point, *const ::widget::Widget)) -> ::style::SubControl```<br>
  /// C++ method: <span style='color: green;'>```virtual QStyle::SubControl QProxyStyle::hitTestComplexControl(QStyle::ComplexControl control, const QStyleOptionComplex* option, const QPoint& pos, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn hit_test_complex_control<'largs, Args>(&'largs self, args: Args) -> ::style::SubControl
    where Args: overloading::ProxyStyleHitTestComplexControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QRect QProxyStyle::itemPixmapRect(const QRect& r, int flags, const QPixmap& pixmap) const```</span>
  ///
  ///
  pub fn item_pixmap_rect(&self,
                          r: &::qt_core::rect::Rect,
                          flags: ::libc::c_int,
                          pixmap: &::qt_gui::pixmap::Pixmap)
                          -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProxyStyle_itemPixmapRect_to_output(self as *const ::proxy_style::ProxyStyle,
                                                                 r as *const ::qt_core::rect::Rect,
                                                                 flags,
                                                                 pixmap as *const ::qt_gui::pixmap::Pixmap,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QProxyStyle::itemTextRect(const QFontMetrics& fm, const QRect& r, int flags, bool enabled, const QString& text) const```</span>
  ///
  ///
  pub fn item_text_rect(&self,
                        fm: &::qt_gui::font_metrics::FontMetrics,
                        r: &::qt_core::rect::Rect,
                        flags: ::libc::c_int,
                        enabled: bool,
                        text: &::qt_core::string::String)
                        -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProxyStyle_itemTextRect_to_output(self as *const ::proxy_style::ProxyStyle,
                                                               fm as *const ::qt_gui::font_metrics::FontMetrics,
                                                               r as *const ::qt_core::rect::Rect,
                                                               flags,
                                                               enabled,
                                                               text as *const ::qt_core::string::String,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation) const```</span>
  ///
  ///
  pub fn layout_spacing(&self,
                        control1: &::size_policy::ControlType,
                        control2: &::size_policy::ControlType,
                        orientation: &::qt_core::qt::Orientation)
                        -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation(self as *const ::proxy_style::ProxyStyle, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation) }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::layoutSpacing```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn layout_spacing_unsafe(&self, (&::size_policy::ControlType, &::size_policy::ControlType, &::qt_core::qt::Orientation, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn layout_spacing_unsafe(&self, (&::size_policy::ControlType, &::size_policy::ControlType, &::qt_core::qt::Orientation, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn layout_spacing_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ProxyStyleLayoutSpacingUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QProxyStyle::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QProxyStyle_metaObject(self as *const ::proxy_style::ProxyStyle) }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::QProxyStyle```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QProxyStyle::QProxyStyle()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QProxyStyle::QProxyStyle(const QString& key)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle>
    where Args: overloading::ProxyStyleNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QProxyStyle::QProxyStyle(QStyle* style = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(style: *mut ::style::Style) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle> {
    let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_new_style(style);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::pixelMetric(QStyle::PixelMetric metric) const```</span>
  ///
  ///
  pub fn pixel_metric(&self, metric: ::style::PixelMetric) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProxyStyle_pixelMetric_metric(self as *const ::proxy_style::ProxyStyle, metric) }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::pixelMetric```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixel_metric_unsafe(&self, (::style::PixelMetric, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::pixelMetric(QStyle::PixelMetric metric, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixel_metric_unsafe(&self, (::style::PixelMetric, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::pixelMetric(QStyle::PixelMetric metric, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn pixel_metric_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ProxyStylePixelMetricUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::polish(QPalette& pal)```</span>
  ///
  ///
  pub fn polish(&mut self, pal: &mut ::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QProxyStyle_polish_pal(self as *mut ::proxy_style::ProxyStyle,
                                                 pal as *mut ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::polish```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn polish_unsafe(&mut self, *mut ::application::Application) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::polish(QApplication* app)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn polish_unsafe(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::polish(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn polish_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ProxyStylePolishUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QProxyStyle_qt_metacall(self as *mut ::proxy_style::ProxyStyle,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QProxyStyle::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QProxyStyle_qt_metacast(self as *mut ::proxy_style::ProxyStyle, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QProxyStyle::setBaseStyle(QStyle* style)```</span>
  ///
  ///
  pub unsafe fn set_base_style(&mut self, style: *mut ::style::Style) {
    ::ffi::qt_widgets_c_QProxyStyle_setBaseStyle(self as *mut ::proxy_style::ProxyStyle, style)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QProxyStyle::sizeFromContents(QStyle::ContentsType type, const QStyleOption* option, const QSize& size, const QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn size_from_contents(&self,
                                   type_: ::style::ContentsType,
                                   option: *const ::style_option::StyleOption,
                                   size: &::qt_core::size::Size,
                                   widget: *const ::widget::Widget)
                                   -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProxyStyle_sizeFromContents_to_output(self as *const ::proxy_style::ProxyStyle,
                                                                 type_,
                                                                 option,
                                                                 size as *const ::qt_core::size::Size,
                                                                 widget,
                                                                 &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QIcon QProxyStyle::standardIcon(QStyle::StandardPixmap standardIcon) const```</span>
  ///
  ///
  pub fn standard_icon(&self, standard_icon: ::style::StandardPixmap) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon(self as *const ::proxy_style::ProxyStyle,
                                                                            standard_icon,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::standardIcon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn standard_icon_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption)) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```virtual QIcon QProxyStyle::standardIcon(QStyle::StandardPixmap standardIcon, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn standard_icon_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```virtual QIcon QProxyStyle::standardIcon(QStyle::StandardPixmap standardIcon, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn standard_icon_unsafe<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::icon::Icon
    where Args: overloading::ProxyStyleStandardIconUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QPalette QProxyStyle::standardPalette() const```</span>
  ///
  ///
  pub fn standard_palette(&self) -> ::qt_gui::palette::Palette {
    {
      let mut object: ::qt_gui::palette::Palette =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProxyStyle_standardPalette_to_output(self as *const ::proxy_style::ProxyStyle,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::standardPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn standard_pixmap(&self, (::style::StandardPixmap, *const ::style_option::StyleOption)) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```virtual QPixmap QProxyStyle::standardPixmap(QStyle::StandardPixmap standardPixmap, const QStyleOption* opt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn standard_pixmap(&self, (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```virtual QPixmap QProxyStyle::standardPixmap(QStyle::StandardPixmap standardPixmap, const QStyleOption* opt, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn standard_pixmap<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>
    where Args: overloading::ProxyStyleStandardPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::styleHint(QStyle::StyleHint hint) const```</span>
  ///
  ///
  pub fn style_hint(&self, hint: ::style::StyleHint) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProxyStyle_styleHint_hint(self as *const ::proxy_style::ProxyStyle, hint) }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::styleHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::styleHint(QStyle::StyleHint hint, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::styleHint(QStyle::StyleHint hint, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget, *mut ::style_hint_return::StyleHintReturn)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QProxyStyle::styleHint(QStyle::StyleHint hint, const QStyleOption* option = ?, const QWidget* widget = ?, QStyleHintReturn* returnData = ?) const```</span>
  ///
  ///
  pub unsafe fn style_hint_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ProxyStyleStyleHintUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QRect QProxyStyle::subControlRect(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn sub_control_rect(&self,
                                 cc: ::style::ComplexControl,
                                 opt: *const ::style_option_complex::StyleOptionComplex,
                                 sc: ::style::SubControl,
                                 widget: *const ::widget::Widget)
                                 -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProxyStyle_subControlRect_to_output(self as *const ::proxy_style::ProxyStyle,
                                                               cc,
                                                               opt,
                                                               sc,
                                                               widget,
                                                               &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QProxyStyle::subElementRect(QStyle::SubElement element, const QStyleOption* option, const QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn sub_element_rect(&self,
                                 element: ::style::SubElement,
                                 option: *const ::style_option::StyleOption,
                                 widget: *const ::widget::Widget)
                                 -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProxyStyle_subElementRect_to_output(self as *const ::proxy_style::ProxyStyle,
                                                               element,
                                                               option,
                                                               widget,
                                                               &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProxyStyle::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProxyStyle_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProxyStyle::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProxyStyle_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProxyStyle::unpolish```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn unpolish(&mut self, *mut ::application::Application) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::unpolish(QApplication* app)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn unpolish(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QProxyStyle::unpolish(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn unpolish<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ProxyStyleUnpolishArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::proxy_style::ProxyStyle {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QProxyStyle_delete
  }
}

impl ::cpp_utils::DynamicCast<::proxy_style::ProxyStyle> for ::common_style::CommonStyle {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::proxy_style::ProxyStyle> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QCommonStyle(self as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::proxy_style::ProxyStyle> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QCommonStyle(self as *const ::common_style::CommonStyle as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::proxy_style::ProxyStyle> for ::style::Style {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::proxy_style::ProxyStyle> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QStyle(self as *mut ::style::Style) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::proxy_style::ProxyStyle> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_dynamic_cast_QProxyStyle_ptr_QStyle(self as *const ::style::Style as *mut ::style::Style) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::proxy_style::ProxyStyle {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QObject_ptr(self as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QObject_ptr(self as *const ::proxy_style::ProxyStyle as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::common_style::CommonStyle> for ::proxy_style::ProxyStyle {
  fn static_cast_mut(&mut self) -> &mut ::common_style::CommonStyle {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QCommonStyle_ptr(self as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::common_style::CommonStyle {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QCommonStyle_ptr(self as *const ::proxy_style::ProxyStyle as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style::Style> for ::proxy_style::ProxyStyle {
  fn static_cast_mut(&mut self) -> &mut ::style::Style {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QStyle_ptr(self as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style::Style {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QStyle_ptr(self as *const ::proxy_style::ProxyStyle as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::proxy_style::ProxyStyle> for ::common_style::CommonStyle {
  unsafe fn static_cast_mut(&mut self) -> &mut ::proxy_style::ProxyStyle {
    let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QCommonStyle(self as *mut ::common_style::CommonStyle);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::proxy_style::ProxyStyle {
    let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QCommonStyle(self as *const ::common_style::CommonStyle as *mut ::common_style::CommonStyle);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::proxy_style::ProxyStyle> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::proxy_style::ProxyStyle {
    let ffi_result =
      ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::proxy_style::ProxyStyle {
    let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::proxy_style::ProxyStyle> for ::style::Style {
  unsafe fn static_cast_mut(&mut self) -> &mut ::proxy_style::ProxyStyle {
    let ffi_result =
      ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QStyle(self as *mut ::style::Style);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::proxy_style::ProxyStyle {
    let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QProxyStyle_ptr_QStyle(self as *const ::style::Style as *mut ::style::Style);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::proxy_style::ProxyStyle {
  type Target = ::common_style::CommonStyle;
  fn deref(&self) -> &::common_style::CommonStyle {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QCommonStyle_ptr(self as *const ::proxy_style::ProxyStyle as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::proxy_style::ProxyStyle {
  fn deref_mut(&mut self) -> &mut ::common_style::CommonStyle {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProxyStyle_G_static_cast_QCommonStyle_ptr(self as *mut ::proxy_style::ProxyStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ProxyStyle::draw_complex_control](../struct.ProxyStyle.html#method.draw_complex_control) method.
  pub trait ProxyStyleDrawComplexControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ();
  }
  impl<'largs> ProxyStyleDrawComplexControlArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let control = self.0;
      let option = self.1;
      let painter = self.2;
      ::ffi::qt_widgets_c_QProxyStyle_drawComplexControl_control_option_painter(original_self as *const ::proxy_style::ProxyStyle, control, option, painter)
    }
  }
  impl<'largs> ProxyStyleDrawComplexControlArgs<'largs>
    for (::style::ComplexControl,
                                                                 *const ::style_option_complex::StyleOptionComplex,
                                                                 *mut ::qt_gui::painter::Painter,
                                                                 *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let control = self.0;
      let option = self.1;
      let painter = self.2;
      let widget = self.3;
      ::ffi::qt_widgets_c_QProxyStyle_drawComplexControl_control_option_painter_widget(original_self as *const ::proxy_style::ProxyStyle, control, option, painter, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::draw_control](../struct.ProxyStyle.html#method.draw_control) method.
  pub trait ProxyStyleDrawControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ();
  }
  impl<'largs> ProxyStyleDrawControlArgs<'largs>
    for (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let element = self.0;
      let option = self.1;
      let painter = self.2;
      ::ffi::qt_widgets_c_QProxyStyle_drawControl_element_option_painter(original_self as *const ::proxy_style::ProxyStyle, element, option, painter)
    }
  }
  impl<'largs> ProxyStyleDrawControlArgs<'largs>
    for (::style::ControlElement,
                                                          *const ::style_option::StyleOption,
                                                          *mut ::qt_gui::painter::Painter,
                                                          *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let element = self.0;
      let option = self.1;
      let painter = self.2;
      let widget = self.3;
      ::ffi::qt_widgets_c_QProxyStyle_drawControl_element_option_painter_widget(original_self as *const ::proxy_style::ProxyStyle, element, option, painter, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::draw_item_text](../struct.ProxyStyle.html#method.draw_item_text) method.
  pub trait ProxyStyleDrawItemTextArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ();
  }
  impl<'largs> ProxyStyleDrawItemTextArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                           &'largs ::qt_core::rect::Rect,
                                                           ::libc::c_int,
                                                           &'largs ::qt_gui::palette::Palette,
                                                           bool,
                                                           &'largs ::qt_core::string::String) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let painter = self.0;
      let rect = self.1;
      let flags = self.2;
      let pal = self.3;
      let enabled = self.4;
      let text = self.5;
      ::ffi::qt_widgets_c_QProxyStyle_drawItemText_painter_rect_flags_pal_enabled_text(original_self as *const ::proxy_style::ProxyStyle, painter, rect as *const ::qt_core::rect::Rect, flags, pal as *const ::qt_gui::palette::Palette, enabled, text as *const ::qt_core::string::String)
    }
  }
  impl<'largs> ProxyStyleDrawItemTextArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                           &'largs ::qt_core::rect::Rect,
                                                           ::libc::c_int,
                                                           &'largs ::qt_gui::palette::Palette,
                                                           bool,
                                                           &'largs ::qt_core::string::String,
                                                           &'largs ::qt_gui::palette::ColorRole) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let painter = self.0;
      let rect = self.1;
      let flags = self.2;
      let pal = self.3;
      let enabled = self.4;
      let text = self.5;
      let text_role = self.6;
      ::ffi::qt_widgets_c_QProxyStyle_drawItemText_painter_rect_flags_pal_enabled_text_textRole(original_self as *const ::proxy_style::ProxyStyle, painter, rect as *const ::qt_core::rect::Rect, flags, pal as *const ::qt_gui::palette::Palette, enabled, text as *const ::qt_core::string::String, text_role as *const ::qt_gui::palette::ColorRole)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::draw_primitive](../struct.ProxyStyle.html#method.draw_primitive) method.
  pub trait ProxyStyleDrawPrimitiveArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ();
  }
  impl<'largs> ProxyStyleDrawPrimitiveArgs<'largs>
    for (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let element = self.0;
      let option = self.1;
      let painter = self.2;
      ::ffi::qt_widgets_c_QProxyStyle_drawPrimitive_element_option_painter(original_self as *const ::proxy_style::ProxyStyle, element, option, painter)
    }
  }
  impl<'largs> ProxyStyleDrawPrimitiveArgs<'largs>
    for (::style::PrimitiveElement,
                                                            *const ::style_option::StyleOption,
                                                            *mut ::qt_gui::painter::Painter,
                                                            *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> () {
      let element = self.0;
      let option = self.1;
      let painter = self.2;
      let widget = self.3;
      ::ffi::qt_widgets_c_QProxyStyle_drawPrimitive_element_option_painter_widget(original_self as *const ::proxy_style::ProxyStyle, element, option, painter, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::hit_test_complex_control](../struct.ProxyStyle.html#method.hit_test_complex_control) method.
  pub trait ProxyStyleHitTestComplexControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::style::SubControl;
  }
  impl<'largs> ProxyStyleHitTestComplexControlArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::style::SubControl {
      let control = self.0;
      let option = self.1;
      let pos = self.2;
      ::ffi::qt_widgets_c_QProxyStyle_hitTestComplexControl_control_option_pos(original_self as *const ::proxy_style::ProxyStyle, control, option, pos as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> ProxyStyleHitTestComplexControlArgs<'largs>
    for (::style::ComplexControl,
                                                                    *const ::style_option_complex::StyleOptionComplex,
                                                                    &'largs ::qt_core::point::Point,
                                                                    *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::style::SubControl {
      let control = self.0;
      let option = self.1;
      let pos = self.2;
      let widget = self.3;
      ::ffi::qt_widgets_c_QProxyStyle_hitTestComplexControl_control_option_pos_widget(original_self as *const ::proxy_style::ProxyStyle, control, option, pos as *const ::qt_core::point::Point, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::layout_spacing_unsafe](../struct.ProxyStyle.html#method.layout_spacing_unsafe) method.
  pub trait ProxyStyleLayoutSpacingUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int;
  }
  impl<'largs> ProxyStyleLayoutSpacingUnsafeArgs<'largs>
    for (&'largs ::size_policy::ControlType,
                                                                  &'largs ::size_policy::ControlType,
                                                                  &'largs ::qt_core::qt::Orientation,
                                                                  *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let control1 = self.0;
      let control2 = self.1;
      let orientation = self.2;
      let option = self.3;
      ::ffi::qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation_option(original_self as *const ::proxy_style::ProxyStyle, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation, option)
    }
  }
  impl<'largs> ProxyStyleLayoutSpacingUnsafeArgs<'largs>
    for (&'largs ::size_policy::ControlType,
                                                                  &'largs ::size_policy::ControlType,
                                                                  &'largs ::qt_core::qt::Orientation,
                                                                  *const ::style_option::StyleOption,
                                                                  *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let control1 = self.0;
      let control2 = self.1;
      let orientation = self.2;
      let option = self.3;
      let widget = self.4;
      ::ffi::qt_widgets_c_QProxyStyle_layoutSpacing_control1_control2_orientation_option_widget(original_self as *const ::proxy_style::ProxyStyle, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::new](../struct.ProxyStyle.html#method.new) method.
  pub trait ProxyStyleNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle>;
  }
  impl<'a> ProxyStyleNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle> {
      let key = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_new_key(key as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ProxyStyleNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::proxy_style::ProxyStyle> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QProxyStyle_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::pixel_metric_unsafe](../struct.ProxyStyle.html#method.pixel_metric_unsafe) method.
  pub trait ProxyStylePixelMetricUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int;
  }
  impl<'largs> ProxyStylePixelMetricUnsafeArgs<'largs> for (::style::PixelMetric, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let metric = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QProxyStyle_pixelMetric_metric_option(original_self as *const ::proxy_style::ProxyStyle,
                                                                metric,
                                                                option)
    }
  }
  impl<'largs> ProxyStylePixelMetricUnsafeArgs<'largs>
    for (::style::PixelMetric, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let metric = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QProxyStyle_pixelMetric_metric_option_widget(original_self as *const ::proxy_style::ProxyStyle, metric, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::polish_unsafe](../struct.ProxyStyle.html#method.polish_unsafe) method.
  pub trait ProxyStylePolishUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::proxy_style::ProxyStyle) -> ();
  }
  impl<'largs> ProxyStylePolishUnsafeArgs<'largs> for *mut ::application::Application {
    unsafe fn exec(self, original_self: &'largs mut ::proxy_style::ProxyStyle) -> () {
      let app = self;
      ::ffi::qt_widgets_c_QProxyStyle_polish_app(original_self as *mut ::proxy_style::ProxyStyle, app)
    }
  }
  impl<'largs> ProxyStylePolishUnsafeArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::proxy_style::ProxyStyle) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QProxyStyle_polish_widget(original_self as *mut ::proxy_style::ProxyStyle, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::standard_icon_unsafe](../struct.ProxyStyle.html#method.standard_icon_unsafe) method.
  pub trait ProxyStyleStandardIconUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::qt_gui::icon::Icon;
  }
  impl<'largs> ProxyStyleStandardIconUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::qt_gui::icon::Icon {
      let standard_icon = self.0;
      let option = self.1;
      {
        let mut object: ::qt_gui::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon_option(original_self as *const ::proxy_style::ProxyStyle, standard_icon, option, &mut object);
        object
      }
    }
  }
  impl<'largs> ProxyStyleStandardIconUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::qt_gui::icon::Icon {
      let standard_icon = self.0;
      let option = self.1;
      let widget = self.2;
      {
        let mut object: ::qt_gui::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QProxyStyle_standardIcon_to_output_standardIcon_option_widget(original_self as *const ::proxy_style::ProxyStyle, standard_icon, option, widget, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::standard_pixmap](../struct.ProxyStyle.html#method.standard_pixmap) method.
  pub trait ProxyStyleStandardPixmapArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs ::proxy_style::ProxyStyle)
                   -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>;
  }
  impl<'largs> ProxyStyleStandardPixmapArgs<'largs> for (::style::StandardPixmap, *const ::style_option::StyleOption) {
    unsafe fn exec(self,
                   original_self: &'largs ::proxy_style::ProxyStyle)
                   -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let standard_pixmap = self.0;
      let opt = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_standardPixmap_as_ptr_standardPixmap_opt(original_self as *const ::proxy_style::ProxyStyle, standard_pixmap, opt);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'largs> ProxyStyleStandardPixmapArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self,
                   original_self: &'largs ::proxy_style::ProxyStyle)
                   -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let standard_pixmap = self.0;
      let opt = self.1;
      let widget = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QProxyStyle_standardPixmap_as_ptr_standardPixmap_opt_widget(original_self as *const ::proxy_style::ProxyStyle, standard_pixmap, opt, widget);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::style_hint_unsafe](../struct.ProxyStyle.html#method.style_hint_unsafe) method.
  pub trait ProxyStyleStyleHintUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int;
  }
  impl<'largs> ProxyStyleStyleHintUnsafeArgs<'largs> for (::style::StyleHint, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let hint = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QProxyStyle_styleHint_hint_option(original_self as *const ::proxy_style::ProxyStyle,
                                                            hint,
                                                            option)
    }
  }
  impl<'largs> ProxyStyleStyleHintUnsafeArgs<'largs>
    for (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let hint = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QProxyStyle_styleHint_hint_option_widget(original_self as *const ::proxy_style::ProxyStyle,
                                                                   hint,
                                                                   option,
                                                                   widget)
    }
  }
  impl<'largs> ProxyStyleStyleHintUnsafeArgs<'largs>
    for (::style::StyleHint,
                                                              *const ::style_option::StyleOption,
                                                              *const ::widget::Widget,
                                                              *mut ::style_hint_return::StyleHintReturn) {
    unsafe fn exec(self, original_self: &'largs ::proxy_style::ProxyStyle) -> ::libc::c_int {
      let hint = self.0;
      let option = self.1;
      let widget = self.2;
      let return_data = self.3;
      ::ffi::qt_widgets_c_QProxyStyle_styleHint_hint_option_widget_returnData(original_self as *const ::proxy_style::ProxyStyle, hint, option, widget, return_data)
    }
  }
  /// This trait represents a set of arguments accepted by [ProxyStyle::unpolish](../struct.ProxyStyle.html#method.unpolish) method.
  pub trait ProxyStyleUnpolishArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::proxy_style::ProxyStyle) -> ();
  }
  impl<'largs> ProxyStyleUnpolishArgs<'largs> for *mut ::application::Application {
    unsafe fn exec(self, original_self: &'largs mut ::proxy_style::ProxyStyle) -> () {
      let app = self;
      ::ffi::qt_widgets_c_QProxyStyle_unpolish_app(original_self as *mut ::proxy_style::ProxyStyle, app)
    }
  }
  impl<'largs> ProxyStyleUnpolishArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::proxy_style::ProxyStyle) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QProxyStyle_unpolish_widget(original_self as *mut ::proxy_style::ProxyStyle, widget)
    }
  }
}
