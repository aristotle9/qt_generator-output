/// C++ type: <span style='color: green;'>```QPagedPaintDevice::Margins```</span>
#[repr(C)]
pub struct Margins([u8; ::type_sizes::QT_GUI_PAGED_PAINT_DEVICE_MARGINS]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Margins {
  unsafe fn new_uninitialized() -> Margins {
    Margins(::std::mem::uninitialized())
  }
}

impl Margins {
  /// C++ method: <span style='color: green;'>```double QPagedPaintDevice::Margins::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_bottom(self as *const ::paged_paint_device::Margins) }
  }

  /// C++ method: <span style='color: green;'>```double QPagedPaintDevice::Margins::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_left(self as *const ::paged_paint_device::Margins) }
  }

  /// C++ method: <span style='color: green;'>```double QPagedPaintDevice::Margins::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_right(self as *const ::paged_paint_device::Margins) }
  }

  /// C++ method: <span style='color: green;'>```void QPagedPaintDevice::Margins::set_bottom(double value)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_set_bottom(self as *mut ::paged_paint_device::Margins, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPagedPaintDevice::Margins::set_left(double value)```</span>
  ///
  ///
  pub fn set_left(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_set_left(self as *mut ::paged_paint_device::Margins, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPagedPaintDevice::Margins::set_right(double value)```</span>
  ///
  ///
  pub fn set_right(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_set_right(self as *mut ::paged_paint_device::Margins, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPagedPaintDevice::Margins::set_top(double value)```</span>
  ///
  ///
  pub fn set_top(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_set_top(self as *mut ::paged_paint_device::Margins, value) }
  }

  /// C++ method: <span style='color: green;'>```double QPagedPaintDevice::Margins::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_top(self as *const ::paged_paint_device::Margins) }
  }
}

impl Drop for ::paged_paint_device::Margins {
  /// C++ method: <span style='color: green;'>```[destructor] void QPagedPaintDevice::Margins::~QPagedPaintDevice::Margins()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_Margins_destructor(self as *mut ::paged_paint_device::Margins) }
  }
}

/// C++ type: <span style='color: green;'>```QPagedPaintDevice::PageSize```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PageSize {
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

/// C++ type: <span style='color: green;'>```QPagedPaintDevice```</span>
#[repr(C)]
pub struct PagedPaintDevice(u8);

impl PagedPaintDevice {
  /// C++ method: <span style='color: green;'>```QPagedPaintDevice::Margins QPagedPaintDevice::margins() const```</span>
  ///
  ///
  pub fn margins(&self) -> ::paged_paint_device::Margins {
    {
      let mut object: ::paged_paint_device::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPagedPaintDevice_margins_to_output(self as *const ::paged_paint_device::PagedPaintDevice,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QPagedPaintDevice::newPage()```</span>
  ///
  ///
  pub fn new_page(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_newPage(self as *mut ::paged_paint_device::PagedPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```QPageLayout QPagedPaintDevice::pageLayout() const```</span>
  ///
  ///
  pub fn page_layout(&self) -> ::page_layout::PageLayout {
    {
      let mut object: ::page_layout::PageLayout =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPagedPaintDevice_pageLayout_to_output(self as *const ::paged_paint_device::PagedPaintDevice,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPagedPaintDevice::PageSize QPagedPaintDevice::pageSize() const```</span>
  ///
  ///
  pub fn page_size(&self) -> ::paged_paint_device::PageSize {
    unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_pageSize(self as *const ::paged_paint_device::PagedPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QPagedPaintDevice::pageSizeMM() const```</span>
  ///
  ///
  pub fn page_size_m_m(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPagedPaintDevice_pageSizeMM_to_output(self as *const ::paged_paint_device::PagedPaintDevice,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPagedPaintDevice::setMargins(const QPagedPaintDevice::Margins& margins)```</span>
  ///
  ///
  pub fn set_margins(&mut self, margins: &::paged_paint_device::Margins) {
    unsafe {
      ::ffi::qt_gui_c_QPagedPaintDevice_setMargins(self as *mut ::paged_paint_device::PagedPaintDevice,
                                                   margins as *const ::paged_paint_device::Margins)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPagedPaintDevice::setPageLayout(const QPageLayout& pageLayout)```</span>
  ///
  ///
  pub fn set_page_layout(&mut self, page_layout: &::page_layout::PageLayout) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPagedPaintDevice_setPageLayout(self as *mut ::paged_paint_device::PagedPaintDevice,
                                                      page_layout as *const ::page_layout::PageLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```QPagedPaintDevice::setPageMargins```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_page_margins(&mut self, &::qt_core::margins_f::MarginsF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPagedPaintDevice::setPageMargins(const QMarginsF& margins)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_page_margins(&mut self, (&::qt_core::margins_f::MarginsF, &::page_layout::Unit)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPagedPaintDevice::setPageMargins(const QMarginsF& margins, QPageLayout::Unit units)```</span>
  ///
  ///
  pub fn set_page_margins<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::PagedPaintDeviceSetPageMarginsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPagedPaintDevice::setPageOrientation(QPageLayout::Orientation orientation)```</span>
  ///
  ///
  pub fn set_page_orientation(&mut self, orientation: &::page_layout::Orientation) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPagedPaintDevice_setPageOrientation(self as *mut ::paged_paint_device::PagedPaintDevice,
                                                           orientation as *const ::page_layout::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```QPagedPaintDevice::setPageSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_page_size(&mut self, &::page_size::PageSize) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPagedPaintDevice::setPageSize(const QPageSize& pageSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_page_size(&mut self, ::paged_paint_device::PageSize) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPagedPaintDevice::setPageSize(QPagedPaintDevice::PageSize size)```</span>
  ///
  ///
  pub fn set_page_size<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::PagedPaintDeviceSetPageSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QPagedPaintDevice::setPageSizeMM(const QSizeF& size)```</span>
  ///
  ///
  pub fn set_page_size_m_m(&mut self, size: &::qt_core::size_f::SizeF) {
    unsafe {
      ::ffi::qt_gui_c_QPagedPaintDevice_setPageSizeMM(self as *mut ::paged_paint_device::PagedPaintDevice,
                                                      size as *const ::qt_core::size_f::SizeF)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::paged_paint_device::PagedPaintDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPagedPaintDevice_delete
  }
}

impl ::cpp_utils::DynamicCast<::paged_paint_device::PagedPaintDevice> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::paged_paint_device::PagedPaintDevice> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QPagedPaintDevice_G_dynamic_cast_QPagedPaintDevice_ptr(self as *mut ::paint_device::PaintDevice)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::paged_paint_device::PagedPaintDevice> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_G_dynamic_cast_QPagedPaintDevice_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::paged_paint_device::PagedPaintDevice {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_G_static_cast_QPaintDevice_ptr(self as *mut ::paged_paint_device::PagedPaintDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_G_static_cast_QPaintDevice_ptr(self as *const ::paged_paint_device::PagedPaintDevice as *mut ::paged_paint_device::PagedPaintDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::paged_paint_device::PagedPaintDevice> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::paged_paint_device::PagedPaintDevice {
    let ffi_result =
      ::ffi::qt_gui_c_QPagedPaintDevice_G_static_cast_QPagedPaintDevice_ptr(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::paged_paint_device::PagedPaintDevice {
    let ffi_result = ::ffi::qt_gui_c_QPagedPaintDevice_G_static_cast_QPagedPaintDevice_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::paged_paint_device::PagedPaintDevice {
  type Target = ::paint_device::PaintDevice;
  fn deref(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_G_static_cast_QPaintDevice_ptr(self as *const ::paged_paint_device::PagedPaintDevice as *mut ::paged_paint_device::PagedPaintDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::paged_paint_device::PagedPaintDevice {
  fn deref_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_G_static_cast_QPaintDevice_ptr(self as *mut ::paged_paint_device::PagedPaintDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PagedPaintDevice::set_page_margins](../struct.PagedPaintDevice.html#method.set_page_margins) method.
  pub trait PagedPaintDeviceSetPageMarginsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::paged_paint_device::PagedPaintDevice) -> bool;
  }
  impl<'largs> PagedPaintDeviceSetPageMarginsArgs<'largs> for &'largs ::qt_core::margins_f::MarginsF {
    fn exec(self, original_self: &'largs mut ::paged_paint_device::PagedPaintDevice) -> bool {
      let margins = self;
      unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_setPageMargins_margins(original_self as *mut ::paged_paint_device::PagedPaintDevice, margins as *const ::qt_core::margins_f::MarginsF) }
    }
  }
  impl<'largs> PagedPaintDeviceSetPageMarginsArgs<'largs>
    for (&'largs ::qt_core::margins_f::MarginsF, &'largs ::page_layout::Unit) {
    fn exec(self, original_self: &'largs mut ::paged_paint_device::PagedPaintDevice) -> bool {
      let margins = self.0;
      let units = self.1;
      unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_setPageMargins_margins_units(original_self as *mut ::paged_paint_device::PagedPaintDevice, margins as *const ::qt_core::margins_f::MarginsF, units as *const ::page_layout::Unit) }
    }
  }
  /// This trait represents a set of arguments accepted by [PagedPaintDevice::set_page_size](../struct.PagedPaintDevice.html#method.set_page_size) method.
  pub trait PagedPaintDeviceSetPageSizeArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::paged_paint_device::PagedPaintDevice) -> Self::ReturnType;
  }
  impl<'largs> PagedPaintDeviceSetPageSizeArgs<'largs> for &'largs ::page_size::PageSize {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::paged_paint_device::PagedPaintDevice) -> bool {
      let page_size = self;
      unsafe { ::ffi::qt_gui_c_QPagedPaintDevice_setPageSize_pageSize(original_self as *mut ::paged_paint_device::PagedPaintDevice, page_size as *const ::page_size::PageSize) }
    }
  }
  impl<'largs> PagedPaintDeviceSetPageSizeArgs<'largs> for ::paged_paint_device::PageSize {
    type ReturnType = ();
    fn exec(self, original_self: &'largs mut ::paged_paint_device::PagedPaintDevice) -> () {
      let size = self;
      unsafe {
        ::ffi::qt_gui_c_QPagedPaintDevice_setPageSize_size(original_self as *mut ::paged_paint_device::PagedPaintDevice, size)
      }
    }
  }
}
