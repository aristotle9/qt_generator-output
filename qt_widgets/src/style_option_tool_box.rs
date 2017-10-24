/// C++ type: <span style='color: green;'>```QStyleOptionToolBox::SelectedPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectedPosition {
  /// C++ enum variant: <span style='color: green;'>```NotAdjacent = 0```</span>
  NotAdjacent = 0,
  /// C++ enum variant: <span style='color: green;'>```NextIsSelected = 1```</span>
  NextIsSelected = 1,
  /// C++ enum variant: <span style='color: green;'>```PreviousIsSelected = 2```</span>
  PreviousIsSelected = 2,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBox```</span>
#[repr(C)]
pub struct StyleOptionToolBox([u8; ::type_sizes::QT_WIDGETS_STYLE_OPTION_TOOL_BOX_STYLE_OPTION_TOOL_BOX]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StyleOptionToolBox {
  unsafe fn new_uninitialized() -> StyleOptionToolBox {
    StyleOptionToolBox(::std::mem::uninitialized())
  }
}

impl StyleOptionToolBox {
  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionToolBox::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolBox_icon(self as *const ::style_option_tool_box::StyleOptionToolBox)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionToolBox::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolBox_icon_mut(self as *mut ::style_option_tool_box::StyleOptionToolBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionToolBox::QStyleOptionToolBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::style_option_tool_box::StyleOptionToolBox```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionToolBox::QStyleOptionToolBox()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_tool_box::StyleOptionToolBox) -> ::style_option_tool_box::StyleOptionToolBox```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::style_option_tool_box::StyleOptionToolBox
    where Args: overloading::StyleOptionToolBoxNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStyleOptionToolBox::TabPosition QStyleOptionToolBox::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::style_option_tool_box::TabPosition {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBox_position(self as *const ::style_option_tool_box::StyleOptionToolBox)
    }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionToolBox::SelectedPosition QStyleOptionToolBox::selectedPosition() const```</span>
  ///
  ///
  pub fn selected_position(&self) -> ::style_option_tool_box::SelectedPosition {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBox_selectedPosition(self as *const ::style_option_tool_box::StyleOptionToolBox) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBox::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBox_set_icon(self as *mut ::style_option_tool_box::StyleOptionToolBox,
                                                       value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBox::set_position(QStyleOptionToolBox::TabPosition value)```</span>
  ///
  ///
  pub fn set_position(&mut self, value: ::style_option_tool_box::TabPosition) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBox_set_position(self as *mut ::style_option_tool_box::StyleOptionToolBox,
                                                           value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBox::set_selectedPosition(QStyleOptionToolBox::SelectedPosition value)```</span>
  ///
  ///
  pub fn set_selected_position(&mut self, value: ::style_option_tool_box::SelectedPosition) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBox_set_selectedPosition(self as *mut ::style_option_tool_box::StyleOptionToolBox, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBox::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBox_set_text(self as *mut ::style_option_tool_box::StyleOptionToolBox,
                                                       value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionToolBox::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolBox_text(self as *const ::style_option_tool_box::StyleOptionToolBox)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionToolBox::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolBox_text_mut(self as *mut ::style_option_tool_box::StyleOptionToolBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::style_option_tool_box::StyleOptionToolBox {
  /// C++ method: <span style='color: green;'>```[destructor] void QStyleOptionToolBox::~QStyleOptionToolBox()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBox_destructor(self as *mut ::style_option_tool_box::StyleOptionToolBox)
    }
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBox::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 7```</span>
  Type = 7,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBox::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 2```</span>
  Version = 2,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBox::TabPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabPosition {
  /// C++ enum variant: <span style='color: green;'>```Beginning = 0```</span>
  Beginning = 0,
  /// C++ enum variant: <span style='color: green;'>```Middle = 1```</span>
  Middle = 1,
  /// C++ enum variant: <span style='color: green;'>```End = 2```</span>
  End = 2,
  /// C++ enum variant: <span style='color: green;'>```OnlyOneTab = 3```</span>
  OnlyOneTab = 3,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_tool_box::StyleOptionToolBox {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tool_box::StyleOptionToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tool_box::StyleOptionToolBox as *mut ::style_option_tool_box::StyleOptionToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tool_box::StyleOptionToolBox> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tool_box::StyleOptionToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOptionToolBox_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_tool_box::StyleOptionToolBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOptionToolBox_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_tool_box::StyleOptionToolBox {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tool_box::StyleOptionToolBox as *mut ::style_option_tool_box::StyleOptionToolBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_tool_box::StyleOptionToolBox {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBox_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tool_box::StyleOptionToolBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionToolBox::new](../struct.StyleOptionToolBox.html#method.new) method.
  pub trait StyleOptionToolBoxNewArgs {
    fn exec(self) -> ::style_option_tool_box::StyleOptionToolBox;
  }
  impl StyleOptionToolBoxNewArgs for () {
    fn exec(self) -> ::style_option_tool_box::StyleOptionToolBox {

      {
        let mut object: ::style_option_tool_box::StyleOptionToolBox =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionToolBox_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StyleOptionToolBoxNewArgs for &'a ::style_option_tool_box::StyleOptionToolBox {
    fn exec(self) -> ::style_option_tool_box::StyleOptionToolBox {
      let other = self;
      {
        let mut object: ::style_option_tool_box::StyleOptionToolBox =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionToolBox_constructor_other(other as *const ::style_option_tool_box::StyleOptionToolBox, &mut object);
        }
        object
      }
    }
  }
}
