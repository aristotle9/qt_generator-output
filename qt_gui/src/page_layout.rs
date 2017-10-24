/// C++ type: <span style='color: green;'>```QPageLayout::Mode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Mode {
  /// C++ enum variant: <span style='color: green;'>```StandardMode = 0```</span>
  Standard = 0,
  /// C++ enum variant: <span style='color: green;'>```FullPageMode = 1```</span>
  FullPage = 1,
}

/// C++ type: <span style='color: green;'>```QPageLayout::Orientation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Orientation {
  /// C++ enum variant: <span style='color: green;'>```Portrait = 0```</span>
  Portrait = 0,
  /// C++ enum variant: <span style='color: green;'>```Landscape = 1```</span>
  Landscape = 1,
}

/// C++ type: <span style='color: green;'>```QPageLayout```</span>
#[repr(C)]
pub struct PageLayout([u8; ::type_sizes::QT_GUI_PAGE_LAYOUT_PAGE_LAYOUT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PageLayout {
  unsafe fn new_uninitialized() -> PageLayout {
    PageLayout(::std::mem::uninitialized())
  }
}

impl PageLayout {
  /// C++ method: <span style='color: green;'>```QPageLayout::fullRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn full_rect(&self, ()) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPageLayout::fullRect() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn full_rect(&self, ::page_layout::Unit) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPageLayout::fullRect(QPageLayout::Unit units) const```</span>
  ///
  ///
  pub fn full_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::PageLayoutFullRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QPageLayout::fullRectPixels(int resolution) const```</span>
  ///
  ///
  pub fn full_rect_pixels(&self, resolution: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_fullRectPixels_to_output(self as *const ::page_layout::PageLayout,
                                                             resolution,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QPageLayout::fullRectPoints() const```</span>
  ///
  ///
  pub fn full_rect_points(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_fullRectPoints_to_output(self as *const ::page_layout::PageLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPageLayout::isEquivalentTo(const QPageLayout& other) const```</span>
  ///
  ///
  pub fn is_equivalent_to(&self, other: &::page_layout::PageLayout) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPageLayout_isEquivalentTo(self as *const ::page_layout::PageLayout,
                                                 other as *const ::page_layout::PageLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPageLayout::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPageLayout_isValid(self as *const ::page_layout::PageLayout) }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::margins```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn margins(&self, ()) -> ::qt_core::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```QMarginsF QPageLayout::margins() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn margins(&self, ::page_layout::Unit) -> ::qt_core::margins_f::MarginsF```<br>
  /// C++ method: <span style='color: green;'>```QMarginsF QPageLayout::margins(QPageLayout::Unit units) const```</span>
  ///
  ///
  pub fn margins<'largs, Args>(&'largs self, args: Args) -> ::qt_core::margins_f::MarginsF
    where Args: overloading::PageLayoutMarginsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMargins QPageLayout::marginsPixels(int resolution) const```</span>
  ///
  ///
  pub fn margins_pixels(&self, resolution: ::libc::c_int) -> ::qt_core::margins::Margins {
    {
      let mut object: ::qt_core::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_marginsPixels_to_output(self as *const ::page_layout::PageLayout,
                                                            resolution,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMargins QPageLayout::marginsPoints() const```</span>
  ///
  ///
  pub fn margins_points(&self) -> ::qt_core::margins::Margins {
    {
      let mut object: ::qt_core::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_marginsPoints_to_output(self as *const ::page_layout::PageLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMarginsF QPageLayout::maximumMargins() const```</span>
  ///
  ///
  pub fn maximum_margins(&self) -> ::qt_core::margins_f::MarginsF {
    {
      let mut object: ::qt_core::margins_f::MarginsF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_maximumMargins_to_output(self as *const ::page_layout::PageLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMarginsF QPageLayout::minimumMargins() const```</span>
  ///
  ///
  pub fn minimum_margins(&self) -> ::qt_core::margins_f::MarginsF {
    {
      let mut object: ::qt_core::margins_f::MarginsF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_minimumMargins_to_output(self as *const ::page_layout::PageLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::Mode QPageLayout::mode() const```</span>
  ///
  ///
  pub fn mode(&self) -> ::page_layout::Mode {
    unsafe { ::ffi::qt_gui_c_QPageLayout_mode(self as *const ::page_layout::PageLayout) }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::QPageLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::page_layout::PageLayout```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageLayout::QPageLayout()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::page_layout::PageLayout) -> ::page_layout::PageLayout```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageLayout::QPageLayout(const QPageLayout& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::page_size::PageSize, ::page_layout::Orientation, &::qt_core::margins_f::MarginsF)) -> ::page_layout::PageLayout```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageLayout::QPageLayout(const QPageSize& pageSize, QPageLayout::Orientation orientation, const QMarginsF& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::page_size::PageSize, ::page_layout::Orientation, &::qt_core::margins_f::MarginsF, ::page_layout::Unit)) -> ::page_layout::PageLayout```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageLayout::QPageLayout(const QPageSize& pageSize, QPageLayout::Orientation orientation, const QMarginsF& margins, QPageLayout::Unit units = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::page_size::PageSize, ::page_layout::Orientation, &::qt_core::margins_f::MarginsF, ::page_layout::Unit, &::qt_core::margins_f::MarginsF)) -> ::page_layout::PageLayout```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageLayout::QPageLayout(const QPageSize& pageSize, QPageLayout::Orientation orientation, const QMarginsF& margins, QPageLayout::Unit units = ?, const QMarginsF& minMargins = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::page_layout::PageLayout
    where Args: overloading::PageLayoutNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPageLayout& QPageLayout::operator=(const QPageLayout& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::page_layout::PageLayout)
                             -> &'l0 mut ::page_layout::PageLayout {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPageLayout_operator_assign(self as *mut ::page_layout::PageLayout,
                                                  other as *const ::page_layout::PageLayout)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::Orientation QPageLayout::orientation() const```</span>
  ///
  ///
  pub fn orientation(&self) -> ::page_layout::Orientation {
    unsafe { ::ffi::qt_gui_c_QPageLayout_orientation(self as *const ::page_layout::PageLayout) }
  }

  /// C++ method: <span style='color: green;'>```QPageSize QPageLayout::pageSize() const```</span>
  ///
  ///
  pub fn page_size(&self) -> ::page_size::PageSize {
    {
      let mut object: ::page_size::PageSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_pageSize_to_output(self as *const ::page_layout::PageLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::paintRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint_rect(&self, ()) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPageLayout::paintRect() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint_rect(&self, ::page_layout::Unit) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPageLayout::paintRect(QPageLayout::Unit units) const```</span>
  ///
  ///
  pub fn paint_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::PageLayoutPaintRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QPageLayout::paintRectPixels(int resolution) const```</span>
  ///
  ///
  pub fn paint_rect_pixels(&self, resolution: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_paintRectPixels_to_output(self as *const ::page_layout::PageLayout,
                                                              resolution,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QPageLayout::paintRectPoints() const```</span>
  ///
  ///
  pub fn paint_rect_points(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_paintRectPoints_to_output(self as *const ::page_layout::PageLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPageLayout::setBottomMargin(double bottomMargin)```</span>
  ///
  ///
  pub fn set_bottom_margin(&mut self, bottom_margin: ::libc::c_double) -> bool {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setBottomMargin(self as *mut ::page_layout::PageLayout, bottom_margin) }
  }

  /// C++ method: <span style='color: green;'>```bool QPageLayout::setLeftMargin(double leftMargin)```</span>
  ///
  ///
  pub fn set_left_margin(&mut self, left_margin: ::libc::c_double) -> bool {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setLeftMargin(self as *mut ::page_layout::PageLayout, left_margin) }
  }

  /// C++ method: <span style='color: green;'>```bool QPageLayout::setMargins(const QMarginsF& margins)```</span>
  ///
  ///
  pub fn set_margins(&mut self, margins: &::qt_core::margins_f::MarginsF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPageLayout_setMargins(self as *mut ::page_layout::PageLayout,
                                             margins as *const ::qt_core::margins_f::MarginsF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPageLayout::setMinimumMargins(const QMarginsF& minMargins)```</span>
  ///
  ///
  pub fn set_minimum_margins(&mut self, min_margins: &::qt_core::margins_f::MarginsF) {
    unsafe {
      ::ffi::qt_gui_c_QPageLayout_setMinimumMargins(self as *mut ::page_layout::PageLayout,
                                                    min_margins as *const ::qt_core::margins_f::MarginsF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPageLayout::setMode(QPageLayout::Mode mode)```</span>
  ///
  ///
  pub fn set_mode(&mut self, mode: ::page_layout::Mode) {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setMode(self as *mut ::page_layout::PageLayout, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QPageLayout::setOrientation(QPageLayout::Orientation orientation)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, orientation: ::page_layout::Orientation) {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setOrientation(self as *mut ::page_layout::PageLayout, orientation) }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::setPageSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_page_size(&mut self, &::page_size::PageSize) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPageLayout::setPageSize(const QPageSize& pageSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_page_size(&mut self, (&::page_size::PageSize, &::qt_core::margins_f::MarginsF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPageLayout::setPageSize(const QPageSize& pageSize, const QMarginsF& minMargins = ?)```</span>
  ///
  ///
  pub fn set_page_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PageLayoutSetPageSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPageLayout::setRightMargin(double rightMargin)```</span>
  ///
  ///
  pub fn set_right_margin(&mut self, right_margin: ::libc::c_double) -> bool {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setRightMargin(self as *mut ::page_layout::PageLayout, right_margin) }
  }

  /// C++ method: <span style='color: green;'>```bool QPageLayout::setTopMargin(double topMargin)```</span>
  ///
  ///
  pub fn set_top_margin(&mut self, top_margin: ::libc::c_double) -> bool {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setTopMargin(self as *mut ::page_layout::PageLayout, top_margin) }
  }

  /// C++ method: <span style='color: green;'>```void QPageLayout::setUnits(QPageLayout::Unit units)```</span>
  ///
  ///
  pub fn set_units(&mut self, units: ::page_layout::Unit) {
    unsafe { ::ffi::qt_gui_c_QPageLayout_setUnits(self as *mut ::page_layout::PageLayout, units) }
  }

  /// C++ method: <span style='color: green;'>```void QPageLayout::swap(QPageLayout& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::page_layout::PageLayout) {
    unsafe {
      ::ffi::qt_gui_c_QPageLayout_swap(self as *mut ::page_layout::PageLayout,
                                       other as *mut ::page_layout::PageLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout::Unit QPageLayout::units() const```</span>
  ///
  ///
  pub fn units(&self) -> ::page_layout::Unit {
    unsafe { ::ffi::qt_gui_c_QPageLayout_units(self as *const ::page_layout::PageLayout) }
  }
}

impl Drop for ::page_layout::PageLayout {
  /// C++ method: <span style='color: green;'>```[destructor] void QPageLayout::~QPageLayout()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPageLayout_destructor(self as *mut ::page_layout::PageLayout) }
  }
}

/// C++ type: <span style='color: green;'>```QPageLayout::Unit```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Unit {
  /// C++ enum variant: <span style='color: green;'>```Millimeter = 0```</span>
  Millimeter = 0,
  /// C++ enum variant: <span style='color: green;'>```Point = 1```</span>
  Point = 1,
  /// C++ enum variant: <span style='color: green;'>```Inch = 2```</span>
  Inch = 2,
  /// C++ enum variant: <span style='color: green;'>```Pica = 3```</span>
  Pica = 3,
  /// C++ enum variant: <span style='color: green;'>```Didot = 4```</span>
  Didot = 4,
  /// C++ enum variant: <span style='color: green;'>```Cicero = 5```</span>
  Cicero = 5,
}

/// C++ method: <span style='color: green;'>```bool operator!=(const QPageLayout& lhs, const QPageLayout& rhs)```</span>
///
///
pub fn op_neq(lhs: &::page_layout::PageLayout, rhs: &::page_layout::PageLayout) -> bool {
  unsafe {
    ::ffi::qt_gui_c_QPageLayout_G_operator_neq(lhs as *const ::page_layout::PageLayout,
                                               rhs as *const ::page_layout::PageLayout)
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QPageLayout& pageLayout)```</span>
///
///
pub fn op_shl(dbg: &::qt_core::debug::Debug, page_layout: &::page_layout::PageLayout) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QPageLayout_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                           page_layout as *const ::page_layout::PageLayout,
                                                           &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QPageLayout& value1, QPageLayout& value2)```</span>
///
///
pub fn swap(value1: &mut ::page_layout::PageLayout, value2: &mut ::page_layout::PageLayout) {
  unsafe {
    ::ffi::qt_gui_c_QPageLayout_G_swap(value1 as *mut ::page_layout::PageLayout,
                                       value2 as *mut ::page_layout::PageLayout)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PageLayout::full_rect](../struct.PageLayout.html#method.full_rect) method.
  pub trait PageLayoutFullRectArgs<'largs> {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> PageLayoutFullRectArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::rect_f::RectF {

      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_fullRect_to_output_no_args(original_self as *const ::page_layout::PageLayout,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PageLayoutFullRectArgs<'largs> for ::page_layout::Unit {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::rect_f::RectF {
      let units = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_fullRect_to_output_units(original_self as *const ::page_layout::PageLayout,
                                                               units,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PageLayout::margins](../struct.PageLayout.html#method.margins) method.
  pub trait PageLayoutMarginsArgs<'largs> {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::margins_f::MarginsF;
  }
  impl<'largs> PageLayoutMarginsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::margins_f::MarginsF {

      {
        let mut object: ::qt_core::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_margins_to_output_no_args(original_self as *const ::page_layout::PageLayout,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PageLayoutMarginsArgs<'largs> for ::page_layout::Unit {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::margins_f::MarginsF {
      let units = self;
      {
        let mut object: ::qt_core::margins_f::MarginsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_margins_to_output_units(original_self as *const ::page_layout::PageLayout,
                                                              units,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PageLayout::new](../struct.PageLayout.html#method.new) method.
  pub trait PageLayoutNewArgs {
    fn exec(self) -> ::page_layout::PageLayout;
  }
  impl PageLayoutNewArgs for () {
    fn exec(self) -> ::page_layout::PageLayout {

      {
        let mut object: ::page_layout::PageLayout =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PageLayoutNewArgs for &'a ::page_layout::PageLayout {
    fn exec(self) -> ::page_layout::PageLayout {
      let other = self;
      {
        let mut object: ::page_layout::PageLayout =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_constructor_other(other as *const ::page_layout::PageLayout, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageLayoutNewArgs
    for (&'a ::page_size::PageSize, ::page_layout::Orientation, &'a ::qt_core::margins_f::MarginsF) {
    fn exec(self) -> ::page_layout::PageLayout {
      let page_size = self.0;
      let orientation = self.1;
      let margins = self.2;
      {
        let mut object: ::page_layout::PageLayout =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins(page_size as *const ::page_size::PageSize, orientation, margins as *const ::qt_core::margins_f::MarginsF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageLayoutNewArgs
    for (&'a ::page_size::PageSize,
                                      ::page_layout::Orientation,
                                      &'a ::qt_core::margins_f::MarginsF,
                                      ::page_layout::Unit) {
    fn exec(self) -> ::page_layout::PageLayout {
      let page_size = self.0;
      let orientation = self.1;
      let margins = self.2;
      let units = self.3;
      {
        let mut object: ::page_layout::PageLayout =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins_units(page_size as *const ::page_size::PageSize, orientation, margins as *const ::qt_core::margins_f::MarginsF, units, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageLayoutNewArgs
    for (&'a ::page_size::PageSize,
                                      ::page_layout::Orientation,
                                      &'a ::qt_core::margins_f::MarginsF,
                                      ::page_layout::Unit,
                                      &'a ::qt_core::margins_f::MarginsF) {
    fn exec(self) -> ::page_layout::PageLayout {
      let page_size = self.0;
      let orientation = self.1;
      let margins = self.2;
      let units = self.3;
      let min_margins = self.4;
      {
        let mut object: ::page_layout::PageLayout =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins_units_minMargins(page_size as *const ::page_size::PageSize, orientation, margins as *const ::qt_core::margins_f::MarginsF, units, min_margins as *const ::qt_core::margins_f::MarginsF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PageLayout::paint_rect](../struct.PageLayout.html#method.paint_rect) method.
  pub trait PageLayoutPaintRectArgs<'largs> {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> PageLayoutPaintRectArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::rect_f::RectF {

      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_paintRect_to_output_no_args(original_self as *const ::page_layout::PageLayout,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PageLayoutPaintRectArgs<'largs> for ::page_layout::Unit {
    fn exec(self, original_self: &'largs ::page_layout::PageLayout) -> ::qt_core::rect_f::RectF {
      let units = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageLayout_paintRect_to_output_units(original_self as *const ::page_layout::PageLayout,
                                                                units,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PageLayout::set_page_size](../struct.PageLayout.html#method.set_page_size) method.
  pub trait PageLayoutSetPageSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::page_layout::PageLayout) -> ();
  }
  impl<'largs> PageLayoutSetPageSizeArgs<'largs> for &'largs ::page_size::PageSize {
    fn exec(self, original_self: &'largs mut ::page_layout::PageLayout) -> () {
      let page_size = self;
      unsafe {
        ::ffi::qt_gui_c_QPageLayout_setPageSize_pageSize(original_self as *mut ::page_layout::PageLayout,
                                                         page_size as *const ::page_size::PageSize)
      }
    }
  }
  impl<'largs> PageLayoutSetPageSizeArgs<'largs>
    for (&'largs ::page_size::PageSize, &'largs ::qt_core::margins_f::MarginsF) {
    fn exec(self, original_self: &'largs mut ::page_layout::PageLayout) -> () {
      let page_size = self.0;
      let min_margins = self.1;
      unsafe { ::ffi::qt_gui_c_QPageLayout_setPageSize_pageSize_minMargins(original_self as *mut ::page_layout::PageLayout, page_size as *const ::page_size::PageSize, min_margins as *const ::qt_core::margins_f::MarginsF) }
    }
  }
}
