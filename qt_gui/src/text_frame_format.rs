/// C++ type: <span style='color: green;'>```QTextFrameFormat::BorderStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BorderStyle {
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Dotted = 1```</span>
  Dotted = 1,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Dashed = 2```</span>
  Dashed = 2,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Solid = 3```</span>
  Solid = 3,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Double = 4```</span>
  Double = 4,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_DotDash = 5```</span>
  DotDash = 5,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_DotDotDash = 6```</span>
  DotDotDash = 6,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Groove = 7```</span>
  Groove = 7,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Ridge = 8```</span>
  Ridge = 8,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Inset = 9```</span>
  Inset = 9,
  /// C++ enum variant: <span style='color: green;'>```BorderStyle_Outset = 10```</span>
  Outset = 10,
}

/// C++ type: <span style='color: green;'>```QTextFrameFormat::Position```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Position {
  /// C++ enum variant: <span style='color: green;'>```InFlow = 0```</span>
  InFlow = 0,
  /// C++ enum variant: <span style='color: green;'>```FloatLeft = 1```</span>
  FloatLeft = 1,
  /// C++ enum variant: <span style='color: green;'>```FloatRight = 2```</span>
  FloatRight = 2,
}

/// C++ type: <span style='color: green;'>```QTextFrameFormat```</span>
#[repr(C)]
pub struct TextFrameFormat([u8; ::type_sizes::QT_GUI_TEXT_FRAME_FORMAT_TEXT_FRAME_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextFrameFormat {
  unsafe fn new_uninitialized() -> TextFrameFormat {
    TextFrameFormat(::std::mem::uninitialized())
  }
}

impl TextFrameFormat {
  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::border() const```</span>
  ///
  ///
  pub fn border(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_border(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTextFrameFormat::borderBrush() const```</span>
  ///
  ///
  pub fn border_brush(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_borderBrush_to_output(self as *const ::text_frame_format::TextFrameFormat,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrameFormat::BorderStyle QTextFrameFormat::borderStyle() const```</span>
  ///
  ///
  pub fn border_style(&self) -> ::text_frame_format::BorderStyle {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_borderStyle(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::bottomMargin() const```</span>
  ///
  ///
  pub fn bottom_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_bottomMargin(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextLength QTextFrameFormat::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::text_length::TextLength {
    {
      let mut object: ::text_length::TextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_height_to_output(self as *const ::text_frame_format::TextFrameFormat,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFrameFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_isValid(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::leftMargin() const```</span>
  ///
  ///
  pub fn left_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_leftMargin(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::margin() const```</span>
  ///
  ///
  pub fn margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_margin(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextFrameFormat::QTextFrameFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_frame_format::TextFrameFormat {
    {
      let mut object: ::text_frame_format::TextFrameFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::padding() const```</span>
  ///
  ///
  pub fn padding(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_padding(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QTextFormat::PageBreakFlag> QTextFrameFormat::pageBreakPolicy() const```</span>
  ///
  ///
  pub fn page_break_policy(&self) -> ::qt_core::flags::Flags<::text_format::PageBreakFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrameFormat_pageBreakPolicy(self as *const ::text_frame_format::TextFrameFormat) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QTextFrameFormat::Position QTextFrameFormat::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::text_frame_format::Position {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_position(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::rightMargin() const```</span>
  ///
  ///
  pub fn right_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_rightMargin(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setBorder(double border)```</span>
  ///
  ///
  pub fn set_border(&mut self, border: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setBorder(self as *mut ::text_frame_format::TextFrameFormat, border) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setBorderBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_border_brush(&mut self, brush: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QTextFrameFormat_setBorderBrush(self as *mut ::text_frame_format::TextFrameFormat,
                                                      brush as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setBorderStyle(QTextFrameFormat::BorderStyle style)```</span>
  ///
  ///
  pub fn set_border_style(&mut self, style: ::text_frame_format::BorderStyle) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setBorderStyle(self as *mut ::text_frame_format::TextFrameFormat, style) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setBottomMargin(double margin)```</span>
  ///
  ///
  pub fn set_bottom_margin(&mut self, margin: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextFrameFormat_setBottomMargin(self as *mut ::text_frame_format::TextFrameFormat, margin)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrameFormat::setHeight```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_height(&mut self, &::text_length::TextLength) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setHeight(const QTextLength& height)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_height(&mut self, ::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setHeight(double height)```</span>
  ///
  ///
  pub fn set_height<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextFrameFormatSetHeightArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setLeftMargin(double margin)```</span>
  ///
  ///
  pub fn set_left_margin(&mut self, margin: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setLeftMargin(self as *mut ::text_frame_format::TextFrameFormat, margin) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setMargin(double margin)```</span>
  ///
  ///
  pub fn set_margin(&mut self, margin: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setMargin(self as *mut ::text_frame_format::TextFrameFormat, margin) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setPadding(double padding)```</span>
  ///
  ///
  pub fn set_padding(&mut self, padding: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setPadding(self as *mut ::text_frame_format::TextFrameFormat, padding) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setPageBreakPolicy(QFlags<QTextFormat::PageBreakFlag> flags)```</span>
  ///
  ///
  pub fn set_page_break_policy(&mut self, flags: ::qt_core::flags::Flags<::text_format::PageBreakFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QTextFrameFormat_setPageBreakPolicy(self as *mut ::text_frame_format::TextFrameFormat,
                                                          flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setPosition(QTextFrameFormat::Position f)```</span>
  ///
  ///
  pub fn set_position(&mut self, f: ::text_frame_format::Position) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setPosition(self as *mut ::text_frame_format::TextFrameFormat, f) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setRightMargin(double margin)```</span>
  ///
  ///
  pub fn set_right_margin(&mut self, margin: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextFrameFormat_setRightMargin(self as *mut ::text_frame_format::TextFrameFormat, margin)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setTopMargin(double margin)```</span>
  ///
  ///
  pub fn set_top_margin(&mut self, margin: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setTopMargin(self as *mut ::text_frame_format::TextFrameFormat, margin) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrameFormat::setWidth```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_width(&mut self, &::text_length::TextLength) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setWidth(const QTextLength& length)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_width(&mut self, ::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextFrameFormat::setWidth(double width)```</span>
  ///
  ///
  pub fn set_width<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextFrameFormatSetWidthArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QTextFrameFormat::topMargin() const```</span>
  ///
  ///
  pub fn top_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_topMargin(self as *const ::text_frame_format::TextFrameFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextLength QTextFrameFormat::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::text_length::TextLength {
    {
      let mut object: ::text_length::TextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_width_to_output(self as *const ::text_frame_format::TextFrameFormat,
                                                         &mut object);
      }
      object
    }
  }
}

impl Drop for ::text_frame_format::TextFrameFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextFrameFormat::~QTextFrameFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextFrameFormat_destructor(self as *mut ::text_frame_format::TextFrameFormat) }
  }
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_frame_format::TextFrameFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrameFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_frame_format::TextFrameFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrameFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_frame_format::TextFrameFormat as *mut ::text_frame_format::TextFrameFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_frame_format::TextFrameFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_frame_format::TextFrameFormat {
    let ffi_result =
      ::ffi::qt_gui_c_QTextFrameFormat_G_static_cast_QTextFrameFormat_ptr(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_frame_format::TextFrameFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextFrameFormat_G_static_cast_QTextFrameFormat_ptr(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_frame_format::TextFrameFormat {
  type Target = ::text_format::TextFormat;
  fn deref(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrameFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_frame_format::TextFrameFormat as *mut ::text_frame_format::TextFrameFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_frame_format::TextFrameFormat {
  fn deref_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrameFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_frame_format::TextFrameFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextFrameFormat::set_height](../struct.TextFrameFormat.html#method.set_height) method.
  pub trait TextFrameFormatSetHeightArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_frame_format::TextFrameFormat) -> ();
  }
  impl<'largs> TextFrameFormatSetHeightArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs mut ::text_frame_format::TextFrameFormat) -> () {
      let height = self;
      unsafe { ::ffi::qt_gui_c_QTextFrameFormat_setHeight_QTextLength(original_self as *mut ::text_frame_format::TextFrameFormat, height as *const ::text_length::TextLength) }
    }
  }
  impl<'largs> TextFrameFormatSetHeightArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::text_frame_format::TextFrameFormat) -> () {
      let height = self;
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_setHeight_double(original_self as *mut ::text_frame_format::TextFrameFormat,
                                                          height)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextFrameFormat::set_width](../struct.TextFrameFormat.html#method.set_width) method.
  pub trait TextFrameFormatSetWidthArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_frame_format::TextFrameFormat) -> ();
  }
  impl<'largs> TextFrameFormatSetWidthArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs mut ::text_frame_format::TextFrameFormat) -> () {
      let length = self;
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_setWidth_length(original_self as *mut ::text_frame_format::TextFrameFormat,
                                                         length as *const ::text_length::TextLength)
      }
    }
  }
  impl<'largs> TextFrameFormatSetWidthArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::text_frame_format::TextFrameFormat) -> () {
      let width = self;
      unsafe {
        ::ffi::qt_gui_c_QTextFrameFormat_setWidth_width(original_self as *mut ::text_frame_format::TextFrameFormat,
                                                        width)
      }
    }
  }
}
