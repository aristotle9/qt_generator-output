/// C++ type: <span style='color: green;'>```QLoggingCategory```</span>
#[repr(C)]
pub struct LoggingCategory([u8; ::type_sizes::QT_CORE_LOGGING_CATEGORY_LOGGING_CATEGORY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for LoggingCategory {
  unsafe fn new_uninitialized() -> LoggingCategory {
    LoggingCategory(::std::mem::uninitialized())
  }
}

impl LoggingCategory {
  /// C++ method: <span style='color: green;'>```const char* QLoggingCategory::categoryName() const```</span>
  ///
  ///
  pub fn category_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_categoryName(self as *const ::logging_category::LoggingCategory) }
  }

  /// C++ method: <span style='color: green;'>```static QLoggingCategory* QLoggingCategory::defaultCategory()```</span>
  ///
  ///
  pub fn default_category() -> *mut ::logging_category::LoggingCategory {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_defaultCategory() }
  }

  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)(QLoggingCategory*) QLoggingCategory::installFilter(void (*FN_PTR)(QLoggingCategory*) arg1)```</span>
  ///
  ///
  pub unsafe fn install_filter(arg1: extern "C" fn(*mut ::logging_category::LoggingCategory))
                               -> extern "C" fn(*mut ::logging_category::LoggingCategory) {
    ::ffi::qt_core_c_QLoggingCategory_installFilter(arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QLoggingCategory::isCriticalEnabled() const```</span>
  ///
  ///
  pub fn is_critical_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_isCriticalEnabled(self as *const ::logging_category::LoggingCategory) }
  }

  /// C++ method: <span style='color: green;'>```bool QLoggingCategory::isDebugEnabled() const```</span>
  ///
  ///
  pub fn is_debug_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_isDebugEnabled(self as *const ::logging_category::LoggingCategory) }
  }

  /// C++ method: <span style='color: green;'>```bool QLoggingCategory::isEnabled(QtMsgType type) const```</span>
  ///
  ///
  pub fn is_enabled(&self, type_: ::message_log_context::MsgType) -> bool {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_isEnabled(self as *const ::logging_category::LoggingCategory, type_) }
  }

  /// C++ method: <span style='color: green;'>```bool QLoggingCategory::isInfoEnabled() const```</span>
  ///
  ///
  pub fn is_info_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_isInfoEnabled(self as *const ::logging_category::LoggingCategory) }
  }

  /// C++ method: <span style='color: green;'>```bool QLoggingCategory::isWarningEnabled() const```</span>
  ///
  ///
  pub fn is_warning_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_isWarningEnabled(self as *const ::logging_category::LoggingCategory) }
  }

  /// C++ method: <span style='color: green;'>```QLoggingCategory::QLoggingCategory```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(*const ::libc::c_char) -> ::logging_category::LoggingCategory```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLoggingCategory::QLoggingCategory(const char* category)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*const ::libc::c_char, ::message_log_context::MsgType)) -> ::logging_category::LoggingCategory```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLoggingCategory::QLoggingCategory(const char* category, QtMsgType severityLevel)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::logging_category::LoggingCategory
    where Args: overloading::LoggingCategoryNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QLoggingCategory& QLoggingCategory::operator()() const```</span>
  ///
  ///
  pub fn op_call<'l0>(&'l0 self) -> &'l0 ::logging_category::LoggingCategory {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QLoggingCategory_operator_call_const(self as *const ::logging_category::LoggingCategory)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLoggingCategory& QLoggingCategory::operator()()```</span>
  ///
  ///
  pub fn op_call_mut<'l0>(&'l0 mut self) -> &'l0 mut ::logging_category::LoggingCategory {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QLoggingCategory_operator_call(self as *mut ::logging_category::LoggingCategory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QLoggingCategory::setEnabled(QtMsgType type, bool enable)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, type_: ::message_log_context::MsgType, enable: bool) {
    unsafe {
      ::ffi::qt_core_c_QLoggingCategory_setEnabled(self as *mut ::logging_category::LoggingCategory,
                                                   type_,
                                                   enable)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QLoggingCategory::setFilterRules(const QString& rules)```</span>
  ///
  ///
  pub fn set_filter_rules(rules: &::string::String) {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_setFilterRules(rules as *const ::string::String) }
  }
}

impl Drop for ::logging_category::LoggingCategory {
  /// C++ method: <span style='color: green;'>```[destructor] void QLoggingCategory::~QLoggingCategory()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QLoggingCategory_destructor(self as *mut ::logging_category::LoggingCategory) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LoggingCategory::new](../struct.LoggingCategory.html#method.new) method.
  pub trait LoggingCategoryNewArgs {
    unsafe fn exec(self) -> ::logging_category::LoggingCategory;
  }
  impl LoggingCategoryNewArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::logging_category::LoggingCategory {
      let category = self;
      {
        let mut object: ::logging_category::LoggingCategory =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QLoggingCategory_constructor_category(category, &mut object);
        object
      }
    }
  }
  impl LoggingCategoryNewArgs for (*const ::libc::c_char, ::message_log_context::MsgType) {
    unsafe fn exec(self) -> ::logging_category::LoggingCategory {
      let category = self.0;
      let severity_level = self.1;
      {
        let mut object: ::logging_category::LoggingCategory =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QLoggingCategory_constructor_category_severityLevel(category, severity_level, &mut object);
        object
      }
    }
  }
}
