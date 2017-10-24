/// C++ type: <span style='color: green;'>```QCommonStyle```</span>
#[repr(C)]
pub struct CommonStyle(u8);

impl CommonStyle {
  /// C++ method: <span style='color: green;'>```QCommonStyle::drawComplexControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QPainter* p, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_complex_control<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::CommonStyleDrawComplexControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommonStyle::drawControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_control(&self, (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::drawControl(QStyle::ControlElement element, const QStyleOption* opt, QPainter* p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_control(&self, (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::drawControl(QStyle::ControlElement element, const QStyleOption* opt, QPainter* p, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_control<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::CommonStyleDrawControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommonStyle::drawPrimitive```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_primitive(&self, (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_primitive(&self, (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter, *const ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption* opt, QPainter* p, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn draw_primitive<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::CommonStyleDrawPrimitiveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QPixmap QCommonStyle::generatedIconPixmap(QIcon::Mode iconMode, const QPixmap& pixmap, const QStyleOption* opt) const```</span>
  ///
  ///
  pub unsafe fn generated_icon_pixmap(&self,
                                      icon_mode: &::qt_gui::icon::Mode,
                                      pixmap: &::qt_gui::pixmap::Pixmap,
                                      opt: *const ::style_option::StyleOption)
                                      -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result =
      ::ffi::qt_widgets_c_QCommonStyle_generatedIconPixmap_as_ptr(self as *const ::common_style::CommonStyle,
                                                                  icon_mode as *const ::qt_gui::icon::Mode,
                                                                  pixmap as *const ::qt_gui::pixmap::Pixmap,
                                                                  opt);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::hitTestComplexControl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn hit_test_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &::qt_core::point::Point)) -> ::style::SubControl```<br>
  /// C++ method: <span style='color: green;'>```virtual QStyle::SubControl QCommonStyle::hitTestComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn hit_test_complex_control(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &::qt_core::point::Point, *const ::widget::Widget)) -> ::style::SubControl```<br>
  /// C++ method: <span style='color: green;'>```virtual QStyle::SubControl QCommonStyle::hitTestComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, const QPoint& pt, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn hit_test_complex_control<'largs, Args>(&'largs self, args: Args) -> ::style::SubControl
    where Args: overloading::CommonStyleHitTestComplexControlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation) const```</span>
  ///
  ///
  pub fn layout_spacing(&self,
                        control1: &::size_policy::ControlType,
                        control2: &::size_policy::ControlType,
                        orientation: &::qt_core::qt::Orientation)
                        -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation(self as *const ::common_style::CommonStyle, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation) }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::layoutSpacing```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn layout_spacing_unsafe(&self, (&::size_policy::ControlType, &::size_policy::ControlType, &::qt_core::qt::Orientation, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation, const QStyleOption* option = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn layout_spacing_unsafe(&self, (&::size_policy::ControlType, &::size_policy::ControlType, &::qt_core::qt::Orientation, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::layoutSpacing(QSizePolicy::ControlType control1, QSizePolicy::ControlType control2, Qt::Orientation orientation, const QStyleOption* option = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn layout_spacing_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::CommonStyleLayoutSpacingUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QCommonStyle::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QCommonStyle_metaObject(self as *const ::common_style::CommonStyle) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QCommonStyle::QCommonStyle()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::common_style::CommonStyle> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommonStyle_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::pixelMetric(QStyle::PixelMetric m) const```</span>
  ///
  ///
  pub fn pixel_metric(&self, m: ::style::PixelMetric) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCommonStyle_pixelMetric_m(self as *const ::common_style::CommonStyle, m) }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::pixelMetric```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixel_metric_unsafe(&self, (::style::PixelMetric, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::pixelMetric(QStyle::PixelMetric m, const QStyleOption* opt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixel_metric_unsafe(&self, (::style::PixelMetric, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::pixelMetric(QStyle::PixelMetric m, const QStyleOption* opt = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn pixel_metric_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::CommonStylePixelMetricUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::polish(QPalette& arg1)```</span>
  ///
  ///
  pub fn polish(&mut self, arg1: &mut ::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QCommonStyle_polish_arg1(self as *mut ::common_style::CommonStyle,
                                                   arg1 as *mut ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::polish```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn polish_unsafe(&mut self, *mut ::application::Application) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::polish(QApplication* app)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn polish_unsafe(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::polish(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn polish_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CommonStylePolishUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QCommonStyle_qt_metacall(self as *mut ::common_style::CommonStyle,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QCommonStyle::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QCommonStyle_qt_metacast(self as *mut ::common_style::CommonStyle, arg1)
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::sizeFromContents```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn size_from_contents(&self, (::style::ContentsType, *const ::style_option::StyleOption, &::qt_core::size::Size)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```virtual QSize QCommonStyle::sizeFromContents(QStyle::ContentsType ct, const QStyleOption* opt, const QSize& contentsSize) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn size_from_contents(&self, (::style::ContentsType, *const ::style_option::StyleOption, &::qt_core::size::Size, *const ::widget::Widget)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```virtual QSize QCommonStyle::sizeFromContents(QStyle::ContentsType ct, const QStyleOption* opt, const QSize& contentsSize, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn size_from_contents<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::CommonStyleSizeFromContentsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QIcon QCommonStyle::standardIcon(QStyle::StandardPixmap standardIcon) const```</span>
  ///
  ///
  pub fn standard_icon(&self, standard_icon: ::style::StandardPixmap) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon(self as *const ::common_style::CommonStyle, standard_icon, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::standardIcon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn standard_icon_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption)) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```virtual QIcon QCommonStyle::standardIcon(QStyle::StandardPixmap standardIcon, const QStyleOption* opt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn standard_icon_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::qt_gui::icon::Icon```<br>
  /// C++ method: <span style='color: green;'>```virtual QIcon QCommonStyle::standardIcon(QStyle::StandardPixmap standardIcon, const QStyleOption* opt = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn standard_icon_unsafe<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::icon::Icon
    where Args: overloading::CommonStyleStandardIconUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QPixmap QCommonStyle::standardPixmap(QStyle::StandardPixmap sp) const```</span>
  ///
  ///
  pub fn standard_pixmap(&self, sp: ::style::StandardPixmap) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp(self as *const ::common_style::CommonStyle, sp)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::standardPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn standard_pixmap_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption)) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```virtual QPixmap QCommonStyle::standardPixmap(QStyle::StandardPixmap sp, const QStyleOption* opt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn standard_pixmap_unsafe(&self, (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```virtual QPixmap QCommonStyle::standardPixmap(QStyle::StandardPixmap sp, const QStyleOption* opt = ?, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn standard_pixmap_unsafe<'largs, Args>(&'largs self,
                                                     args: Args)
                                                     -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>
    where Args: overloading::CommonStyleStandardPixmapUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::styleHint(QStyle::StyleHint sh) const```</span>
  ///
  ///
  pub fn style_hint(&self, sh: ::style::StyleHint) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QCommonStyle_styleHint_sh(self as *const ::common_style::CommonStyle, sh) }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::styleHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::styleHint(QStyle::StyleHint sh, const QStyleOption* opt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::styleHint(QStyle::StyleHint sh, const QStyleOption* opt = ?, const QWidget* w = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn style_hint_unsafe(&self, (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget, *mut ::style_hint_return::StyleHintReturn)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QCommonStyle::styleHint(QStyle::StyleHint sh, const QStyleOption* opt = ?, const QWidget* w = ?, QStyleHintReturn* shret = ?) const```</span>
  ///
  ///
  pub unsafe fn style_hint_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::CommonStyleStyleHintUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommonStyle::subControlRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sub_control_rect(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, ::style::SubControl)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```virtual QRect QCommonStyle::subControlRect(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sub_control_rect(&self, (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, ::style::SubControl, *const ::widget::Widget)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```virtual QRect QCommonStyle::subControlRect(QStyle::ComplexControl cc, const QStyleOptionComplex* opt, QStyle::SubControl sc, const QWidget* w = ?) const```</span>
  ///
  ///
  pub unsafe fn sub_control_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::CommonStyleSubControlRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommonStyle::subElementRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sub_element_rect(&self, (::style::SubElement, *const ::style_option::StyleOption)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```virtual QRect QCommonStyle::subElementRect(QStyle::SubElement r, const QStyleOption* opt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sub_element_rect(&self, (::style::SubElement, *const ::style_option::StyleOption, *const ::widget::Widget)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```virtual QRect QCommonStyle::subElementRect(QStyle::SubElement r, const QStyleOption* opt, const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn sub_element_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::CommonStyleSubElementRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QCommonStyle::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCommonStyle_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCommonStyle::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCommonStyle_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommonStyle::unpolish```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn unpolish(&mut self, *mut ::application::Application) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::unpolish(QApplication* application)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn unpolish(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QCommonStyle::unpolish(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn unpolish<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CommonStyleUnpolishArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::common_style::CommonStyle {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QCommonStyle_delete
  }
}

impl ::cpp_utils::DynamicCast<::common_style::CommonStyle> for ::style::Style {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::common_style::CommonStyle> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_dynamic_cast_QCommonStyle_ptr(self as *mut ::style::Style) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::common_style::CommonStyle> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_dynamic_cast_QCommonStyle_ptr(self as *const ::style::Style as *mut ::style::Style) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::common_style::CommonStyle {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QObject_ptr(self as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QObject_ptr(self as *const ::common_style::CommonStyle as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style::Style> for ::common_style::CommonStyle {
  fn static_cast_mut(&mut self) -> &mut ::style::Style {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QStyle_ptr(self as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style::Style {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QStyle_ptr(self as *const ::common_style::CommonStyle as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::common_style::CommonStyle> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::common_style::CommonStyle {
    let ffi_result =
      ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::common_style::CommonStyle {
    let ffi_result = ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::common_style::CommonStyle> for ::style::Style {
  unsafe fn static_cast_mut(&mut self) -> &mut ::common_style::CommonStyle {
    let ffi_result =
      ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QStyle(self as *mut ::style::Style);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::common_style::CommonStyle {
    let ffi_result = ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QCommonStyle_ptr_QStyle(self as *const ::style::Style as *mut ::style::Style);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::common_style::CommonStyle {
  type Target = ::style::Style;
  fn deref(&self) -> &::style::Style {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QStyle_ptr(self as *const ::common_style::CommonStyle as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::common_style::CommonStyle {
  fn deref_mut(&mut self) -> &mut ::style::Style {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCommonStyle_G_static_cast_QStyle_ptr(self as *mut ::common_style::CommonStyle) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CommonStyle::draw_complex_control](../struct.CommonStyle.html#method.draw_complex_control) method.
  pub trait CommonStyleDrawComplexControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ();
  }
  impl<'largs> CommonStyleDrawComplexControlArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> () {
      let cc = self.0;
      let opt = self.1;
      let p = self.2;
      ::ffi::qt_widgets_c_QCommonStyle_drawComplexControl_cc_opt_p(original_self as *const ::common_style::CommonStyle, cc, opt, p)
    }
  }
  impl<'largs> CommonStyleDrawComplexControlArgs<'largs>
    for (::style::ComplexControl,
                                                                  *const ::style_option_complex::StyleOptionComplex,
                                                                  *mut ::qt_gui::painter::Painter,
                                                                  *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> () {
      let cc = self.0;
      let opt = self.1;
      let p = self.2;
      let w = self.3;
      ::ffi::qt_widgets_c_QCommonStyle_drawComplexControl_cc_opt_p_w(original_self as *const ::common_style::CommonStyle, cc, opt, p, w)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::draw_control](../struct.CommonStyle.html#method.draw_control) method.
  pub trait CommonStyleDrawControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ();
  }
  impl<'largs> CommonStyleDrawControlArgs<'largs>
    for (::style::ControlElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> () {
      let element = self.0;
      let opt = self.1;
      let p = self.2;
      ::ffi::qt_widgets_c_QCommonStyle_drawControl_element_opt_p(original_self as *const ::common_style::CommonStyle,
                                                                 element,
                                                                 opt,
                                                                 p)
    }
  }
  impl<'largs> CommonStyleDrawControlArgs<'largs>
    for (::style::ControlElement,
                                                           *const ::style_option::StyleOption,
                                                           *mut ::qt_gui::painter::Painter,
                                                           *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> () {
      let element = self.0;
      let opt = self.1;
      let p = self.2;
      let w = self.3;
      ::ffi::qt_widgets_c_QCommonStyle_drawControl_element_opt_p_w(original_self as *const ::common_style::CommonStyle, element, opt, p, w)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::draw_primitive](../struct.CommonStyle.html#method.draw_primitive) method.
  pub trait CommonStyleDrawPrimitiveArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ();
  }
  impl<'largs> CommonStyleDrawPrimitiveArgs<'largs>
    for (::style::PrimitiveElement, *const ::style_option::StyleOption, *mut ::qt_gui::painter::Painter) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> () {
      let pe = self.0;
      let opt = self.1;
      let p = self.2;
      ::ffi::qt_widgets_c_QCommonStyle_drawPrimitive_pe_opt_p(original_self as *const ::common_style::CommonStyle,
                                                              pe,
                                                              opt,
                                                              p)
    }
  }
  impl<'largs> CommonStyleDrawPrimitiveArgs<'largs>
    for (::style::PrimitiveElement,
                                                             *const ::style_option::StyleOption,
                                                             *mut ::qt_gui::painter::Painter,
                                                             *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> () {
      let pe = self.0;
      let opt = self.1;
      let p = self.2;
      let w = self.3;
      ::ffi::qt_widgets_c_QCommonStyle_drawPrimitive_pe_opt_p_w(original_self as *const ::common_style::CommonStyle,
                                                                pe,
                                                                opt,
                                                                p,
                                                                w)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::hit_test_complex_control](../struct.CommonStyle.html#method.hit_test_complex_control) method.
  pub trait CommonStyleHitTestComplexControlArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::style::SubControl;
  }
  impl<'largs> CommonStyleHitTestComplexControlArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::style::SubControl {
      let cc = self.0;
      let opt = self.1;
      let pt = self.2;
      ::ffi::qt_widgets_c_QCommonStyle_hitTestComplexControl_cc_opt_pt(original_self as *const ::common_style::CommonStyle, cc, opt, pt as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> CommonStyleHitTestComplexControlArgs<'largs>
    for (::style::ComplexControl,
                                                                     *const ::style_option_complex::StyleOptionComplex,
                                                                     &'largs ::qt_core::point::Point,
                                                                     *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::style::SubControl {
      let cc = self.0;
      let opt = self.1;
      let pt = self.2;
      let w = self.3;
      ::ffi::qt_widgets_c_QCommonStyle_hitTestComplexControl_cc_opt_pt_w(original_self as *const ::common_style::CommonStyle, cc, opt, pt as *const ::qt_core::point::Point, w)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::layout_spacing_unsafe](../struct.CommonStyle.html#method.layout_spacing_unsafe) method.
  pub trait CommonStyleLayoutSpacingUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int;
  }
  impl<'largs> CommonStyleLayoutSpacingUnsafeArgs<'largs>
    for (&'largs ::size_policy::ControlType,
                                                                   &'largs ::size_policy::ControlType,
                                                                   &'largs ::qt_core::qt::Orientation,
                                                                   *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let control1 = self.0;
      let control2 = self.1;
      let orientation = self.2;
      let option = self.3;
      ::ffi::qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation_option(original_self as *const ::common_style::CommonStyle, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation, option)
    }
  }
  impl<'largs> CommonStyleLayoutSpacingUnsafeArgs<'largs>
    for (&'largs ::size_policy::ControlType,
                                                                   &'largs ::size_policy::ControlType,
                                                                   &'largs ::qt_core::qt::Orientation,
                                                                   *const ::style_option::StyleOption,
                                                                   *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let control1 = self.0;
      let control2 = self.1;
      let orientation = self.2;
      let option = self.3;
      let widget = self.4;
      ::ffi::qt_widgets_c_QCommonStyle_layoutSpacing_control1_control2_orientation_option_widget(original_self as *const ::common_style::CommonStyle, control1 as *const ::size_policy::ControlType, control2 as *const ::size_policy::ControlType, orientation as *const ::qt_core::qt::Orientation, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::pixel_metric_unsafe](../struct.CommonStyle.html#method.pixel_metric_unsafe) method.
  pub trait CommonStylePixelMetricUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int;
  }
  impl<'largs> CommonStylePixelMetricUnsafeArgs<'largs> for (::style::PixelMetric, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let m = self.0;
      let opt = self.1;
      ::ffi::qt_widgets_c_QCommonStyle_pixelMetric_m_opt(original_self as *const ::common_style::CommonStyle, m, opt)
    }
  }
  impl<'largs> CommonStylePixelMetricUnsafeArgs<'largs>
    for (::style::PixelMetric, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let m = self.0;
      let opt = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QCommonStyle_pixelMetric_m_opt_widget(original_self as *const ::common_style::CommonStyle,
                                                                m,
                                                                opt,
                                                                widget)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::polish_unsafe](../struct.CommonStyle.html#method.polish_unsafe) method.
  pub trait CommonStylePolishUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::common_style::CommonStyle) -> ();
  }
  impl<'largs> CommonStylePolishUnsafeArgs<'largs> for *mut ::application::Application {
    unsafe fn exec(self, original_self: &'largs mut ::common_style::CommonStyle) -> () {
      let app = self;
      ::ffi::qt_widgets_c_QCommonStyle_polish_app(original_self as *mut ::common_style::CommonStyle, app)
    }
  }
  impl<'largs> CommonStylePolishUnsafeArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::common_style::CommonStyle) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QCommonStyle_polish_widget(original_self as *mut ::common_style::CommonStyle, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::size_from_contents](../struct.CommonStyle.html#method.size_from_contents) method.
  pub trait CommonStyleSizeFromContentsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::size::Size;
  }
  impl<'largs> CommonStyleSizeFromContentsArgs<'largs>
    for (::style::ContentsType, *const ::style_option::StyleOption, &'largs ::qt_core::size::Size) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::size::Size {
      let ct = self.0;
      let opt = self.1;
      let contents_size = self.2;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_sizeFromContents_to_output_ct_opt_contentsSize(original_self as *const ::common_style::CommonStyle, ct, opt, contents_size as *const ::qt_core::size::Size, &mut object);
        object
      }
    }
  }
  impl<'largs> CommonStyleSizeFromContentsArgs<'largs>
    for (::style::ContentsType,
                                                                *const ::style_option::StyleOption,
                                                                &'largs ::qt_core::size::Size,
                                                                *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::size::Size {
      let ct = self.0;
      let opt = self.1;
      let contents_size = self.2;
      let widget = self.3;
      {
        let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_sizeFromContents_to_output_ct_opt_contentsSize_widget(original_self as *const ::common_style::CommonStyle, ct, opt, contents_size as *const ::qt_core::size::Size, widget, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::standard_icon_unsafe](../struct.CommonStyle.html#method.standard_icon_unsafe) method.
  pub trait CommonStyleStandardIconUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_gui::icon::Icon;
  }
  impl<'largs> CommonStyleStandardIconUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_gui::icon::Icon {
      let standard_icon = self.0;
      let opt = self.1;
      {
        let mut object: ::qt_gui::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon_opt(original_self as *const ::common_style::CommonStyle, standard_icon, opt, &mut object);
        object
      }
    }
  }
  impl<'largs> CommonStyleStandardIconUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_gui::icon::Icon {
      let standard_icon = self.0;
      let opt = self.1;
      let widget = self.2;
      {
        let mut object: ::qt_gui::icon::Icon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_standardIcon_to_output_standardIcon_opt_widget(original_self as *const ::common_style::CommonStyle, standard_icon, opt, widget, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::standard_pixmap_unsafe](../struct.CommonStyle.html#method.standard_pixmap_unsafe) method.
  pub trait CommonStyleStandardPixmapUnsafeArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs ::common_style::CommonStyle)
                   -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap>;
  }
  impl<'largs> CommonStyleStandardPixmapUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption) {
    unsafe fn exec(self,
                   original_self: &'largs ::common_style::CommonStyle)
                   -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let sp = self.0;
      let opt = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp_opt(original_self as *const ::common_style::CommonStyle, sp, opt);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'largs> CommonStyleStandardPixmapUnsafeArgs<'largs>
    for (::style::StandardPixmap, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self,
                   original_self: &'largs ::common_style::CommonStyle)
                   -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
      let sp = self.0;
      let opt = self.1;
      let widget = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QCommonStyle_standardPixmap_as_ptr_sp_opt_widget(original_self as *const ::common_style::CommonStyle, sp, opt, widget);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::style_hint_unsafe](../struct.CommonStyle.html#method.style_hint_unsafe) method.
  pub trait CommonStyleStyleHintUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int;
  }
  impl<'largs> CommonStyleStyleHintUnsafeArgs<'largs> for (::style::StyleHint, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let sh = self.0;
      let opt = self.1;
      ::ffi::qt_widgets_c_QCommonStyle_styleHint_sh_opt(original_self as *const ::common_style::CommonStyle, sh, opt)
    }
  }
  impl<'largs> CommonStyleStyleHintUnsafeArgs<'largs>
    for (::style::StyleHint, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let sh = self.0;
      let opt = self.1;
      let w = self.2;
      ::ffi::qt_widgets_c_QCommonStyle_styleHint_sh_opt_w(original_self as *const ::common_style::CommonStyle,
                                                          sh,
                                                          opt,
                                                          w)
    }
  }
  impl<'largs> CommonStyleStyleHintUnsafeArgs<'largs>
    for (::style::StyleHint,
                                                               *const ::style_option::StyleOption,
                                                               *const ::widget::Widget,
                                                               *mut ::style_hint_return::StyleHintReturn) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::libc::c_int {
      let sh = self.0;
      let opt = self.1;
      let w = self.2;
      let shret = self.3;
      ::ffi::qt_widgets_c_QCommonStyle_styleHint_sh_opt_w_shret(original_self as *const ::common_style::CommonStyle,
                                                                sh,
                                                                opt,
                                                                w,
                                                                shret)
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::sub_control_rect](../struct.CommonStyle.html#method.sub_control_rect) method.
  pub trait CommonStyleSubControlRectArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::rect::Rect;
  }
  impl<'largs> CommonStyleSubControlRectArgs<'largs>
    for (::style::ComplexControl, *const ::style_option_complex::StyleOptionComplex, ::style::SubControl) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::rect::Rect {
      let cc = self.0;
      let opt = self.1;
      let sc = self.2;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_subControlRect_to_output_cc_opt_sc(original_self as *const ::common_style::CommonStyle, cc, opt, sc, &mut object);
        object
      }
    }
  }
  impl<'largs> CommonStyleSubControlRectArgs<'largs>
    for (::style::ComplexControl,
                                                              *const ::style_option_complex::StyleOptionComplex,
                                                              ::style::SubControl,
                                                              *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::rect::Rect {
      let cc = self.0;
      let opt = self.1;
      let sc = self.2;
      let w = self.3;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_subControlRect_to_output_cc_opt_sc_w(original_self as *const ::common_style::CommonStyle, cc, opt, sc, w, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::sub_element_rect](../struct.CommonStyle.html#method.sub_element_rect) method.
  pub trait CommonStyleSubElementRectArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::rect::Rect;
  }
  impl<'largs> CommonStyleSubElementRectArgs<'largs> for (::style::SubElement, *const ::style_option::StyleOption) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::rect::Rect {
      let r = self.0;
      let opt = self.1;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_subElementRect_to_output_r_opt(original_self as *const ::common_style::CommonStyle, r, opt, &mut object);
        object
      }
    }
  }
  impl<'largs> CommonStyleSubElementRectArgs<'largs>
    for (::style::SubElement, *const ::style_option::StyleOption, *const ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs ::common_style::CommonStyle) -> ::qt_core::rect::Rect {
      let r = self.0;
      let opt = self.1;
      let widget = self.2;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QCommonStyle_subElementRect_to_output_r_opt_widget(original_self as *const ::common_style::CommonStyle, r, opt, widget, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [CommonStyle::unpolish](../struct.CommonStyle.html#method.unpolish) method.
  pub trait CommonStyleUnpolishArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::common_style::CommonStyle) -> ();
  }
  impl<'largs> CommonStyleUnpolishArgs<'largs> for *mut ::application::Application {
    unsafe fn exec(self, original_self: &'largs mut ::common_style::CommonStyle) -> () {
      let application = self;
      ::ffi::qt_widgets_c_QCommonStyle_unpolish_application(original_self as *mut ::common_style::CommonStyle,
                                                            application)
    }
  }
  impl<'largs> CommonStyleUnpolishArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::common_style::CommonStyle) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QCommonStyle_unpolish_widget(original_self as *mut ::common_style::CommonStyle, widget)
    }
  }
}
