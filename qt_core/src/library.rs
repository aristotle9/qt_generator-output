/// C++ type: <span style='color: green;'>```QLibrary```</span>
#[repr(C)]
pub struct Library(u8);

impl Library {
  /// C++ method: <span style='color: green;'>```QString QLibrary::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibrary_errorString_to_output(self as *const ::library::Library, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QLibrary::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibrary_fileName_to_output(self as *const ::library::Library, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QLibrary::isLibrary(const QString& fileName)```</span>
  ///
  ///
  pub fn is_library(file_name: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QLibrary_isLibrary(file_name as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QLibrary::isLoaded() const```</span>
  ///
  ///
  pub fn is_loaded(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLibrary_isLoaded(self as *const ::library::Library) }
  }

  /// C++ method: <span style='color: green;'>```bool QLibrary::load()```</span>
  ///
  ///
  pub fn load(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QLibrary_load(self as *mut ::library::Library) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QLibrary::LoadHint> QLibrary::loadHints() const```</span>
  ///
  ///
  pub fn load_hints(&self) -> ::flags::Flags<::library::LoadHint> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_loadHints(self as *const ::library::Library) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QLibrary::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QLibrary_metaObject(self as *const ::library::Library) }
  }

  /// C++ method: <span style='color: green;'>```QLibrary::QLibrary```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(const QString& fileName, const QString& version)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::string::String, ::libc::c_int)) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(const QString& fileName, int verNum)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::library::Library>
    where Args: overloading::LibraryNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLibrary::QLibrary```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(const QString& fileName, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, &::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(const QString& fileName, const QString& version, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, ::libc::c_int, *mut ::object::Object)) -> ::cpp_utils::CppBox<::library::Library>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLibrary::QLibrary(const QString& fileName, int verNum, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::library::Library>
    where Args: overloading::LibraryNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void (*FN_PTR)() QLibrary::resolve(const char* symbol)```</span>
  ///
  ///
  pub unsafe fn resolve(&mut self, symbol: *const ::libc::c_char) -> extern "C" fn() {
    ::ffi::qt_core_c_QLibrary_resolve_symbol(self as *mut ::library::Library, symbol)
  }

  /// C++ method: <span style='color: green;'>```QLibrary::resolve```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resolve_static((&::string::String, &::string::String, *const ::libc::c_char)) -> extern "C" fn()```<br>
  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)() QLibrary::resolve(const QString& fileName, const QString& version, const char* symbol)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resolve_static((&::string::String, *const ::libc::c_char)) -> extern "C" fn()```<br>
  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)() QLibrary::resolve(const QString& fileName, const char* symbol)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn resolve_static((&::string::String, ::libc::c_int, *const ::libc::c_char)) -> extern "C" fn()```<br>
  /// C++ method: <span style='color: green;'>```static void (*FN_PTR)() QLibrary::resolve(const QString& fileName, int verNum, const char* symbol)```</span>
  ///
  ///
  pub unsafe fn resolve_static<Args>(args: Args) -> extern "C" fn()
    where Args: overloading::LibraryResolveStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QLibrary::setFileName(const QString& fileName)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file_name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QLibrary_setFileName(self as *mut ::library::Library,
                                            file_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QLibrary::setFileNameAndVersion```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_file_name_and_version(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLibrary::setFileNameAndVersion(const QString& fileName, const QString& version)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_file_name_and_version(&mut self, (&::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLibrary::setFileNameAndVersion(const QString& fileName, int verNum)```</span>
  ///
  ///
  pub fn set_file_name_and_version<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LibrarySetFileNameAndVersionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QLibrary::setLoadHints(QFlags<QLibrary::LoadHint> hints)```</span>
  ///
  ///
  pub fn set_load_hints(&mut self, hints: ::flags::Flags<::library::LoadHint>) {
    unsafe {
      ::ffi::qt_core_c_QLibrary_setLoadHints(self as *mut ::library::Library,
                                             hints.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLibrary::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QLibrary_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLibrary::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QLibrary_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLibrary::unload()```</span>
  ///
  ///
  pub fn unload(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QLibrary_unload(self as *mut ::library::Library) }
  }
}

impl ::cpp_utils::CppDeletable for ::library::Library {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QLibrary_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Library`.
  pub struct Signals<'a>(&'a ::library::Library);
  /// Represents a built-in Qt signal `QLibrary::objectNameChanged`.
  ///
  /// An object of this type can be created from `Library` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Library` object.
  pub struct ObjectNameChanged<'a>(&'a ::library::Library);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QLibrary::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::library::Library {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QLibrary::LoadHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LoadHint {
  /// C++ enum variant: <span style='color: green;'>```ResolveAllSymbolsHint = 1```</span>
  ResolveAllSymbols = 1,
  /// C++ enum variant: <span style='color: green;'>```ExportExternalSymbolsHint = 2```</span>
  ExportExternalSymbols = 2,
  /// C++ enum variant: <span style='color: green;'>```LoadArchiveMemberHint = 4```</span>
  LoadArchiveMember = 4,
  /// C++ enum variant: <span style='color: green;'>```PreventUnloadHint = 8```</span>
  PreventUnload = 8,
  /// C++ enum variant: <span style='color: green;'>```DeepBindHint = 16```</span>
  DeepBind = 16,
}

impl ::flags::FlaggableEnum for LoadHint {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "LoadHint"
  }
}

impl ::cpp_utils::DynamicCast<::library::Library> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::library::Library> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_G_dynamic_cast_QLibrary_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::library::Library> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QLibrary_G_dynamic_cast_QLibrary_ptr(self as *const ::object::Object as *mut ::object::Object)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::library::Library {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_G_static_cast_QObject_ptr(self as *mut ::library::Library) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_G_static_cast_QObject_ptr(self as *const ::library::Library as *mut ::library::Library) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::library::Library> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::library::Library {
    let ffi_result = ::ffi::qt_core_c_QLibrary_G_static_cast_QLibrary_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::library::Library {
    let ffi_result =
      ::ffi::qt_core_c_QLibrary_G_static_cast_QLibrary_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::library::Library {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_G_static_cast_QObject_ptr(self as *const ::library::Library as *mut ::library::Library) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::library::Library {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_G_static_cast_QObject_ptr(self as *mut ::library::Library) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Library::new](../struct.Library.html#method.new) method.
  pub trait LibraryNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::library::Library>;
  }
  impl<'a> LibraryNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let file_name = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_new_fileName(file_name as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> LibraryNewArgs for (&'a ::string::String, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let file_name = self.0;
      let ver_num = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QLibrary_new_fileName_verNum(file_name as *const ::string::String, ver_num) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> LibraryNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let file_name = self.0;
      let version = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QLibrary_new_fileName_version(file_name as *const ::string::String,
                                                       version as *const ::string::String)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl LibraryNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QLibrary_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Library::new_unsafe](../struct.Library.html#method.new_unsafe) method.
  pub trait LibraryNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::library::Library>;
  }
  impl<'a> LibraryNewUnsafeArgs for (&'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let file_name = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QLibrary_new_fileName_parent(file_name as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> LibraryNewUnsafeArgs for (&'a ::string::String, ::libc::c_int, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let file_name = self.0;
      let ver_num = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_core_c_QLibrary_new_fileName_verNum_parent(file_name as *const ::string::String, ver_num, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> LibraryNewUnsafeArgs for (&'a ::string::String, &'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let file_name = self.0;
      let version = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_core_c_QLibrary_new_fileName_version_parent(file_name as *const ::string::String,
                                                                             version as *const ::string::String,
                                                                             parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl LibraryNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::library::Library> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QLibrary_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Library::resolve_static](../struct.Library.html#method.resolve_static) method.
  pub trait LibraryResolveStaticArgs {
    unsafe fn exec(self) -> extern "C" fn();
  }
  impl<'a> LibraryResolveStaticArgs for (&'a ::string::String, *const ::libc::c_char) {
    unsafe fn exec(self) -> extern "C" fn() {
      let file_name = self.0;
      let symbol = self.1;
      ::ffi::qt_core_c_QLibrary_resolve_fileName_symbol(file_name as *const ::string::String, symbol)
    }
  }
  impl<'a> LibraryResolveStaticArgs for (&'a ::string::String, ::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self) -> extern "C" fn() {
      let file_name = self.0;
      let ver_num = self.1;
      let symbol = self.2;
      ::ffi::qt_core_c_QLibrary_resolve_fileName_verNum_symbol(file_name as *const ::string::String, ver_num, symbol)
    }
  }
  impl<'a> LibraryResolveStaticArgs for (&'a ::string::String, &'a ::string::String, *const ::libc::c_char) {
    unsafe fn exec(self) -> extern "C" fn() {
      let file_name = self.0;
      let version = self.1;
      let symbol = self.2;
      ::ffi::qt_core_c_QLibrary_resolve_fileName_version_symbol(file_name as *const ::string::String,
                                                                version as *const ::string::String,
                                                                symbol)
    }
  }
  /// This trait represents a set of arguments accepted by [Library::set_file_name_and_version](../struct.Library.html#method.set_file_name_and_version) method.
  pub trait LibrarySetFileNameAndVersionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::library::Library) -> ();
  }
  impl<'largs> LibrarySetFileNameAndVersionArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::library::Library) -> () {
      let file_name = self.0;
      let ver_num = self.1;
      unsafe {
        ::ffi::qt_core_c_QLibrary_setFileNameAndVersion_fileName_verNum(original_self as *mut ::library::Library,
                                                                        file_name as *const ::string::String,
                                                                        ver_num)
      }
    }
  }
  impl<'largs> LibrarySetFileNameAndVersionArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::library::Library) -> () {
      let file_name = self.0;
      let version = self.1;
      unsafe {
        ::ffi::qt_core_c_QLibrary_setFileNameAndVersion_fileName_version(original_self as *mut ::library::Library,
                                                                         file_name as *const ::string::String,
                                                                         version as *const ::string::String)
      }
    }
  }
}
