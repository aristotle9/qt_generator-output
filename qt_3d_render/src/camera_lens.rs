/// C++ type: <span style='color: green;'>```Qt3DRender::QCameraLens```</span>
#[repr(C)]
pub struct CameraLens(u8);

impl CameraLens {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::aspectRatio() const```</span>
  ///
  ///
  pub fn aspect_ratio(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_aspectRatio(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_bottom(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::exposure() const```</span>
  ///
  ///
  pub fn exposure(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_exposure(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::farPlane() const```</span>
  ///
  ///
  pub fn far_plane(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_farPlane(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::fieldOfView() const```</span>
  ///
  ///
  pub fn field_of_view(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_fieldOfView(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_left(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QCameraLens::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_metaObject(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::nearPlane() const```</span>
  ///
  ///
  pub fn near_plane(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_nearPlane(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCameraLens::QCameraLens()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::camera_lens::CameraLens> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCameraLens::QCameraLens(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::camera_lens::CameraLens> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4 Qt3DRender::QCameraLens::projectionMatrix() const```</span>
  ///
  ///
  pub fn projection_matrix(&self) -> ::cpp_utils::CppBox<::qt_gui::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_projectionMatrix_as_ptr(self as *const ::camera_lens::CameraLens)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCameraLens::ProjectionType Qt3DRender::QCameraLens::projectionType() const```</span>
  ///
  ///
  pub fn projection_type(&self) -> ::camera_lens::ProjectionType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_projectionType(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QCameraLens::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_qt_metacall(self as *mut ::camera_lens::CameraLens,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QCameraLens::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_qt_metacast(self as *mut ::camera_lens::CameraLens, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_right(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setAspectRatio(float aspectRatio)```</span>
  ///
  ///
  pub fn set_aspect_ratio(&mut self, aspect_ratio: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setAspectRatio(self as *mut ::camera_lens::CameraLens, aspect_ratio)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setBottom(float bottom)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, bottom: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setBottom(self as *mut ::camera_lens::CameraLens, bottom) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setExposure(float exposure)```</span>
  ///
  ///
  pub fn set_exposure(&mut self, exposure: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setExposure(self as *mut ::camera_lens::CameraLens, exposure)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setFarPlane(float farPlane)```</span>
  ///
  ///
  pub fn set_far_plane(&mut self, far_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setFarPlane(self as *mut ::camera_lens::CameraLens, far_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setFieldOfView(float fieldOfView)```</span>
  ///
  ///
  pub fn set_field_of_view(&mut self, field_of_view: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setFieldOfView(self as *mut ::camera_lens::CameraLens, field_of_view)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCameraLens::setFrustumProjection(float left, float right, float bottom, float top, float nearPlane, float farPlane)```</span>
  ///
  ///
  pub fn set_frustum_projection(&mut self,
                                left: ::libc::c_float,
                                right: ::libc::c_float,
                                bottom: ::libc::c_float,
                                top: ::libc::c_float,
                                near_plane: ::libc::c_float,
                                far_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setFrustumProjection(self as *mut ::camera_lens::CameraLens,
                                                                        left,
                                                                        right,
                                                                        bottom,
                                                                        top,
                                                                        near_plane,
                                                                        far_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setLeft(float left)```</span>
  ///
  ///
  pub fn set_left(&mut self, left: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setLeft(self as *mut ::camera_lens::CameraLens, left) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setNearPlane(float nearPlane)```</span>
  ///
  ///
  pub fn set_near_plane(&mut self, near_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setNearPlane(self as *mut ::camera_lens::CameraLens, near_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCameraLens::setOrthographicProjection(float left, float right, float bottom, float top, float nearPlane, float farPlane)```</span>
  ///
  ///
  pub fn set_orthographic_projection(&mut self,
                                     left: ::libc::c_float,
                                     right: ::libc::c_float,
                                     bottom: ::libc::c_float,
                                     top: ::libc::c_float,
                                     near_plane: ::libc::c_float,
                                     far_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setOrthographicProjection(self as *mut ::camera_lens::CameraLens,
                                                                             left,
                                                                             right,
                                                                             bottom,
                                                                             top,
                                                                             near_plane,
                                                                             far_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCameraLens::setPerspectiveProjection(float fieldOfView, float aspect, float nearPlane, float farPlane)```</span>
  ///
  ///
  pub fn set_perspective_projection(&mut self,
                                    field_of_view: ::libc::c_float,
                                    aspect: ::libc::c_float,
                                    near_plane: ::libc::c_float,
                                    far_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setPerspectiveProjection(self as *mut ::camera_lens::CameraLens,
                                                                            field_of_view,
                                                                            aspect,
                                                                            near_plane,
                                                                            far_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setProjectionMatrix(const QMatrix4x4& projectionMatrix)```</span>
  ///
  ///
  pub fn set_projection_matrix(&mut self, projection_matrix: &::qt_gui::matrix_4x4::Matrix4X4) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setProjectionMatrix(self as *mut ::camera_lens::CameraLens, projection_matrix as *const ::qt_gui::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setProjectionType(Qt3DRender::QCameraLens::ProjectionType projectionType)```</span>
  ///
  ///
  pub fn set_projection_type(&mut self, projection_type: ::camera_lens::ProjectionType) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setProjectionType(self as *mut ::camera_lens::CameraLens,
                                                                     projection_type)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setRight(float right)```</span>
  ///
  ///
  pub fn set_right(&mut self, right: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setRight(self as *mut ::camera_lens::CameraLens, right) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraLens::setTop(float top)```</span>
  ///
  ///
  pub fn set_top(&mut self, top: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_setTop(self as *mut ::camera_lens::CameraLens, top) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCameraLens::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_top(self as *const ::camera_lens::CameraLens) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCameraLens::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCameraLens::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::camera_lens::CameraLens {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraLens_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CameraLens`.
  pub struct Signals<'a>(&'a ::camera_lens::CameraLens);
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::farPlaneChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().far_plane_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct FarPlaneChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for FarPlaneChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2farPlaneChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FarPlaneChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::nearPlaneChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().near_plane_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct NearPlaneChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for NearPlaneChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nearPlaneChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NearPlaneChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::leftChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().left_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct LeftChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for LeftChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2leftChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LeftChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::shareableChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct ShareableChanged<'a>(&'a ::camera_lens::CameraLens);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::removedFromEntity`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct RemovedFromEntity<'a>(&'a ::camera_lens::CameraLens);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::projectionTypeChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().projection_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct ProjectionTypeChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for ProjectionTypeChanged<'a> {
    type Arguments = (&'static ::camera_lens::ProjectionType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2projectionTypeChanged(Qt3DRender::QCameraLens::ProjectionType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ProjectionTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::aspectRatioChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().aspect_ratio_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct AspectRatioChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for AspectRatioChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aspectRatioChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AspectRatioChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::rightChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().right_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct RightChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for RightChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rightChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RightChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::addedToEntity`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct AddedToEntity<'a>(&'a ::camera_lens::CameraLens);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::projectionMatrixChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().projection_matrix_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct ProjectionMatrixChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for ProjectionMatrixChanged<'a> {
    type Arguments = (&'static ::qt_gui::matrix_4x4::Matrix4X4,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2projectionMatrixChanged(const QMatrix4x4&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ProjectionMatrixChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::fieldOfViewChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().field_of_view_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct FieldOfViewChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for FieldOfViewChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fieldOfViewChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FieldOfViewChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::bottomChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().bottom_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct BottomChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for BottomChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bottomChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BottomChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::topChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().top_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct TopChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for TopChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2topChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TopChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCameraLens::exposureChanged`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.signals().exposure_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct ExposureChanged<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for ExposureChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2exposureChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ExposureChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::farPlaneChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn far_plane_changed(&self) -> FarPlaneChanged {
      FarPlaneChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::nearPlaneChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn near_plane_changed(&self) -> NearPlaneChanged {
      NearPlaneChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::leftChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn left_changed(&self) -> LeftChanged {
      LeftChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::projectionTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn projection_type_changed(&self) -> ProjectionTypeChanged {
      ProjectionTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::aspectRatioChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn aspect_ratio_changed(&self) -> AspectRatioChanged {
      AspectRatioChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::rightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn right_changed(&self) -> RightChanged {
      RightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::projectionMatrixChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn projection_matrix_changed(&self) -> ProjectionMatrixChanged {
      ProjectionMatrixChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::fieldOfViewChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn field_of_view_changed(&self) -> FieldOfViewChanged {
      FieldOfViewChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::bottomChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_changed(&self) -> BottomChanged {
      BottomChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::topChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_changed(&self) -> TopChanged {
      TopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraLens::exposureChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exposure_changed(&self) -> ExposureChanged {
      ExposureChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CameraLens`.
  pub struct Slots<'a>(&'a ::camera_lens::CameraLens);
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setBottom`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetBottom<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetBottom<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBottom(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setProjectionType`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_projection_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetProjectionType<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetProjectionType<'a> {
    type Arguments = (::camera_lens::ProjectionType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setProjectionType(Qt3DRender::QCameraLens::ProjectionType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setProjectionMatrix`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_projection_matrix()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetProjectionMatrix<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetProjectionMatrix<'a> {
    type Arguments = (&'static ::qt_gui::matrix_4x4::Matrix4X4,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setProjectionMatrix(const QMatrix4x4&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setTop`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetTop<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetTop<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTop(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setLeft`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_left()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetLeft<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetLeft<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLeft(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setNearPlane`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_near_plane()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetNearPlane<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetNearPlane<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNearPlane(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setShareable`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetShareable<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setFieldOfView`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_field_of_view()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetFieldOfView<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetFieldOfView<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFieldOfView(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setFarPlane`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_far_plane()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetFarPlane<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetFarPlane<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFarPlane(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setExposure`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_exposure()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetExposure<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetExposure<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExposure(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setRight`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_right()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetRight<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetRight<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRight(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCameraLens::setAspectRatio`.
  ///
  /// An object of this type can be created from `CameraLens` with `object.slots().set_aspect_ratio()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraLens` object.
  pub struct SetAspectRatio<'a>(&'a ::camera_lens::CameraLens);
  impl<'a> ::qt_core::connection::Receiver for SetAspectRatio<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAspectRatio(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bottom(&self) -> SetBottom {
      SetBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setProjectionType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_projection_type(&self) -> SetProjectionType {
      SetProjectionType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setProjectionMatrix`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_projection_matrix(&self) -> SetProjectionMatrix {
      SetProjectionMatrix(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_top(&self) -> SetTop {
      SetTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setLeft`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_left(&self) -> SetLeft {
      SetLeft(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setNearPlane`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_near_plane(&self) -> SetNearPlane {
      SetNearPlane(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setFieldOfView`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_field_of_view(&self) -> SetFieldOfView {
      SetFieldOfView(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setFarPlane`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_far_plane(&self) -> SetFarPlane {
      SetFarPlane(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setExposure`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_exposure(&self) -> SetExposure {
      SetExposure(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setRight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_right(&self) -> SetRight {
      SetRight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraLens::setAspectRatio`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_aspect_ratio(&self) -> SetAspectRatio {
      SetAspectRatio(self.0)
    }
  }
  impl ::camera_lens::CameraLens {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QCameraLens::ProjectionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ProjectionType {
  /// C++ enum variant: <span style='color: green;'>```OrthographicProjection = 0```</span>
  Orthographic = 0,
  /// C++ enum variant: <span style='color: green;'>```PerspectiveProjection = 1```</span>
  Perspective = 1,
  /// C++ enum variant: <span style='color: green;'>```FrustumProjection = 2```</span>
  Frustum = 2,
  /// C++ enum variant: <span style='color: green;'>```CustomProjection = 3```</span>
  Custom = 3,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::camera_lens::CameraLens {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::camera_lens::CameraLens)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::camera_lens::CameraLens as *mut ::camera_lens::CameraLens) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::camera_lens::CameraLens {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::camera_lens::CameraLens)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::camera_lens::CameraLens as *mut ::camera_lens::CameraLens) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::camera_lens::CameraLens {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_QObject_ptr(self as *mut ::camera_lens::CameraLens) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_QObject_ptr(self as *const ::camera_lens::CameraLens as *mut ::camera_lens::CameraLens) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera_lens::CameraLens> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera_lens::CameraLens {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera_lens::CameraLens {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera_lens::CameraLens> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera_lens::CameraLens {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera_lens::CameraLens {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera_lens::CameraLens> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera_lens::CameraLens {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera_lens::CameraLens {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::camera_lens::CameraLens {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::camera_lens::CameraLens as *mut ::camera_lens::CameraLens) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::camera_lens::CameraLens {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::camera_lens::CameraLens)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
