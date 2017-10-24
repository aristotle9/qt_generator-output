/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments::FaceMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FaceMode {
  /// C++ enum variant: <span style='color: green;'>```Front = 1028```</span>
  Front = 1028,
  /// C++ enum variant: <span style='color: green;'>```Back = 1029```</span>
  Back = 1029,
  /// C++ enum variant: <span style='color: green;'>```FrontAndBack = 1032```</span>
  FrontAndBack = 1032,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments::Operation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Operation {
  /// C++ enum variant: <span style='color: green;'>```Zero = 0```</span>
  Zero = 0,
  /// C++ enum variant: <span style='color: green;'>```Invert = 5386```</span>
  Invert = 5386,
  /// C++ enum variant: <span style='color: green;'>```Keep = 7680```</span>
  Keep = 7680,
  /// C++ enum variant: <span style='color: green;'>```Replace = 7681```</span>
  Replace = 7681,
  /// C++ enum variant: <span style='color: green;'>```Increment = 7682```</span>
  Increment = 7682,
  /// C++ enum variant: <span style='color: green;'>```Decrement = 7683```</span>
  Decrement = 7683,
  /// C++ enum variant: <span style='color: green;'>```IncrementWrap = 34055```</span>
  IncrementWrap = 34055,
  /// C++ enum variant: <span style='color: green;'>```DecrementWrap = 34056```</span>
  DecrementWrap = 34056,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments```</span>
#[repr(C)]
pub struct StencilOperationArguments(u8);

impl StencilOperationArguments {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments::Operation Qt3DRender::QStencilOperationArguments::allTestsPassOperation() const```</span>
  ///
  ///
  pub fn all_tests_pass_operation(&self) -> ::stencil_operation_arguments::Operation {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_allTestsPassOperation(self as *const ::stencil_operation_arguments::StencilOperationArguments) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments::Operation Qt3DRender::QStencilOperationArguments::depthTestFailureOperation() const```</span>
  ///
  ///
  pub fn depth_test_failure_operation(&self) -> ::stencil_operation_arguments::Operation {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_depthTestFailureOperation(self as *const ::stencil_operation_arguments::StencilOperationArguments) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments::FaceMode Qt3DRender::QStencilOperationArguments::faceMode() const```</span>
  ///
  ///
  pub fn face_mode(&self) -> ::stencil_operation_arguments::FaceMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_faceMode(self as *const ::stencil_operation_arguments::StencilOperationArguments) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QStencilOperationArguments::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_metaObject(self as *const ::stencil_operation_arguments::StencilOperationArguments) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QStencilOperationArguments::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_qt_metacall(self as *mut ::stencil_operation_arguments::StencilOperationArguments, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QStencilOperationArguments::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_qt_metacast(self as *mut ::stencil_operation_arguments::StencilOperationArguments, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilOperationArguments::setAllTestsPassOperation(Qt3DRender::QStencilOperationArguments::Operation operation)```</span>
  ///
  ///
  pub fn set_all_tests_pass_operation(&mut self, operation: ::stencil_operation_arguments::Operation) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_setAllTestsPassOperation(self as *mut ::stencil_operation_arguments::StencilOperationArguments, operation) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilOperationArguments::setDepthTestFailureOperation(Qt3DRender::QStencilOperationArguments::Operation operation)```</span>
  ///
  ///
  pub fn set_depth_test_failure_operation(&mut self, operation: ::stencil_operation_arguments::Operation) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_setDepthTestFailureOperation(self as *mut ::stencil_operation_arguments::StencilOperationArguments, operation) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilOperationArguments::setStencilTestFailureOperation(Qt3DRender::QStencilOperationArguments::Operation operation)```</span>
  ///
  ///
  pub fn set_stencil_test_failure_operation(&mut self, operation: ::stencil_operation_arguments::Operation) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_setStencilTestFailureOperation(self as *mut ::stencil_operation_arguments::StencilOperationArguments, operation) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments::Operation Qt3DRender::QStencilOperationArguments::stencilTestFailureOperation() const```</span>
  ///
  ///
  pub fn stencil_test_failure_operation(&self) -> ::stencil_operation_arguments::Operation {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_stencilTestFailureOperation(self as *const ::stencil_operation_arguments::StencilOperationArguments) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilOperationArguments::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilOperationArguments::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::stencil_operation_arguments::StencilOperationArguments {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperationArguments_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StencilOperationArguments`.
  pub struct Signals<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  /// Represents a built-in Qt signal `Qt3DRender::QStencilOperationArguments::depthTestFailureOperationChanged`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.signals().depth_test_failure_operation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct DepthTestFailureOperationChanged<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for DepthTestFailureOperationChanged<'a> {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2depthTestFailureOperationChanged(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DepthTestFailureOperationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilOperationArguments::stencilTestFailureOperationChanged`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.signals().stencil_test_failure_operation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct StencilTestFailureOperationChanged<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for StencilTestFailureOperationChanged<'a> {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stencilTestFailureOperationChanged(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StencilTestFailureOperationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilOperationArguments::objectNameChanged`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct ObjectNameChanged<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
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
  /// Represents a built-in Qt signal `Qt3DRender::QStencilOperationArguments::faceModeChanged`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.signals().face_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct FaceModeChanged<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for FaceModeChanged<'a> {
    type Arguments = (::stencil_operation_arguments::FaceMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2faceModeChanged(Qt3DRender::QStencilOperationArguments::FaceMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FaceModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilOperationArguments::allTestsPassOperationChanged`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.signals().all_tests_pass_operation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct AllTestsPassOperationChanged<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for AllTestsPassOperationChanged<'a> {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2allTestsPassOperationChanged(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AllTestsPassOperationChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilOperationArguments::depthTestFailureOperationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_test_failure_operation_changed(&self) -> DepthTestFailureOperationChanged {
      DepthTestFailureOperationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilOperationArguments::stencilTestFailureOperationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stencil_test_failure_operation_changed(&self) -> StencilTestFailureOperationChanged {
      StencilTestFailureOperationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilOperationArguments::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilOperationArguments::faceModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_mode_changed(&self) -> FaceModeChanged {
      FaceModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilOperationArguments::allTestsPassOperationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn all_tests_pass_operation_changed(&self) -> AllTestsPassOperationChanged {
      AllTestsPassOperationChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StencilOperationArguments`.
  pub struct Slots<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  /// Represents a built-in Qt slot `Qt3DRender::QStencilOperationArguments::setAllTestsPassOperation`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.slots().set_all_tests_pass_operation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct SetAllTestsPassOperation<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetAllTestsPassOperation<'a> {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAllTestsPassOperation(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QStencilOperationArguments::setDepthTestFailureOperation`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.slots().set_depth_test_failure_operation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct SetDepthTestFailureOperation<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetDepthTestFailureOperation<'a> {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepthTestFailureOperation(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QStencilOperationArguments::setStencilTestFailureOperation`.
  ///
  /// An object of this type can be created from `StencilOperationArguments` with `object.slots().set_stencil_test_failure_operation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilOperationArguments` object.
  pub struct SetStencilTestFailureOperation<'a>(&'a ::stencil_operation_arguments::StencilOperationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetStencilTestFailureOperation<'a> {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStencilTestFailureOperation(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilOperationArguments::setAllTestsPassOperation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_all_tests_pass_operation(&self) -> SetAllTestsPassOperation {
      SetAllTestsPassOperation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilOperationArguments::setDepthTestFailureOperation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth_test_failure_operation(&self) -> SetDepthTestFailureOperation {
      SetDepthTestFailureOperation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilOperationArguments::setStencilTestFailureOperation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_stencil_test_failure_operation(&self) -> SetStencilTestFailureOperation {
      SetStencilTestFailureOperation(self.0)
    }
  }
  impl ::stencil_operation_arguments::StencilOperationArguments {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::stencil_operation_arguments::StencilOperationArguments {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperationArguments_G_static_cast_QObject_ptr(self as *mut ::stencil_operation_arguments::StencilOperationArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperationArguments_G_static_cast_QObject_ptr(self as *const ::stencil_operation_arguments::StencilOperationArguments as *mut ::stencil_operation_arguments::StencilOperationArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_operation_arguments::StencilOperationArguments> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_operation_arguments::StencilOperationArguments {
let ffi_result = ::ffi::qt_3d_render_c_QStencilOperationArguments_G_static_cast_Qt3DRender_QStencilOperationArguments_ptr(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::stencil_operation_arguments::StencilOperationArguments {
let ffi_result = ::ffi::qt_3d_render_c_QStencilOperationArguments_G_static_cast_Qt3DRender_QStencilOperationArguments_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::stencil_operation_arguments::StencilOperationArguments {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperationArguments_G_static_cast_QObject_ptr(self as *const ::stencil_operation_arguments::StencilOperationArguments as *mut ::stencil_operation_arguments::StencilOperationArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::stencil_operation_arguments::StencilOperationArguments {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperationArguments_G_static_cast_QObject_ptr(self as *mut ::stencil_operation_arguments::StencilOperationArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
