/// C++ type: <span style='color: green;'>```QPolygon```</span>
#[repr(C)]
pub struct Polygon([u8; ::type_sizes::QT_GUI_POLYGON_POLYGON]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Polygon {
  unsafe fn new_uninitialized() -> Polygon {
    Polygon(::std::mem::uninitialized())
  }
}

impl Polygon {
  /// C++ method: <span style='color: green;'>```QVariant QPolygon::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygon_convert_to_QVariant_to_output(self as *const ::polygon::Polygon, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QPolygon::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygon_boundingRect_to_output(self as *const ::polygon::Polygon, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPolygon::containsPoint(const QPoint& pt, Qt::FillRule fillRule) const```</span>
  ///
  ///
  pub fn contains_point(&self, pt: &::qt_core::point::Point, fill_rule: &::qt_core::qt::FillRule) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPolygon_containsPoint(self as *const ::polygon::Polygon,
                                             pt as *const ::qt_core::point::Point,
                                             fill_rule as *const ::qt_core::qt::FillRule)
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygon QPolygon::intersected(const QPolygon& r) const```</span>
  ///
  ///
  pub fn intersected(&self, r: &::polygon::Polygon) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygon_intersected_to_output(self as *const ::polygon::Polygon,
                                                       r as *const ::polygon::Polygon,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygon::QPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::polygon::Polygon) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon(const QPolygon& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect::Rect) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::qt_core::rect::Rect, bool)) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon(const QRect& r, bool closed = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCorePoint) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon(const QVector<QPoint>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::polygon::Polygon
    where Args: overloading::PolygonNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygon::QPolygon(int nPoints, const int* points)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(n_points: ::libc::c_int, points: *const ::libc::c_int) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPolygon_constructor_nPoints_points(n_points, points, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygon& QPolygon::operator=(const QPolygon& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::polygon::Polygon) -> &'l0 mut ::polygon::Polygon {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPolygon_operator_assign(self as *mut ::polygon::Polygon,
                                               other as *const ::polygon::Polygon)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint QPolygon::point(int i) const```</span>
  ///
  ///
  pub fn point(&self, i: ::libc::c_int) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygon_point_to_output(self as *const ::polygon::Polygon, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPolygon::point(int i, int* x, int* y) const```</span>
  ///
  ///
  pub unsafe fn point_unsafe(&self, i: ::libc::c_int, x: *mut ::libc::c_int, y: *mut ::libc::c_int) {
    ::ffi::qt_gui_c_QPolygon_point(self as *const ::polygon::Polygon, i, x, y)
  }

  /// C++ method: <span style='color: green;'>```QPolygon::putPoints```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn put_points(&mut self, (::libc::c_int, ::libc::c_int, &::polygon::Polygon)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygon::putPoints(int index, int nPoints, const QPolygon& from)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn put_points(&mut self, (::libc::c_int, ::libc::c_int, &::polygon::Polygon, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygon::putPoints(int index, int nPoints, const QPolygon& from, int fromIndex = ?)```</span>
  ///
  ///
  pub fn put_points<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PolygonPutPointsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPolygon::putPoints(int index, int nPoints, const int* points)```</span>
  ///
  ///
  pub unsafe fn put_points_unsafe(&mut self,
                                  index: ::libc::c_int,
                                  n_points: ::libc::c_int,
                                  points: *const ::libc::c_int) {
    ::ffi::qt_gui_c_QPolygon_putPoints_index_nPoints_points(self as *mut ::polygon::Polygon, index, n_points, points)
  }

  /// C++ method: <span style='color: green;'>```QPolygon::setPoint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_point(&mut self, (::libc::c_int, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygon::setPoint(int index, const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_point(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygon::setPoint(int index, int x, int y)```</span>
  ///
  ///
  pub fn set_point<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PolygonSetPointArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPolygon::setPoints(int nPoints, const int* points)```</span>
  ///
  ///
  pub unsafe fn set_points(&mut self, n_points: ::libc::c_int, points: *const ::libc::c_int) {
    ::ffi::qt_gui_c_QPolygon_setPoints(self as *mut ::polygon::Polygon, n_points, points)
  }

  /// C++ method: <span style='color: green;'>```QPolygon QPolygon::subtracted(const QPolygon& r) const```</span>
  ///
  ///
  pub fn subtracted(&self, r: &::polygon::Polygon) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygon_subtracted_to_output(self as *const ::polygon::Polygon,
                                                      r as *const ::polygon::Polygon,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPolygon::swap(QPolygon& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::polygon::Polygon) {
    unsafe {
      ::ffi::qt_gui_c_QPolygon_swap(self as *mut ::polygon::Polygon,
                                    other as *mut ::polygon::Polygon)
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygon::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygon::translate(const QPoint& offset)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygon::translate(int dx, int dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PolygonTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygon::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::qt_core::point::Point) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QPolygon::translated(const QPoint& offset) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_int, ::libc::c_int)) -> ::polygon::Polygon```<br>
  /// C++ method: <span style='color: green;'>```QPolygon QPolygon::translated(int dx, int dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::polygon::Polygon
    where Args: overloading::PolygonTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygon QPolygon::united(const QPolygon& r) const```</span>
  ///
  ///
  pub fn united(&self, r: &::polygon::Polygon) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygon_united_to_output(self as *const ::polygon::Polygon,
                                                  r as *const ::polygon::Polygon,
                                                  &mut object);
      }
      object
    }
  }
}

impl Drop for ::polygon::Polygon {
  /// C++ method: <span style='color: green;'>```[destructor] void QPolygon::~QPolygon()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPolygon_destructor(self as *mut ::polygon::Polygon) }
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::polygon::Polygon)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& stream, const QPolygon& polygon)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::polygon_f::PolygonF)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& stream, const QPolygonF& array)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::polygon::Polygon)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPolygon& arg2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::polygon_f::PolygonF)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPolygonF& arg2)```</span>
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
/// Rust arguments: ```fn op_shr((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 mut ::polygon::Polygon)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& stream, QPolygon& polygon)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 mut ::polygon_f::PolygonF)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& stream, QPolygonF& array)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::qt_core::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```swap```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn swap((&mut ::polygon::Polygon, &mut ::polygon::Polygon)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QPolygon& value1, QPolygon& value2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn swap((&mut ::polygon_f::PolygonF, &mut ::polygon_f::PolygonF)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QPolygonF& value1, QPolygonF& value2)```</span>
///
///
pub fn swap<Args>(args: Args) -> ()
  where Args: overloading::SwapArgs
{
  args.exec()
}
impl ::cpp_utils::StaticCast<::vector::VectorQtCorePoint> for ::polygon::Polygon {
  fn static_cast_mut(&mut self) -> &mut ::vector::VectorQtCorePoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPolygon_G_static_cast_QVector_QPoint_ptr(self as *mut ::polygon::Polygon) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::vector::VectorQtCorePoint {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPolygon_G_static_cast_QVector_QPoint_ptr(self as *const ::polygon::Polygon as *mut ::polygon::Polygon) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::polygon::Polygon> for ::vector::VectorQtCorePoint {
  unsafe fn static_cast_mut(&mut self) -> &mut ::polygon::Polygon {
    let ffi_result = ::ffi::qt_gui_c_QPolygon_G_static_cast_QPolygon_ptr(self as *mut ::vector::VectorQtCorePoint);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::polygon::Polygon {
    let ffi_result = ::ffi::qt_gui_c_QPolygon_G_static_cast_QPolygon_ptr(self as *const ::vector::VectorQtCorePoint as *mut ::vector::VectorQtCorePoint);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::polygon::Polygon {
  type Target = ::vector::VectorQtCorePoint;
  fn deref(&self) -> &::vector::VectorQtCorePoint {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPolygon_G_static_cast_QVector_QPoint_ptr(self as *const ::polygon::Polygon as *mut ::polygon::Polygon) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::polygon::Polygon {
  fn deref_mut(&mut self) -> &mut ::vector::VectorQtCorePoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPolygon_G_static_cast_QVector_QPoint_ptr(self as *mut ::polygon::Polygon) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Polygon::new](../struct.Polygon.html#method.new) method.
  pub trait PolygonNewArgs {
    fn exec(self) -> ::polygon::Polygon;
  }
  impl PolygonNewArgs for () {
    fn exec(self) -> ::polygon::Polygon {

      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonNewArgs for &'a ::polygon::Polygon {
    fn exec(self) -> ::polygon::Polygon {
      let other = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_constructor_other(other as *const ::polygon::Polygon, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonNewArgs for &'a ::qt_core::rect::Rect {
    fn exec(self) -> ::polygon::Polygon {
      let r = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_constructor_r(r as *const ::qt_core::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonNewArgs for (&'a ::qt_core::rect::Rect, bool) {
    fn exec(self) -> ::polygon::Polygon {
      let r = self.0;
      let closed = self.1;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_constructor_r_closed(r as *const ::qt_core::rect::Rect, closed, &mut object);
        }
        object
      }
    }
  }
  impl PolygonNewArgs for ::libc::c_int {
    fn exec(self) -> ::polygon::Polygon {
      let size = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonNewArgs for &'a ::vector::VectorQtCorePoint {
    fn exec(self) -> ::polygon::Polygon {
      let v = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_constructor_v(v as *const ::vector::VectorQtCorePoint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Polygon::put_points](../struct.Polygon.html#method.put_points) method.
  pub trait PolygonPutPointsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> ();
  }
  impl<'largs> PolygonPutPointsArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::polygon::Polygon) {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> () {
      let index = self.0;
      let n_points = self.1;
      let from = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPolygon_putPoints_index_nPoints_from(original_self as *mut ::polygon::Polygon,
                                                              index,
                                                              n_points,
                                                              from as *const ::polygon::Polygon)
      }
    }
  }
  impl<'largs> PolygonPutPointsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::polygon::Polygon, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> () {
      let index = self.0;
      let n_points = self.1;
      let from = self.2;
      let from_index = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPolygon_putPoints_index_nPoints_from_fromIndex(original_self as *mut ::polygon::Polygon,
                                                                        index,
                                                                        n_points,
                                                                        from as *const ::polygon::Polygon,
                                                                        from_index)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Polygon::set_point](../struct.Polygon.html#method.set_point) method.
  pub trait PolygonSetPointArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> ();
  }
  impl<'largs> PolygonSetPointArgs<'largs> for (::libc::c_int, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> () {
      let index = self.0;
      let p = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPolygon_setPoint_index_p(original_self as *mut ::polygon::Polygon,
                                                  index,
                                                  p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> PolygonSetPointArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> () {
      let index = self.0;
      let x = self.1;
      let y = self.2;
      unsafe { ::ffi::qt_gui_c_QPolygon_setPoint_index_x_y(original_self as *mut ::polygon::Polygon, index, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Polygon::translate](../struct.Polygon.html#method.translate) method.
  pub trait PolygonTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> ();
  }
  impl<'largs> PolygonTranslateArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_gui_c_QPolygon_translate_dx_dy(original_self as *mut ::polygon::Polygon, dx, dy) }
    }
  }
  impl<'largs> PolygonTranslateArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::polygon::Polygon) -> () {
      let offset = self;
      unsafe {
        ::ffi::qt_gui_c_QPolygon_translate_offset(original_self as *mut ::polygon::Polygon,
                                                  offset as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Polygon::translated](../struct.Polygon.html#method.translated) method.
  pub trait PolygonTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::polygon::Polygon) -> ::polygon::Polygon;
  }
  impl<'largs> PolygonTranslatedArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::polygon::Polygon) -> ::polygon::Polygon {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_translated_to_output_dx_dy(original_self as *const ::polygon::Polygon,
                                                              dx,
                                                              dy,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PolygonTranslatedArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::polygon::Polygon) -> ::polygon::Polygon {
      let offset = self;
      {
        let mut object: ::polygon::Polygon =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_translated_to_output_offset(original_self as *const ::polygon::Polygon,
                                                               offset as *const ::qt_core::point::Point,
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
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::polygon_f::PolygonF) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let stream = self.0;
      let array = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPolygon_G_operator_shl_stream_array(stream as *mut ::qt_core::data_stream::DataStream,
                                                               array as *const ::polygon_f::PolygonF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::polygon::Polygon) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let stream = self.0;
      let polygon = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPolygon_G_operator_shl_stream_polygon(stream as *mut ::qt_core::data_stream::DataStream,
                                                                 polygon as *const ::polygon::Polygon)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::polygon::Polygon) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_G_operator_shl_to_output_QDebug_QPolygon(arg1 as *const ::qt_core::debug::Debug,
                                                                            arg2 as *const ::polygon::Polygon,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::polygon_f::PolygonF) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygon_G_operator_shl_to_output_QDebug_QPolygonF(arg1 as *const ::qt_core::debug::Debug,
                                                                             arg2 as *const ::polygon_f::PolygonF,
                                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::qt_core::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::qt_core::data_stream::DataStream, &'largs mut ::polygon_f::PolygonF) {
    fn exec(self) -> &'largs mut ::qt_core::data_stream::DataStream {
      let stream = self.0;
      let array = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPolygon_G_operator_shr_stream_array(stream as *mut ::qt_core::data_stream::DataStream,
                                                               array as *mut ::polygon_f::PolygonF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::qt_core::data_stream::DataStream, &'largs mut ::polygon::Polygon) {
    fn exec(self) -> &'largs mut ::qt_core::data_stream::DataStream {
      let stream = self.0;
      let polygon = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QPolygon_G_operator_shr_stream_polygon(stream as *mut ::qt_core::data_stream::DataStream,
                                                                 polygon as *mut ::polygon::Polygon)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [swap](../fn.swap.html) method.
  pub trait SwapArgs {
    fn exec(self) -> ();
  }
  impl<'a> SwapArgs for (&'a mut ::polygon_f::PolygonF, &'a mut ::polygon_f::PolygonF) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPolygon_G_swap_QPolygonF_QPolygonF(value1 as *mut ::polygon_f::PolygonF,
                                                            value2 as *mut ::polygon_f::PolygonF)
      }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::polygon::Polygon, &'a mut ::polygon::Polygon) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPolygon_G_swap_QPolygon_QPolygon(value1 as *mut ::polygon::Polygon,
                                                          value2 as *mut ::polygon::Polygon)
      }
    }
  }
}
