/// C++ type: <span style='color: green;'>```QStyleOptionHeader::SectionPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SectionPosition {
  /// C++ enum variant: <span style='color: green;'>```Beginning = 0```</span>
  Beginning = 0,
  /// C++ enum variant: <span style='color: green;'>```Middle = 1```</span>
  Middle = 1,
  /// C++ enum variant: <span style='color: green;'>```End = 2```</span>
  End = 2,
  /// C++ enum variant: <span style='color: green;'>```OnlyOneSection = 3```</span>
  OnlyOneSection = 3,
}

/// C++ type: <span style='color: green;'>```QStyleOptionHeader::SelectedPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectedPosition {
  /// C++ enum variant: <span style='color: green;'>```NotAdjacent = 0```</span>
  NotAdjacent = 0,
  /// C++ enum variant: <span style='color: green;'>```NextIsSelected = 1```</span>
  NextIsSelected = 1,
  /// C++ enum variant: <span style='color: green;'>```PreviousIsSelected = 2```</span>
  PreviousIsSelected = 2,
  /// C++ enum variant: <span style='color: green;'>```NextAndPreviousAreSelected = 3```</span>
  NextAndPreviousAreSelected = 3,
}

/// C++ type: <span style='color: green;'>```QStyleOptionHeader::SortIndicator```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SortIndicator {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```SortUp = 1```</span>
  SortUp = 1,
  /// C++ enum variant: <span style='color: green;'>```SortDown = 2```</span>
  SortDown = 2,
}

/// C++ type: <span style='color: green;'>```QStyleOptionHeader```</span>
#[repr(C)]
pub struct StyleOptionHeader(u8);

impl StyleOptionHeader {
  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionHeader::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_icon(self as *const ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionHeader::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_icon_mut(self as *mut ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionHeader::QStyleOptionHeader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_header::StyleOptionHeader>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionHeader::QStyleOptionHeader()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_header::StyleOptionHeader) -> ::cpp_utils::CppBox<::style_option_header::StyleOptionHeader>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_header::StyleOptionHeader>
    where Args: overloading::StyleOptionHeaderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const Qt::Orientation& QStyleOptionHeader::orientation() const```</span>
  ///
  ///
  pub fn orientation<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::Orientation {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionHeader_orientation(self as *const ::style_option_header::StyleOptionHeader)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::Orientation& QStyleOptionHeader::orientation_mut()```</span>
  ///
  ///
  pub fn orientation_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::Orientation {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionHeader_orientation_mut(self as *mut ::style_option_header::StyleOptionHeader)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionHeader::SectionPosition QStyleOptionHeader::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::style_option_header::SectionPosition {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_position(self as *const ::style_option_header::StyleOptionHeader) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionHeader::section() const```</span>
  ///
  ///
  pub fn section(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_section(self as *const ::style_option_header::StyleOptionHeader) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionHeader::SelectedPosition QStyleOptionHeader::selectedPosition() const```</span>
  ///
  ///
  pub fn selected_position(&self) -> ::style_option_header::SelectedPosition {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_selectedPosition(self as *const ::style_option_header::StyleOptionHeader)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_icon(self as *mut ::style_option_header::StyleOptionHeader,
                                                      value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_orientation(Qt::Orientation value)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, value: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_orientation(self as *mut ::style_option_header::StyleOptionHeader,
                                                             value as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_position(QStyleOptionHeader::SectionPosition value)```</span>
  ///
  ///
  pub fn set_position(&mut self, value: ::style_option_header::SectionPosition) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_position(self as *mut ::style_option_header::StyleOptionHeader, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_section(int value)```</span>
  ///
  ///
  pub fn set_section(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_section(self as *mut ::style_option_header::StyleOptionHeader, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_selectedPosition(QStyleOptionHeader::SelectedPosition value)```</span>
  ///
  ///
  pub fn set_selected_position(&mut self, value: ::style_option_header::SelectedPosition) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_selectedPosition(self as *mut ::style_option_header::StyleOptionHeader, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_sortIndicator(QStyleOptionHeader::SortIndicator value)```</span>
  ///
  ///
  pub fn set_sort_indicator(&mut self, value: ::style_option_header::SortIndicator) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_sortIndicator(self as *mut ::style_option_header::StyleOptionHeader,
                                                               value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionHeader::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_set_text(self as *mut ::style_option_header::StyleOptionHeader,
                                                      value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionHeader::SortIndicator QStyleOptionHeader::sortIndicator() const```</span>
  ///
  ///
  pub fn sort_indicator(&self) -> ::style_option_header::SortIndicator {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionHeader_sortIndicator(self as *const ::style_option_header::StyleOptionHeader)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionHeader::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_text(self as *const ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionHeader::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_text_mut(self as *mut ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_header::StyleOptionHeader {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionHeader_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionHeader::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 8```</span>
  Type = 8,
}

/// C++ type: <span style='color: green;'>```QStyleOptionHeader::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_header::StyleOptionHeader {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOption_ptr(self as *const ::style_option_header::StyleOptionHeader as *mut ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_header::StyleOptionHeader> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_header::StyleOptionHeader {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOptionHeader_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_header::StyleOptionHeader {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOptionHeader_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_header::StyleOptionHeader {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOption_ptr(self as *const ::style_option_header::StyleOptionHeader as *mut ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_header::StyleOptionHeader {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_header::StyleOptionHeader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionHeader::new](../struct.StyleOptionHeader.html#method.new) method.
  pub trait StyleOptionHeaderNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_header::StyleOptionHeader>;
  }
  impl StyleOptionHeaderNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_header::StyleOptionHeader> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionHeader_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionHeaderNewArgs for &'a ::style_option_header::StyleOptionHeader {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_header::StyleOptionHeader> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionHeader_new_other(other as *const ::style_option_header::StyleOptionHeader)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
