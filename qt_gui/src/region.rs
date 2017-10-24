/// C++ type: <span style='color: green;'>```QRegion```</span>
#[repr(C)]
pub struct Region(u8);

impl Region {
  /// C++ method: <span style='color: green;'>```QVariant QRegion::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRegion_convert_to_QVariant_to_output(self as *const ::region::Region, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QRect* QRegion::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> *const ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QRegion_begin(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```QRect QRegion::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRegion_boundingRect_to_output(self as *const ::region::Region, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QRect* QRegion::cbegin() const```</span>
  ///
  ///
  pub fn cbegin(&self) -> *const ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QRegion_cbegin(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```const QRect* QRegion::cend() const```</span>
  ///
  ///
  pub fn cend(&self) -> *const ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QRegion_cend(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```QRegion::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::qt_core::point::Point) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRegion::contains(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, &::qt_core::rect::Rect) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRegion::contains(const QRect& r) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::RegionContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRect* QRegion::end() const```</span>
  ///
  ///
  pub fn end(&self) -> *const ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QRegion_end(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```QRegion::intersected```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn intersected(&self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::intersected(const QRect& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn intersected(&self, &::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::intersected(const QRegion& r) const```</span>
  ///
  ///
  pub fn intersected<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::region::Region>
    where Args: overloading::RegionIntersectedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion::intersects```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn intersects(&self, &::qt_core::rect::Rect) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRegion::intersects(const QRect& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn intersects(&self, &::region::Region) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRegion::intersects(const QRegion& r) const```</span>
  ///
  ///
  pub fn intersects<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::RegionIntersectsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QRegion::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QRegion_isEmpty(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```bool QRegion::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QRegion_isNull(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```QRegion::QRegion```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::bitmap::Bitmap) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(const QBitmap& bitmap)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::polygon::Polygon) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(const QPolygon& pa)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::polygon::Polygon, &::qt_core::qt::FillRule)) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(const QPolygon& pa, Qt::FillRule fillRule = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::qt_core::rect::Rect, ::region::RegionType)) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(const QRect& r, QRegion::RegionType t = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(const QRegion& region)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(int x, int y, int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::region::RegionType)) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegion::QRegion(int x, int y, int w, int h, QRegion::RegionType t = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::region::Region>
    where Args: overloading::RegionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegion::operator+```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add(&self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator+(const QRect& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add(&self, &::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator+(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_add<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::region::Region>
    where Args: overloading::RegionOpAddArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::rect::Rect) -> &'l0 mut ::region::Region```<br>
  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator+=(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::region::Region) -> &'l0 mut ::region::Region```<br>
  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator+=(const QRegion& r)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::region::Region
    where Args: overloading::RegionOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator=(const QRegion& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::region::Region) -> &'l0 mut ::region::Region {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_assign(self as *mut ::region::Region,
                                              arg1 as *const ::region::Region)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRegion::operator&```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_and(&self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator&(const QRect& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_and(&self, &::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator&(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_bit_and<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::region::Region>
    where Args: overloading::RegionOpBitAndArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion::operator&=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_and_assign(&mut self, &'l1 ::qt_core::rect::Rect) -> &'l0 mut ::region::Region```<br>
  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator&=(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_and_assign(&mut self, &'l1 ::region::Region) -> &'l0 mut ::region::Region```<br>
  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator&=(const QRegion& r)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::region::Region
    where Args: overloading::RegionOpBitAndAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator|(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_bit_or(&self, r: &::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_bit_or_as_ptr(self as *const ::region::Region,
                                                     r as *const ::region::Region)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator|=(const QRegion& r)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::region::Region) -> &'l0 mut ::region::Region {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_bit_or_assign(self as *mut ::region::Region, r as *const ::region::Region)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator^(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_bit_xor(&self, r: &::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_bit_xor_as_ptr(self as *const ::region::Region,
                                                      r as *const ::region::Region)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator^=(const QRegion& r)```</span>
  ///
  ///
  pub fn op_bit_xor_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::region::Region) -> &'l0 mut ::region::Region {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_bit_xor_assign(self as *mut ::region::Region, r as *const ::region::Region)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QRegion::operator==(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_eq(&self, r: &::region::Region) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QRegion_operator_eq(self as *const ::region::Region,
                                          r as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRegion::operator!=(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_neq(&self, r: &::region::Region) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QRegion_operator_neq(self as *const ::region::Region,
                                           r as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QRegion::operator-(const QRegion& r) const```</span>
  ///
  ///
  pub fn op_sub(&self, r: &::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_sub_as_ptr(self as *const ::region::Region,
                                                  r as *const ::region::Region)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QRegion& QRegion::operator-=(const QRegion& r)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, r: &'l1 ::region::Region) -> &'l0 mut ::region::Region {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_operator_sub_assign(self as *mut ::region::Region, r as *const ::region::Region)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QRegion::rectCount() const```</span>
  ///
  ///
  pub fn rect_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QRegion_rectCount(self as *const ::region::Region) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect> QRegion::rects() const```</span>
  ///
  ///
  pub fn rects(&self) -> ::vector::VectorQtCoreRect {
    {
      let mut object: ::vector::VectorQtCoreRect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRegion_rects_to_output(self as *const ::region::Region, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegion::setRects(const QRect* rect, int num)```</span>
  ///
  ///
  pub unsafe fn set_rects(&mut self, rect: *const ::qt_core::rect::Rect, num: ::libc::c_int) {
    ::ffi::qt_gui_c_QRegion_setRects(self as *mut ::region::Region, rect, num)
  }

  /// C++ method: <span style='color: green;'>```QRegion QRegion::subtracted(const QRegion& r) const```</span>
  ///
  ///
  pub fn subtracted(&self, r: &::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_subtracted_as_ptr(self as *const ::region::Region,
                                                r as *const ::region::Region)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QRegion::swap(QRegion& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QRegion_swap(self as *mut ::region::Region,
                                   other as *mut ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRegion::translate(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QRegion::translate(int dx, int dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::RegionTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::qt_core::point::Point) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::translated(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::translated(int dx, int dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::region::Region>
    where Args: overloading::RegionTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion::united```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn united(&self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::united(const QRect& r) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn united(&self, &::region::Region) -> ::cpp_utils::CppBox<::region::Region>```<br>
  /// C++ method: <span style='color: green;'>```QRegion QRegion::united(const QRegion& r) const```</span>
  ///
  ///
  pub fn united<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::region::Region>
    where Args: overloading::RegionUnitedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegion QRegion::xored(const QRegion& r) const```</span>
  ///
  ///
  pub fn xored(&self, r: &::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRegion_xored_as_ptr(self as *const ::region::Region,
                                           r as *const ::region::Region)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::region::Region {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QRegion_delete
  }
}

/// C++ type: <span style='color: green;'>```QRegion::RegionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RegionType {
  /// C++ enum variant: <span style='color: green;'>```Rectangle = 0```</span>
  Rectangle = 0,
  /// C++ enum variant: <span style='color: green;'>```Ellipse = 1```</span>
  Ellipse = 1,
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QRegion& arg2)```</span>
///
///
pub fn op_shl(arg1: &::qt_core::debug::Debug, arg2: &::region::Region) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QRegion_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                       arg2 as *const ::region::Region,
                                                       &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QRegion& value1, QRegion& value2)```</span>
///
///
pub fn swap(value1: &mut ::region::Region, value2: &mut ::region::Region) {
  unsafe {
    ::ffi::qt_gui_c_QRegion_G_swap(value1 as *mut ::region::Region,
                                   value2 as *mut ::region::Region)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Region::contains](../struct.Region.html#method.contains) method.
  pub trait RegionContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> bool;
  }
  impl<'largs> RegionContainsArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::region::Region) -> bool {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QRegion_contains_p(original_self as *const ::region::Region,
                                           p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> RegionContainsArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::region::Region) -> bool {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QRegion_contains_r(original_self as *const ::region::Region,
                                           r as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::intersected](../struct.Region.html#method.intersected) method.
  pub trait RegionIntersectedArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region>;
  }
  impl<'largs> RegionIntersectedArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_intersected_as_ptr_QRect(original_self as *const ::region::Region,
                                                         r as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RegionIntersectedArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_intersected_as_ptr_QRegion(original_self as *const ::region::Region,
                                                           r as *const ::region::Region)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::intersects](../struct.Region.html#method.intersects) method.
  pub trait RegionIntersectsArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> bool;
  }
  impl<'largs> RegionIntersectsArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::region::Region) -> bool {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QRegion_intersects_QRect(original_self as *const ::region::Region,
                                                 r as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> RegionIntersectsArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs ::region::Region) -> bool {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QRegion_intersects_QRegion(original_self as *const ::region::Region,
                                                   r as *const ::region::Region)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::new](../struct.Region.html#method.new) method.
  pub trait RegionNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region>;
  }
  impl<'a> RegionNewArgs for &'a ::bitmap::Bitmap {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let bitmap = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_bitmap(bitmap as *const ::bitmap::Bitmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RegionNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegionNewArgs for &'a ::polygon::Polygon {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let pa = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_pa(pa as *const ::polygon::Polygon) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegionNewArgs for (&'a ::polygon::Polygon, &'a ::qt_core::qt::FillRule) {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let pa = self.0;
      let fill_rule = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_new_pa_fillRule(pa as *const ::polygon::Polygon,
                                                fill_rule as *const ::qt_core::qt::FillRule)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegionNewArgs for &'a ::qt_core::rect::Rect {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_r(r as *const ::qt_core::rect::Rect) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegionNewArgs for (&'a ::qt_core::rect::Rect, ::region::RegionType) {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self.0;
      let t = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_r_t(r as *const ::qt_core::rect::Rect, t) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegionNewArgs for &'a ::region::Region {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let region = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_region(region as *const ::region::Region) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RegionNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_x_y_w_h(x, y, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RegionNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::region::RegionType) {
    fn exec(self) -> ::cpp_utils::CppBox<::region::Region> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let t = self.4;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegion_new_x_y_w_h_t(x, y, w, h, t) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::op_add](../struct.Region.html#method.op_add) method.
  pub trait RegionOpAddArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region>;
  }
  impl<'largs> RegionOpAddArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_add_as_ptr_QRect(original_self as *const ::region::Region,
                                                          r as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RegionOpAddArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_add_as_ptr_QRegion(original_self as *const ::region::Region,
                                                            r as *const ::region::Region)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::op_add_assign](../struct.Region.html#method.op_add_assign) method.
  pub trait RegionOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::region::Region) -> &'largs mut ::region::Region;
  }
  impl<'largs> RegionOpAddAssignArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::region::Region) -> &'largs mut ::region::Region {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_add_assign_QRect(original_self as *mut ::region::Region,
                                                          r as *const ::qt_core::rect::Rect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> RegionOpAddAssignArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs mut ::region::Region) -> &'largs mut ::region::Region {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_add_assign_QRegion(original_self as *mut ::region::Region,
                                                            r as *const ::region::Region)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Region::op_bit_and](../struct.Region.html#method.op_bit_and) method.
  pub trait RegionOpBitAndArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region>;
  }
  impl<'largs> RegionOpBitAndArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_bit_and_as_ptr_QRect(original_self as *const ::region::Region,
                                                              r as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RegionOpBitAndArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QRegion_operator_bit_and_as_ptr_QRegion(original_self as *const ::region::Region,
                                                                  r as *const ::region::Region)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::op_bit_and_assign](../struct.Region.html#method.op_bit_and_assign) method.
  pub trait RegionOpBitAndAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::region::Region) -> &'largs mut ::region::Region;
  }
  impl<'largs> RegionOpBitAndAssignArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::region::Region) -> &'largs mut ::region::Region {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_bit_and_assign_QRect(original_self as *mut ::region::Region,
                                                              r as *const ::qt_core::rect::Rect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> RegionOpBitAndAssignArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs mut ::region::Region) -> &'largs mut ::region::Region {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_operator_bit_and_assign_QRegion(original_self as *mut ::region::Region,
                                                                r as *const ::region::Region)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Region::translate](../struct.Region.html#method.translate) method.
  pub trait RegionTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::region::Region) -> ();
  }
  impl<'largs> RegionTranslateArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::region::Region) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_gui_c_QRegion_translate_dx_dy(original_self as *mut ::region::Region, dx, dy) }
    }
  }
  impl<'largs> RegionTranslateArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::region::Region) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QRegion_translate_p(original_self as *mut ::region::Region,
                                            p as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::translated](../struct.Region.html#method.translated) method.
  pub trait RegionTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region>;
  }
  impl<'largs> RegionTranslatedArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let dx = self.0;
      let dy = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QRegion_translated_as_ptr_dx_dy(original_self as *const ::region::Region, dx, dy) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RegionTranslatedArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let p = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_translated_as_ptr_p(original_self as *const ::region::Region,
                                                    p as *const ::qt_core::point::Point)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Region::united](../struct.Region.html#method.united) method.
  pub trait RegionUnitedArgs<'largs> {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region>;
  }
  impl<'largs> RegionUnitedArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_united_as_ptr_QRect(original_self as *const ::region::Region,
                                                    r as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RegionUnitedArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs ::region::Region) -> ::cpp_utils::CppBox<::region::Region> {
      let r = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QRegion_united_as_ptr_QRegion(original_self as *const ::region::Region,
                                                      r as *const ::region::Region)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
