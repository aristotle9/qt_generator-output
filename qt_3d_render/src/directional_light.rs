/// C++ type: <span style='color: green;'>```Qt3DRender::QDirectionalLight```</span>
#[repr(C)]
pub struct DirectionalLight(u8);

impl DirectionalLight {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QDirectionalLight::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_metaObject(self as *const ::directional_light::DirectionalLight) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QDirectionalLight::QDirectionalLight()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::directional_light::DirectionalLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QDirectionalLight::QDirectionalLight(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::directional_light::DirectionalLight> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QDirectionalLight::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_qt_metacall(self as *mut ::directional_light::DirectionalLight,
                                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                                   arg2,
                                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QDirectionalLight::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_qt_metacast(self as *mut ::directional_light::DirectionalLight,
                                                                   arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QDirectionalLight::setWorldDirection(const QVector3D& worldDirection)```</span>
  ///
  ///
  pub fn set_world_direction(&mut self, world_direction: &::qt_gui::vector_3d::Vector3D) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_setWorldDirection(self as *mut ::directional_light::DirectionalLight, world_direction as *const ::qt_gui::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QDirectionalLight::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QDirectionalLight::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QDirectionalLight::worldDirection() const```</span>
  ///
  ///
  pub fn world_direction(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_worldDirection_as_ptr(self as *const ::directional_light::DirectionalLight) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::directional_light::DirectionalLight {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QDirectionalLight_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DirectionalLight`.
  pub struct Signals<'a>(&'a ::directional_light::DirectionalLight);
  /// Represents a built-in Qt signal `Qt3DRender::QDirectionalLight::worldDirectionChanged`.
  ///
  /// An object of this type can be created from `DirectionalLight` with `object.signals().world_direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirectionalLight` object.
  pub struct WorldDirectionChanged<'a>(&'a ::directional_light::DirectionalLight);
  impl<'a> ::qt_core::connection::Receiver for WorldDirectionChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2worldDirectionChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorldDirectionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QDirectionalLight::intensityChanged`.
  ///
  /// An object of this type can be created from `DirectionalLight` with `object.signals().intensity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirectionalLight` object.
  pub struct IntensityChanged<'a>(&'a ::directional_light::DirectionalLight);
  impl<'a> ::qt_core::connection::Receiver for IntensityChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2intensityChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IntensityChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QDirectionalLight::colorChanged`.
  ///
  /// An object of this type can be created from `DirectionalLight` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirectionalLight` object.
  pub struct ColorChanged<'a>(&'a ::directional_light::DirectionalLight);
  impl<'a> ::qt_core::connection::Receiver for ColorChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2colorChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColorChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDirectionalLight::worldDirectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn world_direction_changed(&self) -> WorldDirectionChanged {
      WorldDirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDirectionalLight::intensityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn intensity_changed(&self) -> IntensityChanged {
      IntensityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDirectionalLight::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DirectionalLight`.
  pub struct Slots<'a>(&'a ::directional_light::DirectionalLight);
  /// Represents a built-in Qt slot `Qt3DRender::QDirectionalLight::setColor`.
  ///
  /// An object of this type can be created from `DirectionalLight` with `object.slots().set_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirectionalLight` object.
  pub struct SetColor<'a>(&'a ::directional_light::DirectionalLight);
  impl<'a> ::qt_core::connection::Receiver for SetColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QDirectionalLight::setIntensity`.
  ///
  /// An object of this type can be created from `DirectionalLight` with `object.slots().set_intensity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirectionalLight` object.
  pub struct SetIntensity<'a>(&'a ::directional_light::DirectionalLight);
  impl<'a> ::qt_core::connection::Receiver for SetIntensity<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIntensity(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QDirectionalLight::setWorldDirection`.
  ///
  /// An object of this type can be created from `DirectionalLight` with `object.slots().set_world_direction()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirectionalLight` object.
  pub struct SetWorldDirection<'a>(&'a ::directional_light::DirectionalLight);
  impl<'a> ::qt_core::connection::Receiver for SetWorldDirection<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorldDirection(const QVector3D&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDirectionalLight::setColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color(&self) -> SetColor {
      SetColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDirectionalLight::setIntensity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_intensity(&self) -> SetIntensity {
      SetIntensity(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDirectionalLight::setWorldDirection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_world_direction(&self) -> SetWorldDirection {
      SetWorldDirection(self.0)
    }
  }
  impl ::directional_light::DirectionalLight {
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

impl ::cpp_utils::DynamicCast<::directional_light::DirectionalLight> for ::abstract_light::AbstractLight {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::directional_light::DirectionalLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_dynamic_cast_Qt3DRender_QDirectionalLight_ptr(self as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::directional_light::DirectionalLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_dynamic_cast_Qt3DRender_QDirectionalLight_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::directional_light::DirectionalLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::directional_light::DirectionalLight as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::directional_light::DirectionalLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::directional_light::DirectionalLight as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_light::AbstractLight> for ::directional_light::DirectionalLight {
  fn static_cast_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *const ::directional_light::DirectionalLight as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::directional_light::DirectionalLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_QObject_ptr(self as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_QObject_ptr(self as *const ::directional_light::DirectionalLight as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::directional_light::DirectionalLight> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::directional_light::DirectionalLight> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::directional_light::DirectionalLight> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::directional_light::DirectionalLight> for ::abstract_light::AbstractLight {
  unsafe fn static_cast_mut(&mut self) -> &mut ::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DRender_QAbstractLight(self as *mut ::abstract_light::AbstractLight);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::directional_light::DirectionalLight {
    let ffi_result = ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DRender_QAbstractLight(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::directional_light::DirectionalLight {
  type Target = ::abstract_light::AbstractLight;
  fn deref(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *const ::directional_light::DirectionalLight as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::directional_light::DirectionalLight {
  fn deref_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *mut ::directional_light::DirectionalLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
