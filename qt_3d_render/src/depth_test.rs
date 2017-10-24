/// C++ type: <span style='color: green;'>```Qt3DRender::QDepthTest::DepthFunction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DepthFunction {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QDepthTest```</span>
#[repr(C)]
pub struct DepthTest(u8);

impl DepthTest {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QDepthTest::DepthFunction Qt3DRender::QDepthTest::depthFunction() const```</span>
  ///
  ///
  pub fn depth_function(&self) -> ::depth_test::DepthFunction {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_depthFunction(self as *const ::depth_test::DepthTest) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QDepthTest::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_metaObject(self as *const ::depth_test::DepthTest) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QDepthTest::QDepthTest()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::depth_test::DepthTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QDepthTest::QDepthTest(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::depth_test::DepthTest> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QDepthTest::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_qt_metacall(self as *mut ::depth_test::DepthTest,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QDepthTest::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_qt_metacast(self as *mut ::depth_test::DepthTest, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QDepthTest::setDepthFunction(Qt3DRender::QDepthTest::DepthFunction depthFunction)```</span>
  ///
  ///
  pub fn set_depth_function(&mut self, depth_function: ::depth_test::DepthFunction) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_setDepthFunction(self as *mut ::depth_test::DepthTest, depth_function)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QDepthTest::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QDepthTest::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::depth_test::DepthTest {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QDepthTest_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DepthTest`.
  pub struct Signals<'a>(&'a ::depth_test::DepthTest);
  /// Represents a built-in Qt signal `Qt3DRender::QDepthTest::depthFunctionChanged`.
  ///
  /// An object of this type can be created from `DepthTest` with `object.signals().depth_function_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DepthTest` object.
  pub struct DepthFunctionChanged<'a>(&'a ::depth_test::DepthTest);
  impl<'a> ::qt_core::connection::Receiver for DepthFunctionChanged<'a> {
    type Arguments = (::depth_test::DepthFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2depthFunctionChanged(Qt3DRender::QDepthTest::DepthFunction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DepthFunctionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDepthTest::depthFunctionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_function_changed(&self) -> DepthFunctionChanged {
      DepthFunctionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DepthTest`.
  pub struct Slots<'a>(&'a ::depth_test::DepthTest);
  /// Represents a built-in Qt slot `Qt3DRender::QDepthTest::setDepthFunction`.
  ///
  /// An object of this type can be created from `DepthTest` with `object.slots().set_depth_function()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DepthTest` object.
  pub struct SetDepthFunction<'a>(&'a ::depth_test::DepthTest);
  impl<'a> ::qt_core::connection::Receiver for SetDepthFunction<'a> {
    type Arguments = (::depth_test::DepthFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepthFunction(Qt3DRender::QDepthTest::DepthFunction)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDepthTest::setDepthFunction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth_function(&self) -> SetDepthFunction {
      SetDepthFunction(self.0)
    }
  }
  impl ::depth_test::DepthTest {
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

impl ::cpp_utils::DynamicCast<::depth_test::DepthTest> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::depth_test::DepthTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_dynamic_cast_Qt3DRender_QDepthTest_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::depth_test::DepthTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_dynamic_cast_Qt3DRender_QDepthTest_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::depth_test::DepthTest {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::depth_test::DepthTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::depth_test::DepthTest as *mut ::depth_test::DepthTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::depth_test::DepthTest {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::depth_test::DepthTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::depth_test::DepthTest as *mut ::depth_test::DepthTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::depth_test::DepthTest {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_QObject_ptr(self as *mut ::depth_test::DepthTest) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_QObject_ptr(self as *const ::depth_test::DepthTest as *mut ::depth_test::DepthTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::depth_test::DepthTest> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::depth_test::DepthTest {
    let ffi_result = ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::depth_test::DepthTest {
    let ffi_result = ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::depth_test::DepthTest> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::depth_test::DepthTest {
    let ffi_result = ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::depth_test::DepthTest {
    let ffi_result = ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::depth_test::DepthTest> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::depth_test::DepthTest {
    let ffi_result = ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::depth_test::DepthTest {
    let ffi_result = ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QDepthTest_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::depth_test::DepthTest {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::depth_test::DepthTest as *mut ::depth_test::DepthTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::depth_test::DepthTest {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QDepthTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::depth_test::DepthTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
