/// C++ type: <span style='color: green;'>```Qt3DRender::QScissorTest```</span>
#[repr(C)]
pub struct ScissorTest(u8);

impl ScissorTest {
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QScissorTest::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_bottom(self as *const ::scissor_test::ScissorTest) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QScissorTest::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_height(self as *const ::scissor_test::ScissorTest) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QScissorTest::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_left(self as *const ::scissor_test::ScissorTest) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QScissorTest::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_metaObject(self as *const ::scissor_test::ScissorTest) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QScissorTest::QScissorTest()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::scissor_test::ScissorTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QScissorTest::QScissorTest(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::scissor_test::ScissorTest> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QScissorTest::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_qt_metacall(self as *mut ::scissor_test::ScissorTest,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QScissorTest::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_qt_metacast(self as *mut ::scissor_test::ScissorTest, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QScissorTest::setBottom(int bottom)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, bottom: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_setBottom(self as *mut ::scissor_test::ScissorTest, bottom) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QScissorTest::setHeight(int height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_setHeight(self as *mut ::scissor_test::ScissorTest, height) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QScissorTest::setLeft(int left)```</span>
  ///
  ///
  pub fn set_left(&mut self, left: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_setLeft(self as *mut ::scissor_test::ScissorTest, left) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QScissorTest::setWidth(int width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_setWidth(self as *mut ::scissor_test::ScissorTest, width) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QScissorTest::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QScissorTest::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QScissorTest::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_width(self as *const ::scissor_test::ScissorTest) }
  }
}

impl ::cpp_utils::CppDeletable for ::scissor_test::ScissorTest {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QScissorTest_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ScissorTest`.
  pub struct Signals<'a>(&'a ::scissor_test::ScissorTest);
  /// Represents a built-in Qt signal `Qt3DRender::QScissorTest::heightChanged`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct HeightChanged<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QScissorTest::leftChanged`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.signals().left_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct LeftChanged<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for LeftChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2leftChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LeftChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QScissorTest::widthChanged`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct WidthChanged<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QScissorTest::bottomChanged`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.signals().bottom_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct BottomChanged<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for BottomChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bottomChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BottomChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QScissorTest::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QScissorTest::leftChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn left_changed(&self) -> LeftChanged {
      LeftChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QScissorTest::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QScissorTest::bottomChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_changed(&self) -> BottomChanged {
      BottomChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ScissorTest`.
  pub struct Slots<'a>(&'a ::scissor_test::ScissorTest);
  /// Represents a built-in Qt slot `Qt3DRender::QScissorTest::setLeft`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.slots().set_left()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct SetLeft<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for SetLeft<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLeft(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QScissorTest::setBottom`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.slots().set_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct SetBottom<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for SetBottom<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBottom(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QScissorTest::setWidth`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct SetWidth<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QScissorTest::setHeight`.
  ///
  /// An object of this type can be created from `ScissorTest` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScissorTest` object.
  pub struct SetHeight<'a>(&'a ::scissor_test::ScissorTest);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QScissorTest::setLeft`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_left(&self) -> SetLeft {
      SetLeft(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QScissorTest::setBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bottom(&self) -> SetBottom {
      SetBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QScissorTest::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QScissorTest::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
  }
  impl ::scissor_test::ScissorTest {
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

impl ::cpp_utils::DynamicCast<::scissor_test::ScissorTest> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::scissor_test::ScissorTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_dynamic_cast_Qt3DRender_QScissorTest_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::scissor_test::ScissorTest> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_dynamic_cast_Qt3DRender_QScissorTest_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::scissor_test::ScissorTest {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::scissor_test::ScissorTest)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::scissor_test::ScissorTest as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::scissor_test::ScissorTest {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::scissor_test::ScissorTest as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::scissor_test::ScissorTest {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_QObject_ptr(self as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_QObject_ptr(self as *const ::scissor_test::ScissorTest as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scissor_test::ScissorTest> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scissor_test::ScissorTest {
    let ffi_result = ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QScissorTest_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scissor_test::ScissorTest {
    let ffi_result = ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QScissorTest_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scissor_test::ScissorTest> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scissor_test::ScissorTest {
    let ffi_result = ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QScissorTest_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scissor_test::ScissorTest {
    let ffi_result = ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QScissorTest_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scissor_test::ScissorTest> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scissor_test::ScissorTest {
    let ffi_result = ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QScissorTest_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scissor_test::ScissorTest {
    let ffi_result = ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QScissorTest_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scissor_test::ScissorTest {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::scissor_test::ScissorTest as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scissor_test::ScissorTest {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QScissorTest_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::scissor_test::ScissorTest) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
