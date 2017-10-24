/// C++ type: <span style='color: green;'>```QStringRef```</span>
#[repr(C)]
pub struct StringRef([u8; ::type_sizes::QT_CORE_STRING_REF_STRING_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StringRef {
  unsafe fn new_uninitialized() -> StringRef {
    StringRef(::std::mem::uninitialized())
  }
}

impl StringRef {
  /// C++ method: <span style='color: green;'>```QStringRef QStringRef::appendTo(QString* string) const```</span>
  ///
  ///
  pub unsafe fn append_to(&self, string: *mut ::string::String) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QStringRef_appendTo_to_output(self as *const ::string_ref::StringRef, string, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QChar QStringRef::at(int i) const```</span>
  ///
  ///
  pub fn at(&self, i: ::libc::c_int) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_at_to_output(self as *const ::string_ref::StringRef, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_begin(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::cbegin() const```</span>
  ///
  ///
  pub fn cbegin(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_cbegin(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::cend() const```</span>
  ///
  ///
  pub fn cend(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_cend(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QStringRef::chop(int n)```</span>
  ///
  ///
  pub fn chop(&mut self, n: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QStringRef_chop(self as *mut ::string_ref::StringRef, n) }
  }

  /// C++ method: <span style='color: green;'>```void QStringRef::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QStringRef_clear(self as *mut ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::compare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn compare(&self, &::latin1_string::Latin1String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn compare(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(QLatin1String s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn compare(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn compare(&self, (&::byte_array::ByteArray, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(const QByteArray& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn compare(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn compare(&self, (&::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn compare(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn compare(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::compare(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn compare<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringRefCompareArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::compare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn compare_static((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::compare(const QStringRef& s1, QLatin1String s2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn compare_static((&::string_ref::StringRef, &::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::compare(const QStringRef& s1, QLatin1String s2, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn compare_static((&::string_ref::StringRef, &::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::compare(const QStringRef& s1, const QString& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn compare_static((&::string_ref::StringRef, &::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::compare(const QStringRef& s1, const QString& s2, Qt::CaseSensitivity arg3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn compare_static((&::string_ref::StringRef, &::string_ref::StringRef)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::compare(const QStringRef& s1, const QStringRef& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn compare_static((&::string_ref::StringRef, &::string_ref::StringRef, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::compare(const QStringRef& s1, const QStringRef& s2, Qt::CaseSensitivity arg3 = ?)```</span>
  ///
  ///
  pub fn compare_static<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::StringRefCompareStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::constBegin() const```</span>
  ///
  ///
  pub fn const_begin(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_constBegin(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_constData(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::constEnd() const```</span>
  ///
  ///
  pub fn const_end(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_constEnd(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(QChar ch) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, (&::char::Char, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(QChar ch, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn contains(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(QLatin1String str) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn contains(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(QLatin1String str, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn contains(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn contains(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(const QString& str, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn contains(&self, &::string_ref::StringRef) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(const QStringRef& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn contains(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::contains(const QStringRef& str, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringRefContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn count(&self, (&::char::Char, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn count(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn count(&self, (&::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn count(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn count(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::count(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringRefCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_data(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::end() const```</span>
  ///
  ///
  pub fn end(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_end(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::endsWith```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ends_with(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::char::Char, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ends_with(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(QLatin1String s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn ends_with(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn ends_with(&self, &::string_ref::StringRef) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(const QStringRef& c) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::endsWith(const QStringRef& c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn ends_with<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringRefEndsWithArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(QChar ch) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::char::Char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(QChar ch, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index_of(&self, (&::char::Char, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(QChar ch, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn index_of(&self, &::latin1_string::Latin1String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(QLatin1String str) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(QLatin1String str, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(QLatin1String str, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(const QString& str, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string::String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(const QString& str, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn index_of(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(const QStringRef& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(const QStringRef& str, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string_ref::StringRef, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::indexOf(const QStringRef& str, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringRefIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QStringRef::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStringRef_isEmpty(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStringRef_isNull(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::isRightToLeft() const```</span>
  ///
  ///
  pub fn is_right_to_left(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStringRef_isRightToLeft(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(QChar ch) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::char::Char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(QChar ch, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::char::Char, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(QChar ch, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::latin1_string::Latin1String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(QLatin1String str) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(QLatin1String str, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(QLatin1String str, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(const QString& str, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string::String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(const QString& str, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(const QStringRef& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(const QStringRef& str, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string_ref::StringRef, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::lastIndexOf(const QStringRef& str, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringRefLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef QStringRef::left(int n) const```</span>
  ///
  ///
  pub fn left(&self, n: ::libc::c_int) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_left_to_output(self as *const ::string_ref::StringRef, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QStringRef::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QStringRef_length(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::localeAwareCompare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn locale_aware_compare(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::localeAwareCompare(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn locale_aware_compare(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::localeAwareCompare(const QStringRef& s) const```</span>
  ///
  ///
  pub fn locale_aware_compare<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringRefLocaleAwareCompareArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::localeAwareCompare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn locale_aware_compare_static((&::string_ref::StringRef, &::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::localeAwareCompare(const QStringRef& s1, const QString& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn locale_aware_compare_static((&::string_ref::StringRef, &::string_ref::StringRef)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QStringRef::localeAwareCompare(const QStringRef& s1, const QStringRef& s2)```</span>
  ///
  ///
  pub fn locale_aware_compare_static<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::StringRefLocaleAwareCompareStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringRef::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QStringRef::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QStringRef::mid(int pos, int n = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::string_ref::StringRef
    where Args: overloading::StringRefMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::QStringRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringRef::QStringRef()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string_ref::StringRef) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringRef::QStringRef(const QStringRef& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::string_ref::StringRef
    where Args: overloading::StringRefNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringRef::QStringRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::string::String) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringRef::QStringRef(const QString* string)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::string::String, ::libc::c_int, ::libc::c_int)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringRef::QStringRef(const QString* string, int position, int size)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::string_ref::StringRef
    where Args: overloading::StringRefNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringRef& QStringRef::operator=(const QStringRef& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::string_ref::StringRef) -> &'l0 mut ::string_ref::StringRef {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QStringRef_operator_assign_other(self as *mut ::string_ref::StringRef,
                                                        other as *const ::string_ref::StringRef)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStringRef& QStringRef::operator=(const QString* string)```</span>
  ///
  ///
  pub unsafe fn op_assign_unsafe<'l0>(&'l0 mut self,
                                      string: *const ::string::String)
                                      -> &'l0 mut ::string_ref::StringRef {
    let ffi_result = ::ffi::qt_core_c_QStringRef_operator_assign_string(self as *mut ::string_ref::StringRef, string);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::operator==(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_eq(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QStringRef_operator_eq(self as *const ::string_ref::StringRef, s)
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::operator>=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_ge(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QStringRef_operator_ge(self as *const ::string_ref::StringRef, s)
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::operator>(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_gt(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QStringRef_operator_gt(self as *const ::string_ref::StringRef, s)
  }

  /// C++ method: <span style='color: green;'>```QChar QStringRef::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index(&self, i: ::libc::c_int) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_operator_index_to_output(self as *const ::string_ref::StringRef, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::operator<=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_le(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QStringRef_operator_le(self as *const ::string_ref::StringRef, s)
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::operator<(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_lt(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QStringRef_operator_lt(self as *const ::string_ref::StringRef, s)
  }

  /// C++ method: <span style='color: green;'>```bool QStringRef::operator!=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_neq(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QStringRef_operator_neq(self as *const ::string_ref::StringRef, s)
  }

  /// C++ method: <span style='color: green;'>```int QStringRef::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QStringRef_position(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QStringRef::right(int n) const```</span>
  ///
  ///
  pub fn right(&self, n: ::libc::c_int) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_right_to_output(self as *const ::string_ref::StringRef, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QStringRef::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QStringRef_size(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::split```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn split(&self, &::char::Char) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QStringRef::split(QChar sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn split(&self, (&::char::Char, &::string::SplitBehavior)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QStringRef::split(QChar sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn split(&self, (&::char::Char, &::string::SplitBehavior, &::qt::CaseSensitivity)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QStringRef::split(QChar sep, QString::SplitBehavior behavior = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn split(&self, &::string::String) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QStringRef::split(const QString& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn split(&self, (&::string::String, &::string::SplitBehavior)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QStringRef::split(const QString& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn split(&self, (&::string::String, &::string::SplitBehavior, &::qt::CaseSensitivity)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QStringRef::split(const QString& sep, QString::SplitBehavior behavior = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn split<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorStringRef
    where Args: overloading::StringRefSplitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::startsWith```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn starts_with(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::char::Char, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn starts_with(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(QLatin1String s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn starts_with(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn starts_with(&self, &::string_ref::StringRef) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(const QStringRef& c) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStringRef::startsWith(const QStringRef& c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn starts_with<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringRefStartsWithArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QString* QStringRef::string() const```</span>
  ///
  ///
  pub fn string(&self) -> *const ::string::String {
    unsafe { ::ffi::qt_core_c_QStringRef_string(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```double QStringRef::toDouble() const```</span>
  ///
  ///
  pub fn to_double(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QStringRef_toDouble_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```double QStringRef::toDouble(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_double_unsafe(&self, ok: *mut bool) -> ::libc::c_double {
    ::ffi::qt_core_c_QStringRef_toDouble_ok(self as *const ::string_ref::StringRef, ok)
  }

  /// C++ method: <span style='color: green;'>```float QStringRef::toFloat() const```</span>
  ///
  ///
  pub fn to_float(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_core_c_QStringRef_toFloat_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```float QStringRef::toFloat(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_float_unsafe(&self, ok: *mut bool) -> ::libc::c_float {
    ::ffi::qt_core_c_QStringRef_toFloat_ok(self as *const ::string_ref::StringRef, ok)
  }

  /// C++ method: <span style='color: green;'>```int QStringRef::toInt() const```</span>
  ///
  ///
  pub fn to_int(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QStringRef_toInt_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::toInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_int_unsafe(&self, *mut bool) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::toInt(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_int_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringRef::toInt(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_int_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringRefToIntUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray QStringRef::toLatin1() const```</span>
  ///
  ///
  pub fn to_latin1(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_toLatin1_to_output(self as *const ::string_ref::StringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QStringRef::toLocal8Bit() const```</span>
  ///
  ///
  pub fn to_local8_bit(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_toLocal8Bit_to_output(self as *const ::string_ref::StringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```long QStringRef::toLong() const```</span>
  ///
  ///
  pub fn to_long(&self) -> ::libc::c_long {
    unsafe { ::ffi::qt_core_c_QStringRef_toLong_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```qlonglong QStringRef::toLongLong() const```</span>
  ///
  ///
  pub fn to_long_long(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QStringRef_toLongLong_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::toLongLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_long_long_unsafe(&self, *mut bool) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qlonglong QStringRef::toLongLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_long_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qlonglong QStringRef::toLongLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_long_unsafe<'largs, Args>(&'largs self, args: Args) -> i64
    where Args: overloading::StringRefToLongLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::toLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_long_unsafe(&self, *mut bool) -> ::libc::c_long```<br>
  /// C++ method: <span style='color: green;'>```long QStringRef::toLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_long```<br>
  /// C++ method: <span style='color: green;'>```long QStringRef::toLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_long
    where Args: overloading::StringRefToLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```short QStringRef::toShort() const```</span>
  ///
  ///
  pub fn to_short(&self) -> ::libc::c_short {
    unsafe { ::ffi::qt_core_c_QStringRef_toShort_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::toShort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_short_unsafe(&self, *mut bool) -> ::libc::c_short```<br>
  /// C++ method: <span style='color: green;'>```short QStringRef::toShort(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_short_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_short```<br>
  /// C++ method: <span style='color: green;'>```short QStringRef::toShort(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_short_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_short
    where Args: overloading::StringRefToShortUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QStringRef::toString() const```</span>
  ///
  ///
  pub fn to_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_toString_to_output(self as *const ::string_ref::StringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QStringRef::toUInt() const```</span>
  ///
  ///
  pub fn to_u_int(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QStringRef_toUInt_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::toUInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_int_unsafe(&self, *mut bool) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QStringRef::toUInt(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_int_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QStringRef::toUInt(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_int_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_uint
    where Args: overloading::StringRefToUIntUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned long QStringRef::toULong() const```</span>
  ///
  ///
  pub fn to_u_long(&self) -> ::libc::c_ulong {
    unsafe { ::ffi::qt_core_c_QStringRef_toULong_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```qulonglong QStringRef::toULongLong() const```</span>
  ///
  ///
  pub fn to_u_long_long(&self) -> u64 {
    unsafe { ::ffi::qt_core_c_QStringRef_toULongLong_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::toULongLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_long_long_unsafe(&self, *mut bool) -> u64```<br>
  /// C++ method: <span style='color: green;'>```qulonglong QStringRef::toULongLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_long_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> u64```<br>
  /// C++ method: <span style='color: green;'>```qulonglong QStringRef::toULongLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_long_unsafe<'largs, Args>(&'largs self, args: Args) -> u64
    where Args: overloading::StringRefToULongLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef::toULong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_long_unsafe(&self, *mut bool) -> ::libc::c_ulong```<br>
  /// C++ method: <span style='color: green;'>```unsigned long QStringRef::toULong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_ulong```<br>
  /// C++ method: <span style='color: green;'>```unsigned long QStringRef::toULong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_ulong
    where Args: overloading::StringRefToULongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned short QStringRef::toUShort() const```</span>
  ///
  ///
  pub fn to_u_short(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QStringRef_toUShort_no_args(self as *const ::string_ref::StringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef::toUShort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_short_unsafe(&self, *mut bool) -> ::libc::c_ushort```<br>
  /// C++ method: <span style='color: green;'>```unsigned short QStringRef::toUShort(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_short_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_ushort```<br>
  /// C++ method: <span style='color: green;'>```unsigned short QStringRef::toUShort(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_short_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_ushort
    where Args: overloading::StringRefToUShortUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<unsigned int> QStringRef::toUcs4() const```</span>
  ///
  ///
  pub fn to_ucs4(&self) -> ::vector::VectorCUint {
    {
      let mut object: ::vector::VectorCUint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_toUcs4_to_output(self as *const ::string_ref::StringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QStringRef::toUtf8() const```</span>
  ///
  ///
  pub fn to_utf8(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_toUtf8_to_output(self as *const ::string_ref::StringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QStringRef::trimmed() const```</span>
  ///
  ///
  pub fn trimmed(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringRef_trimmed_to_output(self as *const ::string_ref::StringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStringRef::truncate(int pos)```</span>
  ///
  ///
  pub fn truncate(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QStringRef_truncate(self as *mut ::string_ref::StringRef, pos) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QStringRef::unicode() const```</span>
  ///
  ///
  pub fn unicode(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QStringRef_unicode(self as *const ::string_ref::StringRef) }
  }
}

impl Drop for ::string_ref::StringRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QStringRef::~QStringRef()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QStringRef_destructor(self as *mut ::string_ref::StringRef) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StringRef::compare](../struct.StringRef.html#method.compare) method.
  pub trait StringRefCompareArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int;
  }
  impl<'largs> StringRefCompareArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QByteArray(original_self as *const ::string_ref::StringRef,
                                                       s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for (&'largs ::byte_array::ByteArray, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_compare_QByteArray_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::byte_array::ByteArray, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QLatin1String(original_self as *const ::string_ref::StringRef,
                                                          s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_compare_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QString(original_self as *const ::string_ref::StringRef,
                                                    s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QStringRef(original_self as *const ::string_ref::StringRef,
                                                       s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_compare_QStringRef_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::string_ref::StringRef, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefCompareArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QString_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::string::String, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::compare_static](../struct.StringRef.html#method.compare_static) method.
  pub trait StringRefCompareStaticArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl<'a> StringRefCompareStaticArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QStringRef_QLatin1String(s1 as *const ::string_ref::StringRef,
                                                                     s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> StringRefCompareStaticArgs
    for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_compare_QStringRef_QLatin1String_Qt_CaseSensitivity(s1 as *const ::string_ref::StringRef, s2 as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'a> StringRefCompareStaticArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QStringRef_QString(s1 as *const ::string_ref::StringRef,
                                                               s2 as *const ::string::String)
      }
    }
  }
  impl<'a> StringRefCompareStaticArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                  s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> StringRefCompareStaticArgs
    for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let arg3 = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_compare_QStringRef_QStringRef_Qt_CaseSensitivity(s1 as *const ::string_ref::StringRef, s2 as *const ::string_ref::StringRef, arg3 as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'a> StringRefCompareStaticArgs for (&'a ::string_ref::StringRef, &'a ::string::String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let arg3 = self.2;
      unsafe {
        ::ffi::qt_core_c_QStringRef_compare_QStringRef_QString_Qt_CaseSensitivity(s1 as *const ::string_ref::StringRef, s2 as *const ::string::String, arg3 as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::contains](../struct.StringRef.html#method.contains) method.
  pub trait StringRefContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool;
  }
  impl<'largs> StringRefContainsArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let ch = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_contains_QChar(original_self as *const ::string_ref::StringRef,
                                                   ch as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let ch = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_contains_QChar_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef,
                                                                      ch as *const ::char::Char,
                                                                      cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_contains_QLatin1String(original_self as *const ::string_ref::StringRef,
                                                           str as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let str = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_contains_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_contains_QString(original_self as *const ::string_ref::StringRef,
                                                     str as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_contains_QStringRef(original_self as *const ::string_ref::StringRef,
                                                        str as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let str = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_contains_QStringRef_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::string_ref::StringRef, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefContainsArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let str = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_contains_QString_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::string::String, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::count](../struct.StringRef.html#method.count) method.
  pub trait StringRefCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int;
  }
  impl<'largs> StringRefCountArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_count_QChar(original_self as *const ::string_ref::StringRef,
                                                c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringRefCountArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_count_QChar_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef,
                                                                   c as *const ::char::Char,
                                                                   cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringRefCountArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_count_QString(original_self as *const ::string_ref::StringRef,
                                                  s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefCountArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_count_QStringRef(original_self as *const ::string_ref::StringRef,
                                                     s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefCountArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_count_QStringRef_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::string_ref::StringRef, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringRefCountArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_count_QString_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef,
                                                                     s as *const ::string::String,
                                                                     cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringRefCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QStringRef_count_no_args(original_self as *const ::string_ref::StringRef) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::ends_with](../struct.StringRef.html#method.ends_with) method.
  pub trait StringRefEndsWithArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool;
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_endsWith_QChar(original_self as *const ::string_ref::StringRef,
                                                   c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_endsWith_QChar_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef,
                                                                      c as *const ::char::Char,
                                                                      cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_endsWith_QLatin1String(original_self as *const ::string_ref::StringRef,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_endsWith_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_endsWith_QString(original_self as *const ::string_ref::StringRef,
                                                     s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_endsWith_QStringRef(original_self as *const ::string_ref::StringRef,
                                                        c as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_endsWith_QStringRef_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, c as *const ::string_ref::StringRef, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefEndsWithArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_endsWith_QString_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::string::String, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::index_of](../struct.StringRef.html#method.index_of) method.
  pub trait StringRefIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int;
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ch = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QChar(original_self as *const ::string_ref::StringRef,
                                                  ch as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ch = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QChar_int(original_self as *const ::string_ref::StringRef,
                                                      ch as *const ::char::Char,
                                                      from)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ch = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_indexOf_QChar_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, ch as *const ::char::Char, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QLatin1String(original_self as *const ::string_ref::StringRef,
                                                          str as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for (&'largs ::latin1_string::Latin1String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QLatin1String_int(original_self as *const ::string_ref::StringRef,
                                                              str as *const ::latin1_string::Latin1String,
                                                              from)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_indexOf_QLatin1String_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::latin1_string::Latin1String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QString(original_self as *const ::string_ref::StringRef,
                                                    str as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QStringRef(original_self as *const ::string_ref::StringRef,
                                                       str as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QStringRef_int(original_self as *const ::string_ref::StringRef,
                                                           str as *const ::string_ref::StringRef,
                                                           from)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs>
    for (&'largs ::string_ref::StringRef, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_indexOf_QStringRef_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::string_ref::StringRef, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_indexOf_QString_int(original_self as *const ::string_ref::StringRef,
                                                        str as *const ::string::String,
                                                        from)
      }
    }
  }
  impl<'largs> StringRefIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_indexOf_QString_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::string::String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::last_index_of](../struct.StringRef.html#method.last_index_of) method.
  pub trait StringRefLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int;
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ch = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QChar(original_self as *const ::string_ref::StringRef,
                                                      ch as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ch = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QChar_int(original_self as *const ::string_ref::StringRef,
                                                          ch as *const ::char::Char,
                                                          from)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ch = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_lastIndexOf_QChar_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, ch as *const ::char::Char, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QLatin1String(original_self as *const ::string_ref::StringRef,
                                                              str as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for (&'largs ::latin1_string::Latin1String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QLatin1String_int(original_self as *const ::string_ref::StringRef,
                                                                  str as *const ::latin1_string::Latin1String,
                                                                  from)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_lastIndexOf_QLatin1String_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::latin1_string::Latin1String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QString(original_self as *const ::string_ref::StringRef,
                                                        str as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QStringRef(original_self as *const ::string_ref::StringRef,
                                                           str as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QStringRef_int(original_self as *const ::string_ref::StringRef,
                                                               str as *const ::string_ref::StringRef,
                                                               from)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs>
    for (&'largs ::string_ref::StringRef, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_lastIndexOf_QStringRef_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::string_ref::StringRef, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_lastIndexOf_QString_int(original_self as *const ::string_ref::StringRef,
                                                            str as *const ::string::String,
                                                            from)
      }
    }
  }
  impl<'largs> StringRefLastIndexOfArgs<'largs>
    for (&'largs ::string::String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QStringRef_lastIndexOf_QString_int_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, str as *const ::string::String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::locale_aware_compare](../struct.StringRef.html#method.locale_aware_compare) method.
  pub trait StringRefLocaleAwareCompareArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int;
  }
  impl<'largs> StringRefLocaleAwareCompareArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_localeAwareCompare_QString(original_self as *const ::string_ref::StringRef,
                                                               s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefLocaleAwareCompareArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_localeAwareCompare_QStringRef(original_self as *const ::string_ref::StringRef,
                                                                  s as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::locale_aware_compare_static](../struct.StringRef.html#method.locale_aware_compare_static) method.
  pub trait StringRefLocaleAwareCompareStaticArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl<'a> StringRefLocaleAwareCompareStaticArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_localeAwareCompare_QStringRef_QString(s1 as *const ::string_ref::StringRef,
                                                                          s2 as *const ::string::String)
      }
    }
  }
  impl<'a> StringRefLocaleAwareCompareStaticArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_localeAwareCompare_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                             s2 as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::mid](../struct.StringRef.html#method.mid) method.
  pub trait StringRefMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::string_ref::StringRef;
  }
  impl<'largs> StringRefMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::string_ref::StringRef {
      let pos = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_mid_to_output_pos(original_self as *const ::string_ref::StringRef,
                                                        pos,
                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRefMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::string_ref::StringRef {
      let pos = self.0;
      let n = self.1;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_mid_to_output_pos_n(original_self as *const ::string_ref::StringRef,
                                                          pos,
                                                          n,
                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::new](../struct.StringRef.html#method.new) method.
  pub trait StringRefNewArgs {
    fn exec(self) -> ::string_ref::StringRef;
  }
  impl StringRefNewArgs for () {
    fn exec(self) -> ::string_ref::StringRef {

      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StringRefNewArgs for &'a ::string_ref::StringRef {
    fn exec(self) -> ::string_ref::StringRef {
      let other = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_constructor_other(other as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::new_unsafe](../struct.StringRef.html#method.new_unsafe) method.
  pub trait StringRefNewUnsafeArgs {
    unsafe fn exec(self) -> ::string_ref::StringRef;
  }
  impl StringRefNewUnsafeArgs for *const ::string::String {
    unsafe fn exec(self) -> ::string_ref::StringRef {
      let string = self;
      {
        let mut object: ::string_ref::StringRef =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QStringRef_constructor_string(string, &mut object);
        object
      }
    }
  }
  impl StringRefNewUnsafeArgs for (*const ::string::String, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::string_ref::StringRef {
      let string = self.0;
      let position = self.1;
      let size = self.2;
      {
        let mut object: ::string_ref::StringRef =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QStringRef_constructor_string_position_size(string, position, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::split](../struct.StringRef.html#method.split) method.
  pub trait StringRefSplitArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef;
  }
  impl<'largs> StringRefSplitArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef {
      let sep = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_split_to_output_QChar(original_self as *const ::string_ref::StringRef,
                                                            sep as *const ::char::Char,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRefSplitArgs<'largs> for (&'largs ::char::Char, &'largs ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_split_to_output_QChar_QString_SplitBehavior(original_self as *const ::string_ref::StringRef, sep as *const ::char::Char, behavior as *const ::string::SplitBehavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRefSplitArgs<'largs>
    for (&'largs ::char::Char, &'largs ::string::SplitBehavior, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      let cs = self.2;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_split_to_output_QChar_QString_SplitBehavior_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, sep as *const ::char::Char, behavior as *const ::string::SplitBehavior, cs as *const ::qt::CaseSensitivity, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRefSplitArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef {
      let sep = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_split_to_output_QString(original_self as *const ::string_ref::StringRef,
                                                              sep as *const ::string::String,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRefSplitArgs<'largs> for (&'largs ::string::String, &'largs ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_split_to_output_QString_QString_SplitBehavior(original_self as *const ::string_ref::StringRef, sep as *const ::string::String, behavior as *const ::string::SplitBehavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRefSplitArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::SplitBehavior, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      let cs = self.2;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringRef_split_to_output_QString_QString_SplitBehavior_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, sep as *const ::string::String, behavior as *const ::string::SplitBehavior, cs as *const ::qt::CaseSensitivity, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::starts_with](../struct.StringRef.html#method.starts_with) method.
  pub trait StringRefStartsWithArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool;
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_startsWith_QChar(original_self as *const ::string_ref::StringRef,
                                                     c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringRef_startsWith_QChar_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, c as *const ::char::Char, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_startsWith_QLatin1String(original_self as *const ::string_ref::StringRef,
                                                             s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_startsWith_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_startsWith_QString(original_self as *const ::string_ref::StringRef,
                                                       s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QStringRef_startsWith_QStringRef(original_self as *const ::string_ref::StringRef,
                                                          c as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_startsWith_QStringRef_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, c as *const ::string_ref::StringRef, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringRefStartsWithArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string_ref::StringRef) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QStringRef_startsWith_QString_Qt_CaseSensitivity(original_self as *const ::string_ref::StringRef, s as *const ::string::String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_int_unsafe](../struct.StringRef.html#method.to_int_unsafe) method.
  pub trait StringRefToIntUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int;
  }
  impl<'largs> StringRefToIntUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toInt_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToIntUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_int {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toInt_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_long_long_unsafe](../struct.StringRef.html#method.to_long_long_unsafe) method.
  pub trait StringRefToLongLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> i64;
  }
  impl<'largs> StringRefToLongLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> i64 {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toLongLong_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToLongLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> i64 {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toLongLong_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_long_unsafe](../struct.StringRef.html#method.to_long_unsafe) method.
  pub trait StringRefToLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_long;
  }
  impl<'largs> StringRefToLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_long {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toLong_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_long {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toLong_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_short_unsafe](../struct.StringRef.html#method.to_short_unsafe) method.
  pub trait StringRefToShortUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_short;
  }
  impl<'largs> StringRefToShortUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_short {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toShort_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToShortUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_short {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toShort_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_u_int_unsafe](../struct.StringRef.html#method.to_u_int_unsafe) method.
  pub trait StringRefToUIntUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_uint;
  }
  impl<'largs> StringRefToUIntUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_uint {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toUInt_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToUIntUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_uint {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toUInt_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_u_long_long_unsafe](../struct.StringRef.html#method.to_u_long_long_unsafe) method.
  pub trait StringRefToULongLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> u64;
  }
  impl<'largs> StringRefToULongLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> u64 {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toULongLong_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToULongLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> u64 {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toULongLong_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_u_long_unsafe](../struct.StringRef.html#method.to_u_long_unsafe) method.
  pub trait StringRefToULongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_ulong;
  }
  impl<'largs> StringRefToULongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_ulong {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toULong_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToULongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_ulong {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toULong_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [StringRef::to_u_short_unsafe](../struct.StringRef.html#method.to_u_short_unsafe) method.
  pub trait StringRefToUShortUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_ushort;
  }
  impl<'largs> StringRefToUShortUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_ushort {
      let ok = self;
      ::ffi::qt_core_c_QStringRef_toUShort_ok(original_self as *const ::string_ref::StringRef, ok)
    }
  }
  impl<'largs> StringRefToUShortUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_ref::StringRef) -> ::libc::c_ushort {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QStringRef_toUShort_ok_base(original_self as *const ::string_ref::StringRef, ok, base)
    }
  }
}
