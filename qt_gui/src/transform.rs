/// C++ type: <span style='color: green;'>```QTransform```</span>
#[repr(C)]
pub struct Transform([u8; ::type_sizes::QT_GUI_TRANSFORM_TRANSFORM]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Transform {
  unsafe fn new_uninitialized() -> Transform {
    Transform(::std::mem::uninitialized())
  }
}

impl Transform {
  /// C++ method: <span style='color: green;'>```QTransform QTransform::adjoint() const```</span>
  ///
  ///
  pub fn adjoint(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_adjoint_to_output(self as *const ::transform::Transform, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QTransform::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_convert_to_QVariant_to_output(self as *const ::transform::Transform, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::det() const```</span>
  ///
  ///
  pub fn det(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_det(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::determinant() const```</span>
  ///
  ///
  pub fn determinant(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_determinant(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::dx() const```</span>
  ///
  ///
  pub fn dx(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_dx(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::dy() const```</span>
  ///
  ///
  pub fn dy(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_dy(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```static QTransform QTransform::fromScale(double dx, double dy)```</span>
  ///
  ///
  pub fn from_scale(dx: ::libc::c_double, dy: ::libc::c_double) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_fromScale_to_output(dx, dy, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QTransform QTransform::fromTranslate(double dx, double dy)```</span>
  ///
  ///
  pub fn from_translate(dx: ::libc::c_double, dy: ::libc::c_double) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_fromTranslate_to_output(dx, dy, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QTransform::inverted() const```</span>
  ///
  ///
  pub fn inverted(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_inverted_to_output_no_args(self as *const ::transform::Transform, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QTransform::inverted(bool* invertible = ?) const```</span>
  ///
  ///
  pub unsafe fn inverted_unsafe(&self, invertible: *mut bool) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTransform_inverted_to_output_invertible(self as *const ::transform::Transform,
                                                               invertible,
                                                               &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::isAffine() const```</span>
  ///
  ///
  pub fn is_affine(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTransform_isAffine(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::isIdentity() const```</span>
  ///
  ///
  pub fn is_identity(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTransform_isIdentity(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::isInvertible() const```</span>
  ///
  ///
  pub fn is_invertible(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTransform_isInvertible(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::isRotating() const```</span>
  ///
  ///
  pub fn is_rotating(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTransform_isRotating(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::isScaling() const```</span>
  ///
  ///
  pub fn is_scaling(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTransform_isScaling(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::isTranslating() const```</span>
  ///
  ///
  pub fn is_translating(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTransform_isTranslating(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m11() const```</span>
  ///
  ///
  pub fn m11(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m11(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m12() const```</span>
  ///
  ///
  pub fn m12(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m12(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m13() const```</span>
  ///
  ///
  pub fn m13(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m13(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m21() const```</span>
  ///
  ///
  pub fn m21(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m21(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m22() const```</span>
  ///
  ///
  pub fn m22(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m22(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m23() const```</span>
  ///
  ///
  pub fn m23(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m23(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m31() const```</span>
  ///
  ///
  pub fn m31(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m31(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m32() const```</span>
  ///
  ///
  pub fn m32(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m32(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```double QTransform::m33() const```</span>
  ///
  ///
  pub fn m33(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTransform_m33(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```QTransform::map```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::line::Line) -> ::qt_core::line::Line```<br>
  /// C++ method: <span style='color: green;'>```QLine QTransform::map(const QLine& l) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::line_f::LineF) -> ::qt_core::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```QLineF QTransform::map(const QLineF& l) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map(&self, &::painter_path::PainterPath) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QTransform::map(const QPainterPath& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::point::Point) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QTransform::map(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QTransform::map(const QPointF& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn map(&self, &::polygon::Polygon) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QTransform::map(const QPolygon& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn map(&self, &::polygon_f::PolygonF) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QTransform::map(const QPolygonF& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn map(&self, &::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QTransform::map(const QRegion& r) const```</span>
  ///
  ///
  pub fn map<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::TransformMapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTransform::mapRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect(&self, &::qt_core::rect::Rect) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QTransform::mapRect(const QRect& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QTransform::mapRect(const QRectF& arg1) const```</span>
  ///
  ///
  pub fn map_rect<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::TransformMapRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygon QTransform::mapToPolygon(const QRect& r) const```</span>
  ///
  ///
  pub fn map_to_polygon(&self, r: &::qt_core::rect::Rect) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_mapToPolygon_to_output(self as *const ::transform::Transform,
                                                          r as *const ::qt_core::rect::Rect,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform::map```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_unsafe(&self, (::libc::c_double, ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTransform::map(double x, double y, double* tx, double* ty) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_unsafe(&self, (::libc::c_int, ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTransform::map(int x, int y, int* tx, int* ty) const```</span>
  ///
  ///
  pub unsafe fn map_unsafe<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::TransformMapUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTransform::QTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTransform::QTransform()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::matrix::Matrix) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTransform::QTransform(const QMatrix& mtx)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::transform::Transform) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTransform::QTransform(const QTransform& other)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTransform::QTransform(double h11, double h12, double h13, double h21, double h22, double h23, double h31, double h32)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTransform::QTransform(double h11, double h12, double h13, double h21, double h22, double h23, double h31, double h32, double h33 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTransform::QTransform(double h11, double h12, double h21, double h22, double dx, double dy)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::transform::Transform
    where Args: overloading::TransformNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::operator+=(double div)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, div: ::libc::c_double) -> &'l0 mut ::transform::Transform {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTransform_operator_add_assign(self as *mut ::transform::Transform, div) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTransform& QTransform::operator=(const QTransform& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::transform::Transform) -> &'l0 mut ::transform::Transform {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTransform_operator_assign(self as *mut ::transform::Transform,
                                                 arg1 as *const ::transform::Transform)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTransform& QTransform::operator/=(double div)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, div: ::libc::c_double) -> &'l0 mut ::transform::Transform {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTransform_operator_div_assign(self as *mut ::transform::Transform, div) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTransform::operator==(const QTransform& arg1) const```</span>
  ///
  ///
  pub fn op_eq(&self, arg1: &::transform::Transform) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTransform_operator_eq(self as *const ::transform::Transform,
                                             arg1 as *const ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QTransform::operator*(const QTransform& o) const```</span>
  ///
  ///
  pub fn op_mul(&self, o: &::transform::Transform) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_operator_mul_to_output(self as *const ::transform::Transform,
                                                          o as *const ::transform::Transform,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, &'l1 ::transform::Transform) -> &'l0 mut ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::operator*=(const QTransform& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_double) -> &'l0 mut ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::operator*=(double div)```</span>
  ///
  ///
  pub fn op_mul_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::transform::Transform
    where Args: overloading::TransformOpMulAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTransform::operator!=(const QTransform& arg1) const```</span>
  ///
  ///
  pub fn op_neq(&self, arg1: &::transform::Transform) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTransform_operator_neq(self as *const ::transform::Transform,
                                              arg1 as *const ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform& QTransform::operator-=(double div)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, div: ::libc::c_double) -> &'l0 mut ::transform::Transform {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTransform_operator_sub_assign(self as *mut ::transform::Transform, div) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static bool QTransform::quadToQuad(const QPolygonF& one, const QPolygonF& two, QTransform& result)```</span>
  ///
  ///
  pub fn quad_to_quad(one: &::polygon_f::PolygonF,
                      two: &::polygon_f::PolygonF,
                      result: &mut ::transform::Transform)
                      -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTransform_quadToQuad(one as *const ::polygon_f::PolygonF,
                                            two as *const ::polygon_f::PolygonF,
                                            result as *mut ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QTransform::quadToSquare(const QPolygonF& quad, QTransform& result)```</span>
  ///
  ///
  pub fn quad_to_square(quad: &::polygon_f::PolygonF, result: &mut ::transform::Transform) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTransform_quadToSquare(quad as *const ::polygon_f::PolygonF,
                                              result as *mut ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTransform::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTransform_reset(self as *mut ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```QTransform::rotate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn rotate(&mut self, ::libc::c_double) -> &'l0 mut ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::rotate(double a)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn rotate(&mut self, (::libc::c_double, &'l1 ::qt_core::qt::Axis)) -> &'l0 mut ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::rotate(double a, Qt::Axis axis = ?)```</span>
  ///
  ///
  pub fn rotate<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::transform::Transform
    where Args: overloading::TransformRotateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTransform::rotateRadians```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn rotate_radians(&mut self, ::libc::c_double) -> &'l0 mut ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::rotateRadians(double a)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn rotate_radians(&mut self, (::libc::c_double, &'l1 ::qt_core::qt::Axis)) -> &'l0 mut ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::rotateRadians(double a, Qt::Axis axis = ?)```</span>
  ///
  ///
  pub fn rotate_radians<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::transform::Transform
    where Args: overloading::TransformRotateRadiansArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTransform& QTransform::scale(double sx, double sy)```</span>
  ///
  ///
  pub fn scale<'l0>(&'l0 mut self, sx: ::libc::c_double, sy: ::libc::c_double) -> &'l0 mut ::transform::Transform {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTransform_scale(self as *mut ::transform::Transform, sx, sy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QTransform::setMatrix(double m11, double m12, double m13, double m21, double m22, double m23, double m31, double m32, double m33)```</span>
  ///
  ///
  pub fn set_matrix(&mut self,
                    m11: ::libc::c_double,
                    m12: ::libc::c_double,
                    m13: ::libc::c_double,
                    m21: ::libc::c_double,
                    m22: ::libc::c_double,
                    m23: ::libc::c_double,
                    m31: ::libc::c_double,
                    m32: ::libc::c_double,
                    m33: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTransform_setMatrix(self as *mut ::transform::Transform,
                                           m11,
                                           m12,
                                           m13,
                                           m21,
                                           m22,
                                           m23,
                                           m31,
                                           m32,
                                           m33)
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform& QTransform::shear(double sh, double sv)```</span>
  ///
  ///
  pub fn shear<'l0>(&'l0 mut self, sh: ::libc::c_double, sv: ::libc::c_double) -> &'l0 mut ::transform::Transform {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTransform_shear(self as *mut ::transform::Transform, sh, sv) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static bool QTransform::squareToQuad(const QPolygonF& square, QTransform& result)```</span>
  ///
  ///
  pub fn square_to_quad(square: &::polygon_f::PolygonF, result: &mut ::transform::Transform) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTransform_squareToQuad(square as *const ::polygon_f::PolygonF,
                                              result as *mut ::transform::Transform)
    }
  }

  /// C++ method: <span style='color: green;'>```const QMatrix& QTransform::toAffine() const```</span>
  ///
  ///
  pub fn to_affine<'l0>(&'l0 self) -> &'l0 ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTransform_toAffine(self as *const ::transform::Transform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTransform& QTransform::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'l0>(&'l0 mut self, dx: ::libc::c_double, dy: ::libc::c_double) -> &'l0 mut ::transform::Transform {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTransform_translate(self as *mut ::transform::Transform, dx, dy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTransform QTransform::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTransform_transposed_to_output(self as *const ::transform::Transform, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform::TransformationType QTransform::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::transform::TransformationType {
    unsafe { ::ffi::qt_gui_c_QTransform_type(self as *const ::transform::Transform) }
  }
}

impl Drop for ::transform::Transform {
  /// C++ method: <span style='color: green;'>```[destructor] void QTransform::~QTransform()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTransform_destructor(self as *mut ::transform::Transform) }
  }
}

/// C++ type: <span style='color: green;'>```QTransform::TransformationType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TransformationType {
  /// C++ enum variant: <span style='color: green;'>```TxNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```TxTranslate = 1```</span>
  Translate = 1,
  /// C++ enum variant: <span style='color: green;'>```TxScale = 2```</span>
  Scale = 2,
  /// C++ enum variant: <span style='color: green;'>```TxRotate = 4```</span>
  Rotate = 4,
  /// C++ enum variant: <span style='color: green;'>```TxShear = 8```</span>
  Shear = 8,
  /// C++ enum variant: <span style='color: green;'>```TxProject = 16```</span>
  Project = 16,
}

/// C++ method: <span style='color: green;'>```bool qFuzzyCompare(const QTransform& t1, const QTransform& t2)```</span>
///
///
pub fn fuzzy_compare(t1: &::transform::Transform, t2: &::transform::Transform) -> bool {
  unsafe {
    ::ffi::qt_gui_c_QTransform_G_qFuzzyCompare(t1 as *const ::transform::Transform,
                                               t2 as *const ::transform::Transform)
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::transform::Transform) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QTransform& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::transform::Transform, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QTransform& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QTransform operator+(const QTransform& a, double n)```</span>
///
///
pub fn op_add(a: &::transform::Transform, n: ::libc::c_double) -> ::transform::Transform {
  {
    let mut object: ::transform::Transform =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QTransform_G_operator_add_to_output(a as *const ::transform::Transform, n, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QTransform operator/(const QTransform& a, double n)```</span>
///
///
pub fn op_div(a: &::transform::Transform, n: ::libc::c_double) -> ::transform::Transform {
  {
    let mut object: ::transform::Transform =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QTransform_G_operator_div_to_output(a as *const ::transform::Transform, n, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```operator*```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_mul((&::qt_core::line::Line, &::transform::Transform)) -> ::qt_core::line::Line```<br>
/// C++ method: <span style='color: green;'>```QLine operator*(const QLine& l, const QTransform& m)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_mul((&::qt_core::line_f::LineF, &::transform::Transform)) -> ::qt_core::line_f::LineF```<br>
/// C++ method: <span style='color: green;'>```QLineF operator*(const QLineF& l, const QTransform& m)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_mul((&::painter_path::PainterPath, &::transform::Transform)) -> ::painter_path::PainterPath```<br>
/// C++ method: <span style='color: green;'>```QPainterPath operator*(const QPainterPath& p, const QTransform& m)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_mul((&::qt_core::point::Point, &::transform::Transform)) -> ::qt_core::point::Point```<br>
/// C++ method: <span style='color: green;'>```QPoint operator*(const QPoint& p, const QTransform& m)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_mul((&::qt_core::point_f::PointF, &::transform::Transform)) -> ::qt_core::point_f::PointF```<br>
/// C++ method: <span style='color: green;'>```QPointF operator*(const QPointF& p, const QTransform& m)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_mul((&::polygon::Polygon, &::transform::Transform)) -> ::polygon::Polygon```<br>
/// C++ method: <span style='color: green;'>```QPolygon operator*(const QPolygon& a, const QTransform& m)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_mul((&::polygon_f::PolygonF, &::transform::Transform)) -> ::polygon_f::PolygonF```<br>
/// C++ method: <span style='color: green;'>```QPolygonF operator*(const QPolygonF& a, const QTransform& m)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_mul((&::region::Region, &::transform::Transform)) -> ::cpp_utils::CppBox<::region::Region>```<br>
/// C++ method: <span style='color: green;'>```QRegion operator*(const QRegion& r, const QTransform& m)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_mul((&::transform::Transform, ::libc::c_double)) -> ::transform::Transform```<br>
/// C++ method: <span style='color: green;'>```QTransform operator*(const QTransform& a, double n)```</span>
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::transform::Transform)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QTransform& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::transform::Transform)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTransform& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QTransform& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::transform::Transform)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QTransform_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                              arg2 as *mut ::transform::Transform)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTransform operator-(const QTransform& a, double n)```</span>
///
///
pub fn op_sub(a: &::transform::Transform, n: ::libc::c_double) -> ::transform::Transform {
  {
    let mut object: ::transform::Transform =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QTransform_G_operator_sub_to_output(a as *const ::transform::Transform, n, &mut object);
    }
    object
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Transform::map](../struct.Transform.html#method.map) method.
  pub trait TransformMapArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::transform::Transform) -> Self::ReturnType;
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::region::Region {
    type ReturnType = ::cpp_utils::CppBox<::region::Region>;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QTransform_map_as_ptr(original_self as *const ::transform::Transform,
                                              r as *const ::region::Region)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::qt_core::line::Line {
    type ReturnType = ::qt_core::line::Line;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::qt_core::line::Line {
      let l = self;
      {
        let mut object: ::qt_core::line::Line =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QLine(original_self as *const ::transform::Transform,
                                                         l as *const ::qt_core::line::Line,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    type ReturnType = ::qt_core::line_f::LineF;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::qt_core::line_f::LineF {
      let l = self;
      {
        let mut object: ::qt_core::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QLineF(original_self as *const ::transform::Transform,
                                                          l as *const ::qt_core::line_f::LineF,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::painter_path::PainterPath {
    type ReturnType = ::painter_path::PainterPath;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::painter_path::PainterPath {
      let p = self;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QPainterPath(original_self as *const ::transform::Transform,
                                                                p as *const ::painter_path::PainterPath,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::qt_core::point::Point {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::qt_core::point::Point {
      let p = self;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QPoint(original_self as *const ::transform::Transform,
                                                          p as *const ::qt_core::point::Point,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::qt_core::point_f::PointF {
      let p = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QPointF(original_self as *const ::transform::Transform,
                                                           p as *const ::qt_core::point_f::PointF,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::polygon::Polygon {
    type ReturnType = ::polygon::Polygon;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::polygon::Polygon {
      let a = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QPolygon(original_self as *const ::transform::Transform,
                                                            a as *const ::polygon::Polygon,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapArgs<'largs> for &'largs ::polygon_f::PolygonF {
    type ReturnType = ::polygon_f::PolygonF;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::polygon_f::PolygonF {
      let a = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_map_to_output_QPolygonF(original_self as *const ::transform::Transform,
                                                             a as *const ::polygon_f::PolygonF,
                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::map_rect](../struct.Transform.html#method.map_rect) method.
  pub trait TransformMapRectArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::transform::Transform) -> Self::ReturnType;
  }
  impl<'largs> TransformMapRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    type ReturnType = ::qt_core::rect::Rect;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::qt_core::rect::Rect {
      let arg1 = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_mapRect_to_output_QRect(original_self as *const ::transform::Transform,
                                                             arg1 as *const ::qt_core::rect::Rect,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TransformMapRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_core::rect_f::RectF;
    fn exec(self, original_self: &'largs ::transform::Transform) -> ::qt_core::rect_f::RectF {
      let arg1 = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_mapRect_to_output_QRectF(original_self as *const ::transform::Transform,
                                                              arg1 as *const ::qt_core::rect_f::RectF,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::map_unsafe](../struct.Transform.html#method.map_unsafe) method.
  pub trait TransformMapUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::transform::Transform) -> ();
  }
  impl<'largs> TransformMapUnsafeArgs<'largs>
    for (::libc::c_double, ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::transform::Transform) -> () {
      let x = self.0;
      let y = self.1;
      let tx = self.2;
      let ty = self.3;
      ::ffi::qt_gui_c_QTransform_map_double_double_double_double(original_self as *const ::transform::Transform,
                                                                 x,
                                                                 y,
                                                                 tx,
                                                                 ty)
    }
  }
  impl<'largs> TransformMapUnsafeArgs<'largs> for (::libc::c_int, ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::transform::Transform) -> () {
      let x = self.0;
      let y = self.1;
      let tx = self.2;
      let ty = self.3;
      ::ffi::qt_gui_c_QTransform_map_int_int_int_int(original_self as *const ::transform::Transform, x, y, tx, ty)
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::new](../struct.Transform.html#method.new) method.
  pub trait TransformNewArgs {
    fn exec(self) -> ::transform::Transform;
  }
  impl TransformNewArgs
    for (::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double) {
    fn exec(self) -> ::transform::Transform {
      let h11 = self.0;
      let h12 = self.1;
      let h13 = self.2;
      let h21 = self.3;
      let h22 = self.4;
      let h23 = self.5;
      let h31 = self.6;
      let h32 = self.7;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_constructor_h11_h12_h13_h21_h22_h23_h31_h32(h11,
                                                                                 h12,
                                                                                 h13,
                                                                                 h21,
                                                                                 h22,
                                                                                 h23,
                                                                                 h31,
                                                                                 h32,
                                                                                 &mut object);
        }
        object
      }
    }
  }
  impl TransformNewArgs
    for (::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double,
                                 ::libc::c_double) {
    fn exec(self) -> ::transform::Transform {
      let h11 = self.0;
      let h12 = self.1;
      let h13 = self.2;
      let h21 = self.3;
      let h22 = self.4;
      let h23 = self.5;
      let h31 = self.6;
      let h32 = self.7;
      let h33 = self.8;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_constructor_h11_h12_h13_h21_h22_h23_h31_h32_h33(h11,
                                                                                     h12,
                                                                                     h13,
                                                                                     h21,
                                                                                     h22,
                                                                                     h23,
                                                                                     h31,
                                                                                     h32,
                                                                                     h33,
                                                                                     &mut object);
        }
        object
      }
    }
  }
  impl TransformNewArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::transform::Transform {
      let h11 = self.0;
      let h12 = self.1;
      let h21 = self.2;
      let h22 = self.3;
      let dx = self.4;
      let dy = self.5;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_constructor_h11_h12_h21_h22_dx_dy(h11, h12, h21, h22, dx, dy, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TransformNewArgs for &'a ::matrix::Matrix {
    fn exec(self) -> ::transform::Transform {
      let mtx = self;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_constructor_mtx(mtx as *const ::matrix::Matrix, &mut object);
        }
        object
      }
    }
  }
  impl TransformNewArgs for () {
    fn exec(self) -> ::transform::Transform {

      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TransformNewArgs for &'a ::transform::Transform {
    fn exec(self) -> ::transform::Transform {
      let other = self;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_constructor_other(other as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::op_mul_assign](../struct.Transform.html#method.op_mul_assign) method.
  pub trait TransformOpMulAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform;
  }
  impl<'largs> TransformOpMulAssignArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform {
      let arg1 = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QTransform_operator_mul_assign_arg1(original_self as *mut ::transform::Transform,
                                                              arg1 as *const ::transform::Transform)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TransformOpMulAssignArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform {
      let div = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QTransform_operator_mul_assign_div(original_self as *mut ::transform::Transform, div)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::rotate](../struct.Transform.html#method.rotate) method.
  pub trait TransformRotateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform;
  }
  impl<'largs> TransformRotateArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform {
      let a = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTransform_rotate_a(original_self as *mut ::transform::Transform, a) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TransformRotateArgs<'largs> for (::libc::c_double, &'largs ::qt_core::qt::Axis) {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform {
      let a = self.0;
      let axis = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QTransform_rotate_a_axis(original_self as *mut ::transform::Transform,
                                                 a,
                                                 axis as *const ::qt_core::qt::Axis)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::rotate_radians](../struct.Transform.html#method.rotate_radians) method.
  pub trait TransformRotateRadiansArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform;
  }
  impl<'largs> TransformRotateRadiansArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform {
      let a = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QTransform_rotateRadians_a(original_self as *mut ::transform::Transform, a) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TransformRotateRadiansArgs<'largs> for (::libc::c_double, &'largs ::qt_core::qt::Axis) {
    fn exec(self, original_self: &'largs mut ::transform::Transform) -> &'largs mut ::transform::Transform {
      let a = self.0;
      let axis = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QTransform_rotateRadians_a_axis(original_self as *mut ::transform::Transform,
                                                        a,
                                                        axis as *const ::qt_core::qt::Axis)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::transform::Transform {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_gui_c_QTransform_G_qHash_key(key as *const ::transform::Transform) }
    }
  }
  impl<'a> HashArgs for (&'a ::transform::Transform, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_gui_c_QTransform_G_qHash_key_seed(key as *const ::transform::Transform, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_mul](../fn.op_mul.html) method.
  pub trait OpMulArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpMulArgs for (&'a ::region::Region, &'a ::transform::Transform) {
    type ReturnType = ::cpp_utils::CppBox<::region::Region>;
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self.0;
      let m = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QTransform_G_operator_mul_as_ptr(r as *const ::region::Region,
                                                         m as *const ::transform::Transform)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::line_f::LineF, &'a ::transform::Transform) {
    type ReturnType = ::qt_core::line_f::LineF;
    fn exec(self) -> ::qt_core::line_f::LineF {
      let l = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QLineF_QTransform(l as *const ::qt_core::line_f::LineF,
                                                                                m as *const ::transform::Transform,
                                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::line::Line, &'a ::transform::Transform) {
    type ReturnType = ::qt_core::line::Line;
    fn exec(self) -> ::qt_core::line::Line {
      let l = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::line::Line =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QLine_QTransform(l as *const ::qt_core::line::Line,
                                                                               m as *const ::transform::Transform,
                                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::painter_path::PainterPath, &'a ::transform::Transform) {
    type ReturnType = ::painter_path::PainterPath;
    fn exec(self) -> ::painter_path::PainterPath {
      let p = self.0;
      let m = self.1;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QPainterPath_QTransform(p as *const ::painter_path::PainterPath, m as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::point_f::PointF, &'a ::transform::Transform) {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self) -> ::qt_core::point_f::PointF {
      let p = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QPointF_QTransform(p as *const ::qt_core::point_f::PointF, m as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::qt_core::point::Point, &'a ::transform::Transform) {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self) -> ::qt_core::point::Point {
      let p = self.0;
      let m = self.1;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QPoint_QTransform(p as *const ::qt_core::point::Point,
                                                                                m as *const ::transform::Transform,
                                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::polygon_f::PolygonF, &'a ::transform::Transform) {
    type ReturnType = ::polygon_f::PolygonF;
    fn exec(self) -> ::polygon_f::PolygonF {
      let a = self.0;
      let m = self.1;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QPolygonF_QTransform(a as *const ::polygon_f::PolygonF, m as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::polygon::Polygon, &'a ::transform::Transform) {
    type ReturnType = ::polygon::Polygon;
    fn exec(self) -> ::polygon::Polygon {
      let a = self.0;
      let m = self.1;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QPolygon_QTransform(a as *const ::polygon::Polygon,
                                                                                  m as *const ::transform::Transform,
                                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpMulArgs for (&'a ::transform::Transform, ::libc::c_double) {
    type ReturnType = ::transform::Transform;
    fn exec(self) -> ::transform::Transform {
      let a = self.0;
      let n = self.1;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_mul_to_output_QTransform_double(a as *const ::transform::Transform,
                                                                                n,
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
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::transform::Transform) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QTransform_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                                  arg2 as *const ::transform::Transform)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::transform::Transform) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTransform_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                              arg2 as *const ::transform::Transform,
                                                              &mut object);
        }
        object
      }
    }
  }
}
