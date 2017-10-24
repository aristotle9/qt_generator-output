/// C++ type: <span style='color: green;'>```QRectF```</span>
#[repr(C)]
pub struct RectF([u8; ::type_sizes::QT_CORE_RECT_F_RECT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for RectF {
  unsafe fn new_uninitialized() -> RectF {
    RectF(::std::mem::uninitialized())
  }
}

impl RectF {
  /// C++ method: <span style='color: green;'>```void QRectF::adjust(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  pub fn adjust(&mut self, x1: ::libc::c_double, y1: ::libc::c_double, x2: ::libc::c_double, y2: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_adjust(self as *mut ::rect_f::RectF, x1, y1, x2, y2) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::adjusted(double x1, double y1, double x2, double y2) const```</span>
  ///
  ///
  pub fn adjusted(&self,
                  x1: ::libc::c_double,
                  y1: ::libc::c_double,
                  x2: ::libc::c_double,
                  y2: ::libc::c_double)
                  -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_adjusted_to_output(self as *const ::rect_f::RectF, x1, y1, x2, y2, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRectF::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_bottom(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QRectF::bottomLeft() const```</span>
  ///
  ///
  pub fn bottom_left(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_bottomLeft_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QRectF::bottomRight() const```</span>
  ///
  ///
  pub fn bottom_right(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_bottomRight_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QRectF::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_center_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::point_f::PointF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRectF::contains(const QPointF& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, &::rect_f::RectF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRectF::contains(const QRectF& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn contains(&self, (::libc::c_double, ::libc::c_double)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRectF::contains(double x, double y) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::RectFContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRectF::getCoords(double* x1, double* y1, double* x2, double* y2) const```</span>
  ///
  ///
  pub unsafe fn get_coords(&self,
                           x1: *mut ::libc::c_double,
                           y1: *mut ::libc::c_double,
                           x2: *mut ::libc::c_double,
                           y2: *mut ::libc::c_double) {
    ::ffi::qt_core_c_QRectF_getCoords(self as *const ::rect_f::RectF, x1, y1, x2, y2)
  }

  /// C++ method: <span style='color: green;'>```void QRectF::getRect(double* x, double* y, double* w, double* h) const```</span>
  ///
  ///
  pub unsafe fn get_rect(&self,
                         x: *mut ::libc::c_double,
                         y: *mut ::libc::c_double,
                         w: *mut ::libc::c_double,
                         h: *mut ::libc::c_double) {
    ::ffi::qt_core_c_QRectF_getRect(self as *const ::rect_f::RectF, x, y, w, h)
  }

  /// C++ method: <span style='color: green;'>```double QRectF::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_height(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::intersected(const QRectF& other) const```</span>
  ///
  ///
  pub fn intersected(&self, other: &::rect_f::RectF) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_intersected_to_output(self as *const ::rect_f::RectF,
                                                      other as *const ::rect_f::RectF,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRectF::intersects(const QRectF& r) const```</span>
  ///
  ///
  pub fn intersects(&self, r: &::rect_f::RectF) -> bool {
    unsafe { ::ffi::qt_core_c_QRectF_intersects(self as *const ::rect_f::RectF, r as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QRectF::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRectF_isEmpty(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QRectF::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRectF_isNull(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QRectF::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRectF_isValid(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```double QRectF::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_left(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::marginsAdded(const QMarginsF& margins) const```</span>
  ///
  ///
  pub fn margins_added(&self, margins: &::margins_f::MarginsF) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_marginsAdded_to_output(self as *const ::rect_f::RectF,
                                                       margins as *const ::margins_f::MarginsF,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::marginsRemoved(const QMarginsF& margins) const```</span>
  ///
  ///
  pub fn margins_removed(&self, margins: &::margins_f::MarginsF) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_marginsRemoved_to_output(self as *const ::rect_f::RectF,
                                                         margins as *const ::margins_f::MarginsF,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveBottom(double pos)```</span>
  ///
  ///
  pub fn move_bottom(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_moveBottom(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveBottomLeft(const QPointF& p)```</span>
  ///
  ///
  pub fn move_bottom_left(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_moveBottomLeft(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveBottomRight(const QPointF& p)```</span>
  ///
  ///
  pub fn move_bottom_right(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_moveBottomRight(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveCenter(const QPointF& p)```</span>
  ///
  ///
  pub fn move_center(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_moveCenter(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveLeft(double pos)```</span>
  ///
  ///
  pub fn move_left(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_moveLeft(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveRight(double pos)```</span>
  ///
  ///
  pub fn move_right(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_moveRight(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```QRectF::moveTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_to(&mut self, &::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRectF::moveTo(const QPointF& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_to(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRectF::moveTo(double x, double y)```</span>
  ///
  ///
  pub fn move_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RectFMoveToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRectF::moveTop(double pos)```</span>
  ///
  ///
  pub fn move_top(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_moveTop(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveTopLeft(const QPointF& p)```</span>
  ///
  ///
  pub fn move_top_left(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_moveTopLeft(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::moveTopRight(const QPointF& p)```</span>
  ///
  ///
  pub fn move_top_right(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_moveTopRight(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```QRectF::QRectF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRectF::QRectF()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::point_f::PointF, &::point_f::PointF)) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRectF::QRectF(const QPointF& topleft, const QPointF& bottomRight)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::point_f::PointF, &::size_f::SizeF)) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRectF::QRectF(const QPointF& topleft, const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::rect::Rect) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRectF::QRectF(const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRectF::QRectF(double left, double top, double width, double height)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::rect_f::RectF
    where Args: overloading::RectFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRectF QRectF::normalized() const```</span>
  ///
  ///
  pub fn normalized(&self) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_normalized_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF& QRectF::operator+=(const QMarginsF& margins)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, margins: &'l1 ::margins_f::MarginsF) -> &'l0 mut ::rect_f::RectF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRectF_operator_add_assign(self as *mut ::rect_f::RectF,
                                                  margins as *const ::margins_f::MarginsF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::operator&(const QRectF& r) const```</span>
  ///
  ///
  pub fn op_bit_and(&self, r: &::rect_f::RectF) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_operator_bit_and_to_output(self as *const ::rect_f::RectF,
                                                           r as *const ::rect_f::RectF,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF& QRectF::operator&=(const QRectF& r)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::rect_f::RectF) -> &'l0 mut ::rect_f::RectF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRectF_operator_bit_and_assign(self as *mut ::rect_f::RectF, r as *const ::rect_f::RectF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::operator|(const QRectF& r) const```</span>
  ///
  ///
  pub fn op_bit_or(&self, r: &::rect_f::RectF) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_operator_bit_or_to_output(self as *const ::rect_f::RectF,
                                                          r as *const ::rect_f::RectF,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF& QRectF::operator|=(const QRectF& r)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::rect_f::RectF) -> &'l0 mut ::rect_f::RectF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRectF_operator_bit_or_assign(self as *mut ::rect_f::RectF, r as *const ::rect_f::RectF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QRectF::operator-=(const QMarginsF& margins)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, margins: &'l1 ::margins_f::MarginsF) -> &'l0 mut ::rect_f::RectF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRectF_operator_sub_assign(self as *mut ::rect_f::RectF,
                                                  margins as *const ::margins_f::MarginsF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double QRectF::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_right(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setBottom(double pos)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setBottom(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setBottomLeft(const QPointF& p)```</span>
  ///
  ///
  pub fn set_bottom_left(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_setBottomLeft(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setBottomRight(const QPointF& p)```</span>
  ///
  ///
  pub fn set_bottom_right(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_setBottomRight(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setCoords(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  pub fn set_coords(&mut self,
                    x1: ::libc::c_double,
                    y1: ::libc::c_double,
                    x2: ::libc::c_double,
                    y2: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setCoords(self as *mut ::rect_f::RectF, x1, y1, x2, y2) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setHeight(double h)```</span>
  ///
  ///
  pub fn set_height(&mut self, h: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setHeight(self as *mut ::rect_f::RectF, h) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setLeft(double pos)```</span>
  ///
  ///
  pub fn set_left(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setLeft(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setRect(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn set_rect(&mut self, x: ::libc::c_double, y: ::libc::c_double, w: ::libc::c_double, h: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setRect(self as *mut ::rect_f::RectF, x, y, w, h) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setRight(double pos)```</span>
  ///
  ///
  pub fn set_right(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setRight(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setSize(const QSizeF& s)```</span>
  ///
  ///
  pub fn set_size(&mut self, s: &::size_f::SizeF) {
    unsafe { ::ffi::qt_core_c_QRectF_setSize(self as *mut ::rect_f::RectF, s as *const ::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setTop(double pos)```</span>
  ///
  ///
  pub fn set_top(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setTop(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setTopLeft(const QPointF& p)```</span>
  ///
  ///
  pub fn set_top_left(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_setTopLeft(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setTopRight(const QPointF& p)```</span>
  ///
  ///
  pub fn set_top_right(&mut self, p: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QRectF_setTopRight(self as *mut ::rect_f::RectF, p as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setWidth(double w)```</span>
  ///
  ///
  pub fn set_width(&mut self, w: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setWidth(self as *mut ::rect_f::RectF, w) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setX(double pos)```</span>
  ///
  ///
  pub fn set_x(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setX(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRectF::setY(double pos)```</span>
  ///
  ///
  pub fn set_y(&mut self, pos: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QRectF_setY(self as *mut ::rect_f::RectF, pos) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QRectF::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::size_f::SizeF {
    {
      let mut object: ::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_size_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QRectF::toAlignedRect() const```</span>
  ///
  ///
  pub fn to_aligned_rect(&self) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_toAlignedRect_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QRectF::toRect() const```</span>
  ///
  ///
  pub fn to_rect(&self) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_toRect_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRectF::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_top(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QRectF::topLeft() const```</span>
  ///
  ///
  pub fn top_left(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_topLeft_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QRectF::topRight() const```</span>
  ///
  ///
  pub fn top_right(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_topRight_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRectF::translate(const QPointF& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRectF::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RectFTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::point_f::PointF) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QRectF::translated(const QPointF& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_double, ::libc::c_double)) -> ::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QRectF::translated(double dx, double dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::rect_f::RectF
    where Args: overloading::RectFTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QRectF::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_transposed_to_output(self as *const ::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QRectF::united(const QRectF& other) const```</span>
  ///
  ///
  pub fn united(&self, other: &::rect_f::RectF) -> ::rect_f::RectF {
    {
      let mut object: ::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRectF_united_to_output(self as *const ::rect_f::RectF,
                                                 other as *const ::rect_f::RectF,
                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRectF::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_width(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```double QRectF::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_x(self as *const ::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```double QRectF::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QRectF_y(self as *const ::rect_f::RectF) }
  }
}

impl Drop for ::rect_f::RectF {
  /// C++ method: <span style='color: green;'>```[destructor] void QRectF::~QRectF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QRectF_destructor(self as *mut ::rect_f::RectF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RectF::contains](../struct.RectF.html#method.contains) method.
  pub trait RectFContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> bool;
  }
  impl<'largs> RectFContainsArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> bool {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QRectF_contains_p(original_self as *const ::rect_f::RectF,
                                           p as *const ::point_f::PointF)
      }
    }
  }
  impl<'largs> RectFContainsArgs<'largs> for &'largs ::rect_f::RectF {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> bool {
      let r = self;
      unsafe {
        ::ffi::qt_core_c_QRectF_contains_r(original_self as *const ::rect_f::RectF,
                                           r as *const ::rect_f::RectF)
      }
    }
  }
  impl<'largs> RectFContainsArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> bool {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_core_c_QRectF_contains_x_y(original_self as *const ::rect_f::RectF, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [RectF::move_to](../struct.RectF.html#method.move_to) method.
  pub trait RectFMoveToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::rect_f::RectF) -> ();
  }
  impl<'largs> RectFMoveToArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::rect_f::RectF) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QRectF_moveTo_p(original_self as *mut ::rect_f::RectF,
                                         p as *const ::point_f::PointF)
      }
    }
  }
  impl<'largs> RectFMoveToArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::rect_f::RectF) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_core_c_QRectF_moveTo_x_y(original_self as *mut ::rect_f::RectF, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [RectF::new](../struct.RectF.html#method.new) method.
  pub trait RectFNewArgs {
    fn exec(self) -> ::rect_f::RectF;
  }
  impl RectFNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::rect_f::RectF {
      let left = self.0;
      let top = self.1;
      let width = self.2;
      let height = self.3;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_constructor_left_top_width_height(left, top, width, height, &mut object);
        }
        object
      }
    }
  }
  impl RectFNewArgs for () {
    fn exec(self) -> ::rect_f::RectF {

      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> RectFNewArgs for &'a ::rect::Rect {
    fn exec(self) -> ::rect_f::RectF {
      let rect = self;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_constructor_rect(rect as *const ::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  impl<'a> RectFNewArgs for (&'a ::point_f::PointF, &'a ::point_f::PointF) {
    fn exec(self) -> ::rect_f::RectF {
      let topleft = self.0;
      let bottom_right = self.1;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_constructor_topleft_bottomRight(topleft as *const ::point_f::PointF,
                                                                  bottom_right as *const ::point_f::PointF,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> RectFNewArgs for (&'a ::point_f::PointF, &'a ::size_f::SizeF) {
    fn exec(self) -> ::rect_f::RectF {
      let topleft = self.0;
      let size = self.1;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_constructor_topleft_size(topleft as *const ::point_f::PointF,
                                                           size as *const ::size_f::SizeF,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RectF::translate](../struct.RectF.html#method.translate) method.
  pub trait RectFTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::rect_f::RectF) -> ();
  }
  impl<'largs> RectFTranslateArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::rect_f::RectF) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_core_c_QRectF_translate_dx_dy(original_self as *mut ::rect_f::RectF, dx, dy) }
    }
  }
  impl<'largs> RectFTranslateArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::rect_f::RectF) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QRectF_translate_p(original_self as *mut ::rect_f::RectF,
                                            p as *const ::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RectF::translated](../struct.RectF.html#method.translated) method.
  pub trait RectFTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> ::rect_f::RectF;
  }
  impl<'largs> RectFTranslatedArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> ::rect_f::RectF {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_translated_to_output_dx_dy(original_self as *const ::rect_f::RectF,
                                                             dx,
                                                             dy,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RectFTranslatedArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs ::rect_f::RectF) -> ::rect_f::RectF {
      let p = self;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRectF_translated_to_output_p(original_self as *const ::rect_f::RectF,
                                                         p as *const ::point_f::PointF,
                                                         &mut object);
        }
        object
      }
    }
  }
}
