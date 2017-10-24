/// C++ type: <span style='color: green;'>```QPageSize```</span>
#[repr(C)]
pub struct PageSize([u8; ::type_sizes::QT_GUI_PAGE_SIZE_PAGE_SIZE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PageSize {
  unsafe fn new_uninitialized() -> PageSize {
    PageSize(::std::mem::uninitialized())
  }
}

impl PageSize {
  /// C++ method: <span style='color: green;'>```QSizeF QPageSize::definitionSize() const```</span>
  ///
  ///
  pub fn definition_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_definitionSize_to_output_no_args(self as *const ::page_size::PageSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QSizeF QPageSize::definitionSize(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  pub fn definition_size_static(page_size_id: ::page_size::PageSizeId) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_definitionSize_to_output_pageSizeId(page_size_id, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPageSize::Unit QPageSize::definitionUnits() const```</span>
  ///
  ///
  pub fn definition_units(&self) -> ::page_size::Unit {
    unsafe { ::ffi::qt_gui_c_QPageSize_definitionUnits_no_args(self as *const ::page_size::PageSize) }
  }

  /// C++ method: <span style='color: green;'>```static QPageSize::Unit QPageSize::definitionUnits(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  pub fn definition_units_static(page_size_id: ::page_size::PageSizeId) -> ::page_size::Unit {
    unsafe { ::ffi::qt_gui_c_QPageSize_definitionUnits_pageSizeId(page_size_id) }
  }

  /// C++ method: <span style='color: green;'>```QPageSize::PageSizeId QPageSize::id() const```</span>
  ///
  ///
  pub fn id(&self) -> ::page_size::PageSizeId {
    unsafe { ::ffi::qt_gui_c_QPageSize_id_no_args(self as *const ::page_size::PageSize) }
  }

  /// C++ method: <span style='color: green;'>```QPageSize::id```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn id_static(&::qt_core::size::Size) -> ::page_size::PageSizeId```<br>
  /// C++ method: <span style='color: green;'>```static QPageSize::PageSizeId QPageSize::id(const QSize& pointSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn id_static((&::qt_core::size::Size, ::page_size::SizeMatchPolicy)) -> ::page_size::PageSizeId```<br>
  /// C++ method: <span style='color: green;'>```static QPageSize::PageSizeId QPageSize::id(const QSize& pointSize, QPageSize::SizeMatchPolicy matchPolicy = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn id_static((&::qt_core::size_f::SizeF, ::page_size::Unit)) -> ::page_size::PageSizeId```<br>
  /// C++ method: <span style='color: green;'>```static QPageSize::PageSizeId QPageSize::id(const QSizeF& size, QPageSize::Unit units)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn id_static((&::qt_core::size_f::SizeF, ::page_size::Unit, ::page_size::SizeMatchPolicy)) -> ::page_size::PageSizeId```<br>
  /// C++ method: <span style='color: green;'>```static QPageSize::PageSizeId QPageSize::id(const QSizeF& size, QPageSize::Unit units, QPageSize::SizeMatchPolicy matchPolicy = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn id_static(::libc::c_int) -> ::page_size::PageSizeId```<br>
  /// C++ method: <span style='color: green;'>```static QPageSize::PageSizeId QPageSize::id(int windowsId)```</span>
  ///
  ///
  pub fn id_static<Args>(args: Args) -> ::page_size::PageSizeId
    where Args: overloading::PageSizeIdStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QPageSize::isEquivalentTo(const QPageSize& other) const```</span>
  ///
  ///
  pub fn is_equivalent_to(&self, other: &::page_size::PageSize) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPageSize_isEquivalentTo(self as *const ::page_size::PageSize,
                                               other as *const ::page_size::PageSize)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPageSize::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPageSize_isValid(self as *const ::page_size::PageSize) }
  }

  /// C++ method: <span style='color: green;'>```QString QPageSize::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_key_to_output_no_args(self as *const ::page_size::PageSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPageSize::key(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  pub fn key_static(page_size_id: ::page_size::PageSizeId) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_key_to_output_pageSizeId(page_size_id, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QPageSize::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_name_to_output_no_args(self as *const ::page_size::PageSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPageSize::name(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  pub fn name_static(page_size_id: ::page_size::PageSizeId) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_name_to_output_pageSizeId(page_size_id, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPageSize::QPageSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::page_size::PageSizeId) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::page_size::PageSize) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QPageSize& other)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::size::Size) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QSize& pointSize)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, &::qt_core::string::String)) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QSize& pointSize, const QString& name = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, &::qt_core::string::String, ::page_size::SizeMatchPolicy)) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QSize& pointSize, const QString& name = ?, QPageSize::SizeMatchPolicy matchPolicy = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new((&::qt_core::size_f::SizeF, ::page_size::Unit)) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QSizeF& size, QPageSize::Unit units)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((&::qt_core::size_f::SizeF, ::page_size::Unit, &::qt_core::string::String)) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QSizeF& size, QPageSize::Unit units, const QString& name = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((&::qt_core::size_f::SizeF, ::page_size::Unit, &::qt_core::string::String, ::page_size::SizeMatchPolicy)) -> ::page_size::PageSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPageSize::QPageSize(const QSizeF& size, QPageSize::Unit units, const QString& name = ?, QPageSize::SizeMatchPolicy matchPolicy = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::page_size::PageSize
    where Args: overloading::PageSizeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPageSize& QPageSize::operator=(const QPageSize& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::page_size::PageSize) -> &'l0 mut ::page_size::PageSize {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPageSize_operator_assign(self as *mut ::page_size::PageSize,
                                                other as *const ::page_size::PageSize)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF QPageSize::rect(QPageSize::Unit units) const```</span>
  ///
  ///
  pub fn rect(&self, units: ::page_size::Unit) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_rect_to_output(self as *const ::page_size::PageSize, units, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QPageSize::rectPixels(int resolution) const```</span>
  ///
  ///
  pub fn rect_pixels(&self, resolution: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_rectPixels_to_output(self as *const ::page_size::PageSize,
                                                       resolution,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QPageSize::rectPoints() const```</span>
  ///
  ///
  pub fn rect_points(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_rectPoints_to_output(self as *const ::page_size::PageSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QPageSize::size(QPageSize::Unit units) const```</span>
  ///
  ///
  pub fn size(&self, units: ::page_size::Unit) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_size_to_output_units(self as *const ::page_size::PageSize, units, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QPageSize::sizePixels(int resolution) const```</span>
  ///
  ///
  pub fn size_pixels(&self, resolution: ::libc::c_int) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_sizePixels_to_output_resolution(self as *const ::page_size::PageSize,
                                                                  resolution,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QSize QPageSize::sizePixels(QPageSize::PageSizeId pageSizeId, int resolution)```</span>
  ///
  ///
  pub fn size_pixels_static(page_size_id: ::page_size::PageSizeId, resolution: ::libc::c_int) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_sizePixels_to_output_pageSizeId_resolution(page_size_id, resolution, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QPageSize::sizePoints() const```</span>
  ///
  ///
  pub fn size_points(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_sizePoints_to_output_no_args(self as *const ::page_size::PageSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QSize QPageSize::sizePoints(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  pub fn size_points_static(page_size_id: ::page_size::PageSizeId) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_sizePoints_to_output_pageSizeId(page_size_id, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QSizeF QPageSize::size(QPageSize::PageSizeId pageSizeId, QPageSize::Unit units)```</span>
  ///
  ///
  pub fn size_static(page_size_id: ::page_size::PageSizeId, units: ::page_size::Unit) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPageSize_size_to_output_pageSizeId_units(page_size_id, units, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPageSize::swap(QPageSize& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::page_size::PageSize) {
    unsafe {
      ::ffi::qt_gui_c_QPageSize_swap(self as *mut ::page_size::PageSize,
                                     other as *mut ::page_size::PageSize)
    }
  }

  /// C++ method: <span style='color: green;'>```int QPageSize::windowsId() const```</span>
  ///
  ///
  pub fn windows_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPageSize_windowsId_no_args(self as *const ::page_size::PageSize) }
  }

  /// C++ method: <span style='color: green;'>```static int QPageSize::windowsId(QPageSize::PageSizeId pageSizeId)```</span>
  ///
  ///
  pub fn windows_id_static(page_size_id: ::page_size::PageSizeId) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPageSize_windowsId_pageSizeId(page_size_id) }
  }
}

impl Drop for ::page_size::PageSize {
  /// C++ method: <span style='color: green;'>```[destructor] void QPageSize::~QPageSize()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPageSize_destructor(self as *mut ::page_size::PageSize) }
  }
}

/// C++ type: <span style='color: green;'>```QPageSize::PageSizeId```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PageSizeId {
  /// C++ enum variant: <span style='color: green;'>```A4 = 0```</span>
  A4 = 0,
  /// C++ enum variant: <span style='color: green;'>```B5 = 1```</span>
  B5 = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Letter = 2```</span>
  /// - <span style='color: green;'>```AnsiA = 2```</span>
  ///
  Letter = 2,
  /// C++ enum variant: <span style='color: green;'>```Legal = 3```</span>
  Legal = 3,
  /// C++ enum variant: <span style='color: green;'>```Executive = 4```</span>
  Executive = 4,
  /// C++ enum variant: <span style='color: green;'>```A0 = 5```</span>
  A0 = 5,
  /// C++ enum variant: <span style='color: green;'>```A1 = 6```</span>
  A1 = 6,
  /// C++ enum variant: <span style='color: green;'>```A2 = 7```</span>
  A2 = 7,
  /// C++ enum variant: <span style='color: green;'>```A3 = 8```</span>
  A3 = 8,
  /// C++ enum variant: <span style='color: green;'>```A5 = 9```</span>
  A5 = 9,
  /// C++ enum variant: <span style='color: green;'>```A6 = 10```</span>
  A6 = 10,
  /// C++ enum variant: <span style='color: green;'>```A7 = 11```</span>
  A7 = 11,
  /// C++ enum variant: <span style='color: green;'>```A8 = 12```</span>
  A8 = 12,
  /// C++ enum variant: <span style='color: green;'>```A9 = 13```</span>
  A9 = 13,
  /// C++ enum variant: <span style='color: green;'>```B0 = 14```</span>
  B0 = 14,
  /// C++ enum variant: <span style='color: green;'>```B1 = 15```</span>
  B1 = 15,
  /// C++ enum variant: <span style='color: green;'>```B10 = 16```</span>
  B10 = 16,
  /// C++ enum variant: <span style='color: green;'>```B2 = 17```</span>
  B2 = 17,
  /// C++ enum variant: <span style='color: green;'>```B3 = 18```</span>
  B3 = 18,
  /// C++ enum variant: <span style='color: green;'>```B4 = 19```</span>
  B4 = 19,
  /// C++ enum variant: <span style='color: green;'>```B6 = 20```</span>
  B6 = 20,
  /// C++ enum variant: <span style='color: green;'>```B7 = 21```</span>
  B7 = 21,
  /// C++ enum variant: <span style='color: green;'>```B8 = 22```</span>
  B8 = 22,
  /// C++ enum variant: <span style='color: green;'>```B9 = 23```</span>
  B9 = 23,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```C5E = 24```</span>
  /// - <span style='color: green;'>```EnvelopeC5 = 24```</span>
  ///
  C5E = 24,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Comm10E = 25```</span>
  /// - <span style='color: green;'>```Envelope10 = 25```</span>
  ///
  Comm10E = 25,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```DLE = 26```</span>
  /// - <span style='color: green;'>```EnvelopeDL = 26```</span>
  ///
  DLE = 26,
  /// C++ enum variant: <span style='color: green;'>```Folio = 27```</span>
  Folio = 27,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Ledger = 28```</span>
  /// - <span style='color: green;'>```AnsiB = 28```</span>
  ///
  Ledger = 28,
  /// C++ enum variant: <span style='color: green;'>```Tabloid = 29```</span>
  Tabloid = 29,
  /// C++ enum variant: <span style='color: green;'>```Custom = 30```</span>
  Custom = 30,
  /// C++ enum variant: <span style='color: green;'>```A10 = 31```</span>
  A10 = 31,
  /// C++ enum variant: <span style='color: green;'>```A3Extra = 32```</span>
  A3Extra = 32,
  /// C++ enum variant: <span style='color: green;'>```A4Extra = 33```</span>
  A4Extra = 33,
  /// C++ enum variant: <span style='color: green;'>```A4Plus = 34```</span>
  A4Plus = 34,
  /// C++ enum variant: <span style='color: green;'>```A4Small = 35```</span>
  A4Small = 35,
  /// C++ enum variant: <span style='color: green;'>```A5Extra = 36```</span>
  A5Extra = 36,
  /// C++ enum variant: <span style='color: green;'>```B5Extra = 37```</span>
  B5Extra = 37,
  /// C++ enum variant: <span style='color: green;'>```JisB0 = 38```</span>
  JisB0 = 38,
  /// C++ enum variant: <span style='color: green;'>```JisB1 = 39```</span>
  JisB1 = 39,
  /// C++ enum variant: <span style='color: green;'>```JisB2 = 40```</span>
  JisB2 = 40,
  /// C++ enum variant: <span style='color: green;'>```JisB3 = 41```</span>
  JisB3 = 41,
  /// C++ enum variant: <span style='color: green;'>```JisB4 = 42```</span>
  JisB4 = 42,
  /// C++ enum variant: <span style='color: green;'>```JisB5 = 43```</span>
  JisB5 = 43,
  /// C++ enum variant: <span style='color: green;'>```JisB6 = 44```</span>
  JisB6 = 44,
  /// C++ enum variant: <span style='color: green;'>```JisB7 = 45```</span>
  JisB7 = 45,
  /// C++ enum variant: <span style='color: green;'>```JisB8 = 46```</span>
  JisB8 = 46,
  /// C++ enum variant: <span style='color: green;'>```JisB9 = 47```</span>
  JisB9 = 47,
  /// C++ enum variant: <span style='color: green;'>```JisB10 = 48```</span>
  JisB10 = 48,
  /// C++ enum variant: <span style='color: green;'>```AnsiC = 49```</span>
  AnsiC = 49,
  /// C++ enum variant: <span style='color: green;'>```AnsiD = 50```</span>
  AnsiD = 50,
  /// C++ enum variant: <span style='color: green;'>```AnsiE = 51```</span>
  AnsiE = 51,
  /// C++ enum variant: <span style='color: green;'>```LegalExtra = 52```</span>
  LegalExtra = 52,
  /// C++ enum variant: <span style='color: green;'>```LetterExtra = 53```</span>
  LetterExtra = 53,
  /// C++ enum variant: <span style='color: green;'>```LetterPlus = 54```</span>
  LetterPlus = 54,
  /// C++ enum variant: <span style='color: green;'>```LetterSmall = 55```</span>
  LetterSmall = 55,
  /// C++ enum variant: <span style='color: green;'>```TabloidExtra = 56```</span>
  TabloidExtra = 56,
  /// C++ enum variant: <span style='color: green;'>```ArchA = 57```</span>
  ArchA = 57,
  /// C++ enum variant: <span style='color: green;'>```ArchB = 58```</span>
  ArchB = 58,
  /// C++ enum variant: <span style='color: green;'>```ArchC = 59```</span>
  ArchC = 59,
  /// C++ enum variant: <span style='color: green;'>```ArchD = 60```</span>
  ArchD = 60,
  /// C++ enum variant: <span style='color: green;'>```ArchE = 61```</span>
  ArchE = 61,
  /// C++ enum variant: <span style='color: green;'>```Imperial7x9 = 62```</span>
  Imperial7X9 = 62,
  /// C++ enum variant: <span style='color: green;'>```Imperial8x10 = 63```</span>
  Imperial8X10 = 63,
  /// C++ enum variant: <span style='color: green;'>```Imperial9x11 = 64```</span>
  Imperial9X11 = 64,
  /// C++ enum variant: <span style='color: green;'>```Imperial9x12 = 65```</span>
  Imperial9X12 = 65,
  /// C++ enum variant: <span style='color: green;'>```Imperial10x11 = 66```</span>
  Imperial10X11 = 66,
  /// C++ enum variant: <span style='color: green;'>```Imperial10x13 = 67```</span>
  Imperial10X13 = 67,
  /// C++ enum variant: <span style='color: green;'>```Imperial10x14 = 68```</span>
  Imperial10X14 = 68,
  /// C++ enum variant: <span style='color: green;'>```Imperial12x11 = 69```</span>
  Imperial12X11 = 69,
  /// C++ enum variant: <span style='color: green;'>```Imperial15x11 = 70```</span>
  Imperial15X11 = 70,
  /// C++ enum variant: <span style='color: green;'>```ExecutiveStandard = 71```</span>
  ExecutiveStandard = 71,
  /// C++ enum variant: <span style='color: green;'>```Note = 72```</span>
  Note = 72,
  /// C++ enum variant: <span style='color: green;'>```Quarto = 73```</span>
  Quarto = 73,
  /// C++ enum variant: <span style='color: green;'>```Statement = 74```</span>
  Statement = 74,
  /// C++ enum variant: <span style='color: green;'>```SuperA = 75```</span>
  SuperA = 75,
  /// C++ enum variant: <span style='color: green;'>```SuperB = 76```</span>
  SuperB = 76,
  /// C++ enum variant: <span style='color: green;'>```Postcard = 77```</span>
  Postcard = 77,
  /// C++ enum variant: <span style='color: green;'>```DoublePostcard = 78```</span>
  DoublePostcard = 78,
  /// C++ enum variant: <span style='color: green;'>```Prc16K = 79```</span>
  Prc16K = 79,
  /// C++ enum variant: <span style='color: green;'>```Prc32K = 80```</span>
  Prc32K = 80,
  /// C++ enum variant: <span style='color: green;'>```Prc32KBig = 81```</span>
  Prc32KBig = 81,
  /// C++ enum variant: <span style='color: green;'>```FanFoldUS = 82```</span>
  FanFoldUS = 82,
  /// C++ enum variant: <span style='color: green;'>```FanFoldGerman = 83```</span>
  FanFoldGerman = 83,
  /// C++ enum variant: <span style='color: green;'>```FanFoldGermanLegal = 84```</span>
  FanFoldGermanLegal = 84,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeB4 = 85```</span>
  EnvelopeB4 = 85,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeB5 = 86```</span>
  EnvelopeB5 = 86,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeB6 = 87```</span>
  EnvelopeB6 = 87,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC0 = 88```</span>
  EnvelopeC0 = 88,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC1 = 89```</span>
  EnvelopeC1 = 89,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC2 = 90```</span>
  EnvelopeC2 = 90,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC3 = 91```</span>
  EnvelopeC3 = 91,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC4 = 92```</span>
  EnvelopeC4 = 92,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC6 = 93```</span>
  EnvelopeC6 = 93,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC65 = 94```</span>
  EnvelopeC65 = 94,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeC7 = 95```</span>
  EnvelopeC7 = 95,
  /// C++ enum variant: <span style='color: green;'>```Envelope9 = 96```</span>
  Envelope9 = 96,
  /// C++ enum variant: <span style='color: green;'>```Envelope11 = 97```</span>
  Envelope11 = 97,
  /// C++ enum variant: <span style='color: green;'>```Envelope12 = 98```</span>
  Envelope12 = 98,
  /// C++ enum variant: <span style='color: green;'>```Envelope14 = 99```</span>
  Envelope14 = 99,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeMonarch = 100```</span>
  EnvelopeMonarch = 100,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePersonal = 101```</span>
  EnvelopePersonal = 101,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeChou3 = 102```</span>
  EnvelopeChou3 = 102,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeChou4 = 103```</span>
  EnvelopeChou4 = 103,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeInvite = 104```</span>
  EnvelopeInvite = 104,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeItalian = 105```</span>
  EnvelopeItalian = 105,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeKaku2 = 106```</span>
  EnvelopeKaku2 = 106,
  /// C++ enum variant: <span style='color: green;'>```EnvelopeKaku3 = 107```</span>
  EnvelopeKaku3 = 107,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc1 = 108```</span>
  EnvelopePrc1 = 108,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc2 = 109```</span>
  EnvelopePrc2 = 109,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc3 = 110```</span>
  EnvelopePrc3 = 110,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc4 = 111```</span>
  EnvelopePrc4 = 111,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc5 = 112```</span>
  EnvelopePrc5 = 112,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc6 = 113```</span>
  EnvelopePrc6 = 113,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc7 = 114```</span>
  EnvelopePrc7 = 114,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc8 = 115```</span>
  EnvelopePrc8 = 115,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc9 = 116```</span>
  EnvelopePrc9 = 116,
  /// C++ enum variant: <span style='color: green;'>```EnvelopePrc10 = 117```</span>
  EnvelopePrc10 = 117,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```EnvelopeYou4 = 118```</span>
  /// - <span style='color: green;'>```LastPageSize = 118```</span>
  /// - <span style='color: green;'>```NPageSize = 118```</span>
  /// - <span style='color: green;'>```NPaperSize = 118```</span>
  ///
  EnvelopeYou4 = 118,
}

/// C++ type: <span style='color: green;'>```QPageSize::SizeMatchPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SizeMatchPolicy {
  /// C++ enum variant: <span style='color: green;'>```FuzzyMatch = 0```</span>
  Fuzzy = 0,
  /// C++ enum variant: <span style='color: green;'>```FuzzyOrientationMatch = 1```</span>
  FuzzyOrientation = 1,
  /// C++ enum variant: <span style='color: green;'>```ExactMatch = 2```</span>
  Exact = 2,
}

/// C++ type: <span style='color: green;'>```QPageSize::Unit```</span>
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

/// C++ method: <span style='color: green;'>```bool operator!=(const QPageSize& lhs, const QPageSize& rhs)```</span>
///
///
pub fn op_neq(lhs: &::page_size::PageSize, rhs: &::page_size::PageSize) -> bool {
  unsafe {
    ::ffi::qt_gui_c_QPageSize_G_operator_neq(lhs as *const ::page_size::PageSize,
                                             rhs as *const ::page_size::PageSize)
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QPageSize& pageSize)```</span>
///
///
pub fn op_shl(dbg: &::qt_core::debug::Debug, page_size: &::page_size::PageSize) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QPageSize_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                         page_size as *const ::page_size::PageSize,
                                                         &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QPageSize& value1, QPageSize& value2)```</span>
///
///
pub fn swap(value1: &mut ::page_size::PageSize, value2: &mut ::page_size::PageSize) {
  unsafe {
    ::ffi::qt_gui_c_QPageSize_G_swap(value1 as *mut ::page_size::PageSize,
                                     value2 as *mut ::page_size::PageSize)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PageSize::id_static](../struct.PageSize.html#method.id_static) method.
  pub trait PageSizeIdStaticArgs {
    fn exec(self) -> ::page_size::PageSizeId;
  }
  impl<'a> PageSizeIdStaticArgs for &'a ::qt_core::size::Size {
    fn exec(self) -> ::page_size::PageSizeId {
      let point_size = self;
      unsafe { ::ffi::qt_gui_c_QPageSize_id_pointSize(point_size as *const ::qt_core::size::Size) }
    }
  }
  impl<'a> PageSizeIdStaticArgs for (&'a ::qt_core::size::Size, ::page_size::SizeMatchPolicy) {
    fn exec(self) -> ::page_size::PageSizeId {
      let point_size = self.0;
      let match_policy = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPageSize_id_pointSize_matchPolicy(point_size as *const ::qt_core::size::Size, match_policy)
      }
    }
  }
  impl<'a> PageSizeIdStaticArgs for (&'a ::qt_core::size_f::SizeF, ::page_size::Unit) {
    fn exec(self) -> ::page_size::PageSizeId {
      let size = self.0;
      let units = self.1;
      unsafe { ::ffi::qt_gui_c_QPageSize_id_size_units(size as *const ::qt_core::size_f::SizeF, units) }
    }
  }
  impl<'a> PageSizeIdStaticArgs for (&'a ::qt_core::size_f::SizeF, ::page_size::Unit, ::page_size::SizeMatchPolicy) {
    fn exec(self) -> ::page_size::PageSizeId {
      let size = self.0;
      let units = self.1;
      let match_policy = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPageSize_id_size_units_matchPolicy(size as *const ::qt_core::size_f::SizeF,
                                                            units,
                                                            match_policy)
      }
    }
  }
  impl PageSizeIdStaticArgs for ::libc::c_int {
    fn exec(self) -> ::page_size::PageSizeId {
      let windows_id = self;
      unsafe { ::ffi::qt_gui_c_QPageSize_id_windowsId(windows_id) }
    }
  }
  /// This trait represents a set of arguments accepted by [PageSize::new](../struct.PageSize.html#method.new) method.
  pub trait PageSizeNewArgs {
    fn exec(self) -> ::page_size::PageSize;
  }
  impl PageSizeNewArgs for () {
    fn exec(self) -> ::page_size::PageSize {

      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs for &'a ::page_size::PageSize {
    fn exec(self) -> ::page_size::PageSize {
      let other = self;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_other(other as *const ::page_size::PageSize, &mut object);
        }
        object
      }
    }
  }
  impl PageSizeNewArgs for ::page_size::PageSizeId {
    fn exec(self) -> ::page_size::PageSize {
      let page_size_id = self;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_pageSizeId(page_size_id, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs for &'a ::qt_core::size::Size {
    fn exec(self) -> ::page_size::PageSize {
      let point_size = self;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_pointSize(point_size as *const ::qt_core::size::Size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs for (&'a ::qt_core::size::Size, &'a ::qt_core::string::String) {
    fn exec(self) -> ::page_size::PageSize {
      let point_size = self.0;
      let name = self.1;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_pointSize_name(point_size as *const ::qt_core::size::Size,
                                                               name as *const ::qt_core::string::String,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs for (&'a ::qt_core::size::Size, &'a ::qt_core::string::String, ::page_size::SizeMatchPolicy) {
    fn exec(self) -> ::page_size::PageSize {
      let point_size = self.0;
      let name = self.1;
      let match_policy = self.2;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_pointSize_name_matchPolicy(point_size as *const ::qt_core::size::Size, name as *const ::qt_core::string::String, match_policy, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs for (&'a ::qt_core::size_f::SizeF, ::page_size::Unit) {
    fn exec(self) -> ::page_size::PageSize {
      let size = self.0;
      let units = self.1;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_size_units(size as *const ::qt_core::size_f::SizeF, units, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs for (&'a ::qt_core::size_f::SizeF, ::page_size::Unit, &'a ::qt_core::string::String) {
    fn exec(self) -> ::page_size::PageSize {
      let size = self.0;
      let units = self.1;
      let name = self.2;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_size_units_name(size as *const ::qt_core::size_f::SizeF,
                                                                units,
                                                                name as *const ::qt_core::string::String,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'a> PageSizeNewArgs
    for (&'a ::qt_core::size_f::SizeF, ::page_size::Unit, &'a ::qt_core::string::String, ::page_size::SizeMatchPolicy) {
    fn exec(self) -> ::page_size::PageSize {
      let size = self.0;
      let units = self.1;
      let name = self.2;
      let match_policy = self.3;
      {
        let mut object: ::page_size::PageSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPageSize_constructor_size_units_name_matchPolicy(size as *const ::qt_core::size_f::SizeF,
                                                                            units,
                                                                            name as *const ::qt_core::string::String,
                                                                            match_policy,
                                                                            &mut object);
        }
        object
      }
    }
  }
}
