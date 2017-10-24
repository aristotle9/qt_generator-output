/// C++ type: <span style='color: green;'>```QStyleOptionSlider```</span>
#[repr(C)]
pub struct StyleOptionSlider(u8);

impl StyleOptionSlider {
  /// C++ method: <span style='color: green;'>```bool QStyleOptionSlider::dialWrapping() const```</span>
  ///
  ///
  pub fn dial_wrapping(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_dialWrapping(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_maximum(self as *const ::style_option_slider::StyleOptionSlider) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_minimum(self as *const ::style_option_slider::StyleOptionSlider) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionSlider::QStyleOptionSlider```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_slider::StyleOptionSlider>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionSlider::QStyleOptionSlider()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_slider::StyleOptionSlider) -> ::cpp_utils::CppBox<::style_option_slider::StyleOptionSlider>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_slider::StyleOptionSlider>
    where Args: overloading::StyleOptionSliderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```double QStyleOptionSlider::notchTarget() const```</span>
  ///
  ///
  pub fn notch_target(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_notchTarget(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt::Orientation& QStyleOptionSlider::orientation() const```</span>
  ///
  ///
  pub fn orientation<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::Orientation {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionSlider_orientation(self as *const ::style_option_slider::StyleOptionSlider)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::Orientation& QStyleOptionSlider::orientation_mut()```</span>
  ///
  ///
  pub fn orientation_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::Orientation {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionSlider_orientation_mut(self as *mut ::style_option_slider::StyleOptionSlider)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::pageStep() const```</span>
  ///
  ///
  pub fn page_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_pageStep(self as *const ::style_option_slider::StyleOptionSlider) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_dialWrapping(bool value)```</span>
  ///
  ///
  pub fn set_dial_wrapping(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_dialWrapping(self as *mut ::style_option_slider::StyleOptionSlider,
                                                              value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_maximum(int value)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_maximum(self as *mut ::style_option_slider::StyleOptionSlider, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_minimum(int value)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_minimum(self as *mut ::style_option_slider::StyleOptionSlider, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_notchTarget(double value)```</span>
  ///
  ///
  pub fn set_notch_target(&mut self, value: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_notchTarget(self as *mut ::style_option_slider::StyleOptionSlider,
                                                             value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_orientation(Qt::Orientation value)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, value: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_orientation(self as *mut ::style_option_slider::StyleOptionSlider,
                                                             value as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_pageStep(int value)```</span>
  ///
  ///
  pub fn set_page_step(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_pageStep(self as *mut ::style_option_slider::StyleOptionSlider, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_singleStep(int value)```</span>
  ///
  ///
  pub fn set_single_step(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_singleStep(self as *mut ::style_option_slider::StyleOptionSlider,
                                                            value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_sliderPosition(int value)```</span>
  ///
  ///
  pub fn set_slider_position(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_sliderPosition(self as *mut ::style_option_slider::StyleOptionSlider,
                                                                value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_sliderValue(int value)```</span>
  ///
  ///
  pub fn set_slider_value(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_sliderValue(self as *mut ::style_option_slider::StyleOptionSlider,
                                                             value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_tickInterval(int value)```</span>
  ///
  ///
  pub fn set_tick_interval(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_tickInterval(self as *mut ::style_option_slider::StyleOptionSlider,
                                                              value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_tickPosition(QSlider::TickPosition value)```</span>
  ///
  ///
  pub fn set_tick_position(&mut self, value: &::slider::TickPosition) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_tickPosition(self as *mut ::style_option_slider::StyleOptionSlider,
                                                              value as *const ::slider::TickPosition)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSlider::set_upsideDown(bool value)```</span>
  ///
  ///
  pub fn set_upside_down(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_set_upsideDown(self as *mut ::style_option_slider::StyleOptionSlider,
                                                            value)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::singleStep() const```</span>
  ///
  ///
  pub fn single_step(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_singleStep(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::sliderPosition() const```</span>
  ///
  ///
  pub fn slider_position(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_sliderPosition(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::sliderValue() const```</span>
  ///
  ///
  pub fn slider_value(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_sliderValue(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionSlider::tickInterval() const```</span>
  ///
  ///
  pub fn tick_interval(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_tickInterval(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }

  /// C++ method: <span style='color: green;'>```const QSlider::TickPosition& QStyleOptionSlider::tickPosition() const```</span>
  ///
  ///
  pub fn tick_position<'l0>(&'l0 self) -> &'l0 ::slider::TickPosition {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionSlider_tickPosition(self as *const ::style_option_slider::StyleOptionSlider)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSlider::TickPosition& QStyleOptionSlider::tickPosition_mut()```</span>
  ///
  ///
  pub fn tick_position_mut<'l0>(&'l0 mut self) -> &'l0 mut ::slider::TickPosition {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionSlider_tickPosition_mut(self as *mut ::style_option_slider::StyleOptionSlider)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionSlider::upsideDown() const```</span>
  ///
  ///
  pub fn upside_down(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSlider_upsideDown(self as *const ::style_option_slider::StyleOptionSlider)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_slider::StyleOptionSlider {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionSlider_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionSlider::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983041```</span>
  Type = 983041,
}

/// C++ type: <span style='color: green;'>```QStyleOptionSlider::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_slider::StyleOptionSlider {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_slider::StyleOptionSlider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOption_ptr(self as *const ::style_option_slider::StyleOptionSlider as *mut ::style_option_slider::StyleOptionSlider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_slider::StyleOptionSlider {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_slider::StyleOptionSlider) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_slider::StyleOptionSlider as *mut ::style_option_slider::StyleOptionSlider) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_slider::StyleOptionSlider> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_slider::StyleOptionSlider {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_slider::StyleOptionSlider {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_slider::StyleOptionSlider> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_slider::StyleOptionSlider {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_slider::StyleOptionSlider {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionSlider_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_slider::StyleOptionSlider {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_slider::StyleOptionSlider as *mut ::style_option_slider::StyleOptionSlider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_slider::StyleOptionSlider {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_slider::StyleOptionSlider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionSlider::new](../struct.StyleOptionSlider.html#method.new) method.
  pub trait StyleOptionSliderNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_slider::StyleOptionSlider>;
  }
  impl StyleOptionSliderNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_slider::StyleOptionSlider> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSlider_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionSliderNewArgs for &'a ::style_option_slider::StyleOptionSlider {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_slider::StyleOptionSlider> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionSlider_new_other(other as *const ::style_option_slider::StyleOptionSlider)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
