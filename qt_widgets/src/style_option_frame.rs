/// C++ type: <span style='color: green;'>```QStyleOptionFrame::FrameFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FrameFeature {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Flat = 1```</span>
  Flat = 1,
  /// C++ enum variant: <span style='color: green;'>```Rounded = 2```</span>
  Rounded = 2,
}

impl ::qt_core::flags::FlaggableEnum for FrameFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "FrameFeature"
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionFrame```</span>
#[repr(C)]
pub struct StyleOptionFrame(u8);

impl StyleOptionFrame {
  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionFrame::FrameFeature>& QStyleOptionFrame::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::style_option_frame::FrameFeature> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_features(self as *const ::style_option_frame::StyleOptionFrame) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```const QFrame::Shape& QStyleOptionFrame::frameShape() const```</span>
  ///
  ///
  pub fn frame_shape<'l0>(&'l0 self) -> &'l0 ::frame::Shape {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionFrame_frameShape(self as *const ::style_option_frame::StyleOptionFrame)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFrame::Shape& QStyleOptionFrame::frameShape_mut()```</span>
  ///
  ///
  pub fn frame_shape_mut<'l0>(&'l0 mut self) -> &'l0 mut ::frame::Shape {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionFrame_frameShape_mut(self as *mut ::style_option_frame::StyleOptionFrame)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionFrame::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_lineWidth(self as *const ::style_option_frame::StyleOptionFrame) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionFrame::midLineWidth() const```</span>
  ///
  ///
  pub fn mid_line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_midLineWidth(self as *const ::style_option_frame::StyleOptionFrame) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionFrame::QStyleOptionFrame```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_frame::StyleOptionFrame>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionFrame::QStyleOptionFrame()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_frame::StyleOptionFrame) -> ::cpp_utils::CppBox<::style_option_frame::StyleOptionFrame>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_frame::StyleOptionFrame>
    where Args: overloading::StyleOptionFrameNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionFrame::set_features(QFlags<QStyleOptionFrame::FrameFeature> value)```</span>
  ///
  ///
  pub fn set_features(&mut self, value: ::qt_core::flags::Flags<::style_option_frame::FrameFeature>) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionFrame_set_features(self as *mut ::style_option_frame::StyleOptionFrame,
                                                         value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionFrame::set_frameShape(QFrame::Shape value)```</span>
  ///
  ///
  pub fn set_frame_shape(&mut self, value: &::frame::Shape) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionFrame_set_frameShape(self as *mut ::style_option_frame::StyleOptionFrame,
                                                           value as *const ::frame::Shape)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionFrame::set_lineWidth(int value)```</span>
  ///
  ///
  pub fn set_line_width(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionFrame_set_lineWidth(self as *mut ::style_option_frame::StyleOptionFrame, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionFrame::set_midLineWidth(int value)```</span>
  ///
  ///
  pub fn set_mid_line_width(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionFrame_set_midLineWidth(self as *mut ::style_option_frame::StyleOptionFrame, value)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_frame::StyleOptionFrame {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionFrame_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionFrame::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 5```</span>
  Type = 5,
}

/// C++ type: <span style='color: green;'>```QStyleOptionFrame::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 3```</span>
  Version = 3,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_frame::StyleOptionFrame {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_frame::StyleOptionFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOption_ptr(self as *const ::style_option_frame::StyleOptionFrame as *mut ::style_option_frame::StyleOptionFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_frame::StyleOptionFrame> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_frame::StyleOptionFrame {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOptionFrame_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_frame::StyleOptionFrame {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOptionFrame_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_frame::StyleOptionFrame {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOption_ptr(self as *const ::style_option_frame::StyleOptionFrame as *mut ::style_option_frame::StyleOptionFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_frame::StyleOptionFrame {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_frame::StyleOptionFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionFrame::new](../struct.StyleOptionFrame.html#method.new) method.
  pub trait StyleOptionFrameNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_frame::StyleOptionFrame>;
  }
  impl StyleOptionFrameNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_frame::StyleOptionFrame> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFrame_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionFrameNewArgs for &'a ::style_option_frame::StyleOptionFrame {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_frame::StyleOptionFrame> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionFrame_new_other(other as *const ::style_option_frame::StyleOptionFrame)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
