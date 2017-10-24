/// C++ type: <span style='color: green;'>```QCommandLineOption```</span>
#[repr(C)]
pub struct CommandLineOption([u8; ::type_sizes::QT_CORE_COMMAND_LINE_OPTION_COMMAND_LINE_OPTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for CommandLineOption {
  unsafe fn new_uninitialized() -> CommandLineOption {
    CommandLineOption(::std::mem::uninitialized())
  }
}

impl CommandLineOption {
  /// C++ method: <span style='color: green;'>```QStringList QCommandLineOption::defaultValues() const```</span>
  ///
  ///
  pub fn default_values(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineOption_defaultValues_to_output(self as *const ::command_line_option::CommandLineOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QCommandLineOption::description() const```</span>
  ///
  ///
  pub fn description(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineOption_description_to_output(self as *const ::command_line_option::CommandLineOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QCommandLineOption::Flag> QCommandLineOption::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::flags::Flags<::command_line_option::Flag> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QCommandLineOption_flags(self as *const ::command_line_option::CommandLineOption) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QCommandLineOption::isHidden() const```</span>
  ///
  ///
  pub fn is_hidden(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QCommandLineOption_isHidden(self as *const ::command_line_option::CommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QCommandLineOption::names() const```</span>
  ///
  ///
  pub fn names(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineOption_names_to_output(self as *const ::command_line_option::CommandLineOption,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption::QCommandLineOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::command_line_option::CommandLineOption) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QCommandLineOption& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QString& name, const QString& description)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String, &::string::String)) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QString& name, const QString& description, const QString& valueName = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String, &::string::String, &::string::String)) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QString& name, const QString& description, const QString& valueName = ?, const QString& defaultValue = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::string_list::StringList) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QStringList& names)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new((&::string_list::StringList, &::string::String)) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QStringList& names, const QString& description)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((&::string_list::StringList, &::string::String, &::string::String)) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QStringList& names, const QString& description, const QString& valueName = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((&::string_list::StringList, &::string::String, &::string::String, &::string::String)) -> ::command_line_option::CommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineOption::QCommandLineOption(const QStringList& names, const QString& description, const QString& valueName = ?, const QString& defaultValue = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::command_line_option::CommandLineOption
    where Args: overloading::CommandLineOptionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QCommandLineOption& QCommandLineOption::operator=(const QCommandLineOption& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::command_line_option::CommandLineOption)
                             -> &'l0 mut ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QCommandLineOption_operator_assign(self as *mut ::command_line_option::CommandLineOption,
                                                            other as *const ::command_line_option::CommandLineOption)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::setDefaultValue(const QString& defaultValue)```</span>
  ///
  ///
  pub fn set_default_value(&mut self, default_value: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_setDefaultValue(self as *mut ::command_line_option::CommandLineOption,
                                                          default_value as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::setDefaultValues(const QStringList& defaultValues)```</span>
  ///
  ///
  pub fn set_default_values(&mut self, default_values: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_setDefaultValues(self as *mut ::command_line_option::CommandLineOption,
                                                           default_values as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::setDescription(const QString& description)```</span>
  ///
  ///
  pub fn set_description(&mut self, description: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_setDescription(self as *mut ::command_line_option::CommandLineOption,
                                                         description as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::setFlags(QFlags<QCommandLineOption::Flag> aflags)```</span>
  ///
  ///
  pub fn set_flags(&mut self, aflags: ::flags::Flags<::command_line_option::Flag>) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_setFlags(self as *mut ::command_line_option::CommandLineOption,
                                                   aflags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::setHidden(bool hidden)```</span>
  ///
  ///
  pub fn set_hidden(&mut self, hidden: bool) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_setHidden(self as *mut ::command_line_option::CommandLineOption,
                                                    hidden)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::setValueName(const QString& name)```</span>
  ///
  ///
  pub fn set_value_name(&mut self, name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_setValueName(self as *mut ::command_line_option::CommandLineOption,
                                                       name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineOption::swap(QCommandLineOption& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::command_line_option::CommandLineOption) {
    unsafe {
      ::ffi::qt_core_c_QCommandLineOption_swap(self as *mut ::command_line_option::CommandLineOption,
                                               other as *mut ::command_line_option::CommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QCommandLineOption::valueName() const```</span>
  ///
  ///
  pub fn value_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineOption_valueName_to_output(self as *const ::command_line_option::CommandLineOption, &mut object);
      }
      object
    }
  }
}

impl Drop for ::command_line_option::CommandLineOption {
  /// C++ method: <span style='color: green;'>```[destructor] void QCommandLineOption::~QCommandLineOption()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QCommandLineOption_destructor(self as *mut ::command_line_option::CommandLineOption) }
  }
}

/// C++ type: <span style='color: green;'>```QCommandLineOption::Flag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Flag {
  /// C++ enum variant: <span style='color: green;'>```HiddenFromHelp = 1```</span>
  HiddenFromHelp = 1,
  /// C++ enum variant: <span style='color: green;'>```ShortOptionStyle = 2```</span>
  ShortOptionStyle = 2,
}

impl ::flags::FlaggableEnum for Flag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Flag"
  }
}

/// C++ method: <span style='color: green;'>```void swap(QCommandLineOption& value1, QCommandLineOption& value2)```</span>
///
///
pub fn swap(value1: &mut ::command_line_option::CommandLineOption,
            value2: &mut ::command_line_option::CommandLineOption) {
  unsafe {
    ::ffi::qt_core_c_QCommandLineOption_G_swap(value1 as *mut ::command_line_option::CommandLineOption,
                                               value2 as *mut ::command_line_option::CommandLineOption)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CommandLineOption::new](../struct.CommandLineOption.html#method.new) method.
  pub trait CommandLineOptionNewArgs {
    fn exec(self) -> ::command_line_option::CommandLineOption;
  }
  impl<'a> CommandLineOptionNewArgs for &'a ::string::String {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let name = self;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_name(name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let name = self.0;
      let description = self.1;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_name_description(name as *const ::string::String,
                                                                           description as *const ::string::String,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs for (&'a ::string::String, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let name = self.0;
      let description = self.1;
      let value_name = self.2;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_name_description_valueName(name as *const ::string::String, description as *const ::string::String, value_name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs
    for (&'a ::string::String, &'a ::string::String, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let name = self.0;
      let description = self.1;
      let value_name = self.2;
      let default_value = self.3;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_name_description_valueName_defaultValue(name as *const ::string::String, description as *const ::string::String, value_name as *const ::string::String, default_value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs for &'a ::string_list::StringList {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let names = self;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_names(names as *const ::string_list::StringList, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs for (&'a ::string_list::StringList, &'a ::string::String) {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let names = self.0;
      let description = self.1;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_names_description(names as *const ::string_list::StringList, description as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs for (&'a ::string_list::StringList, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let names = self.0;
      let description = self.1;
      let value_name = self.2;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_names_description_valueName(names as *const ::string_list::StringList, description as *const ::string::String, value_name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs
    for (&'a ::string_list::StringList, &'a ::string::String, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let names = self.0;
      let description = self.1;
      let value_name = self.2;
      let default_value = self.3;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_names_description_valueName_defaultValue(names as *const ::string_list::StringList, description as *const ::string::String, value_name as *const ::string::String, default_value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CommandLineOptionNewArgs for &'a ::command_line_option::CommandLineOption {
    fn exec(self) -> ::command_line_option::CommandLineOption {
      let other = self;
      {
        let mut object: ::command_line_option::CommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineOption_constructor_other(other as *const ::command_line_option::CommandLineOption, &mut object);
        }
        object
      }
    }
  }
}
