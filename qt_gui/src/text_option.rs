/// C++ type: <span style='color: green;'>```QTextOption::Flag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Flag {
  /// C++ enum variant: <span style='color: green;'>```IncludeTrailingSpaces = -2147483648```</span>
  IncludeTrailingSpaces = -2147483648,
  /// C++ enum variant: <span style='color: green;'>```ShowTabsAndSpaces = 1```</span>
  ShowTabsAndSpaces = 1,
  /// C++ enum variant: <span style='color: green;'>```ShowLineAndParagraphSeparators = 2```</span>
  ShowLineAndParagraphSeparators = 2,
  /// C++ enum variant: <span style='color: green;'>```AddSpaceForLineAndParagraphSeparators = 4```</span>
  AddSpaceForLineAndParagraphSeparators = 4,
  /// C++ enum variant: <span style='color: green;'>```SuppressColors = 8```</span>
  SuppressColors = 8,
  /// C++ enum variant: <span style='color: green;'>```ShowDocumentTerminator = 16```</span>
  ShowDocumentTerminator = 16,
}

impl ::qt_core::flags::FlaggableEnum for Flag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Flag"
  }
}

/// C++ type: <span style='color: green;'>```QTextOption::Tab```</span>
#[repr(C)]
pub struct Tab([u8; ::type_sizes::QT_GUI_TEXT_OPTION_TAB]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Tab {
  unsafe fn new_uninitialized() -> Tab {
    Tab(::std::mem::uninitialized())
  }
}

impl Tab {
  /// C++ method: <span style='color: green;'>```const QChar& QTextOption::Tab::delimiter() const```</span>
  ///
  ///
  pub fn delimiter<'l0>(&'l0 self) -> &'l0 ::qt_core::char::Char {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextOption_Tab_delimiter(self as *const ::text_option::Tab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QChar& QTextOption::Tab::delimiter_mut()```</span>
  ///
  ///
  pub fn delimiter_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::char::Char {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextOption_Tab_delimiter_mut(self as *mut ::text_option::Tab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab::Tab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_option::Tab```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextOption::Tab::Tab()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::text_option::TabType)) -> ::text_option::Tab```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextOption::Tab::Tab(double pos, QTextOption::TabType tabType)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::text_option::TabType, &::qt_core::char::Char)) -> ::text_option::Tab```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextOption::Tab::Tab(double pos, QTextOption::TabType tabType, QChar delim = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_option::Tab
    where Args: overloading::TabNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QTextOption::Tab::operator==(const QTextOption::Tab& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::text_option::Tab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_Tab_operator_eq(self as *const ::text_option::Tab,
                                                  other as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextOption::Tab::operator!=(const QTextOption::Tab& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::text_option::Tab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_Tab_operator_neq(self as *const ::text_option::Tab,
                                                   other as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextOption::Tab::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextOption_Tab_position(self as *const ::text_option::Tab) }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::Tab::set_delimiter(QChar value)```</span>
  ///
  ///
  pub fn set_delimiter(&mut self, value: &::qt_core::char::Char) {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_Tab_set_delimiter(self as *mut ::text_option::Tab,
                                                    value as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::Tab::set_position(double value)```</span>
  ///
  ///
  pub fn set_position(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextOption_Tab_set_position(self as *mut ::text_option::Tab, value) }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::Tab::set_type(QTextOption::TabType value)```</span>
  ///
  ///
  pub fn set_type(&mut self, value: ::text_option::TabType) {
    unsafe { ::ffi::qt_gui_c_QTextOption_Tab_set_type(self as *mut ::text_option::Tab, value) }
  }

  /// C++ method: <span style='color: green;'>```QTextOption::TabType QTextOption::Tab::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::text_option::TabType {
    unsafe { ::ffi::qt_gui_c_QTextOption_Tab_type(self as *const ::text_option::Tab) }
  }
}

impl Drop for ::text_option::Tab {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextOption::Tab::~QTextOption::Tab()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextOption_Tab_destructor(self as *mut ::text_option::Tab) }
  }
}

/// C++ type: <span style='color: green;'>```QTextOption::TabType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabType {
  /// C++ enum variant: <span style='color: green;'>```LeftTab = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```RightTab = 1```</span>
  Right = 1,
  /// C++ enum variant: <span style='color: green;'>```CenterTab = 2```</span>
  Center = 2,
  /// C++ enum variant: <span style='color: green;'>```DelimiterTab = 3```</span>
  Delimiter = 3,
}

/// C++ type: <span style='color: green;'>```QTextOption```</span>
#[repr(C)]
pub struct TextOption([u8; ::type_sizes::QT_GUI_TEXT_OPTION_TEXT_OPTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextOption {
  unsafe fn new_uninitialized() -> TextOption {
    TextOption(::std::mem::uninitialized())
  }
}

impl TextOption {
  /// C++ method: <span style='color: green;'>```QFlags<QTextOption::Flag> QTextOption::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::qt_core::flags::Flags<::text_option::Flag> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextOption_flags(self as *const ::text_option::TextOption) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QTextOption::QTextOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_option::TextOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextOption::QTextOption()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_option::TextOption) -> ::text_option::TextOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextOption::QTextOption(const QTextOption& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_option::TextOption
    where Args: overloading::TextOptionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextOption& QTextOption::operator=(const QTextOption& o)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, o: &'l1 ::text_option::TextOption) -> &'l0 mut ::text_option::TextOption {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextOption_operator_assign(self as *mut ::text_option::TextOption,
                                                  o as *const ::text_option::TextOption)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setFlags(QFlags<QTextOption::Flag> flags)```</span>
  ///
  ///
  pub fn set_flags(&mut self, flags: ::qt_core::flags::Flags<::text_option::Flag>) {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_setFlags(self as *mut ::text_option::TextOption,
                                           flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setTabArray(const QList<double>& tabStops)```</span>
  ///
  ///
  pub fn set_tab_array(&mut self, tab_stops: &::list::ListCDouble) {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_setTabArray(self as *mut ::text_option::TextOption,
                                              tab_stops as *const ::list::ListCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setTabStop(double tabStop)```</span>
  ///
  ///
  pub fn set_tab_stop(&mut self, tab_stop: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextOption_setTabStop(self as *mut ::text_option::TextOption, tab_stop) }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setTabs(const QList<QTextOption::Tab>& tabStops)```</span>
  ///
  ///
  pub fn set_tabs(&mut self, tab_stops: &::list::ListTextOptionTab) {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_setTabs(self as *mut ::text_option::TextOption,
                                          tab_stops as *const ::list::ListTextOptionTab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setTextDirection(Qt::LayoutDirection aDirection)```</span>
  ///
  ///
  pub fn set_text_direction(&mut self, a_direction: &::qt_core::qt::LayoutDirection) {
    unsafe {
      ::ffi::qt_gui_c_QTextOption_setTextDirection(self as *mut ::text_option::TextOption,
                                                   a_direction as *const ::qt_core::qt::LayoutDirection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setUseDesignMetrics(bool b)```</span>
  ///
  ///
  pub fn set_use_design_metrics(&mut self, b: bool) {
    unsafe { ::ffi::qt_gui_c_QTextOption_setUseDesignMetrics(self as *mut ::text_option::TextOption, b) }
  }

  /// C++ method: <span style='color: green;'>```void QTextOption::setWrapMode(QTextOption::WrapMode wrap)```</span>
  ///
  ///
  pub fn set_wrap_mode(&mut self, wrap: ::text_option::WrapMode) {
    unsafe { ::ffi::qt_gui_c_QTextOption_setWrapMode(self as *mut ::text_option::TextOption, wrap) }
  }

  /// C++ method: <span style='color: green;'>```QList<double> QTextOption::tabArray() const```</span>
  ///
  ///
  pub fn tab_array(&self) -> ::list::ListCDouble {
    {
      let mut object: ::list::ListCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextOption_tabArray_to_output(self as *const ::text_option::TextOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextOption::tabStop() const```</span>
  ///
  ///
  pub fn tab_stop(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextOption_tabStop(self as *const ::text_option::TextOption) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab> QTextOption::tabs() const```</span>
  ///
  ///
  pub fn tabs(&self) -> ::list::ListTextOptionTab {
    {
      let mut object: ::list::ListTextOptionTab =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextOption_tabs_to_output(self as *const ::text_option::TextOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextOption::useDesignMetrics() const```</span>
  ///
  ///
  pub fn use_design_metrics(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextOption_useDesignMetrics(self as *const ::text_option::TextOption) }
  }

  /// C++ method: <span style='color: green;'>```QTextOption::WrapMode QTextOption::wrapMode() const```</span>
  ///
  ///
  pub fn wrap_mode(&self) -> ::text_option::WrapMode {
    unsafe { ::ffi::qt_gui_c_QTextOption_wrapMode(self as *const ::text_option::TextOption) }
  }
}

impl Drop for ::text_option::TextOption {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextOption::~QTextOption()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextOption_destructor(self as *mut ::text_option::TextOption) }
  }
}

/// C++ type: <span style='color: green;'>```QTextOption::WrapMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WrapMode {
  /// C++ enum variant: <span style='color: green;'>```NoWrap = 0```</span>
  NoWrap = 0,
  /// C++ enum variant: <span style='color: green;'>```WordWrap = 1```</span>
  WordWrap = 1,
  /// C++ enum variant: <span style='color: green;'>```ManualWrap = 2```</span>
  ManualWrap = 2,
  /// C++ enum variant: <span style='color: green;'>```WrapAnywhere = 3```</span>
  WrapAnywhere = 3,
  /// C++ enum variant: <span style='color: green;'>```WrapAtWordBoundaryOrAnywhere = 4```</span>
  WrapAtWordBoundaryOrAnywhere = 4,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Tab::new](../struct.Tab.html#method.new) method.
  pub trait TabNewArgs {
    fn exec(self) -> ::text_option::Tab;
  }
  impl TabNewArgs for () {
    fn exec(self) -> ::text_option::Tab {

      {
        let mut object: ::text_option::Tab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextOption_Tab_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl TabNewArgs for (::libc::c_double, ::text_option::TabType) {
    fn exec(self) -> ::text_option::Tab {
      let pos = self.0;
      let tab_type = self.1;
      {
        let mut object: ::text_option::Tab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextOption_Tab_constructor_pos_tabType(pos, tab_type, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TabNewArgs for (::libc::c_double, ::text_option::TabType, &'a ::qt_core::char::Char) {
    fn exec(self) -> ::text_option::Tab {
      let pos = self.0;
      let tab_type = self.1;
      let delim = self.2;
      {
        let mut object: ::text_option::Tab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextOption_Tab_constructor_pos_tabType_delim(pos,
                                                                        tab_type,
                                                                        delim as *const ::qt_core::char::Char,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextOption::new](../struct.TextOption.html#method.new) method.
  pub trait TextOptionNewArgs {
    fn exec(self) -> ::text_option::TextOption;
  }
  impl TextOptionNewArgs for () {
    fn exec(self) -> ::text_option::TextOption {

      {
        let mut object: ::text_option::TextOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextOption_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextOptionNewArgs for &'a ::text_option::TextOption {
    fn exec(self) -> ::text_option::TextOption {
      let o = self;
      {
        let mut object: ::text_option::TextOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextOption_constructor_o(o as *const ::text_option::TextOption, &mut object);
        }
        object
      }
    }
  }
}
