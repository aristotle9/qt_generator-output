/// C++ type: <span style='color: green;'>```Qt3DRender::QLevelOfDetail```</span>
#[repr(C)]
pub struct LevelOfDetail(u8);

impl LevelOfDetail {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera* Qt3DRender::QLevelOfDetail::camera() const```</span>
  ///
  ///
  pub fn camera(&self) -> *mut ::camera::Camera {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_camera(self as *const ::level_of_detail::LevelOfDetail) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLevelOfDetailBoundingSphere Qt3DRender::QLevelOfDetail::createBoundingSphere(const QVector3D& center, float radius)```</span>
  ///
  ///
  pub fn create_bounding_sphere(&mut self,
                                center: &::qt_gui::vector_3d::Vector3D,
                                radius: ::libc::c_float)
                                -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
    {
      let mut object: ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_createBoundingSphere_to_output(self as *mut ::level_of_detail::LevelOfDetail, center as *const ::qt_gui::vector_3d::Vector3D, radius, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QLevelOfDetail::currentIndex() const```</span>
  ///
  ///
  pub fn current_index(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_currentIndex(self as *const ::level_of_detail::LevelOfDetail)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QLevelOfDetail::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_metaObject(self as *const ::level_of_detail::LevelOfDetail)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetail::QLevelOfDetail()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::level_of_detail::LevelOfDetail> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetail::QLevelOfDetail(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::level_of_detail::LevelOfDetail> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QLevelOfDetail::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_qt_metacall(self as *mut ::level_of_detail::LevelOfDetail,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QLevelOfDetail::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_qt_metacast(self as *mut ::level_of_detail::LevelOfDetail, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QLevelOfDetail::setCamera(Qt3DRender::QCamera* camera)```</span>
  ///
  ///
  pub unsafe fn set_camera(&mut self, camera: *mut ::camera::Camera) {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_setCamera(self as *mut ::level_of_detail::LevelOfDetail, camera)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QLevelOfDetail::setCurrentIndex(int currentIndex)```</span>
  ///
  ///
  pub fn set_current_index(&mut self, current_index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_setCurrentIndex(self as *mut ::level_of_detail::LevelOfDetail,
                                                                      current_index)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QLevelOfDetail::setThresholdType(Qt3DRender::QLevelOfDetail::ThresholdType thresholdType)```</span>
  ///
  ///
  pub fn set_threshold_type(&mut self, threshold_type: ::level_of_detail::ThresholdType) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_setThresholdType(self as *mut ::level_of_detail::LevelOfDetail,
                                                                       threshold_type)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QLevelOfDetail::setThresholds(const QVector<double>& thresholds)```</span>
  ///
  ///
  pub fn set_thresholds(&mut self, thresholds: &::qt_gui::vector::VectorCDouble) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_setThresholds(self as *mut ::level_of_detail::LevelOfDetail, thresholds as *const ::qt_gui::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QLevelOfDetail::setVolumeOverride(const Qt3DRender::QLevelOfDetailBoundingSphere& volumeOverride)```</span>
  ///
  ///
  pub fn set_volume_override(&mut self,
                             volume_override: &::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_setVolumeOverride(self as *mut ::level_of_detail::LevelOfDetail, volume_override as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLevelOfDetail::ThresholdType Qt3DRender::QLevelOfDetail::thresholdType() const```</span>
  ///
  ///
  pub fn threshold_type(&self) -> ::level_of_detail::ThresholdType {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_thresholdType(self as *const ::level_of_detail::LevelOfDetail)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double> Qt3DRender::QLevelOfDetail::thresholds() const```</span>
  ///
  ///
  pub fn thresholds(&self) -> ::qt_gui::vector::VectorCDouble {
    {
      let mut object: ::qt_gui::vector::VectorCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_thresholds_to_output(self as *const ::level_of_detail::LevelOfDetail, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QLevelOfDetail::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QLevelOfDetail::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLevelOfDetailBoundingSphere Qt3DRender::QLevelOfDetail::volumeOverride() const```</span>
  ///
  ///
  pub fn volume_override(&self) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
    {
      let mut object: ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_volumeOverride_to_output(self as *const ::level_of_detail::LevelOfDetail, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::level_of_detail::LevelOfDetail {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetail_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `LevelOfDetail`.
  pub struct Signals<'a>(&'a ::level_of_detail::LevelOfDetail);
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::thresholdsChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().thresholds_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct ThresholdsChanged<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::thresholdTypeChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().threshold_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct ThresholdTypeChanged<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::cameraChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().camera_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct CameraChanged<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::volumeOverrideChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().volume_override_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct VolumeOverrideChanged<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::removedFromEntity`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct RemovedFromEntity<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::addedToEntity`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct AddedToEntity<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::currentIndexChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().current_index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct CurrentIndexChanged<'a>(&'a ::level_of_detail::LevelOfDetail);
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
  /// Represents a built-in Qt signal `Qt3DRender::QLevelOfDetail::shareableChanged`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct ShareableChanged<'a>(&'a ::level_of_detail::LevelOfDetail);
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
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::thresholdsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn thresholds_changed(&self) -> ThresholdsChanged {
      ThresholdsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::thresholdTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn threshold_type_changed(&self) -> ThresholdTypeChanged {
      ThresholdTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::cameraChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn camera_changed(&self) -> CameraChanged {
      CameraChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::volumeOverrideChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn volume_override_changed(&self) -> VolumeOverrideChanged {
      VolumeOverrideChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed(&self) -> CurrentIndexChanged {
      CurrentIndexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QLevelOfDetail::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `LevelOfDetail`.
  pub struct Slots<'a>(&'a ::level_of_detail::LevelOfDetail);
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetail::setCamera`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.slots().set_camera()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct SetCamera<'a>(&'a ::level_of_detail::LevelOfDetail);
  impl<'a> ::qt_core::connection::Receiver for SetCamera<'a> {
    type Arguments = (*mut ::camera::Camera,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCamera(Qt3DRender::QCamera*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetail::setCurrentIndex`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct SetCurrentIndex<'a>(&'a ::level_of_detail::LevelOfDetail);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetail::setVolumeOverride`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.slots().set_volume_override()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct SetVolumeOverride<'a>(&'a ::level_of_detail::LevelOfDetail);
  impl<'a> ::qt_core::connection::Receiver for SetVolumeOverride<'a> {
    type Arguments = (&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVolumeOverride(const Qt3DRender::QLevelOfDetailBoundingSphere&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetail::setThresholds`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.slots().set_thresholds()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct SetThresholds<'a>(&'a ::level_of_detail::LevelOfDetail);
  impl<'a> ::qt_core::connection::Receiver for SetThresholds<'a> {
    type Arguments = (&'static ::qt_gui::vector::VectorCDouble,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setThresholds(const QVector< double >&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetail::setShareable`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct SetShareable<'a>(&'a ::level_of_detail::LevelOfDetail);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QLevelOfDetail::setThresholdType`.
  ///
  /// An object of this type can be created from `LevelOfDetail` with `object.slots().set_threshold_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LevelOfDetail` object.
  pub struct SetThresholdType<'a>(&'a ::level_of_detail::LevelOfDetail);
  impl<'a> ::qt_core::connection::Receiver for SetThresholdType<'a> {
    type Arguments = (::level_of_detail::ThresholdType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setThresholdType(Qt3DRender::QLevelOfDetail::ThresholdType)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetail::setCamera`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_camera(&self) -> SetCamera {
      SetCamera(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetail::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetail::setVolumeOverride`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_volume_override(&self) -> SetVolumeOverride {
      SetVolumeOverride(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetail::setThresholds`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_thresholds(&self) -> SetThresholds {
      SetThresholds(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetail::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QLevelOfDetail::setThresholdType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_threshold_type(&self) -> SetThresholdType {
      SetThresholdType(self.0)
    }
  }
  impl ::level_of_detail::LevelOfDetail {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QLevelOfDetail::ThresholdType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ThresholdType {
  /// C++ enum variant: <span style='color: green;'>```DistanceToCameraThreshold = 0```</span>
  DistanceToCamera = 0,
  /// C++ enum variant: <span style='color: green;'>```ProjectedScreenPixelSizeThreshold = 1```</span>
  ProjectedScreenPixelSize = 1,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::level_of_detail::LevelOfDetail {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::level_of_detail::LevelOfDetail as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::level_of_detail::LevelOfDetail {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::level_of_detail::LevelOfDetail as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::level_of_detail::LevelOfDetail {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_QObject_ptr(self as *mut ::level_of_detail::LevelOfDetail)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_QObject_ptr(self as *const ::level_of_detail::LevelOfDetail as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail::LevelOfDetail> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail::LevelOfDetail {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::level_of_detail::LevelOfDetail {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail::LevelOfDetail> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail::LevelOfDetail {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::level_of_detail::LevelOfDetail {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::level_of_detail::LevelOfDetail> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::level_of_detail::LevelOfDetail {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::level_of_detail::LevelOfDetail {
    let ffi_result = ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DRender_QLevelOfDetail_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::level_of_detail::LevelOfDetail {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::level_of_detail::LevelOfDetail as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::level_of_detail::LevelOfDetail {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLevelOfDetail_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::level_of_detail::LevelOfDetail) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
