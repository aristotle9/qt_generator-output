/// C++ type: <span style='color: green;'>```QMatrix```</span>
#[repr(C)]
pub struct Matrix([u8; ::type_sizes::QT_GUI_MATRIX_MATRIX]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Matrix {
  unsafe fn new_uninitialized() -> Matrix {
    Matrix(::std::mem::uninitialized())
  }
}

impl Matrix {
  /// C++ method: <span style='color: green;'>```QVariant QMatrix::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMatrix_convert_to_QVariant_to_output(self as *const ::matrix::Matrix, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::determinant() const```</span>
  ///
  ///
  pub fn determinant(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_determinant(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::dx() const```</span>
  ///
  ///
  pub fn dx(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_dx(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::dy() const```</span>
  ///
  ///
  pub fn dy(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_dy(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QMatrix::inverted() const```</span>
  ///
  ///
  pub fn inverted(&self) -> ::matrix::Matrix {
    {
      let mut object: ::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMatrix_inverted_to_output_no_args(self as *const ::matrix::Matrix, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QMatrix::inverted(bool* invertible = ?) const```</span>
  ///
  ///
  pub unsafe fn inverted_unsafe(&self, invertible: *mut bool) -> ::matrix::Matrix {
    {
      let mut object: ::matrix::Matrix = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QMatrix_inverted_to_output_invertible(self as *const ::matrix::Matrix, invertible, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix::isIdentity() const```</span>
  ///
  ///
  pub fn is_identity(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QMatrix_isIdentity(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix::isInvertible() const```</span>
  ///
  ///
  pub fn is_invertible(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QMatrix_isInvertible(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::m11() const```</span>
  ///
  ///
  pub fn m11(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_m11(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::m12() const```</span>
  ///
  ///
  pub fn m12(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_m12(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::m21() const```</span>
  ///
  ///
  pub fn m21(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_m21(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix::m22() const```</span>
  ///
  ///
  pub fn m22(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix_m22(self as *const ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix::map```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::line::Line) -> ::qt_core::line::Line```<br>
  /// C++ method: <span style='color: green;'>```QLine QMatrix::map(const QLine& l) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::line_f::LineF) -> ::qt_core::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```QLineF QMatrix::map(const QLineF& l) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map(&self, &::painter_path::PainterPath) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QMatrix::map(const QPainterPath& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::point::Point) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QMatrix::map(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QMatrix::map(const QPointF& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map(&self, &::polygon::Polygon) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QMatrix::map(const QPolygon& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn map(&self, &::polygon_f::PolygonF) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QMatrix::map(const QPolygonF& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn map(&self, &::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QMatrix::map(const QRegion& r) const```</span>
  ///
  ///
  pub fn map<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::MatrixMapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix::mapRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect(&self, &::qt_core::rect::Rect) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QMatrix::mapRect(const QRect& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QMatrix::mapRect(const QRectF& arg1) const```</span>
  ///
  ///
  pub fn map_rect<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::MatrixMapRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygon QMatrix::mapToPolygon(const QRect& r) const```</span>
  ///
  ///
  pub fn map_to_polygon(&self, r: &::qt_core::rect::Rect) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMatrix_mapToPolygon_to_output(self as *const ::matrix::Matrix,
                                                       r as *const ::qt_core::rect::Rect,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix::map```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_unsafe(&self, (::libc::c_double, ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix::map(double x, double y, double* tx, double* ty) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_unsafe(&self, (::libc::c_int, ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix::map(int x, int y, int* tx, int* ty) const```</span>
  ///
  ///
  pub unsafe fn map_unsafe<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::MatrixMapUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix::QMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::matrix::Matrix```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix::QMatrix()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::matrix::Matrix) -> ::matrix::Matrix```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix::QMatrix(const QMatrix& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::matrix::Matrix```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix::QMatrix(double m11, double m12, double m21, double m22, double dx, double dy)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::matrix::Matrix
    where Args: overloading::MatrixNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMatrix& QMatrix::operator=(const QMatrix& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::matrix::Matrix) -> &'l0 mut ::matrix::Matrix {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QMatrix_operator_assign(self as *mut ::matrix::Matrix,
                                              arg1 as *const ::matrix::Matrix)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix::operator==(const QMatrix& arg1) const```</span>
  ///
  ///
  pub fn op_eq(&self, arg1: &::matrix::Matrix) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QMatrix_operator_eq(self as *const ::matrix::Matrix,
                                          arg1 as *const ::matrix::Matrix)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QMatrix::operator*(const QMatrix& o) const```</span>
  ///
  ///
  pub fn op_mul(&self, o: &::matrix::Matrix) -> ::matrix::Matrix {
    {
      let mut object: ::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMatrix_operator_mul_to_output(self as *const ::matrix::Matrix,
                                                       o as *const ::matrix::Matrix,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix& QMatrix::operator*=(const QMatrix& arg1)```</span>
  ///
  ///
  pub fn op_mul_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::matrix::Matrix) -> &'l0 mut ::matrix::Matrix {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QMatrix_operator_mul_assign(self as *mut ::matrix::Matrix,
                                                  arg1 as *const ::matrix::Matrix)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix::operator!=(const QMatrix& arg1) const```</span>
  ///
  ///
  pub fn op_neq(&self, arg1: &::matrix::Matrix) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QMatrix_operator_neq(self as *const ::matrix::Matrix,
                                           arg1 as *const ::matrix::Matrix)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMatrix_reset(self as *mut ::matrix::Matrix) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix& QMatrix::rotate(double a)```</span>
  ///
  ///
  pub fn rotate<'l0>(&'l0 mut self, a: ::libc::c_double) -> &'l0 mut ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix_rotate(self as *mut ::matrix::Matrix, a) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMatrix& QMatrix::scale(double sx, double sy)```</span>
  ///
  ///
  pub fn scale<'l0>(&'l0 mut self, sx: ::libc::c_double, sy: ::libc::c_double) -> &'l0 mut ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix_scale(self as *mut ::matrix::Matrix, sx, sy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QMatrix::setMatrix(double m11, double m12, double m21, double m22, double dx, double dy)```</span>
  ///
  ///
  pub fn set_matrix(&mut self,
                    m11: ::libc::c_double,
                    m12: ::libc::c_double,
                    m21: ::libc::c_double,
                    m22: ::libc::c_double,
                    dx: ::libc::c_double,
                    dy: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QMatrix_setMatrix(self as *mut ::matrix::Matrix, m11, m12, m21, m22, dx, dy) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix& QMatrix::shear(double sh, double sv)```</span>
  ///
  ///
  pub fn shear<'l0>(&'l0 mut self, sh: ::libc::c_double, sv: ::libc::c_double) -> &'l0 mut ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix_shear(self as *mut ::matrix::Matrix, sh, sv) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMatrix& QMatrix::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'l0>(&'l0 mut self, dx: ::libc::c_double, dy: ::libc::c_double) -> &'l0 mut ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix_translate(self as *mut ::matrix::Matrix, dx, dy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::matrix::Matrix {
  /// C++ method: <span style='color: green;'>```[destructor] void QMatrix::~QMatrix()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMatrix_destructor(self as *mut ::matrix::Matrix) }
  }
}

/// C++ method: <span style='color: green;'>```bool qFuzzyCompare(const QMatrix& m1, const QMatrix& m2)```</span>
///
///
pub fn fuzzy_compare(m1: &::matrix::Matrix, m2: &::matrix::Matrix) -> bool {
  unsafe { ::ffi::qt_gui_c_QMatrix_G_qFuzzyCompare(m1 as *const ::matrix::Matrix, m2 as *const ::matrix::Matrix) }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::matrix::Matrix) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QMatrix& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::matrix::Matrix, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QMatrix& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator*```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_mul((&::qt_core::line::Line, &::matrix::Matrix)) -> ::qt_core::line::Line```<br>
/// C++ method: <span style='color: green;'>```QLine operator*(const QLine& l, const QMatrix& m)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_mul((&::qt_core::line_f::LineF, &::matrix::Matrix)) -> ::qt_core::line_f::LineF```<br>
/// C++ method: <span style='color: green;'>```QLineF operator*(const QLineF& l, const QMatrix& m)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_mul((&::painter_path::PainterPath, &::matrix::Matrix)) -> ::painter_path::PainterPath```<br>
/// C++ method: <span style='color: green;'>```QPainterPath operator*(const QPainterPath& p, const QMatrix& m)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_mul((&::qt_core::point::Point, &::matrix::Matrix)) -> ::qt_core::point::Point```<br>
/// C++ method: <span style='color: green;'>```QPoint operator*(const QPoint& p, const QMatrix& m)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_mul((&::qt_core::point_f::PointF, &::matrix::Matrix)) -> ::qt_core::point_f::PointF```<br>
/// C++ method: <span style='color: green;'>```QPointF operator*(const QPointF& p, const QMatrix& m)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_mul((&::polygon::Polygon, &::matrix::Matrix)) -> ::polygon::Polygon```<br>
/// C++ method: <span style='color: green;'>```QPolygon operator*(const QPolygon& a, const QMatrix& m)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_mul((&::polygon_f::PolygonF, &::matrix::Matrix)) -> ::polygon_f::PolygonF```<br>
/// C++ method: <span style='color: green;'>```QPolygonF operator*(const QPolygonF& a, const QMatrix& m)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_mul((&::region::Region, &::matrix::Matrix)) -> ::cpp_utils::CppBox<::region::Region>```<br>
/// C++ method: <span style='color: green;'>```QRegion operator*(const QRegion& r, const QMatrix& m)```</span>
///
///
pub fn op_mul<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpMulArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::matrix::Matrix)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QMatrix& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::matrix::Matrix)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QMatrix& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QMatrix& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::matrix::Matrix)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QMatrix_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                           arg2 as *mut ::matrix::Matrix)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Matrix::map](../struct.Matrix.html#method.map) method.
  pub trait MatrixMapArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> Self::ReturnType;
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::region::Region {
    type ReturnType = ::cpp_utils::CppBox<::region::Region>;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QMatrix_map_as_ptr(original_self as *const ::matrix::Matrix,
                                           r as *const ::region::Region)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::qt_core::line::Line {
    type ReturnType = ::qt_core::line::Line;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::qt_core::line::Line {
      let l = self;
      {
        let mut object: ::qt_core::line::Line =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QLine(original_self as *const ::matrix::Matrix,
                                                      l as *const ::qt_core::line::Line,
                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    type ReturnType = ::qt_core::line_f::LineF;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::qt_core::line_f::LineF {
      let l = self;
      {
        let mut object: ::qt_core::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QLineF(original_self as *const ::matrix::Matrix,
                                                       l as *const ::qt_core::line_f::LineF,
                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::painter_path::PainterPath {
    type ReturnType = ::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::painter_path::PainterPath {
      let p = self;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QPainterPath(original_self as *const ::matrix::Matrix,
                                                             p as *const ::painter_path::PainterPath,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::qt_core::point::Point {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::qt_core::point::Point {
      let p = self;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QPoint(original_self as *const ::matrix::Matrix,
                                                       p as *const ::qt_core::point::Point,
                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::qt_core::point_f::PointF {
      let p = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QPointF(original_self as *const ::matrix::Matrix,
                                                        p as *const ::qt_core::point_f::PointF,
                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::polygon::Polygon {
    type ReturnType = ::polygon::Polygon;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::polygon::Polygon {
      let a = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QPolygon(original_self as *const ::matrix::Matrix,
                                                         a as *const ::polygon::Polygon,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapArgs<'largs> for &'largs ::polygon_f::PolygonF {
    type ReturnType = ::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::polygon_f::PolygonF {
      let a = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_map_to_output_QPolygonF(original_self as *const ::matrix::Matrix,
                                                          a as *const ::polygon_f::PolygonF,
                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix::map_rect](../struct.Matrix.html#method.map_rect) method.
  pub trait MatrixMapRectArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> Self::ReturnType;
  }
  impl<'largs> MatrixMapRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    type ReturnType = ::qt_core::rect::Rect;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::qt_core::rect::Rect {
      let arg1 = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_mapRect_to_output_QRect(original_self as *const ::matrix::Matrix,
                                                          arg1 as *const ::qt_core::rect::Rect,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MatrixMapRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_core::rect_f::RectF;
    fn exec(self, original_self: &'largs ::matrix::Matrix) -> ::qt_core::rect_f::RectF {
      let arg1 = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_mapRect_to_output_QRectF(original_self as *const ::matrix::Matrix,
                                                           arg1 as *const ::qt_core::rect_f::RectF,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix::map_unsafe](../struct.Matrix.html#method.map_unsafe) method.
  pub trait MatrixMapUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::matrix::Matrix) -> ();
  }
  impl<'largs> MatrixMapUnsafeArgs<'largs>
    for (::libc::c_double, ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::matrix::Matrix) -> () {
      let x = self.0;
      let y = self.1;
      let tx = self.2;
      let ty = self.3;
      ::ffi::qt_gui_c_QMatrix_map_double_double_double_double(original_self as *const ::matrix::Matrix, x, y, tx, ty)
    }
  }
  impl<'largs> MatrixMapUnsafeArgs<'largs> for (::libc::c_int, ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::matrix::Matrix) -> () {
      let x = self.0;
      let y = self.1;
      let tx = self.2;
      let ty = self.3;
      ::ffi::qt_gui_c_QMatrix_map_int_int_int_int(original_self as *const ::matrix::Matrix, x, y, tx, ty)
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix::new](../struct.Matrix.html#method.new) method.
  pub trait MatrixNewArgs {
    fn exec(self) -> ::matrix::Matrix;
  }
  impl MatrixNewArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::matrix::Matrix {
      let m11 = self.0;
      let m12 = self.1;
      let m21 = self.2;
      let m22 = self.3;
      let dx = self.4;
      let dy = self.5;
      {
        let mut object: ::matrix::Matrix =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_constructor_m11_m12_m21_m22_dx_dy(m11, m12, m21, m22, dx, dy, &mut object);
        }
        object
      }
    }
  }
  impl MatrixNewArgs for () {
    fn exec(self) -> ::matrix::Matrix {

      {
        let mut object: ::matrix::Matrix =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MatrixNewArgs for &'a ::matrix::Matrix {
    fn exec(self) -> ::matrix::Matrix {
      let other = self;
      {
        let mut object: ::matrix::Matrix =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_constructor_other(other as *const ::matrix::Matrix, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::matrix::Matrix {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_gui_c_QMatrix_G_qHash_key(key as *const ::matrix::Matrix) }
    }
  }
  impl<'a> HashArgs for (&'a ::matrix::Matrix, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_gui_c_QMatrix_G_qHash_key_seed(key as *const ::matrix::Matrix, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_mul](../fn.op_mul.html) method.
  pub trait OpMulArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpMulArgs for (&'a ::region::Region, &'a ::matrix::Matrix) {
    type ReturnType = ::cpp_utils::CppBox<::region::Region>;
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self.0;
      let m = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QMatrix_G_operator_mul_as_ptr(r as *const ::region::Region, m as *const ::matrix::Matrix)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::line_f::LineF, &'a ::matrix::Matrix) {
    type ReturnType = ::qt_core::line_f::LineF;
    fn exec(self) -> ::qt_core::line_f::LineF {
      let l = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QLineF_QMatrix(l as *const ::qt_core::line_f::LineF,
                                                                          m as *const ::matrix::Matrix,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::line::Line, &'a ::matrix::Matrix) {
    type ReturnType = ::qt_core::line::Line;
    fn exec(self) -> ::qt_core::line::Line {
      let l = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::line::Line =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QLine_QMatrix(l as *const ::qt_core::line::Line,
                                                                         m as *const ::matrix::Matrix,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::painter_path::PainterPath, &'a ::matrix::Matrix) {
    type ReturnType = ::painter_path::PainterPath;
    fn exec(self) -> ::painter_path::PainterPath {
      let p = self.0;
      let m = self.1;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QPainterPath_QMatrix(p as *const ::painter_path::PainterPath, m as *const ::matrix::Matrix, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::point_f::PointF, &'a ::matrix::Matrix) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self) -> ::qt_core::point_f::PointF {
      let p = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QPointF_QMatrix(p as *const ::qt_core::point_f::PointF,
                                                                           m as *const ::matrix::Matrix,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::point::Point, &'a ::matrix::Matrix) {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self) -> ::qt_core::point::Point {
      let p = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QPoint_QMatrix(p as *const ::qt_core::point::Point,
                                                                          m as *const ::matrix::Matrix,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::polygon_f::PolygonF, &'a ::matrix::Matrix) {
    type ReturnType = ::polygon_f::PolygonF;
    fn exec(self) -> ::polygon_f::PolygonF {
      let a = self.0;
      let m = self.1;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QPolygonF_QMatrix(a as *const ::polygon_f::PolygonF,
                                                                             m as *const ::matrix::Matrix,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::polygon::Polygon, &'a ::matrix::Matrix) {
    type ReturnType = ::polygon::Polygon;
    fn exec(self) -> ::polygon::Polygon {
      let a = self.0;
      let m = self.1;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_mul_to_output_QPolygon_QMatrix(a as *const ::polygon::Polygon,
                                                                            m as *const ::matrix::Matrix,
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
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::matrix::Matrix) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QMatrix_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                               arg2 as *const ::matrix::Matrix)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::matrix::Matrix) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                           arg2 as *const ::matrix::Matrix,
                                                           &mut object);
        }
        object
      }
    }
  }
}
