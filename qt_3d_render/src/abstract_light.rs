/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractLight```</span>
#[repr(C)]
pub struct AbstractLight(u8);

impl AbstractLight {
  /// C++ method: <span style='color: green;'>```QColor Qt3DRender::QAbstractLight::color() const```</span>
  ///
  ///
  pub fn color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_color_to_output(self as *const ::abstract_light::AbstractLight, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QAbstractLight::intensity() const```</span>
  ///
  ///
  pub fn intensity(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_intensity(self as *const ::abstract_light::AbstractLight) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QAbstractLight::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_metaObject(self as *const ::abstract_light::AbstractLight)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QAbstractLight::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_qt_metacall(self as *mut ::abstract_light::AbstractLight,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QAbstractLight::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_qt_metacast(self as *mut ::abstract_light::AbstractLight, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractLight::setColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_setColor(self as *mut ::abstract_light::AbstractLight,
                                                               color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractLight::setIntensity(float intensity)```</span>
  ///
  ///
  pub fn set_intensity(&mut self, intensity: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_setIntensity(self as *mut ::abstract_light::AbstractLight,
                                                                   intensity)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAbstractLight::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAbstractLight::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractLight::Type Qt3DRender::QAbstractLight::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::abstract_light::Type {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_type(self as *const ::abstract_light::AbstractLight) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_light::AbstractLight {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractLight_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractLight`.
  pub struct Signals<'a>(&'a ::abstract_light::AbstractLight);
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractLight::addedToEntity`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct AddedToEntity<'a>(&'a ::abstract_light::AbstractLight);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractLight::removedFromEntity`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct RemovedFromEntity<'a>(&'a ::abstract_light::AbstractLight);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractLight::colorChanged`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct ColorChanged<'a>(&'a ::abstract_light::AbstractLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractLight::intensityChanged`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.signals().intensity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct IntensityChanged<'a>(&'a ::abstract_light::AbstractLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractLight::shareableChanged`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct ShareableChanged<'a>(&'a ::abstract_light::AbstractLight);
  impl<'a> ::qt_core::connection::Receiver for ShareableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shareableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShareableChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractLight::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractLight::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractLight::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractLight::intensityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn intensity_changed(&self) -> IntensityChanged {
      IntensityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractLight::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractLight`.
  pub struct Slots<'a>(&'a ::abstract_light::AbstractLight);
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractLight::setShareable`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct SetShareable<'a>(&'a ::abstract_light::AbstractLight);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractLight::setColor`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.slots().set_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct SetColor<'a>(&'a ::abstract_light::AbstractLight);
  impl<'a> ::qt_core::connection::Receiver for SetColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractLight::setIntensity`.
  ///
  /// An object of this type can be created from `AbstractLight` with `object.slots().set_intensity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractLight` object.
  pub struct SetIntensity<'a>(&'a ::abstract_light::AbstractLight);
  impl<'a> ::qt_core::connection::Receiver for SetIntensity<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIntensity(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractLight::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractLight::setColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color(&self) -> SetColor {
      SetColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractLight::setIntensity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_intensity(&self) -> SetIntensity {
      SetIntensity(self.0)
    }
  }
  impl ::abstract_light::AbstractLight {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractLight::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```PointLight = 0```</span>
  Point = 0,
  /// C++ enum variant: <span style='color: green;'>```DirectionalLight = 1```</span>
  Directional = 1,
  /// C++ enum variant: <span style='color: green;'>```SpotLight = 2```</span>
  Spot = 2,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::abstract_light::AbstractLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::abstract_light::AbstractLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_light::AbstractLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_QObject_ptr(self as *mut ::abstract_light::AbstractLight)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_QObject_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_light::AbstractLight> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_light::AbstractLight> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_light::AbstractLight> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_light::AbstractLight {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_light::AbstractLight {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
