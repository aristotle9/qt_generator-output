/// C++ type: <span style='color: green;'>```Qt3DRender::QCamera```</span>
#[repr(C)]
pub struct Camera(u8);

impl Camera {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::aspectRatio() const```</span>
  ///
  ///
  pub fn aspect_ratio(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_aspectRatio(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_bottom(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::exposure() const```</span>
  ///
  ///
  pub fn exposure(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_exposure(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::farPlane() const```</span>
  ///
  ///
  pub fn far_plane(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_farPlane(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::fieldOfView() const```</span>
  ///
  ///
  pub fn field_of_view(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_fieldOfView(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_left(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCameraLens* Qt3DRender::QCamera::lens() const```</span>
  ///
  ///
  pub fn lens(&self) -> *mut ::camera_lens::CameraLens {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_lens(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QCamera::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_metaObject(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::nearPlane() const```</span>
  ///
  ///
  pub fn near_plane(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_nearPlane(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCamera::QCamera()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::camera::Camera> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCamera::QCamera(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::camera::Camera> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QCamera_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera::pan```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pan(&mut self, ::libc::c_float) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::pan(float angle)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pan(&mut self, (::libc::c_float, &::qt_gui::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::pan(float angle, const QVector3D& axis)```</span>
  ///
  ///
  pub fn pan<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CameraPanArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera::panAboutViewCenter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pan_about_view_center(&mut self, ::libc::c_float) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::panAboutViewCenter(float angle)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pan_about_view_center(&mut self, (::libc::c_float, &::qt_gui::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::panAboutViewCenter(float angle, const QVector3D& axis)```</span>
  ///
  ///
  pub fn pan_about_view_center<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CameraPanAboutViewCenterArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QQuaternion Qt3DRender::QCamera::panRotation(float angle) const```</span>
  ///
  ///
  pub fn pan_rotation(&self, angle: ::libc::c_float) -> ::qt_gui::quaternion::Quaternion {
    {
      let mut object: ::qt_gui::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_panRotation_to_output(self as *const ::camera::Camera,
                                                                       angle,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QCamera::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_position_as_ptr(self as *const ::camera::Camera) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4 Qt3DRender::QCamera::projectionMatrix() const```</span>
  ///
  ///
  pub fn projection_matrix(&self) -> ::cpp_utils::CppBox<::qt_gui::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_projectionMatrix_as_ptr(self as *const ::camera::Camera) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QCamera::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QCamera_qt_metacall(self as *mut ::camera::Camera,
                                                         arg1 as *const ::qt_core::meta_object::Call,
                                                         arg2,
                                                         arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QCamera::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QCamera_qt_metacast(self as *mut ::camera::Camera, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_right(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::roll(float angle)```</span>
  ///
  ///
  pub fn roll(&mut self, angle: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_roll(self as *mut ::camera::Camera, angle) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::rollAboutViewCenter(float angle)```</span>
  ///
  ///
  pub fn roll_about_view_center(&mut self, angle: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_rollAboutViewCenter(self as *mut ::camera::Camera, angle) }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion Qt3DRender::QCamera::rollRotation(float angle) const```</span>
  ///
  ///
  pub fn roll_rotation(&self, angle: ::libc::c_float) -> ::qt_gui::quaternion::Quaternion {
    {
      let mut object: ::qt_gui::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_rollRotation_to_output(self as *const ::camera::Camera,
                                                                        angle,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::rotate(const QQuaternion& q)```</span>
  ///
  ///
  pub fn rotate(&mut self, q: &::qt_gui::quaternion::Quaternion) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_rotate(self as *mut ::camera::Camera,
                                                      q as *const ::qt_gui::quaternion::Quaternion)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::rotateAboutViewCenter(const QQuaternion& q)```</span>
  ///
  ///
  pub fn rotate_about_view_center(&mut self, q: &::qt_gui::quaternion::Quaternion) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_rotateAboutViewCenter(self as *mut ::camera::Camera,
                                                                     q as *const ::qt_gui::quaternion::Quaternion)
    }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion Qt3DRender::QCamera::rotation(float angle, const QVector3D& axis) const```</span>
  ///
  ///
  pub fn rotation(&self,
                  angle: ::libc::c_float,
                  axis: &::qt_gui::vector_3d::Vector3D)
                  -> ::qt_gui::quaternion::Quaternion {
    {
      let mut object: ::qt_gui::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_rotation_to_output(self as *const ::camera::Camera,
                                                                    angle,
                                                                    axis as *const ::qt_gui::vector_3d::Vector3D,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setAspectRatio(float aspectRatio)```</span>
  ///
  ///
  pub fn set_aspect_ratio(&mut self, aspect_ratio: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setAspectRatio(self as *mut ::camera::Camera, aspect_ratio) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setBottom(float bottom)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, bottom: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setBottom(self as *mut ::camera::Camera, bottom) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setExposure(float exposure)```</span>
  ///
  ///
  pub fn set_exposure(&mut self, exposure: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setExposure(self as *mut ::camera::Camera, exposure) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setFarPlane(float farPlane)```</span>
  ///
  ///
  pub fn set_far_plane(&mut self, far_plane: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setFarPlane(self as *mut ::camera::Camera, far_plane) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setFieldOfView(float fieldOfView)```</span>
  ///
  ///
  pub fn set_field_of_view(&mut self, field_of_view: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setFieldOfView(self as *mut ::camera::Camera, field_of_view) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setLeft(float left)```</span>
  ///
  ///
  pub fn set_left(&mut self, left: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setLeft(self as *mut ::camera::Camera, left) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setNearPlane(float nearPlane)```</span>
  ///
  ///
  pub fn set_near_plane(&mut self, near_plane: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setNearPlane(self as *mut ::camera::Camera, near_plane) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setPosition(const QVector3D& position)```</span>
  ///
  ///
  pub fn set_position(&mut self, position: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setPosition(self as *mut ::camera::Camera,
                                                           position as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setProjectionMatrix(const QMatrix4x4& projectionMatrix)```</span>
  ///
  ///
  pub fn set_projection_matrix(&mut self, projection_matrix: &::qt_gui::matrix_4x4::Matrix4X4) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setProjectionMatrix(self as *mut ::camera::Camera, projection_matrix as *const ::qt_gui::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setProjectionType(Qt3DRender::QCameraLens::ProjectionType type)```</span>
  ///
  ///
  pub fn set_projection_type(&mut self, type_: &::camera_lens::ProjectionType) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setProjectionType(self as *mut ::camera::Camera,
                                                                 type_ as *const ::camera_lens::ProjectionType)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setRight(float right)```</span>
  ///
  ///
  pub fn set_right(&mut self, right: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setRight(self as *mut ::camera::Camera, right) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setTop(float top)```</span>
  ///
  ///
  pub fn set_top(&mut self, top: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setTop(self as *mut ::camera::Camera, top) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setUpVector(const QVector3D& upVector)```</span>
  ///
  ///
  pub fn set_up_vector(&mut self, up_vector: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setUpVector(self as *mut ::camera::Camera,
                                                           up_vector as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCamera::setViewCenter(const QVector3D& viewCenter)```</span>
  ///
  ///
  pub fn set_view_center(&mut self, view_center: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_setViewCenter(self as *mut ::camera::Camera,
                                                             view_center as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::tilt(float angle)```</span>
  ///
  ///
  pub fn tilt(&mut self, angle: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_tilt(self as *mut ::camera::Camera, angle) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::tiltAboutViewCenter(float angle)```</span>
  ///
  ///
  pub fn tilt_about_view_center(&mut self, angle: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_tiltAboutViewCenter(self as *mut ::camera::Camera, angle) }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion Qt3DRender::QCamera::tiltRotation(float angle) const```</span>
  ///
  ///
  pub fn tilt_rotation(&self, angle: ::libc::c_float) -> ::qt_gui::quaternion::Quaternion {
    {
      let mut object: ::qt_gui::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_tiltRotation_to_output(self as *const ::camera::Camera,
                                                                        angle,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QCamera::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_top(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCamera::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCamera::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCamera_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QTransform* Qt3DRender::QCamera::transform() const```</span>
  ///
  ///
  pub fn transform(&self) -> *mut ::qt_3d_core::transform::Transform {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_transform(self as *const ::camera::Camera) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_gui::vector_3d::Vector3D) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::translate(const QVector3D& vLocal)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (&::qt_gui::vector_3d::Vector3D, ::camera::CameraTranslationOption)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::translate(const QVector3D& vLocal, Qt3DRender::QCamera::CameraTranslationOption option = ?)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CameraTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera::translateWorld```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate_world(&mut self, &::qt_gui::vector_3d::Vector3D) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::translateWorld(const QVector3D& vWorld)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate_world(&mut self, (&::qt_gui::vector_3d::Vector3D, ::camera::CameraTranslationOption)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QCamera::translateWorld(const QVector3D& vWorld, Qt3DRender::QCamera::CameraTranslationOption option = ?)```</span>
  ///
  ///
  pub fn translate_world<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CameraTranslateWorldArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QCamera::upVector() const```</span>
  ///
  ///
  pub fn up_vector(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_upVector_as_ptr(self as *const ::camera::Camera) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QCamera::viewCenter() const```</span>
  ///
  ///
  pub fn view_center(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_viewCenter_as_ptr(self as *const ::camera::Camera) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4 Qt3DRender::QCamera::viewMatrix() const```</span>
  ///
  ///
  pub fn view_matrix(&self) -> ::cpp_utils::CppBox<::qt_gui::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_viewMatrix_as_ptr(self as *const ::camera::Camera) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QCamera::viewVector() const```</span>
  ///
  ///
  pub fn view_vector(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_viewVector_as_ptr(self as *const ::camera::Camera) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::camera::Camera {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QCamera_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Camera`.
  pub struct Signals<'a>(&'a ::camera::Camera);
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::viewVectorChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().view_vector_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct ViewVectorChanged<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for ViewVectorChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2viewVectorChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ViewVectorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::projectionMatrixChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().projection_matrix_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct ProjectionMatrixChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::rightChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().right_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct RightChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::aspectRatioChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().aspect_ratio_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct AspectRatioChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::exposureChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().exposure_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct ExposureChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::bottomChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().bottom_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct BottomChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::viewMatrixChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().view_matrix_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct ViewMatrixChanged<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for ViewMatrixChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2viewMatrixChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ViewMatrixChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::upVectorChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().up_vector_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct UpVectorChanged<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for UpVectorChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2upVectorChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UpVectorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::topChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().top_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct TopChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::viewCenterChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().view_center_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct ViewCenterChanged<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for ViewCenterChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2viewCenterChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ViewCenterChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::projectionTypeChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().projection_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct ProjectionTypeChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::leftChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().left_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct LeftChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::nearPlaneChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().near_plane_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct NearPlaneChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::farPlaneChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().far_plane_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct FarPlaneChanged<'a>(&'a ::camera::Camera);
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
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::positionChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct PositionChanged<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for PositionChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2positionChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PositionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QCamera::fieldOfViewChanged`.
  ///
  /// An object of this type can be created from `Camera` with `object.signals().field_of_view_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct FieldOfViewChanged<'a>(&'a ::camera::Camera);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::viewVectorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn view_vector_changed(&self) -> ViewVectorChanged {
      ViewVectorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::projectionMatrixChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn projection_matrix_changed(&self) -> ProjectionMatrixChanged {
      ProjectionMatrixChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::rightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn right_changed(&self) -> RightChanged {
      RightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::aspectRatioChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn aspect_ratio_changed(&self) -> AspectRatioChanged {
      AspectRatioChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::exposureChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exposure_changed(&self) -> ExposureChanged {
      ExposureChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::bottomChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_changed(&self) -> BottomChanged {
      BottomChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::viewMatrixChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn view_matrix_changed(&self) -> ViewMatrixChanged {
      ViewMatrixChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::upVectorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn up_vector_changed(&self) -> UpVectorChanged {
      UpVectorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::topChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_changed(&self) -> TopChanged {
      TopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::viewCenterChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn view_center_changed(&self) -> ViewCenterChanged {
      ViewCenterChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::projectionTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn projection_type_changed(&self) -> ProjectionTypeChanged {
      ProjectionTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::leftChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn left_changed(&self) -> LeftChanged {
      LeftChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::nearPlaneChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn near_plane_changed(&self) -> NearPlaneChanged {
      NearPlaneChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::farPlaneChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn far_plane_changed(&self) -> FarPlaneChanged {
      FarPlaneChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::positionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn position_changed(&self) -> PositionChanged {
      PositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCamera::fieldOfViewChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn field_of_view_changed(&self) -> FieldOfViewChanged {
      FieldOfViewChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Camera`.
  pub struct Slots<'a>(&'a ::camera::Camera);
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setPosition`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_position()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetPosition<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetPosition<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPosition(const QVector3D&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setFarPlane`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_far_plane()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetFarPlane<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetFarPlane<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFarPlane(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setProjectionMatrix`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_projection_matrix()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetProjectionMatrix<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetProjectionMatrix<'a> {
    type Arguments = (&'static ::qt_gui::matrix_4x4::Matrix4X4,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setProjectionMatrix(const QMatrix4x4&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setFieldOfView`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_field_of_view()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetFieldOfView<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetFieldOfView<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFieldOfView(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setUpVector`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_up_vector()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetUpVector<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetUpVector<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setUpVector(const QVector3D&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setProjectionType`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_projection_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetProjectionType<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetProjectionType<'a> {
    type Arguments = (&'static ::camera_lens::ProjectionType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setProjectionType(Qt3DRender::QCameraLens::ProjectionType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setNearPlane`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_near_plane()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetNearPlane<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetNearPlane<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNearPlane(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setAspectRatio`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_aspect_ratio()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetAspectRatio<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetAspectRatio<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAspectRatio(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setBottom`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_bottom()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetBottom<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetBottom<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBottom(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setExposure`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_exposure()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetExposure<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetExposure<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExposure(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setViewCenter`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_view_center()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetViewCenter<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetViewCenter<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setViewCenter(const QVector3D&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setLeft`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_left()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetLeft<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetLeft<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLeft(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setTop`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_top()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetTop<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetTop<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTop(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QCamera::setRight`.
  ///
  /// An object of this type can be created from `Camera` with `object.slots().set_right()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Camera` object.
  pub struct SetRight<'a>(&'a ::camera::Camera);
  impl<'a> ::qt_core::connection::Receiver for SetRight<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRight(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setPosition`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_position(&self) -> SetPosition {
      SetPosition(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setFarPlane`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_far_plane(&self) -> SetFarPlane {
      SetFarPlane(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setProjectionMatrix`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_projection_matrix(&self) -> SetProjectionMatrix {
      SetProjectionMatrix(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setFieldOfView`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_field_of_view(&self) -> SetFieldOfView {
      SetFieldOfView(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setUpVector`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_up_vector(&self) -> SetUpVector {
      SetUpVector(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setProjectionType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_projection_type(&self) -> SetProjectionType {
      SetProjectionType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setNearPlane`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_near_plane(&self) -> SetNearPlane {
      SetNearPlane(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setAspectRatio`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_aspect_ratio(&self) -> SetAspectRatio {
      SetAspectRatio(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setBottom`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bottom(&self) -> SetBottom {
      SetBottom(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setExposure`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_exposure(&self) -> SetExposure {
      SetExposure(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setViewCenter`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_view_center(&self) -> SetViewCenter {
      SetViewCenter(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setLeft`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_left(&self) -> SetLeft {
      SetLeft(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setTop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_top(&self) -> SetTop {
      SetTop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCamera::setRight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_right(&self) -> SetRight {
      SetRight(self.0)
    }
  }
  impl ::camera::Camera {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QCamera::CameraTranslationOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CameraTranslationOption {
  /// C++ enum variant: <span style='color: green;'>```TranslateViewCenter = 0```</span>
  TranslateViewCenter = 0,
  /// C++ enum variant: <span style='color: green;'>```DontTranslateViewCenter = 1```</span>
  DontTranslateViewCenter = 1,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::entity::Entity> for ::camera::Camera {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::camera::Camera) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::camera::Camera as *mut ::camera::Camera) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::camera::Camera {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::camera::Camera) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::camera::Camera as *mut ::camera::Camera) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::camera::Camera {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_QObject_ptr(self as *mut ::camera::Camera) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_QObject_ptr(self as *const ::camera::Camera as *mut ::camera::Camera) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera::Camera> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera::Camera {
    let ffi_result = ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera::Camera {
    let ffi_result = ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera::Camera> for ::qt_3d_core::entity::Entity {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera::Camera {
    let ffi_result = ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QEntity(self as *mut ::qt_3d_core::entity::Entity);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera::Camera {
    let ffi_result = ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QEntity(self as *const ::qt_3d_core::entity::Entity as *mut ::qt_3d_core::entity::Entity);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera::Camera> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera::Camera {
    let ffi_result = ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera::Camera {
    let ffi_result = ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::camera::Camera {
  type Target = ::qt_3d_core::entity::Entity;
  fn deref(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::camera::Camera as *mut ::camera::Camera) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::camera::Camera {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::camera::Camera) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Camera::pan_about_view_center](../struct.Camera.html#method.pan_about_view_center) method.
  pub trait CameraPanAboutViewCenterArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> ();
  }
  impl<'largs> CameraPanAboutViewCenterArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let angle = self;
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_panAboutViewCenter_angle(original_self as *mut ::camera::Camera, angle)
      }
    }
  }
  impl<'largs> CameraPanAboutViewCenterArgs<'largs> for (::libc::c_float, &'largs ::qt_gui::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let angle = self.0;
      let axis = self.1;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_panAboutViewCenter_angle_axis(original_self as *mut ::camera::Camera, angle, axis as *const ::qt_gui::vector_3d::Vector3D) }
    }
  }
  /// This trait represents a set of arguments accepted by [Camera::pan](../struct.Camera.html#method.pan) method.
  pub trait CameraPanArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> ();
  }
  impl<'largs> CameraPanArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let angle = self;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_pan_angle(original_self as *mut ::camera::Camera, angle) }
    }
  }
  impl<'largs> CameraPanArgs<'largs> for (::libc::c_float, &'largs ::qt_gui::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let angle = self.0;
      let axis = self.1;
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_pan_angle_axis(original_self as *mut ::camera::Camera,
                                                                angle,
                                                                axis as *const ::qt_gui::vector_3d::Vector3D)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Camera::translate](../struct.Camera.html#method.translate) method.
  pub trait CameraTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> ();
  }
  impl<'largs> CameraTranslateArgs<'largs> for &'largs ::qt_gui::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let v_local = self;
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_translate_vLocal(original_self as *mut ::camera::Camera,
                                                                  v_local as *const ::qt_gui::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> CameraTranslateArgs<'largs>
    for (&'largs ::qt_gui::vector_3d::Vector3D, ::camera::CameraTranslationOption) {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let v_local = self.0;
      let option = self.1;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_translate_vLocal_option(original_self as *mut ::camera::Camera, v_local as *const ::qt_gui::vector_3d::Vector3D, option) }
    }
  }
  /// This trait represents a set of arguments accepted by [Camera::translate_world](../struct.Camera.html#method.translate_world) method.
  pub trait CameraTranslateWorldArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> ();
  }
  impl<'largs> CameraTranslateWorldArgs<'largs> for &'largs ::qt_gui::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let v_world = self;
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QCamera_translateWorld_vWorld(original_self as *mut ::camera::Camera, v_world as *const ::qt_gui::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> CameraTranslateWorldArgs<'largs>
    for (&'largs ::qt_gui::vector_3d::Vector3D, ::camera::CameraTranslationOption) {
    fn exec(self, original_self: &'largs mut ::camera::Camera) -> () {
      let v_world = self.0;
      let option = self.1;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCamera_translateWorld_vWorld_option(original_self as *mut ::camera::Camera, v_world as *const ::qt_gui::vector_3d::Vector3D, option) }
    }
  }
}
