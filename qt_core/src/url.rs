/// C++ type: <span style='color: green;'>```QUrl::ComponentFormattingOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComponentFormattingOption {
  /// C++ enum variant: <span style='color: green;'>```PrettyDecoded = 0```</span>
  PrettyDecoded = 0,
  /// C++ enum variant: <span style='color: green;'>```EncodeSpaces = 1048576```</span>
  EncodeSpaces = 1048576,
  /// C++ enum variant: <span style='color: green;'>```EncodeUnicode = 2097152```</span>
  EncodeUnicode = 2097152,
  /// C++ enum variant: <span style='color: green;'>```EncodeDelimiters = 12582912```</span>
  EncodeDelimiters = 12582912,
  /// C++ enum variant: <span style='color: green;'>```EncodeReserved = 16777216```</span>
  EncodeReserved = 16777216,
  /// C++ enum variant: <span style='color: green;'>```FullyEncoded = 32505856```</span>
  FullyEncoded = 32505856,
  /// C++ enum variant: <span style='color: green;'>```DecodeReserved = 33554432```</span>
  DecodeReserved = 33554432,
  /// C++ enum variant: <span style='color: green;'>```FullyDecoded = 133169152```</span>
  FullyDecoded = 133169152,
}

impl ::flags::FlaggableEnum for ComponentFormattingOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ComponentFormattingOption"
  }
}

/// C++ type: <span style='color: green;'>```QUrl::ParsingMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ParsingMode {
  /// C++ enum variant: <span style='color: green;'>```TolerantMode = 0```</span>
  Tolerant = 0,
  /// C++ enum variant: <span style='color: green;'>```StrictMode = 1```</span>
  Strict = 1,
  /// C++ enum variant: <span style='color: green;'>```DecodedMode = 2```</span>
  Decoded = 2,
}

/// C++ type: <span style='color: green;'>```QUrl```</span>
#[repr(C)]
pub struct Url([u8; ::type_sizes::QT_CORE_URL_URL]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Url {
  unsafe fn new_uninitialized() -> Url {
    Url(::std::mem::uninitialized())
  }
}

impl Url {
  /// C++ method: <span style='color: green;'>```QUrl QUrl::adjusted(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options) const```</span>
  ///
  ///
  pub fn adjusted(&self,
                  options: &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
                  -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_adjusted_to_output(self as *const ::url::Url, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::authority```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn authority(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::authority() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn authority(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::authority(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn authority<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlAuthorityArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QUrl::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QUrl_clear(self as *mut ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```QString QUrl::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_errorString_to_output(self as *const ::url::Url, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::fileName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn file_name(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::fileName() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn file_name(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::fileName(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn file_name<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlFileNameArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::fragment```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fragment(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::fragment() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fragment(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::fragment(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn fragment<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlFragmentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QUrl::fromAce(const QByteArray& arg1)```</span>
  ///
  ///
  pub fn from_ace(arg1: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_fromAce_to_output(arg1 as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::fromEncoded```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_encoded(&::byte_array::ByteArray) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QUrl::fromEncoded(const QByteArray& url)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_encoded((&::byte_array::ByteArray, ::url::ParsingMode)) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QUrl::fromEncoded(const QByteArray& url, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn from_encoded<Args>(args: Args) -> ::url::Url
    where Args: overloading::UrlFromEncodedArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QUrl QUrl::fromLocalFile(const QString& localfile)```</span>
  ///
  ///
  pub fn from_local_file(localfile: &::string::String) -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_fromLocalFile_to_output(localfile as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QUrl::fromPercentEncoding(const QByteArray& arg1)```</span>
  ///
  ///
  pub fn from_percent_encoding(arg1: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_fromPercentEncoding_to_output(arg1 as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::fromStringList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_string_list(&::string_list::StringList) -> ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QUrl::fromStringList(const QStringList& uris)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_string_list((&::string_list::StringList, ::url::ParsingMode)) -> ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QUrl::fromStringList(const QStringList& uris, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn from_string_list<Args>(args: Args) -> ::list::ListUrl
    where Args: overloading::UrlFromStringListArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrl::fromUserInput```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_user_input(&::string::String) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QUrl::fromUserInput(const QString& userInput)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_user_input((&::string::String, &::string::String)) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QUrl::fromUserInput(const QString& userInput, const QString& workingDirectory)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_user_input((&::string::String, &::string::String, ::flags::Flags<::url::UserInputResolutionOption>)) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QUrl::fromUserInput(const QString& userInput, const QString& workingDirectory, QFlags<QUrl::UserInputResolutionOption> options = ?)```</span>
  ///
  ///
  pub fn from_user_input<Args>(args: Args) -> ::url::Url
    where Args: overloading::UrlFromUserInputArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QUrl::hasFragment() const```</span>
  ///
  ///
  pub fn has_fragment(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_hasFragment(self as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::hasQuery() const```</span>
  ///
  ///
  pub fn has_query(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_hasQuery(self as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```QUrl::host```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn host(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::host() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn host(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::host(QFlags<QUrl::ComponentFormattingOption> arg1 = ?) const```</span>
  ///
  ///
  pub fn host<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlHostArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QStringList QUrl::idnWhitelist()```</span>
  ///
  ///
  pub fn idn_whitelist() -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_idnWhitelist_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_isEmpty(self as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::isLocalFile() const```</span>
  ///
  ///
  pub fn is_local_file(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_isLocalFile(self as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::isParentOf(const QUrl& url) const```</span>
  ///
  ///
  pub fn is_parent_of(&self, url: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_isParentOf(self as *const ::url::Url, url as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::isRelative() const```</span>
  ///
  ///
  pub fn is_relative(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_isRelative(self as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_isValid(self as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::matches(const QUrl& url, QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options) const```</span>
  ///
  ///
  pub fn matches(&self,
                 url: &::url::Url,
                 options: &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)
                 -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_matches(self as *const ::url::Url, url as *const ::url::Url, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) }
  }

  /// C++ method: <span style='color: green;'>```QUrl::QUrl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrl::QUrl()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrl::QUrl(const QString& url)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, ::url::ParsingMode)) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrl::QUrl(const QString& url, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::url::Url) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUrl::QUrl(const QUrl& copy)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::url::Url
    where Args: overloading::UrlNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrl::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::string::String) -> &'l0 mut ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```QUrl& QUrl::operator=(const QString& url)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::url::Url) -> &'l0 mut ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```QUrl& QUrl::operator=(const QUrl& copy)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::url::Url
    where Args: overloading::UrlOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QUrl::operator==(const QUrl& url) const```</span>
  ///
  ///
  pub fn op_eq(&self, url: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_operator_eq(self as *const ::url::Url, url as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::operator<(const QUrl& url) const```</span>
  ///
  ///
  pub fn op_lt(&self, url: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_operator_lt(self as *const ::url::Url, url as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QUrl::operator!=(const QUrl& url) const```</span>
  ///
  ///
  pub fn op_neq(&self, url: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QUrl_operator_neq(self as *const ::url::Url, url as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```QUrl::password```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn password(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::password() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn password(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::password(QFlags<QUrl::ComponentFormattingOption> arg1 = ?) const```</span>
  ///
  ///
  pub fn password<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlPasswordArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::path```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn path(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::path() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn path(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::path(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn path<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlPathArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::port```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn port(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QUrl::port() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn port(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QUrl::port(int defaultPort = ?) const```</span>
  ///
  ///
  pub fn port<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::UrlPortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::query```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn query(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::query() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn query(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::query(QFlags<QUrl::ComponentFormattingOption> arg1 = ?) const```</span>
  ///
  ///
  pub fn query<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlQueryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl QUrl::resolved(const QUrl& relative) const```</span>
  ///
  ///
  pub fn resolved(&self, relative: &::url::Url) -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_resolved_to_output(self as *const ::url::Url,
                                                 relative as *const ::url::Url,
                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QUrl::scheme() const```</span>
  ///
  ///
  pub fn scheme(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_scheme_to_output(self as *const ::url::Url, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::setAuthority```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_authority(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setAuthority(const QString& authority)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_authority(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setAuthority(const QString& authority, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_authority<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetAuthorityArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::setFragment```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_fragment(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setFragment(const QString& fragment)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_fragment(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setFragment(const QString& fragment, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_fragment<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetFragmentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::setHost```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_host(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setHost(const QString& host)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_host(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setHost(const QString& host, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_host<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetHostArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static void QUrl::setIdnWhitelist(const QStringList& arg1)```</span>
  ///
  ///
  pub fn set_idn_whitelist(arg1: &::string_list::StringList) {
    unsafe { ::ffi::qt_core_c_QUrl_setIdnWhitelist(arg1 as *const ::string_list::StringList) }
  }

  /// C++ method: <span style='color: green;'>```QUrl::setPassword```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_password(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setPassword(const QString& password)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_password(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setPassword(const QString& password, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_password<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetPasswordArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::setPath```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_path(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setPath(const QString& path)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_path(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setPath(const QString& path, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_path<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetPathArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QUrl::setPort(int port)```</span>
  ///
  ///
  pub fn set_port(&mut self, port: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QUrl_setPort(self as *mut ::url::Url, port) }
  }

  /// C++ method: <span style='color: green;'>```QUrl::setQuery```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_query(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setQuery(const QString& query)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_query(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setQuery(const QString& query, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_query(&mut self, &::url_query::UrlQuery) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setQuery(const QUrlQuery& query)```</span>
  ///
  ///
  pub fn set_query<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetQueryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QUrl::setScheme(const QString& scheme)```</span>
  ///
  ///
  pub fn set_scheme(&mut self, scheme: &::string::String) {
    unsafe { ::ffi::qt_core_c_QUrl_setScheme(self as *mut ::url::Url, scheme as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QUrl::setUrl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_url(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setUrl(const QString& url)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_url(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setUrl(const QString& url, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_url<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetUrlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::setUserInfo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_user_info(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setUserInfo(const QString& userInfo)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_user_info(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setUserInfo(const QString& userInfo, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_user_info<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetUserInfoArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::setUserName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_user_name(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setUserName(const QString& userName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_user_name(&mut self, (&::string::String, ::url::ParsingMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QUrl::setUserName(const QString& userName, QUrl::ParsingMode mode = ?)```</span>
  ///
  ///
  pub fn set_user_name<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::UrlSetUserNameArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QUrl::swap(QUrl& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::url::Url) {
    unsafe { ::ffi::qt_core_c_QUrl_swap(self as *mut ::url::Url, other as *mut ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QUrl::toAce(const QString& arg1)```</span>
  ///
  ///
  pub fn to_ace(arg1: &::string::String) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_toAce_to_output(arg1 as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::toDisplayString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_display_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::toDisplayString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_display_string(&self, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::toDisplayString(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn to_display_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlToDisplayStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::toEncoded```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_encoded(&self, ()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QUrl::toEncoded() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_encoded(&self, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QUrl::toEncoded(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn to_encoded<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::UrlToEncodedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QUrl::toLocalFile() const```</span>
  ///
  ///
  pub fn to_local_file(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUrl_toLocalFile_to_output(self as *const ::url::Url, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl::toPercentEncoding```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_percent_encoding(&::string::String) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QUrl::toPercentEncoding(const QString& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_percent_encoding((&::string::String, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QUrl::toPercentEncoding(const QString& arg1, const QByteArray& exclude = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_percent_encoding((&::string::String, &::byte_array::ByteArray, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QUrl::toPercentEncoding(const QString& arg1, const QByteArray& exclude = ?, const QByteArray& include = ?)```</span>
  ///
  ///
  pub fn to_percent_encoding<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::UrlToPercentEncodingArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrl::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::toString(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlToStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::toStringList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string_list(&::list::ListUrl) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QUrl::toStringList(const QList<QUrl>& uris)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string_list((&::list::ListUrl, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QUrl::toStringList(const QList<QUrl>& uris, QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options = ?)```</span>
  ///
  ///
  pub fn to_string_list<Args>(args: Args) -> ::string_list::StringList
    where Args: overloading::UrlToStringListArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUrl::topLevelDomain```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn top_level_domain(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::topLevelDomain() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn top_level_domain(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::topLevelDomain(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn top_level_domain<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlTopLevelDomainArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::url```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn url(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::url() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn url(&self, &::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::url(QUrlTwoFlags<QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn url<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlUrlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::userInfo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn user_info(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::userInfo() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn user_info(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::userInfo(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn user_info<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlUserInfoArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl::userName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn user_name(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::userName() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn user_name(&self, ::flags::Flags<::url::ComponentFormattingOption>) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QUrl::userName(QFlags<QUrl::ComponentFormattingOption> options = ?) const```</span>
  ///
  ///
  pub fn user_name<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::UrlUserNameArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::url::Url {
  /// C++ method: <span style='color: green;'>```[destructor] void QUrl::~QUrl()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QUrl_destructor(self as *mut ::url::Url) }
  }
}

/// C++ type: <span style='color: green;'>```QUrl::UrlFormattingOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UrlFormattingOption {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```RemoveScheme = 1```</span>
  RemoveScheme = 1,
  /// C++ enum variant: <span style='color: green;'>```RemovePassword = 2```</span>
  RemovePassword = 2,
  /// C++ enum variant: <span style='color: green;'>```RemoveUserInfo = 6```</span>
  RemoveUserInfo = 6,
  /// C++ enum variant: <span style='color: green;'>```RemovePort = 8```</span>
  RemovePort = 8,
  /// C++ enum variant: <span style='color: green;'>```RemoveAuthority = 30```</span>
  RemoveAuthority = 30,
  /// C++ enum variant: <span style='color: green;'>```RemovePath = 32```</span>
  RemovePath = 32,
  /// C++ enum variant: <span style='color: green;'>```RemoveQuery = 64```</span>
  RemoveQuery = 64,
  /// C++ enum variant: <span style='color: green;'>```RemoveFragment = 128```</span>
  RemoveFragment = 128,
  /// C++ enum variant: <span style='color: green;'>```PreferLocalFile = 512```</span>
  PreferLocalFile = 512,
  /// C++ enum variant: <span style='color: green;'>```StripTrailingSlash = 1024```</span>
  StripTrailingSlash = 1024,
  /// C++ enum variant: <span style='color: green;'>```RemoveFilename = 2048```</span>
  RemoveFilename = 2048,
  /// C++ enum variant: <span style='color: green;'>```NormalizePathSegments = 4096```</span>
  NormalizePathSegments = 4096,
}

impl ::flags::FlaggableEnum for UrlFormattingOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "UrlFormattingOption"
  }
}

/// C++ type: <span style='color: green;'>```QUrl::UserInputResolutionOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UserInputResolutionOption {
  /// C++ enum variant: <span style='color: green;'>```DefaultResolution = 0```</span>
  DefaultResolution = 0,
  /// C++ enum variant: <span style='color: green;'>```AssumeLocalFile = 1```</span>
  AssumeLocalFile = 1,
}

impl ::flags::FlaggableEnum for UserInputResolutionOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "UserInputResolutionOption"
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
/// Rust arguments: ```fn hash(&::url::Url) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QUrl& url)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::url::Url, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QUrl& url, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::url::Url)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QUrl& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::url::Url)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QUrl& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QUrl& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 mut ::url::Url)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QUrl_G_operator_shr(arg1 as *mut ::data_stream::DataStream,
                                         arg2 as *mut ::url::Url)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QUrl& value1, QUrl& value2)```</span>
///
///
pub fn swap(value1: &mut ::url::Url, value2: &mut ::url::Url) {
  unsafe { ::ffi::qt_core_c_QUrl_G_swap(value1 as *mut ::url::Url, value2 as *mut ::url::Url) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Url::authority](../struct.Url.html#method.authority) method.
  pub trait UrlAuthorityArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlAuthorityArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_authority_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlAuthorityArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_authority_to_output_options(original_self as *const ::url::Url,
                                                            options.to_int() as ::libc::c_uint,
                                                            &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::file_name](../struct.Url.html#method.file_name) method.
  pub trait UrlFileNameArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlFileNameArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fileName_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlFileNameArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fileName_to_output_options(original_self as *const ::url::Url,
                                                           options.to_int() as ::libc::c_uint,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::fragment](../struct.Url.html#method.fragment) method.
  pub trait UrlFragmentArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlFragmentArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fragment_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlFragmentArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fragment_to_output_options(original_self as *const ::url::Url,
                                                           options.to_int() as ::libc::c_uint,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::from_encoded](../struct.Url.html#method.from_encoded) method.
  pub trait UrlFromEncodedArgs {
    fn exec(self) -> ::url::Url;
  }
  impl<'a> UrlFromEncodedArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::url::Url {
      let url = self;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromEncoded_to_output_url(url as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlFromEncodedArgs for (&'a ::byte_array::ByteArray, ::url::ParsingMode) {
    fn exec(self) -> ::url::Url {
      let url = self.0;
      let mode = self.1;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromEncoded_to_output_url_mode(url as *const ::byte_array::ByteArray,
                                                               mode,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::from_string_list](../struct.Url.html#method.from_string_list) method.
  pub trait UrlFromStringListArgs {
    fn exec(self) -> ::list::ListUrl;
  }
  impl<'a> UrlFromStringListArgs for &'a ::string_list::StringList {
    fn exec(self) -> ::list::ListUrl {
      let uris = self;
      {
        let mut object: ::list::ListUrl =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromStringList_to_output_uris(uris as *const ::string_list::StringList, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlFromStringListArgs for (&'a ::string_list::StringList, ::url::ParsingMode) {
    fn exec(self) -> ::list::ListUrl {
      let uris = self.0;
      let mode = self.1;
      {
        let mut object: ::list::ListUrl =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromStringList_to_output_uris_mode(uris as *const ::string_list::StringList,
                                                                   mode,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::from_user_input](../struct.Url.html#method.from_user_input) method.
  pub trait UrlFromUserInputArgs {
    fn exec(self) -> ::url::Url;
  }
  impl<'a> UrlFromUserInputArgs for &'a ::string::String {
    fn exec(self) -> ::url::Url {
      let user_input = self;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromUserInput_to_output_userInput(user_input as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlFromUserInputArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::url::Url {
      let user_input = self.0;
      let working_directory = self.1;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromUserInput_to_output_userInput_workingDirectory(user_input as *const ::string::String, working_directory as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlFromUserInputArgs
    for (&'a ::string::String, &'a ::string::String, ::flags::Flags<::url::UserInputResolutionOption>) {
    fn exec(self) -> ::url::Url {
      let user_input = self.0;
      let working_directory = self.1;
      let options = self.2;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_fromUserInput_to_output_userInput_workingDirectory_options(user_input as *const ::string::String, working_directory as *const ::string::String, options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::host](../struct.Url.html#method.host) method.
  pub trait UrlHostArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlHostArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_host_to_output_arg1(original_self as *const ::url::Url,
                                                    arg1.to_int() as ::libc::c_uint,
                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlHostArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_host_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::new](../struct.Url.html#method.new) method.
  pub trait UrlNewArgs {
    fn exec(self) -> ::url::Url;
  }
  impl<'a> UrlNewArgs for &'a ::url::Url {
    fn exec(self) -> ::url::Url {
      let copy = self;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_constructor_copy(copy as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl UrlNewArgs for () {
    fn exec(self) -> ::url::Url {

      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlNewArgs for &'a ::string::String {
    fn exec(self) -> ::url::Url {
      let url = self;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_constructor_url(url as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlNewArgs for (&'a ::string::String, ::url::ParsingMode) {
    fn exec(self) -> ::url::Url {
      let url = self.0;
      let mode = self.1;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_constructor_url_mode(url as *const ::string::String, mode, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::op_assign](../struct.Url.html#method.op_assign) method.
  pub trait UrlOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> &'largs mut ::url::Url;
  }
  impl<'largs> UrlOpAssignArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs mut ::url::Url) -> &'largs mut ::url::Url {
      let copy = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QUrl_operator_assign_copy(original_self as *mut ::url::Url, copy as *const ::url::Url)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> UrlOpAssignArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> &'largs mut ::url::Url {
      let url = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QUrl_operator_assign_url(original_self as *mut ::url::Url,
                                                  url as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Url::password](../struct.Url.html#method.password) method.
  pub trait UrlPasswordArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlPasswordArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_password_to_output_arg1(original_self as *const ::url::Url,
                                                        arg1.to_int() as ::libc::c_uint,
                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlPasswordArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_password_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::path](../struct.Url.html#method.path) method.
  pub trait UrlPathArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlPathArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_path_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlPathArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_path_to_output_options(original_self as *const ::url::Url,
                                                       options.to_int() as ::libc::c_uint,
                                                       &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::port](../struct.Url.html#method.port) method.
  pub trait UrlPortArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::libc::c_int;
  }
  impl<'largs> UrlPortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::url::Url) -> ::libc::c_int {
      let default_port = self;
      unsafe { ::ffi::qt_core_c_QUrl_port_defaultPort(original_self as *const ::url::Url, default_port) }
    }
  }
  impl<'largs> UrlPortArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QUrl_port_no_args(original_self as *const ::url::Url) }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::query](../struct.Url.html#method.query) method.
  pub trait UrlQueryArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlQueryArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_query_to_output_arg1(original_self as *const ::url::Url,
                                                     arg1.to_int() as ::libc::c_uint,
                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlQueryArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_query_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_authority](../struct.Url.html#method.set_authority) method.
  pub trait UrlSetAuthorityArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetAuthorityArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let authority = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setAuthority_authority(original_self as *mut ::url::Url,
                                                     authority as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetAuthorityArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let authority = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setAuthority_authority_mode(original_self as *mut ::url::Url,
                                                          authority as *const ::string::String,
                                                          mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_fragment](../struct.Url.html#method.set_fragment) method.
  pub trait UrlSetFragmentArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetFragmentArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let fragment = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setFragment_fragment(original_self as *mut ::url::Url,
                                                   fragment as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetFragmentArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let fragment = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setFragment_fragment_mode(original_self as *mut ::url::Url,
                                                        fragment as *const ::string::String,
                                                        mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_host](../struct.Url.html#method.set_host) method.
  pub trait UrlSetHostArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetHostArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let host = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setHost_host(original_self as *mut ::url::Url,
                                           host as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetHostArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let host = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setHost_host_mode(original_self as *mut ::url::Url,
                                                host as *const ::string::String,
                                                mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_password](../struct.Url.html#method.set_password) method.
  pub trait UrlSetPasswordArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetPasswordArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let password = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setPassword_password(original_self as *mut ::url::Url,
                                                   password as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetPasswordArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let password = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setPassword_password_mode(original_self as *mut ::url::Url,
                                                        password as *const ::string::String,
                                                        mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_path](../struct.Url.html#method.set_path) method.
  pub trait UrlSetPathArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetPathArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let path = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setPath_path(original_self as *mut ::url::Url,
                                           path as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetPathArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let path = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setPath_path_mode(original_self as *mut ::url::Url,
                                                path as *const ::string::String,
                                                mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_query](../struct.Url.html#method.set_query) method.
  pub trait UrlSetQueryArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetQueryArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let query = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setQuery_QString(original_self as *mut ::url::Url,
                                               query as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetQueryArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let query = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setQuery_QString_QUrl_ParsingMode(original_self as *mut ::url::Url,
                                                                query as *const ::string::String,
                                                                mode)
      }
    }
  }
  impl<'largs> UrlSetQueryArgs<'largs> for &'largs ::url_query::UrlQuery {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let query = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setQuery_QUrlQuery(original_self as *mut ::url::Url,
                                                 query as *const ::url_query::UrlQuery)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_url](../struct.Url.html#method.set_url) method.
  pub trait UrlSetUrlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetUrlArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let url = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setUrl_url(original_self as *mut ::url::Url,
                                         url as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetUrlArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let url = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setUrl_url_mode(original_self as *mut ::url::Url,
                                              url as *const ::string::String,
                                              mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_user_info](../struct.Url.html#method.set_user_info) method.
  pub trait UrlSetUserInfoArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetUserInfoArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let user_info = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setUserInfo_userInfo(original_self as *mut ::url::Url,
                                                   user_info as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetUserInfoArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let user_info = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setUserInfo_userInfo_mode(original_self as *mut ::url::Url,
                                                        user_info as *const ::string::String,
                                                        mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::set_user_name](../struct.Url.html#method.set_user_name) method.
  pub trait UrlSetUserNameArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::url::Url) -> ();
  }
  impl<'largs> UrlSetUserNameArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let user_name = self;
      unsafe {
        ::ffi::qt_core_c_QUrl_setUserName_userName(original_self as *mut ::url::Url,
                                                   user_name as *const ::string::String)
      }
    }
  }
  impl<'largs> UrlSetUserNameArgs<'largs> for (&'largs ::string::String, ::url::ParsingMode) {
    fn exec(self, original_self: &'largs mut ::url::Url) -> () {
      let user_name = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QUrl_setUserName_userName_mode(original_self as *mut ::url::Url,
                                                        user_name as *const ::string::String,
                                                        mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::to_display_string](../struct.Url.html#method.to_display_string) method.
  pub trait UrlToDisplayStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlToDisplayStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toDisplayString_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlToDisplayStringArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
    let options = self;
    {
let mut object: ::string::String = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrl_toDisplayString_to_output_options(original_self as *const ::url::Url, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [Url::to_encoded](../struct.Url.html#method.to_encoded) method.
  pub trait UrlToEncodedArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::byte_array::ByteArray;
  }
  impl<'largs> UrlToEncodedArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toEncoded_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlToEncodedArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url::Url) -> ::byte_array::ByteArray {
    let options = self;
    {
let mut object: ::byte_array::ByteArray = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrl_toEncoded_to_output_options(original_self as *const ::url::Url, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [Url::to_percent_encoding](../struct.Url.html#method.to_percent_encoding) method.
  pub trait UrlToPercentEncodingArgs {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> UrlToPercentEncodingArgs for &'a ::string::String {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toPercentEncoding_to_output_arg1(arg1 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlToPercentEncodingArgs for (&'a ::string::String, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let exclude = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toPercentEncoding_to_output_arg1_exclude(arg1 as *const ::string::String,
                                                                         exclude as *const ::byte_array::ByteArray,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlToPercentEncodingArgs for (&'a ::string::String, &'a ::byte_array::ByteArray, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let exclude = self.1;
      let include = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toPercentEncoding_to_output_arg1_exclude_include(arg1 as *const ::string::String, exclude as *const ::byte_array::ByteArray, include as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::to_string](../struct.Url.html#method.to_string) method.
  pub trait UrlToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toString_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlToStringArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
    let options = self;
    {
let mut object: ::string::String = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrl_toString_to_output_options(original_self as *const ::url::Url, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [Url::to_string_list](../struct.Url.html#method.to_string_list) method.
  pub trait UrlToStringListArgs {
    fn exec(self) -> ::string_list::StringList;
  }
  impl<'a> UrlToStringListArgs for &'a ::list::ListUrl {
    fn exec(self) -> ::string_list::StringList {
      let uris = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_toStringList_to_output_uris(uris as *const ::list::ListUrl, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UrlToStringListArgs for (&'a ::list::ListUrl,&'a ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption) {

  fn exec(self, ) -> ::string_list::StringList {
    let uris = self.0;
let options = self.1;
    {
let mut object: ::string_list::StringList = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrl_toStringList_to_output_uris_options(uris as *const ::list::ListUrl, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [Url::top_level_domain](../struct.Url.html#method.top_level_domain) method.
  pub trait UrlTopLevelDomainArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlTopLevelDomainArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_topLevelDomain_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlTopLevelDomainArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_topLevelDomain_to_output_options(original_self as *const ::url::Url,
                                                                 options.to_int() as ::libc::c_uint,
                                                                 &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::url](../struct.Url.html#method.url) method.
  pub trait UrlUrlArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlUrlArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_url_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlUrlArgs<'largs> for &'largs ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption {

  fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
    let options = self;
    {
let mut object: ::string::String = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QUrl_url_to_output_options(original_self as *const ::url::Url, options as *const ::url_two_flags::UrlTwoFlagsUrlUrlFormattingOptionUrlComponentFormattingOption, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [Url::user_info](../struct.Url.html#method.user_info) method.
  pub trait UrlUserInfoArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlUserInfoArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_userInfo_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlUserInfoArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_userInfo_to_output_options(original_self as *const ::url::Url,
                                                           options.to_int() as ::libc::c_uint,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Url::user_name](../struct.Url.html#method.user_name) method.
  pub trait UrlUserNameArgs<'largs> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String;
  }
  impl<'largs> UrlUserNameArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_userName_to_output_no_args(original_self as *const ::url::Url, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> UrlUserNameArgs<'largs> for ::flags::Flags<::url::ComponentFormattingOption> {
    fn exec(self, original_self: &'largs ::url::Url) -> ::string::String {
      let options = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_userName_to_output_options(original_self as *const ::url::Url,
                                                           options.to_int() as ::libc::c_uint,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::url::Url {
    fn exec(self) -> ::libc::c_uint {
      let url = self;
      unsafe { ::ffi::qt_core_c_QUrl_G_qHash_url(url as *const ::url::Url) }
    }
  }
  impl<'a> HashArgs for (&'a ::url::Url, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let url = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QUrl_G_qHash_url_seed(url as *const ::url::Url, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::url::Url) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QUrl_G_operator_shl(arg1 as *mut ::data_stream::DataStream,
                                             arg2 as *const ::url::Url)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::url::Url) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUrl_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                         arg2 as *const ::url::Url,
                                                         &mut object);
        }
        object
      }
    }
  }
}
