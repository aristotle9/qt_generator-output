/// C++ type: <span style='color: green;'>```QPen```</span>
#[repr(C)]
pub struct Pen([u8; ::type_sizes::QT_GUI_PEN_PEN]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Pen {
  unsafe fn new_uninitialized() -> Pen {
    Pen(::std::mem::uninitialized())
  }
}

impl Pen {
  /// C++ method: <span style='color: green;'>```QVariant QPen::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPen_convert_to_QVariant_to_output(self as *const ::pen::Pen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QPen::brush() const```</span>
  ///
  ///
  pub fn brush(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPen_brush_to_output(self as *const ::pen::Pen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QPen::color() const```</span>
  ///
  ///
  pub fn color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPen_color_to_output(self as *const ::pen::Pen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPen::dashOffset() const```</span>
  ///
  ///
  pub fn dash_offset(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPen_dashOffset(self as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```QVector<double> QPen::dashPattern() const```</span>
  ///
  ///
  pub fn dash_pattern(&self) -> ::vector::VectorCDouble {
    {
      let mut object: ::vector::VectorCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPen_dashPattern_to_output(self as *const ::pen::Pen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPen::isCosmetic() const```</span>
  ///
  ///
  pub fn is_cosmetic(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPen_isCosmetic(self as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```bool QPen::isDetached()```</span>
  ///
  ///
  pub fn is_detached(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPen_isDetached(self as *mut ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```bool QPen::isSolid() const```</span>
  ///
  ///
  pub fn is_solid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPen_isSolid(self as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```double QPen::miterLimit() const```</span>
  ///
  ///
  pub fn miter_limit(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPen_miterLimit(self as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```QPen::QPen```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::PenStyle) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(Qt::PenStyle arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::brush::Brush, ::libc::c_double)) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(const QBrush& brush, double width)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::brush::Brush, ::libc::c_double, &::qt_core::qt::PenStyle)) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(const QBrush& brush, double width, Qt::PenStyle s = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::brush::Brush, ::libc::c_double, &::qt_core::qt::PenStyle, &::qt_core::qt::PenCapStyle)) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(const QBrush& brush, double width, Qt::PenStyle s = ?, Qt::PenCapStyle c = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::brush::Brush, ::libc::c_double, &::qt_core::qt::PenStyle, &::qt_core::qt::PenCapStyle, &::qt_core::qt::PenJoinStyle)) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(const QBrush& brush, double width, Qt::PenStyle s = ?, Qt::PenCapStyle c = ?, Qt::PenJoinStyle j = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::color::Color) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new(&::pen::Pen) -> ::pen::Pen```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPen::QPen(const QPen& pen)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pen::Pen
    where Args: overloading::PenNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPen& QPen::operator=(const QPen& pen)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, pen: &'l1 ::pen::Pen) -> &'l0 mut ::pen::Pen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPen_operator_assign(self as *mut ::pen::Pen, pen as *const ::pen::Pen) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPen::operator==(const QPen& p) const```</span>
  ///
  ///
  pub fn op_eq(&self, p: &::pen::Pen) -> bool {
    unsafe { ::ffi::qt_gui_c_QPen_operator_eq(self as *const ::pen::Pen, p as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```bool QPen::operator!=(const QPen& p) const```</span>
  ///
  ///
  pub fn op_neq(&self, p: &::pen::Pen) -> bool {
    unsafe { ::ffi::qt_gui_c_QPen_operator_neq(self as *const ::pen::Pen, p as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_brush(&mut self, brush: &::brush::Brush) {
    unsafe { ::ffi::qt_gui_c_QPen_setBrush(self as *mut ::pen::Pen, brush as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setCapStyle(Qt::PenCapStyle pcs)```</span>
  ///
  ///
  pub fn set_cap_style(&mut self, pcs: &::qt_core::qt::PenCapStyle) {
    unsafe {
      ::ffi::qt_gui_c_QPen_setCapStyle(self as *mut ::pen::Pen,
                                       pcs as *const ::qt_core::qt::PenCapStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_color(&mut self, color: &::color::Color) {
    unsafe { ::ffi::qt_gui_c_QPen_setColor(self as *mut ::pen::Pen, color as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setCosmetic(bool cosmetic)```</span>
  ///
  ///
  pub fn set_cosmetic(&mut self, cosmetic: bool) {
    unsafe { ::ffi::qt_gui_c_QPen_setCosmetic(self as *mut ::pen::Pen, cosmetic) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setDashOffset(double doffset)```</span>
  ///
  ///
  pub fn set_dash_offset(&mut self, doffset: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPen_setDashOffset(self as *mut ::pen::Pen, doffset) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setDashPattern(const QVector<double>& pattern)```</span>
  ///
  ///
  pub fn set_dash_pattern(&mut self, pattern: &::vector::VectorCDouble) {
    unsafe {
      ::ffi::qt_gui_c_QPen_setDashPattern(self as *mut ::pen::Pen,
                                          pattern as *const ::vector::VectorCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setJoinStyle(Qt::PenJoinStyle pcs)```</span>
  ///
  ///
  pub fn set_join_style(&mut self, pcs: &::qt_core::qt::PenJoinStyle) {
    unsafe {
      ::ffi::qt_gui_c_QPen_setJoinStyle(self as *mut ::pen::Pen,
                                        pcs as *const ::qt_core::qt::PenJoinStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setMiterLimit(double limit)```</span>
  ///
  ///
  pub fn set_miter_limit(&mut self, limit: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPen_setMiterLimit(self as *mut ::pen::Pen, limit) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setStyle(Qt::PenStyle arg1)```</span>
  ///
  ///
  pub fn set_style(&mut self, arg1: &::qt_core::qt::PenStyle) {
    unsafe {
      ::ffi::qt_gui_c_QPen_setStyle(self as *mut ::pen::Pen,
                                    arg1 as *const ::qt_core::qt::PenStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setWidth(int width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QPen_setWidth(self as *mut ::pen::Pen, width) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::setWidthF(double width)```</span>
  ///
  ///
  pub fn set_width_f(&mut self, width: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPen_setWidthF(self as *mut ::pen::Pen, width) }
  }

  /// C++ method: <span style='color: green;'>```void QPen::swap(QPen& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pen::Pen) {
    unsafe { ::ffi::qt_gui_c_QPen_swap(self as *mut ::pen::Pen, other as *mut ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```int QPen::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPen_width(self as *const ::pen::Pen) }
  }

  /// C++ method: <span style='color: green;'>```double QPen::widthF() const```</span>
  ///
  ///
  pub fn width_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPen_widthF(self as *const ::pen::Pen) }
  }
}

impl Drop for ::pen::Pen {
  /// C++ method: <span style='color: green;'>```[destructor] void QPen::~QPen()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPen_destructor(self as *mut ::pen::Pen) }
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::pen::Pen)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPen& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::pen::Pen)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPen& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPen& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::pen::Pen)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QPen_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                        arg2 as *mut ::pen::Pen)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QPen& value1, QPen& value2)```</span>
///
///
pub fn swap(value1: &mut ::pen::Pen, value2: &mut ::pen::Pen) {
  unsafe { ::ffi::qt_gui_c_QPen_G_swap(value1 as *mut ::pen::Pen, value2 as *mut ::pen::Pen) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Pen::new](../struct.Pen.html#method.new) method.
  pub trait PenNewArgs {
    fn exec(self) -> ::pen::Pen;
  }
  impl<'a> PenNewArgs for &'a ::qt_core::qt::PenStyle {
    fn exec(self) -> ::pen::Pen {
      let arg1 = self;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_arg1(arg1 as *const ::qt_core::qt::PenStyle, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PenNewArgs for (&'a ::brush::Brush, ::libc::c_double) {
    fn exec(self) -> ::pen::Pen {
      let brush = self.0;
      let width = self.1;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_brush_width(brush as *const ::brush::Brush, width, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PenNewArgs for (&'a ::brush::Brush, ::libc::c_double, &'a ::qt_core::qt::PenStyle) {
    fn exec(self) -> ::pen::Pen {
      let brush = self.0;
      let width = self.1;
      let s = self.2;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_brush_width_s(brush as *const ::brush::Brush,
                                                         width,
                                                         s as *const ::qt_core::qt::PenStyle,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> PenNewArgs
    for (&'a ::brush::Brush, ::libc::c_double, &'a ::qt_core::qt::PenStyle, &'a ::qt_core::qt::PenCapStyle) {
    fn exec(self) -> ::pen::Pen {
      let brush = self.0;
      let width = self.1;
      let s = self.2;
      let c = self.3;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_brush_width_s_c(brush as *const ::brush::Brush,
                                                           width,
                                                           s as *const ::qt_core::qt::PenStyle,
                                                           c as *const ::qt_core::qt::PenCapStyle,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> PenNewArgs
    for (&'a ::brush::Brush,
                               ::libc::c_double,
                               &'a ::qt_core::qt::PenStyle,
                               &'a ::qt_core::qt::PenCapStyle,
                               &'a ::qt_core::qt::PenJoinStyle) {
    fn exec(self) -> ::pen::Pen {
      let brush = self.0;
      let width = self.1;
      let s = self.2;
      let c = self.3;
      let j = self.4;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_brush_width_s_c_j(brush as *const ::brush::Brush,
                                                             width,
                                                             s as *const ::qt_core::qt::PenStyle,
                                                             c as *const ::qt_core::qt::PenCapStyle,
                                                             j as *const ::qt_core::qt::PenJoinStyle,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> PenNewArgs for &'a ::color::Color {
    fn exec(self) -> ::pen::Pen {
      let color = self;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_color(color as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  impl PenNewArgs for () {
    fn exec(self) -> ::pen::Pen {

      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PenNewArgs for &'a ::pen::Pen {
    fn exec(self) -> ::pen::Pen {
      let pen = self;
      {
        let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_constructor_pen(pen as *const ::pen::Pen, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::pen::Pen) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QPen_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                            arg2 as *const ::pen::Pen)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::pen::Pen) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPen_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                        arg2 as *const ::pen::Pen,
                                                        &mut object);
        }
        object
      }
    }
  }
}
