/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureWrapMode```</span>
#[repr(C)]
pub struct TextureWrapMode(u8);

impl TextureWrapMode {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QTextureWrapMode::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_metaObject(self as *const ::texture_wrap_mode::TextureWrapMode)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureWrapMode::QTextureWrapMode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureWrapMode::QTextureWrapMode()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::texture_wrap_mode::WrapMode) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureWrapMode::QTextureWrapMode(Qt3DRender::QTextureWrapMode::WrapMode wrapMode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::texture_wrap_mode::WrapMode, ::texture_wrap_mode::WrapMode, ::texture_wrap_mode::WrapMode)) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureWrapMode::QTextureWrapMode(Qt3DRender::QTextureWrapMode::WrapMode x, Qt3DRender::QTextureWrapMode::WrapMode y, Qt3DRender::QTextureWrapMode::WrapMode z)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>
    where Args: overloading::TextureWrapModeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureWrapMode::QTextureWrapMode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((::texture_wrap_mode::WrapMode, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureWrapMode::QTextureWrapMode(Qt3DRender::QTextureWrapMode::WrapMode wrapMode = ?, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::texture_wrap_mode::WrapMode, ::texture_wrap_mode::WrapMode, ::texture_wrap_mode::WrapMode, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureWrapMode::QTextureWrapMode(Qt3DRender::QTextureWrapMode::WrapMode x, Qt3DRender::QTextureWrapMode::WrapMode y, Qt3DRender::QTextureWrapMode::WrapMode z, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>
    where Args: overloading::TextureWrapModeNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QTextureWrapMode::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_qt_metacall(self as *mut ::texture_wrap_mode::TextureWrapMode,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QTextureWrapMode::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_qt_metacast(self as *mut ::texture_wrap_mode::TextureWrapMode,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureWrapMode::setX(Qt3DRender::QTextureWrapMode::WrapMode x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::texture_wrap_mode::WrapMode) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_setX(self as *mut ::texture_wrap_mode::TextureWrapMode, x)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureWrapMode::setY(Qt3DRender::QTextureWrapMode::WrapMode y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::texture_wrap_mode::WrapMode) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_setY(self as *mut ::texture_wrap_mode::TextureWrapMode, y)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureWrapMode::setZ(Qt3DRender::QTextureWrapMode::WrapMode z)```</span>
  ///
  ///
  pub fn set_z(&mut self, z: ::texture_wrap_mode::WrapMode) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_setZ(self as *mut ::texture_wrap_mode::TextureWrapMode, z)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTextureWrapMode::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTextureWrapMode::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureWrapMode::WrapMode Qt3DRender::QTextureWrapMode::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::texture_wrap_mode::WrapMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_x(self as *const ::texture_wrap_mode::TextureWrapMode) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureWrapMode::WrapMode Qt3DRender::QTextureWrapMode::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::texture_wrap_mode::WrapMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_y(self as *const ::texture_wrap_mode::TextureWrapMode) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureWrapMode::WrapMode Qt3DRender::QTextureWrapMode::z() const```</span>
  ///
  ///
  pub fn z(&self) -> ::texture_wrap_mode::WrapMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_z(self as *const ::texture_wrap_mode::TextureWrapMode) }
  }
}

impl ::cpp_utils::CppDeletable for ::texture_wrap_mode::TextureWrapMode {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextureWrapMode`.
  pub struct Signals<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  /// Represents a built-in Qt signal `Qt3DRender::QTextureWrapMode::objectNameChanged`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct ObjectNameChanged<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
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
  /// Represents a built-in Qt signal `Qt3DRender::QTextureWrapMode::yChanged`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct YChanged<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  impl<'a> ::qt_core::connection::Receiver for YChanged<'a> {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yChanged(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureWrapMode::zChanged`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.signals().z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct ZChanged<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  impl<'a> ::qt_core::connection::Receiver for ZChanged<'a> {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2zChanged(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ZChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureWrapMode::xChanged`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct XChanged<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  impl<'a> ::qt_core::connection::Receiver for XChanged<'a> {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xChanged(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureWrapMode::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureWrapMode::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureWrapMode::zChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_changed(&self) -> ZChanged {
      ZChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureWrapMode::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextureWrapMode`.
  pub struct Slots<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  /// Represents a built-in Qt slot `Qt3DRender::QTextureWrapMode::setZ`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.slots().set_z()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct SetZ<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  impl<'a> ::qt_core::connection::Receiver for SetZ<'a> {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setZ(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureWrapMode::setX`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.slots().set_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct SetX<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  impl<'a> ::qt_core::connection::Receiver for SetX<'a> {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setX(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureWrapMode::setY`.
  ///
  /// An object of this type can be created from `TextureWrapMode` with `object.slots().set_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureWrapMode` object.
  pub struct SetY<'a>(&'a ::texture_wrap_mode::TextureWrapMode);
  impl<'a> ::qt_core::connection::Receiver for SetY<'a> {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setY(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureWrapMode::setZ`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_z(&self) -> SetZ {
      SetZ(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureWrapMode::setX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x(&self) -> SetX {
      SetX(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureWrapMode::setY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y(&self) -> SetY {
      SetY(self.0)
    }
  }
  impl ::texture_wrap_mode::TextureWrapMode {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureWrapMode::WrapMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WrapMode {
  /// C++ enum variant: <span style='color: green;'>```Repeat = 10497```</span>
  Repeat = 10497,
  /// C++ enum variant: <span style='color: green;'>```ClampToBorder = 33069```</span>
  ClampToBorder = 33069,
  /// C++ enum variant: <span style='color: green;'>```ClampToEdge = 33071```</span>
  ClampToEdge = 33071,
  /// C++ enum variant: <span style='color: green;'>```MirroredRepeat = 33648```</span>
  MirroredRepeat = 33648,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::texture_wrap_mode::TextureWrapMode {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureWrapMode_G_static_cast_QObject_ptr(self as *mut ::texture_wrap_mode::TextureWrapMode) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureWrapMode_G_static_cast_QObject_ptr(self as *const ::texture_wrap_mode::TextureWrapMode as *mut ::texture_wrap_mode::TextureWrapMode) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_wrap_mode::TextureWrapMode> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_wrap_mode::TextureWrapMode {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureWrapMode_G_static_cast_Qt3DRender_QTextureWrapMode_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_wrap_mode::TextureWrapMode {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureWrapMode_G_static_cast_Qt3DRender_QTextureWrapMode_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::texture_wrap_mode::TextureWrapMode {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureWrapMode_G_static_cast_QObject_ptr(self as *const ::texture_wrap_mode::TextureWrapMode as *mut ::texture_wrap_mode::TextureWrapMode) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::texture_wrap_mode::TextureWrapMode {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureWrapMode_G_static_cast_QObject_ptr(self as *mut ::texture_wrap_mode::TextureWrapMode) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextureWrapMode::new](../struct.TextureWrapMode.html#method.new) method.
  pub trait TextureWrapModeNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>;
  }
  impl TextureWrapModeNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode> {

      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TextureWrapModeNewArgs for ::texture_wrap_mode::WrapMode {
    fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode> {
      let wrap_mode = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_wrapMode(wrap_mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TextureWrapModeNewArgs
    for (::texture_wrap_mode::WrapMode, ::texture_wrap_mode::WrapMode, ::texture_wrap_mode::WrapMode) {
    fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode> {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_x_y_z(x, y, z) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextureWrapMode::new_unsafe](../struct.TextureWrapMode.html#method.new_unsafe) method.
  pub trait TextureWrapModeNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode>;
  }
  impl TextureWrapModeNewUnsafeArgs for (::texture_wrap_mode::WrapMode, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode> {
      let wrap_mode = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_wrapMode_parent(wrap_mode, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TextureWrapModeNewUnsafeArgs
    for (::texture_wrap_mode::WrapMode,
                                             ::texture_wrap_mode::WrapMode,
                                             ::texture_wrap_mode::WrapMode,
                                             *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::texture_wrap_mode::TextureWrapMode> {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      let parent = self.3;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_x_y_z_parent(x, y, z, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
