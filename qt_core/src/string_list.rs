/// C++ type: <span style='color: green;'>```QStringList```</span>
#[repr(C)]
pub struct StringList([u8; ::type_sizes::QT_CORE_STRING_LIST_STRING_LIST]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StringList {
  unsafe fn new_uninitialized() -> StringList {
    StringList(::std::mem::uninitialized())
  }
}

impl StringList {
  /// C++ method: <span style='color: green;'>```QStringList::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringList::contains(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringList::contains(const QString& str, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringListContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &mut ::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::indexOf(QRegExp& rx) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&mut ::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::indexOf(QRegExp& rx, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index_of(&self, &::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::indexOf(const QRegExp& rx) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn index_of(&self, (&::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::indexOf(const QRegExp& rx, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn index_of(&self, &::regular_expression::RegularExpression) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::indexOf(const QRegularExpression& re) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn index_of(&self, (&::regular_expression::RegularExpression, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::indexOf(const QRegularExpression& re, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringListIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &mut ::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::lastIndexOf(QRegExp& rx) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&mut ::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::lastIndexOf(QRegExp& rx, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::lastIndexOf(const QRegExp& rx) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::lastIndexOf(const QRegExp& rx, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::regular_expression::RegularExpression) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::lastIndexOf(const QRegularExpression& re) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::regular_expression::RegularExpression, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringList::lastIndexOf(const QRegularExpression& re, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringListLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList::QStringList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringList::QStringList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListString) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringList::QStringList(const QList<QString>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringList::QStringList(const QString& i)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::string_list::StringList
    where Args: overloading::StringListNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringList QStringList::operator+(const QStringList& other) const```</span>
  ///
  ///
  pub fn op_add(&self, other: &::string_list::StringList) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringList_operator_add_to_output(self as *const ::string_list::StringList,
                                                            other as *const ::string_list::StringList,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList& QStringList::operator=(const QList<QString>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::list::ListString) -> &'l0 mut ::string_list::StringList {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QStringList_operator_assign(self as *mut ::string_list::StringList,
                                                   other as *const ::list::ListString)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStringList::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListString) -> &'l0 mut ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList& QStringList::operator<<(const QList<QString>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::string::String) -> &'l0 mut ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList& QStringList::operator<<(const QString& str)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::string_list::StringList) -> &'l0 mut ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList& QStringList::operator<<(const QStringList& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string_list::StringList
    where Args: overloading::StringListOpShlArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::string_list::StringList {
  /// C++ method: <span style='color: green;'>```[destructor] void QStringList::~QStringList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QStringList_destructor(self as *mut ::string_list::StringList) }
  }
}

/// C++ method: <span style='color: green;'>```QStringList operator+(const QList<QString>& one, const QStringList& other)```</span>
///
///
pub fn op_add(one: &::list::ListString, other: &::string_list::StringList) -> ::string_list::StringList {
  {
    let mut object: ::string_list::StringList =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QStringList_G_operator_add_to_output(one as *const ::list::ListString,
                                                            other as *const ::string_list::StringList,
                                                            &mut object);
    }
    object
  }
}

impl ::cpp_utils::StaticCast<::list::ListString> for ::string_list::StringList {
  fn static_cast_mut(&mut self) -> &mut ::list::ListString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStringList_G_static_cast_QList_QString_ptr(self as *mut ::string_list::StringList) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::list::ListString {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringList_G_static_cast_QList_QString_ptr(self as *const ::string_list::StringList as *mut ::string_list::StringList) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::string_list::StringList> for ::list::ListString {
  unsafe fn static_cast_mut(&mut self) -> &mut ::string_list::StringList {
    let ffi_result = ::ffi::qt_core_c_QStringList_G_static_cast_QStringList_ptr(self as *mut ::list::ListString);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::string_list::StringList {
    let ffi_result = ::ffi::qt_core_c_QStringList_G_static_cast_QStringList_ptr(self as *const ::list::ListString as *mut ::list::ListString);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::string_list::StringList {
  type Target = ::list::ListString;
  fn deref(&self) -> &::list::ListString {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStringList_G_static_cast_QList_QString_ptr(self as *const ::string_list::StringList as *mut ::string_list::StringList) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::string_list::StringList {
  fn deref_mut(&mut self) -> &mut ::list::ListString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStringList_G_static_cast_QList_QString_ptr(self as *mut ::string_list::StringList) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StringList::contains](../struct.StringList.html#method.contains) method.
  pub trait StringListContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> bool;
  }
  impl<'largs> StringListContainsArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> bool {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringList_contains_str(original_self as *const ::string_list::StringList,
                                                  str as *const ::string::String)
      }
    }
  }
  impl<'largs> StringListContainsArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> bool {
      let str = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringList_contains_str_cs(original_self as *const ::string_list::StringList,
                                                     str as *const ::string::String,
                                                     cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringList::index_of](../struct.StringList.html#method.index_of) method.
  pub trait StringListIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int;
  }
  impl<'largs> StringListIndexOfArgs<'largs> for &'largs mut ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self;
      unsafe {
        ::ffi::qt_core_c_QStringList_indexOf_QRegExp_ref(original_self as *const ::string_list::StringList,
                                                         rx as *mut ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringListIndexOfArgs<'largs> for (&'largs mut ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringList_indexOf_QRegExp_ref_int(original_self as *const ::string_list::StringList,
                                                             rx as *mut ::reg_exp::RegExp,
                                                             from)
      }
    }
  }
  impl<'largs> StringListIndexOfArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self;
      unsafe {
        ::ffi::qt_core_c_QStringList_indexOf_const_QRegExp_ref(original_self as *const ::string_list::StringList,
                                                               rx as *const ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringListIndexOfArgs<'largs> for (&'largs ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringList_indexOf_const_QRegExp_ref_int(original_self as *const ::string_list::StringList,
                                                                   rx as *const ::reg_exp::RegExp,
                                                                   from)
      }
    }
  }
  impl<'largs> StringListIndexOfArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let re = self;
      unsafe { ::ffi::qt_core_c_QStringList_indexOf_const_QRegularExpression_ref(original_self as *const ::string_list::StringList, re as *const ::regular_expression::RegularExpression) }
    }
  }
  impl<'largs> StringListIndexOfArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let re = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QStringList_indexOf_const_QRegularExpression_ref_int(original_self as *const ::string_list::StringList, re as *const ::regular_expression::RegularExpression, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringList::last_index_of](../struct.StringList.html#method.last_index_of) method.
  pub trait StringListLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int;
  }
  impl<'largs> StringListLastIndexOfArgs<'largs> for &'largs mut ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self;
      unsafe {
        ::ffi::qt_core_c_QStringList_lastIndexOf_QRegExp_ref(original_self as *const ::string_list::StringList,
                                                             rx as *mut ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringListLastIndexOfArgs<'largs> for (&'largs mut ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringList_lastIndexOf_QRegExp_ref_int(original_self as *const ::string_list::StringList,
                                                                 rx as *mut ::reg_exp::RegExp,
                                                                 from)
      }
    }
  }
  impl<'largs> StringListLastIndexOfArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self;
      unsafe {
        ::ffi::qt_core_c_QStringList_lastIndexOf_const_QRegExp_ref(original_self as *const ::string_list::StringList,
                                                                   rx as *const ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringListLastIndexOfArgs<'largs> for (&'largs ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let rx = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QStringList_lastIndexOf_const_QRegExp_ref_int(original_self as *const ::string_list::StringList, rx as *const ::reg_exp::RegExp, from) }
    }
  }
  impl<'largs> StringListLastIndexOfArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let re = self;
      unsafe { ::ffi::qt_core_c_QStringList_lastIndexOf_const_QRegularExpression_ref(original_self as *const ::string_list::StringList, re as *const ::regular_expression::RegularExpression) }
    }
  }
  impl<'largs> StringListLastIndexOfArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_list::StringList) -> ::libc::c_int {
      let re = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QStringList_lastIndexOf_const_QRegularExpression_ref_int(original_self as *const ::string_list::StringList, re as *const ::regular_expression::RegularExpression, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringList::new](../struct.StringList.html#method.new) method.
  pub trait StringListNewArgs {
    fn exec(self) -> ::string_list::StringList;
  }
  impl<'a> StringListNewArgs for &'a ::string::String {
    fn exec(self) -> ::string_list::StringList {
      let i = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringList_constructor_i(i as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StringListNewArgs for &'a ::list::ListString {
    fn exec(self) -> ::string_list::StringList {
      let l = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringList_constructor_l(l as *const ::list::ListString, &mut object);
        }
        object
      }
    }
  }
  impl StringListNewArgs for () {
    fn exec(self) -> ::string_list::StringList {

      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringList_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringList::op_shl](../struct.StringList.html#method.op_shl) method.
  pub trait StringListOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string_list::StringList) -> &'largs mut ::string_list::StringList;
  }
  impl<'largs> StringListOpShlArgs<'largs> for &'largs ::list::ListString {
    fn exec(self, original_self: &'largs mut ::string_list::StringList) -> &'largs mut ::string_list::StringList {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QStringList_operator_shl_QList_QString(original_self as *mut ::string_list::StringList,
                                                                  l as *const ::list::ListString)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringListOpShlArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string_list::StringList) -> &'largs mut ::string_list::StringList {
      let str = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QStringList_operator_shl_QString(original_self as *mut ::string_list::StringList,
                                                            str as *const ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringListOpShlArgs<'largs> for &'largs ::string_list::StringList {
    fn exec(self, original_self: &'largs mut ::string_list::StringList) -> &'largs mut ::string_list::StringList {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QStringList_operator_shl_QStringList(original_self as *mut ::string_list::StringList,
                                                                l as *const ::string_list::StringList)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
