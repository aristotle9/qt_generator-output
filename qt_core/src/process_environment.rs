/// C++ type: <span style='color: green;'>```QProcessEnvironment```</span>
#[repr(C)]
pub struct ProcessEnvironment([u8; ::type_sizes::QT_CORE_PROCESS_ENVIRONMENT_PROCESS_ENVIRONMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ProcessEnvironment {
  unsafe fn new_uninitialized() -> ProcessEnvironment {
    ProcessEnvironment(::std::mem::uninitialized())
  }
}

impl ProcessEnvironment {
  /// C++ method: <span style='color: green;'>```void QProcessEnvironment::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QProcessEnvironment_clear(self as *mut ::process_environment::ProcessEnvironment) }
  }

  /// C++ method: <span style='color: green;'>```bool QProcessEnvironment::contains(const QString& name) const```</span>
  ///
  ///
  pub fn contains(&self, name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QProcessEnvironment_contains(self as *const ::process_environment::ProcessEnvironment,
                                                    name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QProcessEnvironment::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, &::process_environment::ProcessEnvironment) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcessEnvironment::insert(const QProcessEnvironment& e)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcessEnvironment::insert(const QString& name, const QString& value)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ProcessEnvironmentInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QProcessEnvironment::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QProcessEnvironment_isEmpty(self as *const ::process_environment::ProcessEnvironment) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QProcessEnvironment::keys() const```</span>
  ///
  ///
  pub fn keys(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcessEnvironment_keys_to_output(self as *const ::process_environment::ProcessEnvironment,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProcessEnvironment::QProcessEnvironment```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::process_environment::ProcessEnvironment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QProcessEnvironment::QProcessEnvironment()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::process_environment::ProcessEnvironment) -> ::process_environment::ProcessEnvironment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QProcessEnvironment::QProcessEnvironment(const QProcessEnvironment& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::process_environment::ProcessEnvironment
    where Args: overloading::ProcessEnvironmentNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QProcessEnvironment& QProcessEnvironment::operator=(const QProcessEnvironment& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::process_environment::ProcessEnvironment)
                             -> &'l0 mut ::process_environment::ProcessEnvironment {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QProcessEnvironment_operator_assign(self as *mut ::process_environment::ProcessEnvironment, other as *const ::process_environment::ProcessEnvironment)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QProcessEnvironment::operator==(const QProcessEnvironment& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::process_environment::ProcessEnvironment) -> bool {
    unsafe {
      ::ffi::qt_core_c_QProcessEnvironment_operator_eq(self as *const ::process_environment::ProcessEnvironment,
                                                       other as *const ::process_environment::ProcessEnvironment)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QProcessEnvironment::operator!=(const QProcessEnvironment& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::process_environment::ProcessEnvironment) -> bool {
    unsafe {
      ::ffi::qt_core_c_QProcessEnvironment_operator_neq(self as *const ::process_environment::ProcessEnvironment,
                                                        other as *const ::process_environment::ProcessEnvironment)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProcessEnvironment::remove(const QString& name)```</span>
  ///
  ///
  pub fn remove(&mut self, name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QProcessEnvironment_remove(self as *mut ::process_environment::ProcessEnvironment,
                                                  name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProcessEnvironment::swap(QProcessEnvironment& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::process_environment::ProcessEnvironment) {
    unsafe {
      ::ffi::qt_core_c_QProcessEnvironment_swap(self as *mut ::process_environment::ProcessEnvironment,
                                                other as *mut ::process_environment::ProcessEnvironment)
    }
  }

  /// C++ method: <span style='color: green;'>```static QProcessEnvironment QProcessEnvironment::systemEnvironment()```</span>
  ///
  ///
  pub fn system_environment() -> ::process_environment::ProcessEnvironment {
    {
      let mut object: ::process_environment::ProcessEnvironment =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcessEnvironment_systemEnvironment_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QProcessEnvironment::toStringList() const```</span>
  ///
  ///
  pub fn to_string_list(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcessEnvironment_toStringList_to_output(self as *const ::process_environment::ProcessEnvironment, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProcessEnvironment::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QProcessEnvironment::value(const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QProcessEnvironment::value(const QString& name, const QString& defaultValue = ?) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::ProcessEnvironmentValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::process_environment::ProcessEnvironment {
  /// C++ method: <span style='color: green;'>```[destructor] void QProcessEnvironment::~QProcessEnvironment()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QProcessEnvironment_destructor(self as *mut ::process_environment::ProcessEnvironment) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ProcessEnvironment::insert](../struct.ProcessEnvironment.html#method.insert) method.
  pub trait ProcessEnvironmentInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process_environment::ProcessEnvironment) -> ();
  }
  impl<'largs> ProcessEnvironmentInsertArgs<'largs> for &'largs ::process_environment::ProcessEnvironment {
    fn exec(self, original_self: &'largs mut ::process_environment::ProcessEnvironment) -> () {
      let e = self;
      unsafe {
        ::ffi::qt_core_c_QProcessEnvironment_insert_e(original_self as *mut ::process_environment::ProcessEnvironment,
                                                      e as *const ::process_environment::ProcessEnvironment)
      }
    }
  }
  impl<'largs> ProcessEnvironmentInsertArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::process_environment::ProcessEnvironment) -> () {
      let name = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QProcessEnvironment_insert_name_value(original_self as *mut ::process_environment::ProcessEnvironment, name as *const ::string::String, value as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [ProcessEnvironment::new](../struct.ProcessEnvironment.html#method.new) method.
  pub trait ProcessEnvironmentNewArgs {
    fn exec(self) -> ::process_environment::ProcessEnvironment;
  }
  impl ProcessEnvironmentNewArgs for () {
    fn exec(self) -> ::process_environment::ProcessEnvironment {

      {
        let mut object: ::process_environment::ProcessEnvironment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QProcessEnvironment_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> ProcessEnvironmentNewArgs for &'a ::process_environment::ProcessEnvironment {
    fn exec(self) -> ::process_environment::ProcessEnvironment {
      let other = self;
      {
        let mut object: ::process_environment::ProcessEnvironment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QProcessEnvironment_constructor_other(other as *const ::process_environment::ProcessEnvironment, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ProcessEnvironment::value](../struct.ProcessEnvironment.html#method.value) method.
  pub trait ProcessEnvironmentValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::process_environment::ProcessEnvironment) -> ::string::String;
  }
  impl<'largs> ProcessEnvironmentValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::process_environment::ProcessEnvironment) -> ::string::String {
      let name = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QProcessEnvironment_value_to_output_name(original_self as *const ::process_environment::ProcessEnvironment, name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ProcessEnvironmentValueArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::process_environment::ProcessEnvironment) -> ::string::String {
      let name = self.0;
      let default_value = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QProcessEnvironment_value_to_output_name_defaultValue(original_self as *const ::process_environment::ProcessEnvironment, name as *const ::string::String, default_value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
}
