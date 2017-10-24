/// C++ type: <span style='color: green;'>```Qt3DRender::QColorMask```</span>
#[repr(C)]
pub struct ColorMask(u8);

impl ColorMask {
  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QColorMask::isAlphaMasked() const```</span>
  ///
  ///
  pub fn is_alpha_masked(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_isAlphaMasked(self as *const ::color_mask::ColorMask) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QColorMask::isBlueMasked() const```</span>
  ///
  ///
  pub fn is_blue_masked(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_isBlueMasked(self as *const ::color_mask::ColorMask) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QColorMask::isGreenMasked() const```</span>
  ///
  ///
  pub fn is_green_masked(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_isGreenMasked(self as *const ::color_mask::ColorMask) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QColorMask::isRedMasked() const```</span>
  ///
  ///
  pub fn is_red_masked(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_isRedMasked(self as *const ::color_mask::ColorMask) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QColorMask::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_metaObject(self as *const ::color_mask::ColorMask) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QColorMask::QColorMask()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::color_mask::ColorMask> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QColorMask::QColorMask(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::color_mask::ColorMask> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QColorMask::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_qt_metacall(self as *mut ::color_mask::ColorMask,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QColorMask::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_qt_metacast(self as *mut ::color_mask::ColorMask, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QColorMask::setAlphaMasked(bool alphaMasked)```</span>
  ///
  ///
  pub fn set_alpha_masked(&mut self, alpha_masked: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_setAlphaMasked(self as *mut ::color_mask::ColorMask, alpha_masked)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QColorMask::setBlueMasked(bool blueMasked)```</span>
  ///
  ///
  pub fn set_blue_masked(&mut self, blue_masked: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_setBlueMasked(self as *mut ::color_mask::ColorMask, blue_masked)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QColorMask::setGreenMasked(bool greenMasked)```</span>
  ///
  ///
  pub fn set_green_masked(&mut self, green_masked: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_setGreenMasked(self as *mut ::color_mask::ColorMask, green_masked)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QColorMask::setRedMasked(bool redMasked)```</span>
  ///
  ///
  pub fn set_red_masked(&mut self, red_masked: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_setRedMasked(self as *mut ::color_mask::ColorMask, red_masked)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QColorMask::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QColorMask::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::color_mask::ColorMask {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QColorMask_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ColorMask`.
  pub struct Signals<'a>(&'a ::color_mask::ColorMask);
  /// Represents a built-in Qt signal `Qt3DRender::QColorMask::greenMaskedChanged`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.signals().green_masked_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct GreenMaskedChanged<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for GreenMaskedChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2greenMaskedChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GreenMaskedChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QColorMask::alphaMaskedChanged`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.signals().alpha_masked_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct AlphaMaskedChanged<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for AlphaMaskedChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2alphaMaskedChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AlphaMaskedChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QColorMask::redMaskedChanged`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.signals().red_masked_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct RedMaskedChanged<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for RedMaskedChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2redMaskedChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RedMaskedChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QColorMask::blueMaskedChanged`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.signals().blue_masked_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct BlueMaskedChanged<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for BlueMaskedChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2blueMaskedChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BlueMaskedChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QColorMask::greenMaskedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn green_masked_changed(&self) -> GreenMaskedChanged {
      GreenMaskedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QColorMask::alphaMaskedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alpha_masked_changed(&self) -> AlphaMaskedChanged {
      AlphaMaskedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QColorMask::redMaskedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn red_masked_changed(&self) -> RedMaskedChanged {
      RedMaskedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QColorMask::blueMaskedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn blue_masked_changed(&self) -> BlueMaskedChanged {
      BlueMaskedChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ColorMask`.
  pub struct Slots<'a>(&'a ::color_mask::ColorMask);
  /// Represents a built-in Qt slot `Qt3DRender::QColorMask::setBlueMasked`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.slots().set_blue_masked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct SetBlueMasked<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for SetBlueMasked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBlueMasked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QColorMask::setRedMasked`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.slots().set_red_masked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct SetRedMasked<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for SetRedMasked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRedMasked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QColorMask::setAlphaMasked`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.slots().set_alpha_masked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct SetAlphaMasked<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for SetAlphaMasked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAlphaMasked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QColorMask::setGreenMasked`.
  ///
  /// An object of this type can be created from `ColorMask` with `object.slots().set_green_masked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorMask` object.
  pub struct SetGreenMasked<'a>(&'a ::color_mask::ColorMask);
  impl<'a> ::qt_core::connection::Receiver for SetGreenMasked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGreenMasked(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QColorMask::setBlueMasked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_blue_masked(&self) -> SetBlueMasked {
      SetBlueMasked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QColorMask::setRedMasked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_red_masked(&self) -> SetRedMasked {
      SetRedMasked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QColorMask::setAlphaMasked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_alpha_masked(&self) -> SetAlphaMasked {
      SetAlphaMasked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QColorMask::setGreenMasked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_green_masked(&self) -> SetGreenMasked {
      SetGreenMasked(self.0)
    }
  }
  impl ::color_mask::ColorMask {
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

impl ::cpp_utils::DynamicCast<::color_mask::ColorMask> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::color_mask::ColorMask> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QColorMask_G_dynamic_cast_Qt3DRender_QColorMask_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::color_mask::ColorMask> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QColorMask_G_dynamic_cast_Qt3DRender_QColorMask_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::color_mask::ColorMask {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::color_mask::ColorMask)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::color_mask::ColorMask as *mut ::color_mask::ColorMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::color_mask::ColorMask {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::color_mask::ColorMask)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::color_mask::ColorMask as *mut ::color_mask::ColorMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::color_mask::ColorMask {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QColorMask_G_static_cast_QObject_ptr(self as *mut ::color_mask::ColorMask) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QColorMask_G_static_cast_QObject_ptr(self as *const ::color_mask::ColorMask as *mut ::color_mask::ColorMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_mask::ColorMask> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_mask::ColorMask {
    let ffi_result = ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_mask::ColorMask {
    let ffi_result = ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_mask::ColorMask> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_mask::ColorMask {
    let ffi_result = ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_mask::ColorMask {
    let ffi_result = ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_mask::ColorMask> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_mask::ColorMask {
    let ffi_result = ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_mask::ColorMask {
    let ffi_result = ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::color_mask::ColorMask {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::color_mask::ColorMask as *mut ::color_mask::ColorMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::color_mask::ColorMask {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::color_mask::ColorMask)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
