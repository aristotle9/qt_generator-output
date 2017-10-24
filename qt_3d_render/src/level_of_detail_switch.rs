/// C++ type: <span style='color: green;'>```Qt3DRender::QLevelOfDetailSwitch```</span>
#[repr(C)]
pub struct LevelOfDetailSwitch(u8);

impl LevelOfDetailSwitch {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QLevelOfDetailSwitch::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_metaObject(self as *const ::level_of_detail_switch::LevelOfDetailSwitch) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetailSwitch::QLevelOfDetailSwitch()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::level_of_detail_switch::LevelOfDetailSwitch> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetailSwitch::QLevelOfDetailSwitch(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::level_of_detail_switch::LevelOfDetailSwitch> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QLevelOfDetailSwitch::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_qt_metacall(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QLevelOfDetailSwitch::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_qt_metacast(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QLevelOfDetailSwitch::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QLevelOfDetailSwitch::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::level_of_detail_switch::LevelOfDetailSwitch {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailSwitch_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `LevelOfDetailSwitch`.
  pub struct Signals<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::cameraChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.signals().camera_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct CameraChanged<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for CameraChanged<'a> {
    type Arguments = (*mut ::camera::Camera,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cameraChanged(Qt3DRender::QCamera*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CameraChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::currentIndexChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.signals().current_index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct CurrentIndexChanged<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for CurrentIndexChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIndexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIndexChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::thresholdTypeChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.signals().threshold_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct ThresholdTypeChanged<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for ThresholdTypeChanged<'a> {
    type Arguments = (::level_of_detail::ThresholdType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2thresholdTypeChanged(Qt3DRender::QLevelOfDetail::ThresholdType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ThresholdTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::thresholdsChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.signals().thresholds_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct ThresholdsChanged<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for ThresholdsChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector::VectorCDouble,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2thresholdsChanged(const QVector< double >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ThresholdsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::volumeOverrideChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.signals().volume_override_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct VolumeOverrideChanged<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for VolumeOverrideChanged<'a> {
    type Arguments = (&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2volumeOverrideChanged(const Qt3DRender::QLevelOfDetailBoundingSphere&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VolumeOverrideChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::cameraChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn camera_changed(&self) -> CameraChanged {
      CameraChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed(&self) -> CurrentIndexChanged {
      CurrentIndexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::thresholdTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn threshold_type_changed(&self) -> ThresholdTypeChanged {
      ThresholdTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::thresholdsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn thresholds_changed(&self) -> ThresholdsChanged {
      ThresholdsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetailSwitch::volumeOverrideChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn volume_override_changed(&self) -> VolumeOverrideChanged {
      VolumeOverrideChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `LevelOfDetailSwitch`.
  pub struct Slots<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setThresholdType`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.slots().set_threshold_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct SetThresholdType<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for SetThresholdType<'a> {
    type Arguments = (::level_of_detail::ThresholdType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setThresholdType(Qt3DRender::QLevelOfDetail::ThresholdType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setCamera`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.slots().set_camera()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct SetCamera<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for SetCamera<'a> {
    type Arguments = (*mut ::camera::Camera,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCamera(Qt3DRender::QCamera*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setCurrentIndex`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct SetCurrentIndex<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setThresholds`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.slots().set_thresholds()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct SetThresholds<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for SetThresholds<'a> {
    type Arguments = (&'static ::qt_gui::vector::VectorCDouble,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setThresholds(const QVector< double >&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setVolumeOverride`.
  ///
  /// An object of this type can be created from `LevelOfDetailSwitch` with `object.slots().set_volume_override()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetailSwitch` object.
  pub struct SetVolumeOverride<'a>(&'a ::level_of_detail_switch::LevelOfDetailSwitch);
  impl<'a> ::qt_core::connection::Receiver for SetVolumeOverride<'a> {
    type Arguments = (&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVolumeOverride(const Qt3DRender::QLevelOfDetailBoundingSphere&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setThresholdType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_threshold_type(&self) -> SetThresholdType {
      SetThresholdType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setCamera`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_camera(&self) -> SetCamera {
      SetCamera(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setThresholds`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_thresholds(&self) -> SetThresholds {
      SetThresholds(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetailSwitch::setVolumeOverride`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_volume_override(&self) -> SetVolumeOverride {
      SetVolumeOverride(self.0)
    }
  }
  impl ::level_of_detail_switch::LevelOfDetailSwitch {
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

impl ::cpp_utils::DynamicCast<::level_of_detail_switch::LevelOfDetailSwitch> for ::level_of_detail::LevelOfDetail {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::level_of_detail_switch::LevelOfDetailSwitch> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_dynamic_cast_Qt3DRender_QLevelOfDetailSwitch_ptr(self as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::level_of_detail_switch::LevelOfDetailSwitch> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_dynamic_cast_Qt3DRender_QLevelOfDetailSwitch_ptr(self as *const ::level_of_detail::LevelOfDetail as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::level_of_detail_switch::LevelOfDetailSwitch {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::level_of_detail_switch::LevelOfDetailSwitch as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::level_of_detail_switch::LevelOfDetailSwitch {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::level_of_detail_switch::LevelOfDetailSwitch as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::level_of_detail::LevelOfDetail> for ::level_of_detail_switch::LevelOfDetailSwitch {
  fn static_cast_mut(&mut self) -> &mut ::level_of_detail::LevelOfDetail {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetail_ptr(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::level_of_detail::LevelOfDetail {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetail_ptr(self as *const ::level_of_detail_switch::LevelOfDetailSwitch as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::level_of_detail_switch::LevelOfDetailSwitch {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_QObject_ptr(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_QObject_ptr(self as *const ::level_of_detail_switch::LevelOfDetailSwitch as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail_switch::LevelOfDetailSwitch> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail_switch::LevelOfDetailSwitch {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::level_of_detail_switch::LevelOfDetailSwitch {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail_switch::LevelOfDetailSwitch> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail_switch::LevelOfDetailSwitch {
let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::level_of_detail_switch::LevelOfDetailSwitch {
let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail_switch::LevelOfDetailSwitch> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail_switch::LevelOfDetailSwitch {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::level_of_detail_switch::LevelOfDetailSwitch {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail_switch::LevelOfDetailSwitch> for ::level_of_detail::LevelOfDetail {
unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail_switch::LevelOfDetailSwitch {
let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_Qt3DRender_QLevelOfDetail(self as *mut ::level_of_detail::LevelOfDetail);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::level_of_detail_switch::LevelOfDetailSwitch {
let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetailSwitch_ptr_Qt3DRender_QLevelOfDetail(self as *const ::level_of_detail::LevelOfDetail as *mut ::level_of_detail::LevelOfDetail);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::level_of_detail_switch::LevelOfDetailSwitch {
  type Target = ::level_of_detail::LevelOfDetail;
  fn deref(&self) -> &::level_of_detail::LevelOfDetail {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetail_ptr(self as *const ::level_of_detail_switch::LevelOfDetailSwitch as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::level_of_detail_switch::LevelOfDetailSwitch {
  fn deref_mut(&mut self) -> &mut ::level_of_detail::LevelOfDetail {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetailSwitch_G_static_cast_Qt3DRender_QLevelOfDetail_ptr(self as *mut ::level_of_detail_switch::LevelOfDetailSwitch) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
