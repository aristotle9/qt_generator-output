/// C++ type: <span style='color: green;'>```QGlyphRun```</span>
#[repr(C)]
pub struct GlyphRun([u8; ::type_sizes::QT_GUI_GLYPH_RUN_GLYPH_RUN]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for GlyphRun {
  unsafe fn new_uninitialized() -> GlyphRun {
    GlyphRun(::std::mem::uninitialized())
  }
}

impl GlyphRun {
  /// C++ method: <span style='color: green;'>```QRectF QGlyphRun::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGlyphRun_boundingRect_to_output(self as *const ::glyph_run::GlyphRun, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_clear(self as *mut ::glyph_run::GlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QGlyphRun::GlyphRunFlag> QGlyphRun::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::qt_core::flags::Flags<::glyph_run::GlyphRunFlag> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGlyphRun_flags(self as *const ::glyph_run::GlyphRun) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32> QGlyphRun::glyphIndexes() const```</span>
  ///
  ///
  pub fn glyph_indexes(&self) -> ::vector::VectorU32 {
    {
      let mut object: ::vector::VectorU32 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGlyphRun_glyphIndexes_to_output(self as *const ::glyph_run::GlyphRun, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_isEmpty(self as *const ::glyph_run::GlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::isRightToLeft() const```</span>
  ///
  ///
  pub fn is_right_to_left(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_isRightToLeft(self as *const ::glyph_run::GlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun::QGlyphRun```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::glyph_run::GlyphRun```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGlyphRun::QGlyphRun()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::glyph_run::GlyphRun) -> ::glyph_run::GlyphRun```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGlyphRun::QGlyphRun(const QGlyphRun& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::glyph_run::GlyphRun
    where Args: overloading::GlyphRunNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGlyphRun& QGlyphRun::operator=(const QGlyphRun& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::glyph_run::GlyphRun) -> &'l0 mut ::glyph_run::GlyphRun {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QGlyphRun_operator_assign(self as *mut ::glyph_run::GlyphRun,
                                                other as *const ::glyph_run::GlyphRun)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::operator==(const QGlyphRun& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::glyph_run::GlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_operator_eq(self as *const ::glyph_run::GlyphRun,
                                            other as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::operator!=(const QGlyphRun& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::glyph_run::GlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_operator_neq(self as *const ::glyph_run::GlyphRun,
                                             other as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::overline() const```</span>
  ///
  ///
  pub fn overline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_overline(self as *const ::glyph_run::GlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF> QGlyphRun::positions() const```</span>
  ///
  ///
  pub fn positions(&self) -> ::qt_core::vector::VectorPointF {
    {
      let mut object: ::qt_core::vector::VectorPointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGlyphRun_positions_to_output(self as *const ::glyph_run::GlyphRun, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRawFont QGlyphRun::rawFont() const```</span>
  ///
  ///
  pub fn raw_font(&self) -> ::raw_font::RawFont {
    {
      let mut object: ::raw_font::RawFont =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGlyphRun_rawFont_to_output(self as *const ::glyph_run::GlyphRun, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setBoundingRect(const QRectF& boundingRect)```</span>
  ///
  ///
  pub fn set_bounding_rect(&mut self, bounding_rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_setBoundingRect(self as *mut ::glyph_run::GlyphRun,
                                                bounding_rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun::setFlag```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_flag(&mut self, ::glyph_run::GlyphRunFlag) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGlyphRun::setFlag(QGlyphRun::GlyphRunFlag flag)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_flag(&mut self, (::glyph_run::GlyphRunFlag, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGlyphRun::setFlag(QGlyphRun::GlyphRunFlag flag, bool enabled = ?)```</span>
  ///
  ///
  pub fn set_flag<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GlyphRunSetFlagArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGlyphRun::setFlags(QFlags<QGlyphRun::GlyphRunFlag> flags)```</span>
  ///
  ///
  pub fn set_flags(&mut self, flags: ::qt_core::flags::Flags<::glyph_run::GlyphRunFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_setFlags(self as *mut ::glyph_run::GlyphRun,
                                         flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setGlyphIndexes(const QVector<quint32>& glyphIndexes)```</span>
  ///
  ///
  pub fn set_glyph_indexes(&mut self, glyph_indexes: &::vector::VectorU32) {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_setGlyphIndexes(self as *mut ::glyph_run::GlyphRun,
                                                glyph_indexes as *const ::vector::VectorU32)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setOverline(bool overline)```</span>
  ///
  ///
  pub fn set_overline(&mut self, overline: bool) {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_setOverline(self as *mut ::glyph_run::GlyphRun, overline) }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setPositions(const QVector<QPointF>& positions)```</span>
  ///
  ///
  pub fn set_positions(&mut self, positions: &::qt_core::vector::VectorPointF) {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_setPositions(self as *mut ::glyph_run::GlyphRun,
                                             positions as *const ::qt_core::vector::VectorPointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setRawData(const quint32* glyphIndexArray, const QPointF* glyphPositionArray, int size)```</span>
  ///
  ///
  pub unsafe fn set_raw_data(&mut self,
                             glyph_index_array: *const u32,
                             glyph_position_array: *const ::qt_core::point_f::PointF,
                             size: ::libc::c_int) {
    ::ffi::qt_gui_c_QGlyphRun_setRawData(self as *mut ::glyph_run::GlyphRun,
                                         glyph_index_array,
                                         glyph_position_array,
                                         size)
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setRawFont(const QRawFont& rawFont)```</span>
  ///
  ///
  pub fn set_raw_font(&mut self, raw_font: &::raw_font::RawFont) {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_setRawFont(self as *mut ::glyph_run::GlyphRun,
                                           raw_font as *const ::raw_font::RawFont)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setRightToLeft(bool on)```</span>
  ///
  ///
  pub fn set_right_to_left(&mut self, on: bool) {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_setRightToLeft(self as *mut ::glyph_run::GlyphRun, on) }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setStrikeOut(bool strikeOut)```</span>
  ///
  ///
  pub fn set_strike_out(&mut self, strike_out: bool) {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_setStrikeOut(self as *mut ::glyph_run::GlyphRun, strike_out) }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::setUnderline(bool underline)```</span>
  ///
  ///
  pub fn set_underline(&mut self, underline: bool) {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_setUnderline(self as *mut ::glyph_run::GlyphRun, underline) }
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::strikeOut() const```</span>
  ///
  ///
  pub fn strike_out(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_strikeOut(self as *const ::glyph_run::GlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```void QGlyphRun::swap(QGlyphRun& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QGlyphRun_swap(self as *mut ::glyph_run::GlyphRun,
                                     other as *mut ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGlyphRun::underline() const```</span>
  ///
  ///
  pub fn underline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_underline(self as *const ::glyph_run::GlyphRun) }
  }
}

impl Drop for ::glyph_run::GlyphRun {
  /// C++ method: <span style='color: green;'>```[destructor] void QGlyphRun::~QGlyphRun()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QGlyphRun_destructor(self as *mut ::glyph_run::GlyphRun) }
  }
}

/// C++ type: <span style='color: green;'>```QGlyphRun::GlyphRunFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GlyphRunFlag {
  /// C++ enum variant: <span style='color: green;'>```Overline = 1```</span>
  Overline = 1,
  /// C++ enum variant: <span style='color: green;'>```Underline = 2```</span>
  Underline = 2,
  /// C++ enum variant: <span style='color: green;'>```StrikeOut = 4```</span>
  StrikeOut = 4,
  /// C++ enum variant: <span style='color: green;'>```RightToLeft = 8```</span>
  RightToLeft = 8,
  /// C++ enum variant: <span style='color: green;'>```SplitLigature = 16```</span>
  SplitLigature = 16,
}

impl ::qt_core::flags::FlaggableEnum for GlyphRunFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "GlyphRunFlag"
  }
}

/// C++ method: <span style='color: green;'>```void swap(QGlyphRun& value1, QGlyphRun& value2)```</span>
///
///
pub fn swap(value1: &mut ::glyph_run::GlyphRun, value2: &mut ::glyph_run::GlyphRun) {
  unsafe {
    ::ffi::qt_gui_c_QGlyphRun_G_swap(value1 as *mut ::glyph_run::GlyphRun,
                                     value2 as *mut ::glyph_run::GlyphRun)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GlyphRun::new](../struct.GlyphRun.html#method.new) method.
  pub trait GlyphRunNewArgs {
    fn exec(self) -> ::glyph_run::GlyphRun;
  }
  impl GlyphRunNewArgs for () {
    fn exec(self) -> ::glyph_run::GlyphRun {

      {
        let mut object: ::glyph_run::GlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QGlyphRun_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> GlyphRunNewArgs for &'a ::glyph_run::GlyphRun {
    fn exec(self) -> ::glyph_run::GlyphRun {
      let other = self;
      {
        let mut object: ::glyph_run::GlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QGlyphRun_constructor_other(other as *const ::glyph_run::GlyphRun, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GlyphRun::set_flag](../struct.GlyphRun.html#method.set_flag) method.
  pub trait GlyphRunSetFlagArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::glyph_run::GlyphRun) -> ();
  }
  impl<'largs> GlyphRunSetFlagArgs<'largs> for ::glyph_run::GlyphRunFlag {
    fn exec(self, original_self: &'largs mut ::glyph_run::GlyphRun) -> () {
      let flag = self;
      unsafe { ::ffi::qt_gui_c_QGlyphRun_setFlag_flag(original_self as *mut ::glyph_run::GlyphRun, flag) }
    }
  }
  impl<'largs> GlyphRunSetFlagArgs<'largs> for (::glyph_run::GlyphRunFlag, bool) {
    fn exec(self, original_self: &'largs mut ::glyph_run::GlyphRun) -> () {
      let flag = self.0;
      let enabled = self.1;
      unsafe {
        ::ffi::qt_gui_c_QGlyphRun_setFlag_flag_enabled(original_self as *mut ::glyph_run::GlyphRun, flag, enabled)
      }
    }
  }
}
