/// C++ type: <span style='color: green;'>```QStaticText::PerformanceHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PerformanceHint {
  /// C++ enum variant: <span style='color: green;'>```ModerateCaching = 0```</span>
  Moderate = 0,
  /// C++ enum variant: <span style='color: green;'>```AggressiveCaching = 1```</span>
  Aggressive = 1,
}

/// C++ type: <span style='color: green;'>```QStaticText```</span>
#[repr(C)]
pub struct StaticText([u8; ::type_sizes::QT_GUI_STATIC_TEXT_STATIC_TEXT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StaticText {
  unsafe fn new_uninitialized() -> StaticText {
    StaticText(::std::mem::uninitialized())
  }
}

impl StaticText {
  /// C++ method: <span style='color: green;'>```QStaticText::QStaticText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::static_text::StaticText```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStaticText::QStaticText()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::static_text::StaticText) -> ::static_text::StaticText```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStaticText::QStaticText(const QStaticText& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::static_text::StaticText```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStaticText::QStaticText(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::static_text::StaticText
    where Args: overloading::StaticTextNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStaticText& QStaticText::operator=(const QStaticText& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::static_text::StaticText)
                             -> &'l0 mut ::static_text::StaticText {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QStaticText_operator_assign(self as *mut ::static_text::StaticText,
                                                  arg1 as *const ::static_text::StaticText)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStaticText::operator==(const QStaticText& arg1) const```</span>
  ///
  ///
  pub fn op_eq(&self, arg1: &::static_text::StaticText) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QStaticText_operator_eq(self as *const ::static_text::StaticText,
                                              arg1 as *const ::static_text::StaticText)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStaticText::operator!=(const QStaticText& arg1) const```</span>
  ///
  ///
  pub fn op_neq(&self, arg1: &::static_text::StaticText) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QStaticText_operator_neq(self as *const ::static_text::StaticText,
                                               arg1 as *const ::static_text::StaticText)
    }
  }

  /// C++ method: <span style='color: green;'>```QStaticText::PerformanceHint QStaticText::performanceHint() const```</span>
  ///
  ///
  pub fn performance_hint(&self) -> ::static_text::PerformanceHint {
    unsafe { ::ffi::qt_gui_c_QStaticText_performanceHint(self as *const ::static_text::StaticText) }
  }

  /// C++ method: <span style='color: green;'>```QStaticText::prepare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn prepare(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStaticText::prepare()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn prepare(&mut self, &::transform::Transform) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStaticText::prepare(const QTransform& matrix = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn prepare(&mut self, (&::transform::Transform, &::font::Font)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStaticText::prepare(const QTransform& matrix = ?, const QFont& font = ?)```</span>
  ///
  ///
  pub fn prepare<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StaticTextPrepareArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStaticText::setPerformanceHint(QStaticText::PerformanceHint performanceHint)```</span>
  ///
  ///
  pub fn set_performance_hint(&mut self, performance_hint: ::static_text::PerformanceHint) {
    unsafe { ::ffi::qt_gui_c_QStaticText_setPerformanceHint(self as *mut ::static_text::StaticText, performance_hint) }
  }

  /// C++ method: <span style='color: green;'>```void QStaticText::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStaticText_setText(self as *mut ::static_text::StaticText,
                                          text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStaticText::setTextFormat(Qt::TextFormat textFormat)```</span>
  ///
  ///
  pub fn set_text_format(&mut self, text_format: &::qt_core::qt::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QStaticText_setTextFormat(self as *mut ::static_text::StaticText,
                                                text_format as *const ::qt_core::qt::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStaticText::setTextOption(const QTextOption& textOption)```</span>
  ///
  ///
  pub fn set_text_option(&mut self, text_option: &::text_option::TextOption) {
    unsafe {
      ::ffi::qt_gui_c_QStaticText_setTextOption(self as *mut ::static_text::StaticText,
                                                text_option as *const ::text_option::TextOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStaticText::setTextWidth(double textWidth)```</span>
  ///
  ///
  pub fn set_text_width(&mut self, text_width: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QStaticText_setTextWidth(self as *mut ::static_text::StaticText, text_width) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QStaticText::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStaticText_size_to_output(self as *const ::static_text::StaticText, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStaticText::swap(QStaticText& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::static_text::StaticText) {
    unsafe {
      ::ffi::qt_gui_c_QStaticText_swap(self as *mut ::static_text::StaticText,
                                       other as *mut ::static_text::StaticText)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStaticText::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStaticText_text_to_output(self as *const ::static_text::StaticText, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextOption QStaticText::textOption() const```</span>
  ///
  ///
  pub fn text_option(&self) -> ::text_option::TextOption {
    {
      let mut object: ::text_option::TextOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStaticText_textOption_to_output(self as *const ::static_text::StaticText, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QStaticText::textWidth() const```</span>
  ///
  ///
  pub fn text_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QStaticText_textWidth(self as *const ::static_text::StaticText) }
  }
}

impl Drop for ::static_text::StaticText {
  /// C++ method: <span style='color: green;'>```[destructor] void QStaticText::~QStaticText()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QStaticText_destructor(self as *mut ::static_text::StaticText) }
  }
}

/// C++ method: <span style='color: green;'>```void swap(QStaticText& value1, QStaticText& value2)```</span>
///
///
pub fn swap(value1: &mut ::static_text::StaticText, value2: &mut ::static_text::StaticText) {
  unsafe {
    ::ffi::qt_gui_c_QStaticText_G_swap(value1 as *mut ::static_text::StaticText,
                                       value2 as *mut ::static_text::StaticText)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StaticText::new](../struct.StaticText.html#method.new) method.
  pub trait StaticTextNewArgs {
    fn exec(self) -> ::static_text::StaticText;
  }
  impl StaticTextNewArgs for () {
    fn exec(self) -> ::static_text::StaticText {

      {
        let mut object: ::static_text::StaticText =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStaticText_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StaticTextNewArgs for &'a ::static_text::StaticText {
    fn exec(self) -> ::static_text::StaticText {
      let other = self;
      {
        let mut object: ::static_text::StaticText =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStaticText_constructor_other(other as *const ::static_text::StaticText, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StaticTextNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::static_text::StaticText {
      let text = self;
      {
        let mut object: ::static_text::StaticText =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStaticText_constructor_text(text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StaticText::prepare](../struct.StaticText.html#method.prepare) method.
  pub trait StaticTextPrepareArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::static_text::StaticText) -> ();
  }
  impl<'largs> StaticTextPrepareArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs mut ::static_text::StaticText) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_gui_c_QStaticText_prepare_matrix(original_self as *mut ::static_text::StaticText,
                                                   matrix as *const ::transform::Transform)
      }
    }
  }
  impl<'largs> StaticTextPrepareArgs<'largs> for (&'largs ::transform::Transform, &'largs ::font::Font) {
    fn exec(self, original_self: &'largs mut ::static_text::StaticText) -> () {
      let matrix = self.0;
      let font = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStaticText_prepare_matrix_font(original_self as *mut ::static_text::StaticText,
                                                        matrix as *const ::transform::Transform,
                                                        font as *const ::font::Font)
      }
    }
  }
  impl<'largs> StaticTextPrepareArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::static_text::StaticText) -> () {

      unsafe { ::ffi::qt_gui_c_QStaticText_prepare_no_args(original_self as *mut ::static_text::StaticText) }
    }
  }
}
