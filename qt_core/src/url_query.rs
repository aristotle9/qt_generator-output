/// C++ type: <span style='color: green;'>```QUrlQuery```</span>
#[repr(C)]
pub struct UrlQuery([u8; ::type_sizes::QT_CORE_URL_QUERY_URL_QUERY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for UrlQuery {
  unsafe fn new_uninitialized() -> UrlQuery {
    UrlQuery(::std::mem::uninitialized())
  }
}

impl UrlQuery {
  /// C++ method: <span style='color: green;'>```void QUrlQuery::addQueryItem(const QString& key, const QString& value)```</span>
  ///
  ///
  pub fn add_query_item(&mut self, key: &::string::String, value: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_addQueryItem(self as *mut ::url_query::UrlQuery,
                                              key as *const ::string::String,
                                              value as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QUrlQuery_clear(self as *mut ::url_query::UrlQuery) }
  }

  /// C++ method: <span style='color: green;'>```static QChar QUrlQuery::defaultQueryPairDelimiter()```</span>
  ///
  ///
  pub fn default_query_pair_delimiter() -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrlQuery_defaultQueryPairDelimiter_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QChar QUrlQuery::defaultQueryValueDelimiter()```</span>
  ///
  ///
  pub fn default_query_value_delimiter() -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrlQuery_defaultQueryValueDelimiter_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUrlQuery::hasQueryItem(const QString& key) const```</span>
  ///
  ///
  pub fn has_query_item(&self, key: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_hasQueryItem(self as *const ::url_query::UrlQuery,
                                              key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUrlQuery::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrlQuery_isEmpty(self as *const ::url_query::UrlQuery) }
  }

  /// C++ method: <span style='color: green;'>```QUrlQuery::QUrlQuery```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::url_query::UrlQuery```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlQuery::QUrlQuery()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::url_query::UrlQuery```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlQuery::QUrlQuery(const QString& queryString)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::url::Url) -> ::url_query::UrlQuery```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlQuery::QUrlQuery(const QUrl& url)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::url_query::UrlQuery) -> ::url_query::UrlQuery```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrlQuery::QUrlQuery(const QUrlQuery& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::url_query::UrlQuery
    where Args: overloading::UrlQueryNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrlQuery& QUrlQuery::operator=(const QUrlQuery& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::url_query::UrlQuery) -> &'l0 mut ::url_query::UrlQuery {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QUrlQuery_operator_assign(self as *mut ::url_query::UrlQuery,
                                                 other as *const ::url_query::UrlQuery)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QUrlQuery::operator==(const QUrlQuery& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::url_query::UrlQuery) -> bool {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_operator_eq(self as *const ::url_query::UrlQuery,
                                             other as *const ::url_query::UrlQuery)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUrlQuery::operator!=(const QUrlQuery& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::url_query::UrlQuery) -> bool {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_operator_neq(self as *const ::url_query::UrlQuery,
                                              other as *const ::url_query::UrlQuery)
    }
  }

  /// C++ method: <span style='color: green;'>```QChar QUrlQuery::queryPairDelimiter() const```</span>
  ///
  ///
  pub fn query_pair_delimiter(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrlQuery_queryPairDelimiter_to_output(self as *const ::url_query::UrlQuery, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QChar QUrlQuery::queryValueDelimiter() const```</span>
  ///
  ///
  pub fn query_value_delimiter(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrlQuery_queryValueDelimiter_to_output(self as *const ::url_query::UrlQuery, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::removeAllQueryItems(const QString& key)```</span>
  ///
  ///
  pub fn remove_all_query_items(&mut self, key: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_removeAllQueryItems(self as *mut ::url_query::UrlQuery,
                                                     key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::removeQueryItem(const QString& key)```</span>
  ///
  ///
  pub fn remove_query_item(&mut self, key: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_removeQueryItem(self as *mut ::url_query::UrlQuery,
                                                 key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::setQuery(const QString& queryString)```</span>
  ///
  ///
  pub fn set_query(&mut self, query_string: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_setQuery(self as *mut ::url_query::UrlQuery,
                                          query_string as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter)```</span>
  ///
  ///
  pub fn set_query_delimiters(&mut self, value_delimiter: &::char::Char, pair_delimiter: &::char::Char) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_setQueryDelimiters(self as *mut ::url_query::UrlQuery,
                                                    value_delimiter as *const ::char::Char,
                                                    pair_delimiter as *const ::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::setQueryItems(const QList<QPair<QString, QString>>& query)```</span>
  ///
  ///
  pub fn set_query_items(&mut self, query: &::list::ListPairPairStringString) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_setQueryItems(self as *mut ::url_query::UrlQuery,
                                               query as *const ::list::ListPairPairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```void QUrlQuery::swap(QUrlQuery& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::url_query::UrlQuery) {
    unsafe {
      ::ffi::qt_core_c_QUrlQuery_swap(self as *mut ::url_query::UrlQuery,
                                      other as *mut ::url_query::UrlQuery)
    }
  }
}

impl Drop for ::url_query::UrlQuery {
  /// C++ method: <span style='color: green;'>```[destructor] void QUrlQuery::~QUrlQuery()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QUrlQuery_destructor(self as *mut ::url_query::UrlQuery) }
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
/// Rust arguments: ```fn hash(&::url_query::UrlQuery) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QUrlQuery& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::url_query::UrlQuery, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QUrlQuery& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void swap(QUrlQuery& value1, QUrlQuery& value2)```</span>
///
///
pub fn swap(value1: &mut ::url_query::UrlQuery, value2: &mut ::url_query::UrlQuery) {
  unsafe {
    ::ffi::qt_core_c_QUrlQuery_G_swap(value1 as *mut ::url_query::UrlQuery,
                                      value2 as *mut ::url_query::UrlQuery)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [UrlQuery::new](../struct.UrlQuery.html#method.new) method.
  pub trait UrlQueryNewArgs {
    fn exec(self) -> ::url_query::UrlQuery;
  }
  impl UrlQueryNewArgs for () {
    fn exec(self) -> ::url_query::UrlQuery {

      {
        let mut object: ::url_query::UrlQuery =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlQuery_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlQueryNewArgs for &'a ::url_query::UrlQuery {
    fn exec(self) -> ::url_query::UrlQuery {
      let other = self;
      {
        let mut object: ::url_query::UrlQuery =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlQuery_constructor_other(other as *const ::url_query::UrlQuery, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlQueryNewArgs for &'a ::string::String {
    fn exec(self) -> ::url_query::UrlQuery {
      let query_string = self;
      {
        let mut object: ::url_query::UrlQuery =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlQuery_constructor_queryString(query_string as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlQueryNewArgs for &'a ::url::Url {
    fn exec(self) -> ::url_query::UrlQuery {
      let url = self;
      {
        let mut object: ::url_query::UrlQuery =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrlQuery_constructor_url(url as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::url_query::UrlQuery {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QUrlQuery_G_qHash_key(key as *const ::url_query::UrlQuery) }
    }
  }
  impl<'a> HashArgs for (&'a ::url_query::UrlQuery, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QUrlQuery_G_qHash_key_seed(key as *const ::url_query::UrlQuery, seed) }
    }
  }
}
