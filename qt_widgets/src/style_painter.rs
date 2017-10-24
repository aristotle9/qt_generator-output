/// C++ type: <span style='color: green;'>```QStylePainter```</span>
#[repr(C)]
pub struct StylePainter([u8; ::type_sizes::QT_WIDGETS_STYLE_PAINTER_STYLE_PAINTER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StylePainter {
  unsafe fn new_uninitialized() -> StylePainter {
    StylePainter(::std::mem::uninitialized())
  }
}

impl StylePainter {
  /// C++ method: <span style='color: green;'>```QStylePainter::begin```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn begin(&mut self, (*mut ::qt_gui::paint_device::PaintDevice, *mut ::widget::Widget)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStylePainter::begin(QPaintDevice* pd, QWidget* w)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn begin(&mut self, *mut ::widget::Widget) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStylePainter::begin(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn begin<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StylePainterBeginArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStylePainter::drawComplexControl(QStyle::ComplexControl cc, const QStyleOptionComplex& opt)```</span>
  ///
  ///
  pub fn draw_complex_control(&mut self,
                              cc: &::style::ComplexControl,
                              opt: &::style_option_complex::StyleOptionComplex) {
    unsafe {
      ::ffi::qt_widgets_c_QStylePainter_drawComplexControl(self as *mut ::style_painter::StylePainter,
                                                           cc as *const ::style::ComplexControl,
                                                           opt as *const ::style_option_complex::StyleOptionComplex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStylePainter::drawControl(QStyle::ControlElement ce, const QStyleOption& opt)```</span>
  ///
  ///
  pub fn draw_control(&mut self, ce: &::style::ControlElement, opt: &::style_option::StyleOption) {
    unsafe {
      ::ffi::qt_widgets_c_QStylePainter_drawControl(self as *mut ::style_painter::StylePainter,
                                                    ce as *const ::style::ControlElement,
                                                    opt as *const ::style_option::StyleOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStylePainter::drawItemPixmap(const QRect& r, int flags, const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn draw_item_pixmap(&mut self,
                          r: &::qt_core::rect::Rect,
                          flags: ::libc::c_int,
                          pixmap: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QStylePainter_drawItemPixmap(self as *mut ::style_painter::StylePainter,
                                                       r as *const ::qt_core::rect::Rect,
                                                       flags,
                                                       pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```QStylePainter::drawItemText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_item_text(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_gui::palette::Palette, bool, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStylePainter::drawItemText(const QRect& r, int flags, const QPalette& pal, bool enabled, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_item_text(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_gui::palette::Palette, bool, &::qt_core::string::String, &::qt_gui::palette::ColorRole)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStylePainter::drawItemText(const QRect& r, int flags, const QPalette& pal, bool enabled, const QString& text, QPalette::ColorRole textRole = ?)```</span>
  ///
  ///
  pub fn draw_item_text<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StylePainterDrawItemTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStylePainter::drawPrimitive(QStyle::PrimitiveElement pe, const QStyleOption& opt)```</span>
  ///
  ///
  pub fn draw_primitive(&mut self, pe: &::style::PrimitiveElement, opt: &::style_option::StyleOption) {
    unsafe {
      ::ffi::qt_widgets_c_QStylePainter_drawPrimitive(self as *mut ::style_painter::StylePainter,
                                                      pe as *const ::style::PrimitiveElement,
                                                      opt as *const ::style_option::StyleOption)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStylePainter::QStylePainter()```</span>
  ///
  ///
  pub fn new() -> ::style_painter::StylePainter {
    {
      let mut object: ::style_painter::StylePainter =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStylePainter_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStylePainter::QStylePainter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_gui::paint_device::PaintDevice, *mut ::widget::Widget)) -> ::style_painter::StylePainter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStylePainter::QStylePainter(QPaintDevice* pd, QWidget* w)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::style_painter::StylePainter```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStylePainter::QStylePainter(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::style_painter::StylePainter
    where Args: overloading::StylePainterNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStyle* QStylePainter::style() const```</span>
  ///
  ///
  pub fn style(&self) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QStylePainter_style(self as *const ::style_painter::StylePainter) }
  }
}

impl Drop for ::style_painter::StylePainter {
  /// C++ method: <span style='color: green;'>```[destructor] void QStylePainter::~QStylePainter()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QStylePainter_destructor(self as *mut ::style_painter::StylePainter) }
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::painter::Painter> for ::style_painter::StylePainter {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::painter::Painter {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStylePainter_G_static_cast_QPainter_ptr(self as *mut ::style_painter::StylePainter)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::painter::Painter {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStylePainter_G_static_cast_QPainter_ptr(self as *const ::style_painter::StylePainter as *mut ::style_painter::StylePainter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_painter::StylePainter> for ::qt_gui::painter::Painter {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_painter::StylePainter {
    let ffi_result =
      ::ffi::qt_widgets_c_QStylePainter_G_static_cast_QStylePainter_ptr(self as *mut ::qt_gui::painter::Painter);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_painter::StylePainter {
    let ffi_result = ::ffi::qt_widgets_c_QStylePainter_G_static_cast_QStylePainter_ptr(self as *const ::qt_gui::painter::Painter as *mut ::qt_gui::painter::Painter);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_painter::StylePainter {
  type Target = ::qt_gui::painter::Painter;
  fn deref(&self) -> &::qt_gui::painter::Painter {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStylePainter_G_static_cast_QPainter_ptr(self as *const ::style_painter::StylePainter as *mut ::style_painter::StylePainter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_painter::StylePainter {
  fn deref_mut(&mut self) -> &mut ::qt_gui::painter::Painter {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStylePainter_G_static_cast_QPainter_ptr(self as *mut ::style_painter::StylePainter)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StylePainter::begin](../struct.StylePainter.html#method.begin) method.
  pub trait StylePainterBeginArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::style_painter::StylePainter) -> bool;
  }
  impl<'largs> StylePainterBeginArgs<'largs> for (*mut ::qt_gui::paint_device::PaintDevice, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::style_painter::StylePainter) -> bool {
      let pd = self.0;
      let w = self.1;
      ::ffi::qt_widgets_c_QStylePainter_begin_pd_w(original_self as *mut ::style_painter::StylePainter, pd, w)
    }
  }
  impl<'largs> StylePainterBeginArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::style_painter::StylePainter) -> bool {
      let w = self;
      ::ffi::qt_widgets_c_QStylePainter_begin_w(original_self as *mut ::style_painter::StylePainter, w)
    }
  }
  /// This trait represents a set of arguments accepted by [StylePainter::draw_item_text](../struct.StylePainter.html#method.draw_item_text) method.
  pub trait StylePainterDrawItemTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::style_painter::StylePainter) -> ();
  }
  impl<'largs> StylePainterDrawItemTextArgs<'largs>
    for (&'largs ::qt_core::rect::Rect,
                                                             ::libc::c_int,
                                                             &'largs ::qt_gui::palette::Palette,
                                                             bool,
                                                             &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::style_painter::StylePainter) -> () {
      let r = self.0;
      let flags = self.1;
      let pal = self.2;
      let enabled = self.3;
      let text = self.4;
      unsafe { ::ffi::qt_widgets_c_QStylePainter_drawItemText_r_flags_pal_enabled_text(original_self as *mut ::style_painter::StylePainter, r as *const ::qt_core::rect::Rect, flags, pal as *const ::qt_gui::palette::Palette, enabled, text as *const ::qt_core::string::String) }
    }
  }
  impl<'largs> StylePainterDrawItemTextArgs<'largs>
    for (&'largs ::qt_core::rect::Rect,
                                                             ::libc::c_int,
                                                             &'largs ::qt_gui::palette::Palette,
                                                             bool,
                                                             &'largs ::qt_core::string::String,
                                                             &'largs ::qt_gui::palette::ColorRole) {
    fn exec(self, original_self: &'largs mut ::style_painter::StylePainter) -> () {
      let r = self.0;
      let flags = self.1;
      let pal = self.2;
      let enabled = self.3;
      let text = self.4;
      let text_role = self.5;
      unsafe { ::ffi::qt_widgets_c_QStylePainter_drawItemText_r_flags_pal_enabled_text_textRole(original_self as *mut ::style_painter::StylePainter, r as *const ::qt_core::rect::Rect, flags, pal as *const ::qt_gui::palette::Palette, enabled, text as *const ::qt_core::string::String, text_role as *const ::qt_gui::palette::ColorRole) }
    }
  }
  /// This trait represents a set of arguments accepted by [StylePainter::new_unsafe](../struct.StylePainter.html#method.new_unsafe) method.
  pub trait StylePainterNewUnsafeArgs {
    unsafe fn exec(self) -> ::style_painter::StylePainter;
  }
  impl StylePainterNewUnsafeArgs for (*mut ::qt_gui::paint_device::PaintDevice, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::style_painter::StylePainter {
      let pd = self.0;
      let w = self.1;
      {
        let mut object: ::style_painter::StylePainter =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStylePainter_constructor_pd_w(pd, w, &mut object);
        object
      }
    }
  }
  impl StylePainterNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::style_painter::StylePainter {
      let w = self;
      {
        let mut object: ::style_painter::StylePainter =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QStylePainter_constructor_w(w, &mut object);
        object
      }
    }
  }
}
