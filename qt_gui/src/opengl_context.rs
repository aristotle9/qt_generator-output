/// C++ type: <span style='color: green;'>```QOpenGLContext```</span>
#[repr(C)]
pub struct OpenGLContext(u8);

impl OpenGLContext {
  /// C++ method: <span style='color: green;'>```static bool QOpenGLContext::areSharing(QOpenGLContext* first, QOpenGLContext* second)```</span>
  ///
  ///
  pub unsafe fn are_sharing(first: *mut ::opengl_context::OpenGLContext,
                            second: *mut ::opengl_context::OpenGLContext)
                            -> bool {
    ::ffi::qt_gui_c_QOpenGLContext_areSharing(first, second)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLContext::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_create(self as *mut ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```static QOpenGLContext* QOpenGLContext::currentContext()```</span>
  ///
  ///
  pub fn current_context() -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_currentContext() }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLContext::defaultFramebufferObject() const```</span>
  ///
  ///
  pub fn default_framebuffer_object(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_defaultFramebufferObject(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLContext::doneCurrent()```</span>
  ///
  ///
  pub fn done_current(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_doneCurrent(self as *mut ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray> QOpenGLContext::extensions() const```</span>
  ///
  ///
  pub fn extensions(&self) -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLContext_extensions_to_output(self as *const ::opengl_context::OpenGLContext,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLExtraFunctions* QOpenGLContext::extraFunctions() const```</span>
  ///
  ///
  pub fn extra_functions(&self) -> *mut ::opengl_extra_functions::OpenGLExtraFunctions {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_extraFunctions(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat QOpenGLContext::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLContext_format_to_output(self as *const ::opengl_context::OpenGLContext, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLFunctions* QOpenGLContext::functions() const```</span>
  ///
  ///
  pub fn functions(&self) -> *mut ::opengl_functions::OpenGLFunctions {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_functions(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```void (*FN_PTR)() QOpenGLContext::getProcAddress(const QByteArray& procName) const```</span>
  ///
  ///
  pub fn get_proc_address(&self, proc_name: &::qt_core::byte_array::ByteArray) -> extern "C" fn() {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLContext_getProcAddress_QByteArray(self as *const ::opengl_context::OpenGLContext,
                                                               proc_name as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void (*FN_PTR)() QOpenGLContext::getProcAddress(const char* procName) const```</span>
  ///
  ///
  pub unsafe fn get_proc_address_unsafe(&self, proc_name: *const ::libc::c_char) -> extern "C" fn() {
    ::ffi::qt_gui_c_QOpenGLContext_getProcAddress_char(self as *const ::opengl_context::OpenGLContext, proc_name)
  }

  /// C++ method: <span style='color: green;'>```static QOpenGLContext* QOpenGLContext::globalShareContext()```</span>
  ///
  ///
  pub fn global_share_context() -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_globalShareContext() }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLContext::hasExtension(const QByteArray& extension) const```</span>
  ///
  ///
  pub fn has_extension(&self, extension: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLContext_hasExtension(self as *const ::opengl_context::OpenGLContext,
                                                  extension as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLContext::isOpenGLES() const```</span>
  ///
  ///
  pub fn is_opengl_e_s(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_isOpenGLES(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLContext::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_isValid(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLContext::makeCurrent(QSurface* surface)```</span>
  ///
  ///
  pub unsafe fn make_current(&mut self, surface: *mut ::surface::Surface) -> bool {
    ::ffi::qt_gui_c_QOpenGLContext_makeCurrent(self as *mut ::opengl_context::OpenGLContext, surface)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLContext::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_metaObject(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QOpenGLContext::nativeHandle() const```</span>
  ///
  ///
  pub fn native_handle(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLContext_nativeHandle_to_output(self as *const ::opengl_context::OpenGLContext,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLContext::QOpenGLContext()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_context::OpenGLContext> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContext_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLContext::QOpenGLContext(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::opengl_context::OpenGLContext> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLContext_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static void* QOpenGLContext::openGLModuleHandle()```</span>
  ///
  ///
  pub fn opengl_module_handle() -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_openGLModuleHandle() }
  }

  /// C++ method: <span style='color: green;'>```static QOpenGLContext::OpenGLModuleType QOpenGLContext::openGLModuleType()```</span>
  ///
  ///
  pub fn opengl_module_type() -> ::opengl_context::OpenGLModuleType {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_openGLModuleType() }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLContext::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLContext_qt_metacall(self as *mut ::opengl_context::OpenGLContext,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLContext::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLContext_qt_metacast(self as *mut ::opengl_context::OpenGLContext, arg1)
  }

  /// C++ method: <span style='color: green;'>```QScreen* QOpenGLContext::screen() const```</span>
  ///
  ///
  pub fn screen(&self) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_screen(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLContext::setFormat(const QSurfaceFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::surface_format::SurfaceFormat) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLContext_setFormat(self as *mut ::opengl_context::OpenGLContext,
                                               format as *const ::surface_format::SurfaceFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLContext::setNativeHandle(const QVariant& handle)```</span>
  ///
  ///
  pub fn set_native_handle(&mut self, handle: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLContext_setNativeHandle(self as *mut ::opengl_context::OpenGLContext,
                                                     handle as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLContext::setScreen(QScreen* screen)```</span>
  ///
  ///
  pub unsafe fn set_screen(&mut self, screen: *mut ::screen::Screen) {
    ::ffi::qt_gui_c_QOpenGLContext_setScreen(self as *mut ::opengl_context::OpenGLContext, screen)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLContext::setShareContext(QOpenGLContext* shareContext)```</span>
  ///
  ///
  pub unsafe fn set_share_context(&mut self, share_context: *mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QOpenGLContext_setShareContext(self as *mut ::opengl_context::OpenGLContext, share_context)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* QOpenGLContext::shareContext() const```</span>
  ///
  ///
  pub fn share_context(&self) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_shareContext(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContextGroup* QOpenGLContext::shareGroup() const```</span>
  ///
  ///
  pub fn share_group(&self) -> *mut ::opengl_context_group::OpenGLContextGroup {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_shareGroup(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLContext::supportsThreadedOpenGL()```</span>
  ///
  ///
  pub fn supports_threaded_opengl() -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_supportsThreadedOpenGL() }
  }

  /// C++ method: <span style='color: green;'>```QSurface* QOpenGLContext::surface() const```</span>
  ///
  ///
  pub fn surface(&self) -> *mut ::surface::Surface {
    unsafe { ::ffi::qt_gui_c_QOpenGLContext_surface(self as *const ::opengl_context::OpenGLContext) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLContext::swapBuffers(QSurface* surface)```</span>
  ///
  ///
  pub unsafe fn swap_buffers(&mut self, surface: *mut ::surface::Surface) {
    ::ffi::qt_gui_c_QOpenGLContext_swapBuffers(self as *mut ::opengl_context::OpenGLContext, surface)
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLContext::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLContext_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLContext::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLContext_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext::versionFunctions```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn version_functions(&self, ()) -> *mut ::opengl_version_functions::AbstractOpenGLFunctions```<br>
  /// C++ method: <span style='color: green;'>```QAbstractOpenGLFunctions* QOpenGLContext::versionFunctions() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn version_functions(&self, &::opengl_version_profile::OpenGLVersionProfile) -> *mut ::opengl_version_functions::AbstractOpenGLFunctions```<br>
  /// C++ method: <span style='color: green;'>```QAbstractOpenGLFunctions* QOpenGLContext::versionFunctions(const QOpenGLVersionProfile& versionProfile = ?) const```</span>
  ///
  ///
  pub fn version_functions<'largs, Args>(&'largs self,
                                         args: Args)
                                         -> *mut ::opengl_version_functions::AbstractOpenGLFunctions
    where Args: overloading::OpenGLContextVersionFunctionsArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_context::OpenGLContext {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLContext_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLContext`.
  pub struct Signals<'a>(&'a ::opengl_context::OpenGLContext);
  /// Represents a built-in Qt signal `QOpenGLContext::aboutToBeDestroyed`.
  ///
  /// An object of this type can be created from `OpenGLContext` with `object.signals().about_to_be_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLContext` object.
  pub struct AboutToBeDestroyed<'a>(&'a ::opengl_context::OpenGLContext);
  impl<'a> ::qt_core::connection::Receiver for AboutToBeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToBeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToBeDestroyed<'a> {}
  /// Represents a built-in Qt signal `QOpenGLContext::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLContext` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLContext` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_context::OpenGLContext);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QOpenGLContext::aboutToBeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_be_destroyed(&self) -> AboutToBeDestroyed {
      AboutToBeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLContext::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::opengl_context::OpenGLContext {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QOpenGLContext::OpenGLModuleType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OpenGLModuleType {
  /// C++ enum variant: <span style='color: green;'>```LibGL = 0```</span>
  LibGL = 0,
  /// C++ enum variant: <span style='color: green;'>```LibGLES = 1```</span>
  LibGLES = 1,
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::opengl_version_profile::OpenGLVersionProfile) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QOpenGLVersionProfile& v)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::opengl_version_profile::OpenGLVersionProfile, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QOpenGLVersionProfile& v, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator==(const QOpenGLVersionProfile& lhs, const QOpenGLVersionProfile& rhs)```</span>
///
///
pub fn op_eq(lhs: &::opengl_version_profile::OpenGLVersionProfile,
             rhs: &::opengl_version_profile::OpenGLVersionProfile)
             -> bool {
  unsafe {
    ::ffi::qt_gui_c_QOpenGLContext_G_operator_eq(lhs as *const ::opengl_version_profile::OpenGLVersionProfile,
                                                 rhs as *const ::opengl_version_profile::OpenGLVersionProfile)
  }
}

/// C++ method: <span style='color: green;'>```bool operator!=(const QOpenGLVersionProfile& lhs, const QOpenGLVersionProfile& rhs)```</span>
///
///
pub fn op_neq(lhs: &::opengl_version_profile::OpenGLVersionProfile,
              rhs: &::opengl_version_profile::OpenGLVersionProfile)
              -> bool {
  unsafe {
    ::ffi::qt_gui_c_QOpenGLContext_G_operator_neq(lhs as *const ::opengl_version_profile::OpenGLVersionProfile,
                                                  rhs as *const ::opengl_version_profile::OpenGLVersionProfile)
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_context::OpenGLContext {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLContext_G_static_cast_QObject_ptr(self as *mut ::opengl_context::OpenGLContext) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContext_G_static_cast_QObject_ptr(self as *const ::opengl_context::OpenGLContext as *mut ::opengl_context::OpenGLContext) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_context::OpenGLContext> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_context::OpenGLContext {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLContext_G_static_cast_QOpenGLContext_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_context::OpenGLContext {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLContext_G_static_cast_QOpenGLContext_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_context::OpenGLContext {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContext_G_static_cast_QObject_ptr(self as *const ::opengl_context::OpenGLContext as *mut ::opengl_context::OpenGLContext) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_context::OpenGLContext {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLContext_G_static_cast_QObject_ptr(self as *mut ::opengl_context::OpenGLContext) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLContext::version_functions](../struct.OpenGLContext.html#method.version_functions) method.
  pub trait OpenGLContextVersionFunctionsArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::opengl_context::OpenGLContext)
            -> *mut ::opengl_version_functions::AbstractOpenGLFunctions;
  }
  impl<'largs> OpenGLContextVersionFunctionsArgs<'largs> for () {
    fn exec(self,
            original_self: &'largs ::opengl_context::OpenGLContext)
            -> *mut ::opengl_version_functions::AbstractOpenGLFunctions {

      unsafe {
        ::ffi::qt_gui_c_QOpenGLContext_versionFunctions_no_args(original_self as *const ::opengl_context::OpenGLContext)
      }
    }
  }
  impl<'largs> OpenGLContextVersionFunctionsArgs<'largs> for &'largs ::opengl_version_profile::OpenGLVersionProfile {
    fn exec(self,
            original_self: &'largs ::opengl_context::OpenGLContext)
            -> *mut ::opengl_version_functions::AbstractOpenGLFunctions {
      let version_profile = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLContext_versionFunctions_versionProfile(original_self as *const ::opengl_context::OpenGLContext, version_profile as *const ::opengl_version_profile::OpenGLVersionProfile) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::opengl_version_profile::OpenGLVersionProfile {
    fn exec(self) -> ::libc::c_uint {
      let v = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLContext_G_qHash_v(v as *const ::opengl_version_profile::OpenGLVersionProfile) }
    }
  }
  impl<'a> HashArgs for (&'a ::opengl_version_profile::OpenGLVersionProfile, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let v = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLContext_G_qHash_v_seed(v as *const ::opengl_version_profile::OpenGLVersionProfile,
                                                      seed)
      }
    }
  }
}
