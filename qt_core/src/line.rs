/// C++ type: <span style='color: green;'>```QLine```</span>
#[repr(C)]
pub struct Line([u8; ::type_sizes::QT_CORE_LINE_LINE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Line {
  unsafe fn new_uninitialized() -> Line {
    Line(::std::mem::uninitialized())
  }
}

impl Line {
  /// C++ method: <span style='color: green;'>```QPoint QLine::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLine_center_to_output(self as *const ::line::Line, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QLine::dx() const```</span>
  ///
  ///
  pub fn dx(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLine_dx(self as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```int QLine::dy() const```</span>
  ///
  ///
  pub fn dy(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLine_dy(self as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```bool QLine::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLine_isNull(self as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```QLine::QLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::line::Line```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLine::QLine()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::point::Point, &::point::Point)) -> ::line::Line```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLine::QLine(const QPoint& pt1, const QPoint& pt2)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::line::Line```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLine::QLine(int x1, int y1, int x2, int y2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::line::Line
    where Args: overloading::LineNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QLine::operator==(const QLine& d) const```</span>
  ///
  ///
  pub fn op_eq(&self, d: &::line::Line) -> bool {
    unsafe { ::ffi::qt_core_c_QLine_operator_eq(self as *const ::line::Line, d as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```bool QLine::operator!=(const QLine& d) const```</span>
  ///
  ///
  pub fn op_neq(&self, d: &::line::Line) -> bool {
    unsafe { ::ffi::qt_core_c_QLine_operator_neq(self as *const ::line::Line, d as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QLine::p1() const```</span>
  ///
  ///
  pub fn p1(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLine_p1_to_output(self as *const ::line::Line, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QLine::p2() const```</span>
  ///
  ///
  pub fn p2(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLine_p2_to_output(self as *const ::line::Line, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QLine::setLine(int x1, int y1, int x2, int y2)```</span>
  ///
  ///
  pub fn set_line(&mut self, x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QLine_setLine(self as *mut ::line::Line, x1, y1, x2, y2) }
  }

  /// C++ method: <span style='color: green;'>```void QLine::setP1(const QPoint& p1)```</span>
  ///
  ///
  pub fn set_p1(&mut self, p1: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QLine_setP1(self as *mut ::line::Line, p1 as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QLine::setP2(const QPoint& p2)```</span>
  ///
  ///
  pub fn set_p2(&mut self, p2: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QLine_setP2(self as *mut ::line::Line, p2 as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QLine::setPoints(const QPoint& p1, const QPoint& p2)```</span>
  ///
  ///
  pub fn set_points(&mut self, p1: &::point::Point, p2: &::point::Point) {
    unsafe {
      ::ffi::qt_core_c_QLine_setPoints(self as *mut ::line::Line,
                                       p1 as *const ::point::Point,
                                       p2 as *const ::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QLine::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLine::translate(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLine::translate(int dx, int dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LineTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLine::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::point::Point) -> ::line::Line```<br>
  /// C++ method: <span style='color: green;'>```QLine QLine::translated(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_int, ::libc::c_int)) -> ::line::Line```<br>
  /// C++ method: <span style='color: green;'>```QLine QLine::translated(int dx, int dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::line::Line
    where Args: overloading::LineTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QLine::x1() const```</span>
  ///
  ///
  pub fn x1(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLine_x1(self as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```int QLine::x2() const```</span>
  ///
  ///
  pub fn x2(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLine_x2(self as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```int QLine::y1() const```</span>
  ///
  ///
  pub fn y1(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLine_y1(self as *const ::line::Line) }
  }

  /// C++ method: <span style='color: green;'>```int QLine::y2() const```</span>
  ///
  ///
  pub fn y2(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLine_y2(self as *const ::line::Line) }
  }
}

impl Drop for ::line::Line {
  /// C++ method: <span style='color: green;'>```[destructor] void QLine::~QLine()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QLine_destructor(self as *mut ::line::Line) }
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::line::Line)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QLine& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::line_f::LineF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QLineF& arg2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::line::Line)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug d, const QLine& p)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::line_f::LineF)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug d, const QLineF& p)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator>>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::line::Line)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QLine& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::line_f::LineF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QLineF& arg2)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Line::new](../struct.Line.html#method.new) method.
  pub trait LineNewArgs {
    fn exec(self) -> ::line::Line;
  }
  impl LineNewArgs for () {
    fn exec(self) -> ::line::Line {

      {
        let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> LineNewArgs for (&'a ::point::Point, &'a ::point::Point) {
    fn exec(self) -> ::line::Line {
      let pt1 = self.0;
      let pt2 = self.1;
      {
        let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_constructor_pt1_pt2(pt1 as *const ::point::Point,
                                                     pt2 as *const ::point::Point,
                                                     &mut object);
        }
        object
      }
    }
  }
  impl LineNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::line::Line {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      {
        let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_constructor_x1_y1_x2_y2(x1, y1, x2, y2, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Line::translate](../struct.Line.html#method.translate) method.
  pub trait LineTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::line::Line) -> ();
  }
  impl<'largs> LineTranslateArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::line::Line) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_core_c_QLine_translate_dx_dy(original_self as *mut ::line::Line, dx, dy) }
    }
  }
  impl<'largs> LineTranslateArgs<'largs> for &'largs ::point::Point {
    fn exec(self, original_self: &'largs mut ::line::Line) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QLine_translate_p(original_self as *mut ::line::Line,
                                           p as *const ::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Line::translated](../struct.Line.html#method.translated) method.
  pub trait LineTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::line::Line) -> ::line::Line;
  }
  impl<'largs> LineTranslatedArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::line::Line) -> ::line::Line {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_translated_to_output_dx_dy(original_self as *const ::line::Line, dx, dy, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> LineTranslatedArgs<'largs> for &'largs ::point::Point {
    fn exec(self, original_self: &'largs ::line::Line) -> ::line::Line {
      let p = self;
      {
        let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_translated_to_output_p(original_self as *const ::line::Line,
                                                        p as *const ::point::Point,
                                                        &mut object);
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
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::line::Line) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QLine_G_operator_shl_QDataStream_QLine(arg1 as *mut ::data_stream::DataStream,
                                                                arg2 as *const ::line::Line)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::line_f::LineF) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QLine_G_operator_shl_QDataStream_QLineF(arg1 as *mut ::data_stream::DataStream,
                                                                   arg2 as *const ::line_f::LineF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::line::Line) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let d = self.0;
      let p = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_G_operator_shl_to_output_QDebug_QLine(d as *const ::debug::Debug,
                                                                       p as *const ::line::Line,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::line_f::LineF) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let d = self.0;
      let p = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLine_G_operator_shl_to_output_QDebug_QLineF(d as *const ::debug::Debug,
                                                                        p as *const ::line_f::LineF,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::line::Line) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QLine_G_operator_shr_QDataStream_QLine(arg1 as *mut ::data_stream::DataStream,
                                                                arg2 as *mut ::line::Line)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::line_f::LineF) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QLine_G_operator_shr_QDataStream_QLineF(arg1 as *mut ::data_stream::DataStream,
                                                                   arg2 as *mut ::line_f::LineF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
