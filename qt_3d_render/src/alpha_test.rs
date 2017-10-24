/// C++ type: <span style='color: green;'>```Qt3DRender::QAlphaTest::AlphaFunction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AlphaFunction {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QAlphaTest```</span>
#[repr(C)]
pub struct AlphaTest(u8);

impl AlphaTest {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAlphaTest::AlphaFunction Qt3DRender::QAlphaTest::alphaFunction() const```</span>
  ///
  ///
  pub fn alpha_function(&self) -> ::alpha_test::AlphaFunction {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_alphaFunction(self as *const ::alpha_test::AlphaTest) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QAlphaTest::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_metaObject(self as *const ::alpha_test::AlphaTest) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAlphaTest::QAlphaTest()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::alpha_test::AlphaTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAlphaTest::QAlphaTest(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::alpha_test::AlphaTest> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QAlphaTest::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_qt_metacall(self as *mut ::alpha_test::AlphaTest,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QAlphaTest::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_qt_metacast(self as *mut ::alpha_test::AlphaTest, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QAlphaTest::referenceValue() const```</span>
  ///
  ///
  pub fn reference_value(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_referenceValue(self as *const ::alpha_test::AlphaTest) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAlphaTest::setAlphaFunction(Qt3DRender::QAlphaTest::AlphaFunction alphaFunction)```</span>
  ///
  ///
  pub fn set_alpha_function(&mut self, alpha_function: ::alpha_test::AlphaFunction) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_setAlphaFunction(self as *mut ::alpha_test::AlphaTest, alpha_function)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAlphaTest::setReferenceValue(float referenceValue)```</span>
  ///
  ///
  pub fn set_reference_value(&mut self, reference_value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_setReferenceValue(self as *mut ::alpha_test::AlphaTest,
                                                                    reference_value)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAlphaTest::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAlphaTest::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::alpha_test::AlphaTest {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QAlphaTest_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AlphaTest`.
  pub struct Signals<'a>(&'a ::alpha_test::AlphaTest);
  /// Represents a built-in Qt signal `Qt3DRender::QAlphaTest::alphaFunctionChanged`.
  ///
  /// An object of this type can be created from `AlphaTest` with `object.signals().alpha_function_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AlphaTest` object.
  pub struct AlphaFunctionChanged<'a>(&'a ::alpha_test::AlphaTest);
  impl<'a> ::qt_core::connection::Receiver for AlphaFunctionChanged<'a> {
    type Arguments = (::alpha_test::AlphaFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2alphaFunctionChanged(Qt3DRender::QAlphaTest::AlphaFunction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AlphaFunctionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAlphaTest::referenceValueChanged`.
  ///
  /// An object of this type can be created from `AlphaTest` with `object.signals().reference_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AlphaTest` object.
  pub struct ReferenceValueChanged<'a>(&'a ::alpha_test::AlphaTest);
  impl<'a> ::qt_core::connection::Receiver for ReferenceValueChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2referenceValueChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ReferenceValueChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAlphaTest::alphaFunctionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alpha_function_changed(&self) -> AlphaFunctionChanged {
      AlphaFunctionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAlphaTest::referenceValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reference_value_changed(&self) -> ReferenceValueChanged {
      ReferenceValueChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AlphaTest`.
  pub struct Slots<'a>(&'a ::alpha_test::AlphaTest);
  /// Represents a built-in Qt slot `Qt3DRender::QAlphaTest::setAlphaFunction`.
  ///
  /// An object of this type can be created from `AlphaTest` with `object.slots().set_alpha_function()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AlphaTest` object.
  pub struct SetAlphaFunction<'a>(&'a ::alpha_test::AlphaTest);
  impl<'a> ::qt_core::connection::Receiver for SetAlphaFunction<'a> {
    type Arguments = (::alpha_test::AlphaFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAlphaFunction(Qt3DRender::QAlphaTest::AlphaFunction)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAlphaTest::setReferenceValue`.
  ///
  /// An object of this type can be created from `AlphaTest` with `object.slots().set_reference_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AlphaTest` object.
  pub struct SetReferenceValue<'a>(&'a ::alpha_test::AlphaTest);
  impl<'a> ::qt_core::connection::Receiver for SetReferenceValue<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setReferenceValue(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAlphaTest::setAlphaFunction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_alpha_function(&self) -> SetAlphaFunction {
      SetAlphaFunction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAlphaTest::setReferenceValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_reference_value(&self) -> SetReferenceValue {
      SetReferenceValue(self.0)
    }
  }
  impl ::alpha_test::AlphaTest {
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

impl ::cpp_utils::DynamicCast<::alpha_test::AlphaTest> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::alpha_test::AlphaTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_dynamic_cast_Qt3DRender_QAlphaTest_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::alpha_test::AlphaTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_dynamic_cast_Qt3DRender_QAlphaTest_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::alpha_test::AlphaTest {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::alpha_test::AlphaTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::alpha_test::AlphaTest as *mut ::alpha_test::AlphaTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::alpha_test::AlphaTest {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::alpha_test::AlphaTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::alpha_test::AlphaTest as *mut ::alpha_test::AlphaTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::alpha_test::AlphaTest {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_QObject_ptr(self as *mut ::alpha_test::AlphaTest) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_QObject_ptr(self as *const ::alpha_test::AlphaTest as *mut ::alpha_test::AlphaTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::alpha_test::AlphaTest> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::alpha_test::AlphaTest {
    let ffi_result = ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::alpha_test::AlphaTest {
    let ffi_result = ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::alpha_test::AlphaTest> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::alpha_test::AlphaTest {
    let ffi_result = ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::alpha_test::AlphaTest {
    let ffi_result = ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::alpha_test::AlphaTest> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::alpha_test::AlphaTest {
    let ffi_result = ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::alpha_test::AlphaTest {
    let ffi_result = ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QAlphaTest_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::alpha_test::AlphaTest {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::alpha_test::AlphaTest as *mut ::alpha_test::AlphaTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::alpha_test::AlphaTest {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QAlphaTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::alpha_test::AlphaTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
