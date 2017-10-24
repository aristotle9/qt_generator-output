/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilTestArguments::StencilFaceMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StencilFaceMode {
  /// C++ enum variant: <span style='color: green;'>```Front = 1028```</span>
  Front = 1028,
  /// C++ enum variant: <span style='color: green;'>```Back = 1029```</span>
  Back = 1029,
  /// C++ enum variant: <span style='color: green;'>```FrontAndBack = 1032```</span>
  FrontAndBack = 1032,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilTestArguments::StencilFunction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StencilFunction {
  /// C++ enum variant: <span style='color: green;'>```Never = 512```</span>
  Never = 512,
  /// C++ enum variant: <span style='color: green;'>```Less = 513```</span>
  Less = 513,
  /// C++ enum variant: <span style='color: green;'>```Equal = 514```</span>
  Equal = 514,
  /// C++ enum variant: <span style='color: green;'>```LessOrEqual = 515```</span>
  LessOrEqual = 515,
  /// C++ enum variant: <span style='color: green;'>```Greater = 516```</span>
  Greater = 516,
  /// C++ enum variant: <span style='color: green;'>```NotEqual = 517```</span>
  NotEqual = 517,
  /// C++ enum variant: <span style='color: green;'>```GreaterOrEqual = 518```</span>
  GreaterOrEqual = 518,
  /// C++ enum variant: <span style='color: green;'>```Always = 519```</span>
  Always = 519,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilTestArguments```</span>
#[repr(C)]
pub struct StencilTestArguments(u8);

impl StencilTestArguments {
  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QStencilTestArguments::comparisonMask() const```</span>
  ///
  ///
  pub fn comparison_mask(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_comparisonMask(self as *const ::stencil_test_arguments::StencilTestArguments) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilTestArguments::StencilFaceMode Qt3DRender::QStencilTestArguments::faceMode() const```</span>
  ///
  ///
  pub fn face_mode(&self) -> ::stencil_test_arguments::StencilFaceMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_faceMode(self as *const ::stencil_test_arguments::StencilTestArguments) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QStencilTestArguments::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_metaObject(self as *const ::stencil_test_arguments::StencilTestArguments) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QStencilTestArguments::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_qt_metacall(self as *mut ::stencil_test_arguments::StencilTestArguments, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QStencilTestArguments::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_qt_metacast(self as *mut ::stencil_test_arguments::StencilTestArguments, arg1)
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QStencilTestArguments::referenceValue() const```</span>
  ///
  ///
  pub fn reference_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_referenceValue(self as *const ::stencil_test_arguments::StencilTestArguments) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilTestArguments::setComparisonMask(unsigned int comparisonMask)```</span>
  ///
  ///
  pub fn set_comparison_mask(&mut self, comparison_mask: ::libc::c_uint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_setComparisonMask(self as *mut ::stencil_test_arguments::StencilTestArguments, comparison_mask) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilTestArguments::setReferenceValue(int referenceValue)```</span>
  ///
  ///
  pub fn set_reference_value(&mut self, reference_value: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_setReferenceValue(self as *mut ::stencil_test_arguments::StencilTestArguments, reference_value) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QStencilTestArguments::setStencilFunction(Qt3DRender::QStencilTestArguments::StencilFunction stencilFunction)```</span>
  ///
  ///
  pub fn set_stencil_function(&mut self, stencil_function: ::stencil_test_arguments::StencilFunction) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_setStencilFunction(self as *mut ::stencil_test_arguments::StencilTestArguments, stencil_function) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilTestArguments::StencilFunction Qt3DRender::QStencilTestArguments::stencilFunction() const```</span>
  ///
  ///
  pub fn stencil_function(&self) -> ::stencil_test_arguments::StencilFunction {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_stencilFunction(self as *const ::stencil_test_arguments::StencilTestArguments) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilTestArguments::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilTestArguments::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::stencil_test_arguments::StencilTestArguments {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilTestArguments_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StencilTestArguments`.
  pub struct Signals<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  /// Represents a built-in Qt signal `Qt3DRender::QStencilTestArguments::referenceValueChanged`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.signals().reference_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct ReferenceValueChanged<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for ReferenceValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2referenceValueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ReferenceValueChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilTestArguments::comparisonMaskChanged`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.signals().comparison_mask_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct ComparisonMaskChanged<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for ComparisonMaskChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2comparisonMaskChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ComparisonMaskChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilTestArguments::stencilFunctionChanged`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.signals().stencil_function_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct StencilFunctionChanged<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for StencilFunctionChanged<'a> {
    type Arguments = (::stencil_test_arguments::StencilFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stencilFunctionChanged(Qt3DRender::QStencilTestArguments::StencilFunction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StencilFunctionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilTestArguments::faceModeChanged`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.signals().face_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct FaceModeChanged<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for FaceModeChanged<'a> {
    type Arguments = (::stencil_test_arguments::StencilFaceMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2faceModeChanged(Qt3DRender::QStencilTestArguments::StencilFaceMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FaceModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QStencilTestArguments::objectNameChanged`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct ObjectNameChanged<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
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
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilTestArguments::referenceValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reference_value_changed(&self) -> ReferenceValueChanged {
      ReferenceValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilTestArguments::comparisonMaskChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn comparison_mask_changed(&self) -> ComparisonMaskChanged {
      ComparisonMaskChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilTestArguments::stencilFunctionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stencil_function_changed(&self) -> StencilFunctionChanged {
      StencilFunctionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilTestArguments::faceModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_mode_changed(&self) -> FaceModeChanged {
      FaceModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QStencilTestArguments::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StencilTestArguments`.
  pub struct Slots<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  /// Represents a built-in Qt slot `Qt3DRender::QStencilTestArguments::setStencilFunction`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.slots().set_stencil_function()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct SetStencilFunction<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for SetStencilFunction<'a> {
    type Arguments = (::stencil_test_arguments::StencilFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStencilFunction(Qt3DRender::QStencilTestArguments::StencilFunction)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QStencilTestArguments::setComparisonMask`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.slots().set_comparison_mask()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct SetComparisonMask<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for SetComparisonMask<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setComparisonMask(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QStencilTestArguments::setReferenceValue`.
  ///
  /// An object of this type can be created from `StencilTestArguments` with `object.slots().set_reference_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StencilTestArguments` object.
  pub struct SetReferenceValue<'a>(&'a ::stencil_test_arguments::StencilTestArguments);
  impl<'a> ::qt_core::connection::Receiver for SetReferenceValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setReferenceValue(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilTestArguments::setStencilFunction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_stencil_function(&self) -> SetStencilFunction {
      SetStencilFunction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilTestArguments::setComparisonMask`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_comparison_mask(&self) -> SetComparisonMask {
      SetComparisonMask(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QStencilTestArguments::setReferenceValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_reference_value(&self) -> SetReferenceValue {
      SetReferenceValue(self.0)
    }
  }
  impl ::stencil_test_arguments::StencilTestArguments {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::stencil_test_arguments::StencilTestArguments {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilTestArguments_G_static_cast_QObject_ptr(self as *mut ::stencil_test_arguments::StencilTestArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilTestArguments_G_static_cast_QObject_ptr(self as *const ::stencil_test_arguments::StencilTestArguments as *mut ::stencil_test_arguments::StencilTestArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_test_arguments::StencilTestArguments> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_test_arguments::StencilTestArguments {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilTestArguments_G_static_cast_Qt3DRender_QStencilTestArguments_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_test_arguments::StencilTestArguments {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilTestArguments_G_static_cast_Qt3DRender_QStencilTestArguments_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::stencil_test_arguments::StencilTestArguments {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilTestArguments_G_static_cast_QObject_ptr(self as *const ::stencil_test_arguments::StencilTestArguments as *mut ::stencil_test_arguments::StencilTestArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::stencil_test_arguments::StencilTestArguments {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilTestArguments_G_static_cast_QObject_ptr(self as *mut ::stencil_test_arguments::StencilTestArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
