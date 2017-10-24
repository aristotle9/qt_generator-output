/// C++ type: <span style='color: green;'>```QRect```</span>
#[repr(C)]
pub struct Rect([u8; ::type_sizes::QT_CORE_RECT_RECT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Rect {
  unsafe fn new_uninitialized() -> Rect {
    Rect(::std::mem::uninitialized())
  }
}

impl Rect {
  /// C++ method: <span style='color: green;'>```void QRect::adjust(int x1, int y1, int x2, int y2)```</span>
  ///
  ///
  pub fn adjust(&mut self, x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_adjust(self as *mut ::rect::Rect, x1, y1, x2, y2) }
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::adjusted(int x1, int y1, int x2, int y2) const```</span>
  ///
  ///
  pub fn adjusted(&self, x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_adjusted_to_output(self as *const ::rect::Rect, x1, y1, x2, y2, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QRect::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_bottom(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QRect::bottomLeft() const```</span>
  ///
  ///
  pub fn bottom_left(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_bottomLeft_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QRect::bottomRight() const```</span>
  ///
  ///
  pub fn bottom_right(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_bottomRight_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QRect::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_center_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::point::Point) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRect::contains(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, (&::point::Point, bool)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRect::contains(const QPoint& p, bool proper = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn contains(&self, &::rect::Rect) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRect::contains(const QRect& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn contains(&self, (&::rect::Rect, bool)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRect::contains(const QRect& r, bool proper = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn contains(&self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRect::contains(int x, int y) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn contains(&self, (::libc::c_int, ::libc::c_int, bool)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRect::contains(int x, int y, bool proper) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::RectContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRect::getCoords(int* x1, int* y1, int* x2, int* y2) const```</span>
  ///
  ///
  pub unsafe fn get_coords(&self,
                           x1: *mut ::libc::c_int,
                           y1: *mut ::libc::c_int,
                           x2: *mut ::libc::c_int,
                           y2: *mut ::libc::c_int) {
    ::ffi::qt_core_c_QRect_getCoords(self as *const ::rect::Rect, x1, y1, x2, y2)
  }

  /// C++ method: <span style='color: green;'>```void QRect::getRect(int* x, int* y, int* w, int* h) const```</span>
  ///
  ///
  pub unsafe fn get_rect(&self,
                         x: *mut ::libc::c_int,
                         y: *mut ::libc::c_int,
                         w: *mut ::libc::c_int,
                         h: *mut ::libc::c_int) {
    ::ffi::qt_core_c_QRect_getRect(self as *const ::rect::Rect, x, y, w, h)
  }

  /// C++ method: <span style='color: green;'>```int QRect::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_height(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::intersected(const QRect& other) const```</span>
  ///
  ///
  pub fn intersected(&self, other: &::rect::Rect) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_intersected_to_output(self as *const ::rect::Rect,
                                                     other as *const ::rect::Rect,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRect::intersects(const QRect& r) const```</span>
  ///
  ///
  pub fn intersects(&self, r: &::rect::Rect) -> bool {
    unsafe { ::ffi::qt_core_c_QRect_intersects(self as *const ::rect::Rect, r as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```bool QRect::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRect_isEmpty(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```bool QRect::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRect_isNull(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```bool QRect::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRect_isValid(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```int QRect::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_left(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::marginsAdded(const QMargins& margins) const```</span>
  ///
  ///
  pub fn margins_added(&self, margins: &::margins::Margins) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_marginsAdded_to_output(self as *const ::rect::Rect,
                                                      margins as *const ::margins::Margins,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::marginsRemoved(const QMargins& margins) const```</span>
  ///
  ///
  pub fn margins_removed(&self, margins: &::margins::Margins) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_marginsRemoved_to_output(self as *const ::rect::Rect,
                                                        margins as *const ::margins::Margins,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveBottom(int pos)```</span>
  ///
  ///
  pub fn move_bottom(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_moveBottom(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveBottomLeft(const QPoint& p)```</span>
  ///
  ///
  pub fn move_bottom_left(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_moveBottomLeft(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveBottomRight(const QPoint& p)```</span>
  ///
  ///
  pub fn move_bottom_right(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_moveBottomRight(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveCenter(const QPoint& p)```</span>
  ///
  ///
  pub fn move_center(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_moveCenter(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveLeft(int pos)```</span>
  ///
  ///
  pub fn move_left(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_moveLeft(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveRight(int pos)```</span>
  ///
  ///
  pub fn move_right(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_moveRight(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```QRect::moveTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_to(&mut self, &::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRect::moveTo(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_to(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRect::moveTo(int x, int t)```</span>
  ///
  ///
  pub fn move_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RectMoveToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRect::moveTop(int pos)```</span>
  ///
  ///
  pub fn move_top(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_moveTop(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveTopLeft(const QPoint& p)```</span>
  ///
  ///
  pub fn move_top_left(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_moveTopLeft(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::moveTopRight(const QPoint& p)```</span>
  ///
  ///
  pub fn move_top_right(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_moveTopRight(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```QRect::QRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRect::QRect()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::point::Point, &::point::Point)) -> ::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRect::QRect(const QPoint& topleft, const QPoint& bottomright)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::point::Point, &::size::Size)) -> ::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRect::QRect(const QPoint& topleft, const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRect::QRect(int left, int top, int width, int height)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::rect::Rect
    where Args: overloading::RectNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRect QRect::normalized() const```</span>
  ///
  ///
  pub fn normalized(&self) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_normalized_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect& QRect::operator+=(const QMargins& margins)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, margins: &'l1 ::margins::Margins) -> &'l0 mut ::rect::Rect {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRect_operator_add_assign(self as *mut ::rect::Rect,
                                                 margins as *const ::margins::Margins)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::operator&(const QRect& r) const```</span>
  ///
  ///
  pub fn op_bit_and(&self, r: &::rect::Rect) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_operator_bit_and_to_output(self as *const ::rect::Rect,
                                                          r as *const ::rect::Rect,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect& QRect::operator&=(const QRect& r)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::rect::Rect) -> &'l0 mut ::rect::Rect {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QRect_operator_bit_and_assign(self as *mut ::rect::Rect, r as *const ::rect::Rect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::operator|(const QRect& r) const```</span>
  ///
  ///
  pub fn op_bit_or(&self, r: &::rect::Rect) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_operator_bit_or_to_output(self as *const ::rect::Rect,
                                                         r as *const ::rect::Rect,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect& QRect::operator|=(const QRect& r)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::rect::Rect) -> &'l0 mut ::rect::Rect {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QRect_operator_bit_or_assign(self as *mut ::rect::Rect, r as *const ::rect::Rect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QRect::operator-=(const QMargins& margins)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, margins: &'l1 ::margins::Margins) -> &'l0 mut ::rect::Rect {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRect_operator_sub_assign(self as *mut ::rect::Rect,
                                                 margins as *const ::margins::Margins)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QRect::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_right(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setBottom(int pos)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setBottom(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setBottomLeft(const QPoint& p)```</span>
  ///
  ///
  pub fn set_bottom_left(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_setBottomLeft(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setBottomRight(const QPoint& p)```</span>
  ///
  ///
  pub fn set_bottom_right(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_setBottomRight(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setCoords(int x1, int y1, int x2, int y2)```</span>
  ///
  ///
  pub fn set_coords(&mut self, x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setCoords(self as *mut ::rect::Rect, x1, y1, x2, y2) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setHeight(int h)```</span>
  ///
  ///
  pub fn set_height(&mut self, h: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setHeight(self as *mut ::rect::Rect, h) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setLeft(int pos)```</span>
  ///
  ///
  pub fn set_left(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setLeft(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setRect(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn set_rect(&mut self, x: ::libc::c_int, y: ::libc::c_int, w: ::libc::c_int, h: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setRect(self as *mut ::rect::Rect, x, y, w, h) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setRight(int pos)```</span>
  ///
  ///
  pub fn set_right(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setRight(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setSize(const QSize& s)```</span>
  ///
  ///
  pub fn set_size(&mut self, s: &::size::Size) {
    unsafe { ::ffi::qt_core_c_QRect_setSize(self as *mut ::rect::Rect, s as *const ::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setTop(int pos)```</span>
  ///
  ///
  pub fn set_top(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setTop(self as *mut ::rect::Rect, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setTopLeft(const QPoint& p)```</span>
  ///
  ///
  pub fn set_top_left(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_setTopLeft(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setTopRight(const QPoint& p)```</span>
  ///
  ///
  pub fn set_top_right(&mut self, p: &::point::Point) {
    unsafe { ::ffi::qt_core_c_QRect_setTopRight(self as *mut ::rect::Rect, p as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setWidth(int w)```</span>
  ///
  ///
  pub fn set_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setWidth(self as *mut ::rect::Rect, w) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setX(int x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setX(self as *mut ::rect::Rect, x) }
  }

  /// C++ method: <span style='color: green;'>```void QRect::setY(int y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QRect_setY(self as *mut ::rect::Rect, y) }
  }

  /// C++ method: <span style='color: green;'>```QSize QRect::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_size_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QRect::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_top(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QRect::topLeft() const```</span>
  ///
  ///
  pub fn top_left(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_topLeft_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QRect::topRight() const```</span>
  ///
  ///
  pub fn top_right(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_topRight_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRect::translate(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRect::translate(int dx, int dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RectTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::point::Point) -> ::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QRect::translated(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_int, ::libc::c_int)) -> ::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QRect::translated(int dx, int dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::rect::Rect
    where Args: overloading::RectTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QRect::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_transposed_to_output(self as *const ::rect::Rect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QRect::united(const QRect& other) const```</span>
  ///
  ///
  pub fn united(&self, other: &::rect::Rect) -> ::rect::Rect {
    {
      let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRect_united_to_output(self as *const ::rect::Rect,
                                                other as *const ::rect::Rect,
                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QRect::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_width(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```int QRect::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_x(self as *const ::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```int QRect::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRect_y(self as *const ::rect::Rect) }
  }
}

impl Drop for ::rect::Rect {
  /// C++ method: <span style='color: green;'>```[destructor] void QRect::~QRect()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QRect_destructor(self as *mut ::rect::Rect) }
  }
}

/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_add((&::margins::Margins, &::rect::Rect)) -> ::rect::Rect```<br>
/// C++ method: <span style='color: green;'>```QRect operator+(const QMargins& margins, const QRect& rectangle)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_add((&::rect::Rect, &::margins::Margins)) -> ::rect::Rect```<br>
/// C++ method: <span style='color: green;'>```QRect operator+(const QRect& rectangle, const QMargins& margins)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_add((&::margins_f::MarginsF, &::rect_f::RectF)) -> ::rect_f::RectF```<br>
/// C++ method: <span style='color: green;'>```QRectF operator+(const QMarginsF& lhs, const QRectF& rhs)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_add((&::rect_f::RectF, &::margins_f::MarginsF)) -> ::rect_f::RectF```<br>
/// C++ method: <span style='color: green;'>```QRectF operator+(const QRectF& lhs, const QMarginsF& rhs)```</span>
///
///
pub fn op_add<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpAddArgs
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::rect::Rect)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QRect& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::rect_f::RectF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QRectF& arg2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::rect::Rect)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QRect& arg2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::rect_f::RectF)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QRectF& arg2)```</span>
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
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::rect::Rect)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QRect& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::rect_f::RectF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QRectF& arg2)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator-```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_sub((&::rect::Rect, &::margins::Margins)) -> ::rect::Rect```<br>
/// C++ method: <span style='color: green;'>```QRect operator-(const QRect& lhs, const QMargins& rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_sub((&::rect_f::RectF, &::margins_f::MarginsF)) -> ::rect_f::RectF```<br>
/// C++ method: <span style='color: green;'>```QRectF operator-(const QRectF& lhs, const QMarginsF& rhs)```</span>
///
///
pub fn op_sub<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpSubArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Rect::contains](../struct.Rect.html#method.contains) method.
  pub trait RectContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool;
  }
  impl<'largs> RectContainsArgs<'largs> for &'largs ::point::Point {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QRect_contains_p(original_self as *const ::rect::Rect,
                                          p as *const ::point::Point)
      }
    }
  }
  impl<'largs> RectContainsArgs<'largs> for (&'largs ::point::Point, bool) {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool {
      let p = self.0;
      let proper = self.1;
      unsafe {
        ::ffi::qt_core_c_QRect_contains_p_proper(original_self as *const ::rect::Rect,
                                                 p as *const ::point::Point,
                                                 proper)
      }
    }
  }
  impl<'largs> RectContainsArgs<'largs> for &'largs ::rect::Rect {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool {
      let r = self;
      unsafe {
        ::ffi::qt_core_c_QRect_contains_r(original_self as *const ::rect::Rect,
                                          r as *const ::rect::Rect)
      }
    }
  }
  impl<'largs> RectContainsArgs<'largs> for (&'largs ::rect::Rect, bool) {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool {
      let r = self.0;
      let proper = self.1;
      unsafe {
        ::ffi::qt_core_c_QRect_contains_r_proper(original_self as *const ::rect::Rect,
                                                 r as *const ::rect::Rect,
                                                 proper)
      }
    }
  }
  impl<'largs> RectContainsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_core_c_QRect_contains_x_y(original_self as *const ::rect::Rect, x, y) }
    }
  }
  impl<'largs> RectContainsArgs<'largs> for (::libc::c_int, ::libc::c_int, bool) {
    fn exec(self, original_self: &'largs ::rect::Rect) -> bool {
      let x = self.0;
      let y = self.1;
      let proper = self.2;
      unsafe { ::ffi::qt_core_c_QRect_contains_x_y_proper(original_self as *const ::rect::Rect, x, y, proper) }
    }
  }
  /// This trait represents a set of arguments accepted by [Rect::move_to](../struct.Rect.html#method.move_to) method.
  pub trait RectMoveToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::rect::Rect) -> ();
  }
  impl<'largs> RectMoveToArgs<'largs> for &'largs ::point::Point {
    fn exec(self, original_self: &'largs mut ::rect::Rect) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QRect_moveTo_p(original_self as *mut ::rect::Rect,
                                        p as *const ::point::Point)
      }
    }
  }
  impl<'largs> RectMoveToArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::rect::Rect) -> () {
      let x = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_core_c_QRect_moveTo_x_t(original_self as *mut ::rect::Rect, x, t) }
    }
  }
  /// This trait represents a set of arguments accepted by [Rect::new](../struct.Rect.html#method.new) method.
  pub trait RectNewArgs {
    fn exec(self) -> ::rect::Rect;
  }
  impl RectNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::rect::Rect {
      let left = self.0;
      let top = self.1;
      let width = self.2;
      let height = self.3;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_constructor_left_top_width_height(left, top, width, height, &mut object);
        }
        object
      }
    }
  }
  impl RectNewArgs for () {
    fn exec(self) -> ::rect::Rect {

      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> RectNewArgs for (&'a ::point::Point, &'a ::point::Point) {
    fn exec(self) -> ::rect::Rect {
      let topleft = self.0;
      let bottomright = self.1;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_constructor_topleft_bottomright(topleft as *const ::point::Point,
                                                                 bottomright as *const ::point::Point,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> RectNewArgs for (&'a ::point::Point, &'a ::size::Size) {
    fn exec(self) -> ::rect::Rect {
      let topleft = self.0;
      let size = self.1;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_constructor_topleft_size(topleft as *const ::point::Point,
                                                          size as *const ::size::Size,
                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Rect::translate](../struct.Rect.html#method.translate) method.
  pub trait RectTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::rect::Rect) -> ();
  }
  impl<'largs> RectTranslateArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::rect::Rect) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_core_c_QRect_translate_dx_dy(original_self as *mut ::rect::Rect, dx, dy) }
    }
  }
  impl<'largs> RectTranslateArgs<'largs> for &'largs ::point::Point {
    fn exec(self, original_self: &'largs mut ::rect::Rect) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QRect_translate_p(original_self as *mut ::rect::Rect,
                                           p as *const ::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Rect::translated](../struct.Rect.html#method.translated) method.
  pub trait RectTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::rect::Rect) -> ::rect::Rect;
  }
  impl<'largs> RectTranslatedArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::rect::Rect) -> ::rect::Rect {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_translated_to_output_dx_dy(original_self as *const ::rect::Rect, dx, dy, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RectTranslatedArgs<'largs> for &'largs ::point::Point {
    fn exec(self, original_self: &'largs ::rect::Rect) -> ::rect::Rect {
      let p = self;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_translated_to_output_p(original_self as *const ::rect::Rect,
                                                        p as *const ::point::Point,
                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_add](../fn.op_add.html) method.
  pub trait OpAddArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpAddArgs for (&'a ::margins_f::MarginsF, &'a ::rect_f::RectF) {
    type ReturnType = ::rect_f::RectF;
    fn exec(self) -> ::rect_f::RectF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_add_to_output_QMarginsF_QRectF(lhs as *const ::margins_f::MarginsF,
                                                                           rhs as *const ::rect_f::RectF,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::margins::Margins, &'a ::rect::Rect) {
    type ReturnType = ::rect::Rect;
    fn exec(self) -> ::rect::Rect {
      let margins = self.0;
      let rectangle = self.1;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_add_to_output_QMargins_QRect(margins as *const ::margins::Margins,
                                                                         rectangle as *const ::rect::Rect,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::rect_f::RectF, &'a ::margins_f::MarginsF) {
    type ReturnType = ::rect_f::RectF;
    fn exec(self) -> ::rect_f::RectF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_add_to_output_QRectF_QMarginsF(lhs as *const ::rect_f::RectF,
                                                                           rhs as *const ::margins_f::MarginsF,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::rect::Rect, &'a ::margins::Margins) {
    type ReturnType = ::rect::Rect;
    fn exec(self) -> ::rect::Rect {
      let rectangle = self.0;
      let margins = self.1;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_add_to_output_QRect_QMargins(rectangle as *const ::rect::Rect,
                                                                         margins as *const ::margins::Margins,
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
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::rect::Rect) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QRect_G_operator_shl_QDataStream_QRect(arg1 as *mut ::data_stream::DataStream,
                                                                arg2 as *const ::rect::Rect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::rect_f::RectF) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_shl_QDataStream_QRectF(arg1 as *mut ::data_stream::DataStream,
                                                                   arg2 as *const ::rect_f::RectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::rect::Rect) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_shl_to_output_QDebug_QRect(arg1 as *const ::debug::Debug,
                                                                       arg2 as *const ::rect::Rect,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::rect_f::RectF) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_shl_to_output_QDebug_QRectF(arg1 as *const ::debug::Debug,
                                                                        arg2 as *const ::rect_f::RectF,
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
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::rect::Rect) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QRect_G_operator_shr_QDataStream_QRect(arg1 as *mut ::data_stream::DataStream,
                                                                arg2 as *mut ::rect::Rect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::rect_f::RectF) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_shr_QDataStream_QRectF(arg1 as *mut ::data_stream::DataStream,
                                                                   arg2 as *mut ::rect_f::RectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [op_sub](../fn.op_sub.html) method.
  pub trait OpSubArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpSubArgs for (&'a ::rect_f::RectF, &'a ::margins_f::MarginsF) {
    type ReturnType = ::rect_f::RectF;
    fn exec(self) -> ::rect_f::RectF {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_sub_to_output_QRectF_QMarginsF(lhs as *const ::rect_f::RectF,
                                                                           rhs as *const ::margins_f::MarginsF,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpSubArgs for (&'a ::rect::Rect, &'a ::margins::Margins) {
    type ReturnType = ::rect::Rect;
    fn exec(self) -> ::rect::Rect {
      let lhs = self.0;
      let rhs = self.1;
      {
        let mut object: ::rect::Rect = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRect_G_operator_sub_to_output_QRect_QMargins(lhs as *const ::rect::Rect,
                                                                         rhs as *const ::margins::Margins,
                                                                         &mut object);
        }
        object
      }
    }
  }
}
