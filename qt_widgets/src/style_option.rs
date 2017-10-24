/// C++ type: <span style='color: green;'>```QStyleOption::OptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OptionType {
  /// C++ enum variant: <span style='color: green;'>```SO_Default = 0```</span>
  Default = 0,
  /// C++ enum variant: <span style='color: green;'>```SO_FocusRect = 1```</span>
  FocusRect = 1,
  /// C++ enum variant: <span style='color: green;'>```SO_Button = 2```</span>
  Button = 2,
  /// C++ enum variant: <span style='color: green;'>```SO_Tab = 3```</span>
  Tab = 3,
  /// C++ enum variant: <span style='color: green;'>```SO_MenuItem = 4```</span>
  MenuItem = 4,
  /// C++ enum variant: <span style='color: green;'>```SO_Frame = 5```</span>
  Frame = 5,
  /// C++ enum variant: <span style='color: green;'>```SO_ProgressBar = 6```</span>
  ProgressBar = 6,
  /// C++ enum variant: <span style='color: green;'>```SO_ToolBox = 7```</span>
  ToolBox = 7,
  /// C++ enum variant: <span style='color: green;'>```SO_Header = 8```</span>
  Header = 8,
  /// C++ enum variant: <span style='color: green;'>```SO_DockWidget = 9```</span>
  DockWidget = 9,
  /// C++ enum variant: <span style='color: green;'>```SO_ViewItem = 10```</span>
  ViewItem = 10,
  /// C++ enum variant: <span style='color: green;'>```SO_TabWidgetFrame = 11```</span>
  TabWidgetFrame = 11,
  /// C++ enum variant: <span style='color: green;'>```SO_TabBarBase = 12```</span>
  TabBarBase = 12,
  /// C++ enum variant: <span style='color: green;'>```SO_RubberBand = 13```</span>
  RubberBand = 13,
  /// C++ enum variant: <span style='color: green;'>```SO_ToolBar = 14```</span>
  ToolBar = 14,
  /// C++ enum variant: <span style='color: green;'>```SO_GraphicsItem = 15```</span>
  GraphicsItem = 15,
  /// C++ enum variant: <span style='color: green;'>```SO_CustomBase = 3840```</span>
  CustomBase = 3840,
  /// C++ enum variant: <span style='color: green;'>```SO_Complex = 983040```</span>
  Complex = 983040,
  /// C++ enum variant: <span style='color: green;'>```SO_Slider = 983041```</span>
  Slider = 983041,
  /// C++ enum variant: <span style='color: green;'>```SO_SpinBox = 983042```</span>
  SpinBox = 983042,
  /// C++ enum variant: <span style='color: green;'>```SO_ToolButton = 983043```</span>
  ToolButton = 983043,
  /// C++ enum variant: <span style='color: green;'>```SO_ComboBox = 983044```</span>
  ComboBox = 983044,
  /// C++ enum variant: <span style='color: green;'>```SO_TitleBar = 983045```</span>
  TitleBar = 983045,
  /// C++ enum variant: <span style='color: green;'>```SO_GroupBox = 983046```</span>
  GroupBox = 983046,
  /// C++ enum variant: <span style='color: green;'>```SO_SizeGrip = 983047```</span>
  SizeGrip = 983047,
  /// C++ enum variant: <span style='color: green;'>```SO_ComplexCustomBase = 251658240```</span>
  ComplexCustomBase = 251658240,
}

/// C++ type: <span style='color: green;'>```QStyleOption```</span>
#[repr(C)]
pub struct StyleOption(u8);

impl StyleOption {
  /// C++ method: <span style='color: green;'>```const Qt::LayoutDirection& QStyleOption::direction() const```</span>
  ///
  ///
  pub fn direction<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::LayoutDirection {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_direction(self as *const ::style_option::StyleOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::LayoutDirection& QStyleOption::direction_mut()```</span>
  ///
  ///
  pub fn direction_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::LayoutDirection {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOption_direction_mut(self as *mut ::style_option::StyleOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFontMetrics& QStyleOption::fontMetrics() const```</span>
  ///
  ///
  pub fn font_metrics<'l0>(&'l0 self) -> &'l0 ::qt_gui::font_metrics::FontMetrics {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOption_fontMetrics(self as *const ::style_option::StyleOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics& QStyleOption::fontMetrics_mut()```</span>
  ///
  ///
  pub fn font_metrics_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::font_metrics::FontMetrics {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOption_fontMetrics_mut(self as *mut ::style_option::StyleOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::init(const QWidget* w)```</span>
  ///
  ///
  pub unsafe fn init(&mut self, w: *const ::widget::Widget) {
    ::ffi::qt_widgets_c_QStyleOption_init(self as *mut ::style_option::StyleOption, w)
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::initFrom(const QWidget* w)```</span>
  ///
  ///
  pub unsafe fn init_from(&mut self, w: *const ::widget::Widget) {
    ::ffi::qt_widgets_c_QStyleOption_initFrom(self as *mut ::style_option::StyleOption, w)
  }

  /// C++ method: <span style='color: green;'>```QStyleOption::QStyleOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option::StyleOption>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOption::QStyleOption()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option::StyleOption) -> ::cpp_utils::CppBox<::style_option::StyleOption>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOption::QStyleOption(const QStyleOption& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::style_option::StyleOption>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOption::QStyleOption(int version = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::style_option::StyleOption>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOption::QStyleOption(int version = ?, int type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option::StyleOption>
    where Args: overloading::StyleOptionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStyleOption& QStyleOption::operator=(const QStyleOption& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::style_option::StyleOption)
                             -> &'l0 mut ::style_option::StyleOption {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QStyleOption_operator_assign(self as *mut ::style_option::StyleOption,
                                                       other as *const ::style_option::StyleOption)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPalette& QStyleOption::palette() const```</span>
  ///
  ///
  pub fn palette<'l0>(&'l0 self) -> &'l0 ::qt_gui::palette::Palette {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_palette(self as *const ::style_option::StyleOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPalette& QStyleOption::palette_mut()```</span>
  ///
  ///
  pub fn palette_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::palette::Palette {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_palette_mut(self as *mut ::style_option::StyleOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QStyleOption::rect() const```</span>
  ///
  ///
  pub fn rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_rect(self as *const ::style_option::StyleOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOption::rect_mut()```</span>
  ///
  ///
  pub fn rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_rect_mut(self as *mut ::style_option::StyleOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_direction(Qt::LayoutDirection value)```</span>
  ///
  ///
  pub fn set_direction(&mut self, value: &::qt_core::qt::LayoutDirection) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOption_set_direction(self as *mut ::style_option::StyleOption,
                                                     value as *const ::qt_core::qt::LayoutDirection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_fontMetrics(QFontMetrics value)```</span>
  ///
  ///
  pub fn set_font_metrics(&mut self, value: &::qt_gui::font_metrics::FontMetrics) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOption_set_fontMetrics(self as *mut ::style_option::StyleOption,
                                                       value as *const ::qt_gui::font_metrics::FontMetrics)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_palette(QPalette value)```</span>
  ///
  ///
  pub fn set_palette(&mut self, value: &::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOption_set_palette(self as *mut ::style_option::StyleOption,
                                                   value as *const ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_rect(QRect value)```</span>
  ///
  ///
  pub fn set_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOption_set_rect(self as *mut ::style_option::StyleOption,
                                                value as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_styleObject(QObject* value)```</span>
  ///
  ///
  pub unsafe fn set_style_object(&mut self, value: *mut ::qt_core::object::Object) {
    ::ffi::qt_widgets_c_QStyleOption_set_styleObject(self as *mut ::style_option::StyleOption, value)
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_type(int value)```</span>
  ///
  ///
  pub fn set_type(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOption_set_type(self as *mut ::style_option::StyleOption, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOption::set_version(int value)```</span>
  ///
  ///
  pub fn set_version(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOption_set_version(self as *mut ::style_option::StyleOption, value) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QStyleOption::styleObject() const```</span>
  ///
  ///
  pub fn style_object(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_widgets_c_QStyleOption_styleObject(self as *const ::style_option::StyleOption) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOption::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOption_type(self as *const ::style_option::StyleOption) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOption::version() const```</span>
  ///
  ///
  pub fn version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOption_version(self as *const ::style_option::StyleOption) }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option::StyleOption {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOption_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOption::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// C++ enum variant: <span style='color: green;'>```Type = 0```</span>
  Type = 0,
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 1,
}

/// C++ type: <span style='color: green;'>```QStyleOption::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::style_option::StyleOption)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QStyleOption& option)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::style_option::OptionType)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QStyleOption::OptionType& optionType)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> ::qt_core::debug::Debug
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOption::new](../struct.StyleOption.html#method.new) method.
  pub trait StyleOptionNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option::StyleOption>;
  }
  impl StyleOptionNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option::StyleOption> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionNewArgs for &'a ::style_option::StyleOption {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option::StyleOption> {
      let other = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QStyleOption_new_other(other as *const ::style_option::StyleOption) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StyleOptionNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option::StyleOption> {
      let version = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_new_version(version) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StyleOptionNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option::StyleOption> {
      let version = self.0;
      let type_ = self.1;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOption_new_version_type(version, type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    fn exec(self) -> ::qt_core::debug::Debug;
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::style_option::StyleOption) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let option = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOption_G_operator_shl_to_output_debug_option(debug as *const ::qt_core::debug::Debug, option as *const ::style_option::StyleOption, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::style_option::OptionType) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let option_type = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOption_G_operator_shl_to_output_debug_optionType(debug as *const ::qt_core::debug::Debug, option_type as *const ::style_option::OptionType, &mut object);
        }
        object
      }
    }
  }
}
