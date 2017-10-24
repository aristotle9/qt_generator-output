/// C++ type: <span style='color: green;'>```QMimeType```</span>
#[repr(C)]
pub struct MimeType([u8; ::type_sizes::QT_CORE_MIME_TYPE_MIME_TYPE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MimeType {
  unsafe fn new_uninitialized() -> MimeType {
    MimeType(::std::mem::uninitialized())
  }
}

impl MimeType {
  /// C++ method: <span style='color: green;'>```QStringList QMimeType::aliases() const```</span>
  ///
  ///
  pub fn aliases(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_aliases_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QMimeType::allAncestors() const```</span>
  ///
  ///
  pub fn all_ancestors(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_allAncestors_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeType::comment() const```</span>
  ///
  ///
  pub fn comment(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_comment_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeType::filterString() const```</span>
  ///
  ///
  pub fn filter_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_filterString_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeType::genericIconName() const```</span>
  ///
  ///
  pub fn generic_icon_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_genericIconName_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QMimeType::globPatterns() const```</span>
  ///
  ///
  pub fn glob_patterns(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_globPatterns_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeType::iconName() const```</span>
  ///
  ///
  pub fn icon_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_iconName_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeType::inherits(const QString& mimeTypeName) const```</span>
  ///
  ///
  pub fn inherits(&self, mime_type_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMimeType_inherits(self as *const ::mime_type::MimeType,
                                          mime_type_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeType::isDefault() const```</span>
  ///
  ///
  pub fn is_default(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeType_isDefault(self as *const ::mime_type::MimeType) }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeType::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMimeType_isValid(self as *const ::mime_type::MimeType) }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeType::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_name_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType::QMimeType```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMimeType::QMimeType()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::mime_type::MimeType) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMimeType::QMimeType(const QMimeType& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::mime_type::MimeType
    where Args: overloading::MimeTypeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMimeType& QMimeType::operator=(const QMimeType& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::mime_type::MimeType) -> &'l0 mut ::mime_type::MimeType {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMimeType_operator_assign(self as *mut ::mime_type::MimeType,
                                                 other as *const ::mime_type::MimeType)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMimeType::operator==(const QMimeType& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::mime_type::MimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMimeType_operator_eq(self as *const ::mime_type::MimeType,
                                             other as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMimeType::operator!=(const QMimeType& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::mime_type::MimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMimeType_operator_neq(self as *const ::mime_type::MimeType,
                                              other as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QMimeType::parentMimeTypes() const```</span>
  ///
  ///
  pub fn parent_mime_types(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_parentMimeTypes_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeType::preferredSuffix() const```</span>
  ///
  ///
  pub fn preferred_suffix(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_preferredSuffix_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QMimeType::suffixes() const```</span>
  ///
  ///
  pub fn suffixes(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeType_suffixes_to_output(self as *const ::mime_type::MimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMimeType::swap(QMimeType& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::mime_type::MimeType) {
    unsafe {
      ::ffi::qt_core_c_QMimeType_swap(self as *mut ::mime_type::MimeType,
                                      other as *mut ::mime_type::MimeType)
    }
  }
}

impl Drop for ::mime_type::MimeType {
  /// C++ method: <span style='color: green;'>```[destructor] void QMimeType::~QMimeType()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMimeType_destructor(self as *mut ::mime_type::MimeType) }
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::mime_type::MimeType) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QMimeType& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::mime_type::MimeType, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QMimeType& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QMimeType& mime)```</span>
///
///
pub fn op_shl(debug: &::debug::Debug, mime: &::mime_type::MimeType) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QMimeType_G_operator_shl_to_output(debug as *const ::debug::Debug,
                                                          mime as *const ::mime_type::MimeType,
                                                          &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QMimeType& value1, QMimeType& value2)```</span>
///
///
pub fn swap(value1: &mut ::mime_type::MimeType, value2: &mut ::mime_type::MimeType) {
  unsafe {
    ::ffi::qt_core_c_QMimeType_G_swap(value1 as *mut ::mime_type::MimeType,
                                      value2 as *mut ::mime_type::MimeType)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MimeType::new](../struct.MimeType.html#method.new) method.
  pub trait MimeTypeNewArgs {
    fn exec(self) -> ::mime_type::MimeType;
  }
  impl MimeTypeNewArgs for () {
    fn exec(self) -> ::mime_type::MimeType {

      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMimeType_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MimeTypeNewArgs for &'a ::mime_type::MimeType {
    fn exec(self) -> ::mime_type::MimeType {
      let other = self;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMimeType_constructor_other(other as *const ::mime_type::MimeType, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::mime_type::MimeType {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QMimeType_G_qHash_key(key as *const ::mime_type::MimeType) }
    }
  }
  impl<'a> HashArgs for (&'a ::mime_type::MimeType, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QMimeType_G_qHash_key_seed(key as *const ::mime_type::MimeType, seed) }
    }
  }
}
