/// C++ type: <span style='color: green;'>```Qt3DCore::QTransform```</span>
#[repr(C)]
pub struct Transform(u8);

impl Transform {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QTransform::fromAxesAndAngles```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_axes_and_angles((&::qt_gui::vector_3d::Vector3D, ::libc::c_float, &::qt_gui::vector_3d::Vector3D, ::libc::c_float)) -> ::qt_gui::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion Qt3DCore::QTransform::fromAxesAndAngles(const QVector3D& axis1, float angle1, const QVector3D& axis2, float angle2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_axes_and_angles((&::qt_gui::vector_3d::Vector3D, ::libc::c_float, &::qt_gui::vector_3d::Vector3D, ::libc::c_float, &::qt_gui::vector_3d::Vector3D, ::libc::c_float)) -> ::qt_gui::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion Qt3DCore::QTransform::fromAxesAndAngles(const QVector3D& axis1, float angle1, const QVector3D& axis2, float angle2, const QVector3D& axis3, float angle3)```</span>
  ///
  ///
  pub fn from_axes_and_angles<Args>(args: Args) -> ::qt_gui::quaternion::Quaternion
    where Args: overloading::TransformFromAxesAndAnglesArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QTransform::fromAxisAndAngle```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_axis_and_angle((&::qt_gui::vector_3d::Vector3D, ::libc::c_float)) -> ::qt_gui::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion Qt3DCore::QTransform::fromAxisAndAngle(const QVector3D& axis, float angle)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_axis_and_angle((::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::qt_gui::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion Qt3DCore::QTransform::fromAxisAndAngle(float x, float y, float z, float angle)```</span>
  ///
  ///
  pub fn from_axis_and_angle<Args>(args: Args) -> ::qt_gui::quaternion::Quaternion
    where Args: overloading::TransformFromAxisAndAngleArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QTransform::fromEulerAngles```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_euler_angles(&::qt_gui::vector_3d::Vector3D) -> ::qt_gui::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion Qt3DCore::QTransform::fromEulerAngles(const QVector3D& eulerAngles)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_euler_angles((::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::qt_gui::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion Qt3DCore::QTransform::fromEulerAngles(float pitch, float yaw, float roll)```</span>
  ///
  ///
  pub fn from_euler_angles<Args>(args: Args) -> ::qt_gui::quaternion::Quaternion
    where Args: overloading::TransformFromEulerAnglesArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMatrix4x4 Qt3DCore::QTransform::matrix() const```</span>
  ///
  ///
  pub fn matrix(&self) -> ::cpp_utils::CppBox<::qt_gui::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_matrix_as_ptr(self as *const ::transform::Transform) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DCore::QTransform::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_metaObject(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QTransform::QTransform()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::transform::Transform> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QTransform::QTransform(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::node::Node) -> ::cpp_utils::CppBox<::transform::Transform> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QTransform_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DCore::QTransform::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_core_c_Qt3DCore_QTransform_qt_metacall(self as *mut ::transform::Transform,
                                                        arg1 as *const ::qt_core::meta_object::Call,
                                                        arg2,
                                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DCore::QTransform::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_core_c_Qt3DCore_QTransform_qt_metacast(self as *mut ::transform::Transform, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QMatrix4x4 Qt3DCore::QTransform::rotateAround(const QVector3D& point, float angle, const QVector3D& axis)```</span>
  ///
  ///
  pub fn rotate_around(point: &::qt_gui::vector_3d::Vector3D,
                       angle: ::libc::c_float,
                       axis: &::qt_gui::vector_3d::Vector3D)
                       -> ::cpp_utils::CppBox<::qt_gui::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QTransform_rotateAround_as_ptr(point as *const ::qt_gui::vector_3d::Vector3D,
                                                                    angle,
                                                                    axis as *const ::qt_gui::vector_3d::Vector3D)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion Qt3DCore::QTransform::rotation() const```</span>
  ///
  ///
  pub fn rotation(&self) -> ::qt_gui::quaternion::Quaternion {
    {
      let mut object: ::qt_gui::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QTransform_rotation_to_output(self as *const ::transform::Transform, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DCore::QTransform::rotationX() const```</span>
  ///
  ///
  pub fn rotation_x(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_rotationX(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DCore::QTransform::rotationY() const```</span>
  ///
  ///
  pub fn rotation_y(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_rotationY(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DCore::QTransform::rotationZ() const```</span>
  ///
  ///
  pub fn rotation_z(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_rotationZ(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DCore::QTransform::scale() const```</span>
  ///
  ///
  pub fn scale(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_scale(self as *const ::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DCore::QTransform::scale3D() const```</span>
  ///
  ///
  pub fn scale_3d(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_scale3D_as_ptr(self as *const ::transform::Transform) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setMatrix(const QMatrix4x4& matrix)```</span>
  ///
  ///
  pub fn set_matrix(&mut self, matrix: &::qt_gui::matrix_4x4::Matrix4X4) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setMatrix(self as *mut ::transform::Transform,
                                                        matrix as *const ::qt_gui::matrix_4x4::Matrix4X4)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setRotation(const QQuaternion& rotation)```</span>
  ///
  ///
  pub fn set_rotation(&mut self, rotation: &::qt_gui::quaternion::Quaternion) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setRotation(self as *mut ::transform::Transform,
                                                          rotation as *const ::qt_gui::quaternion::Quaternion)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setRotationX(float rotationX)```</span>
  ///
  ///
  pub fn set_rotation_x(&mut self, rotation_x: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setRotationX(self as *mut ::transform::Transform, rotation_x) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setRotationY(float rotationY)```</span>
  ///
  ///
  pub fn set_rotation_y(&mut self, rotation_y: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setRotationY(self as *mut ::transform::Transform, rotation_y) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setRotationZ(float rotationZ)```</span>
  ///
  ///
  pub fn set_rotation_z(&mut self, rotation_z: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setRotationZ(self as *mut ::transform::Transform, rotation_z) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setScale(float scale)```</span>
  ///
  ///
  pub fn set_scale(&mut self, scale: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setScale(self as *mut ::transform::Transform, scale) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setScale3D(const QVector3D& scale)```</span>
  ///
  ///
  pub fn set_scale_3d(&mut self, scale: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setScale3D(self as *mut ::transform::Transform,
                                                         scale as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QTransform::setTranslation(const QVector3D& translation)```</span>
  ///
  ///
  pub fn set_translation(&mut self, translation: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QTransform_setTranslation(self as *mut ::transform::Transform,
                                                             translation as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QTransform::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QTransform_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QTransform::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QTransform_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DCore::QTransform::translation() const```</span>
  ///
  ///
  pub fn translation(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QTransform_translation_as_ptr(self as *const ::transform::Transform) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::transform::Transform {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QTransform_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Transform`.
  pub struct Signals<'a>(&'a ::transform::Transform);
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::matrixChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().matrix_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct MatrixChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for MatrixChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2matrixChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MatrixChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::rotationZChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().rotation_z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct RotationZChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for RotationZChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rotationZChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RotationZChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::translationChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().translation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct TranslationChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for TranslationChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2translationChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TranslationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::shareableChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct ShareableChanged<'a>(&'a ::transform::Transform);
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
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::scale3DChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().scale_3d_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct Scale3DChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for Scale3DChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scale3DChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Scale3DChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::removedFromEntity`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct RemovedFromEntity<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::rotationXChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().rotation_x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct RotationXChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for RotationXChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rotationXChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RotationXChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::rotationYChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().rotation_y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct RotationYChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for RotationYChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rotationYChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RotationYChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::rotationChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().rotation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct RotationChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for RotationChanged<'a> {
    type Arguments = (&'static ::qt_gui::quaternion::Quaternion,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rotationChanged(const QQuaternion&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RotationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::addedToEntity`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct AddedToEntity<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QTransform::scaleChanged`.
  ///
  /// An object of this type can be created from `Transform` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct ScaleChanged<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for ScaleChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::matrixChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn matrix_changed(&self) -> MatrixChanged {
      MatrixChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::rotationZChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_z_changed(&self) -> RotationZChanged {
      RotationZChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::translationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn translation_changed(&self) -> TranslationChanged {
      TranslationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::scale3DChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_3d_changed(&self) -> Scale3DChanged {
      Scale3DChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::rotationXChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_x_changed(&self) -> RotationXChanged {
      RotationXChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::rotationYChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_y_changed(&self) -> RotationYChanged {
      RotationYChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::rotationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_changed(&self) -> RotationChanged {
      RotationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QTransform::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Transform`.
  pub struct Slots<'a>(&'a ::transform::Transform);
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setShareable`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetShareable<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setRotation`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_rotation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetRotation<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetRotation<'a> {
    type Arguments = (&'static ::qt_gui::quaternion::Quaternion,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRotation(const QQuaternion&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setTranslation`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_translation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetTranslation<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetTranslation<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTranslation(const QVector3D&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setScale3D`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_scale_3d()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetScale3D<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetScale3D<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setScale3D(const QVector3D&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setRotationY`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_rotation_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetRotationY<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetRotationY<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRotationY(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setScale`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_scale()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetScale<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetScale<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setScale(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setRotationX`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_rotation_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetRotationX<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetRotationX<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRotationX(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setMatrix`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_matrix()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetMatrix<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetMatrix<'a> {
    type Arguments = (&'static ::qt_gui::matrix_4x4::Matrix4X4,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMatrix(const QMatrix4x4&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QTransform::setRotationZ`.
  ///
  /// An object of this type can be created from `Transform` with `object.slots().set_rotation_z()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Transform` object.
  pub struct SetRotationZ<'a>(&'a ::transform::Transform);
  impl<'a> ::qt_core::connection::Receiver for SetRotationZ<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRotationZ(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setRotation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rotation(&self) -> SetRotation {
      SetRotation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setTranslation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_translation(&self) -> SetTranslation {
      SetTranslation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setScale3D`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_scale_3d(&self) -> SetScale3D {
      SetScale3D(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setRotationY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rotation_y(&self) -> SetRotationY {
      SetRotationY(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setScale`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_scale(&self) -> SetScale {
      SetScale(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setRotationX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rotation_x(&self) -> SetRotationX {
      SetRotationX(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setMatrix`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_matrix(&self) -> SetMatrix {
      SetMatrix(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QTransform::setRotationZ`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rotation_z(&self) -> SetRotationZ {
      SetRotationZ(self.0)
    }
  }
  impl ::transform::Transform {
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

impl ::cpp_utils::DynamicCast<::transform::Transform> for ::component::Component {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::transform::Transform> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(self as *mut ::component::Component) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::transform::Transform> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(self as *const ::component::Component as *mut ::component::Component) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::transform::Transform> for ::node::Node {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::transform::Transform> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(self as *mut ::node::Node)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::transform::Transform> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(self as *const ::node::Node as *mut ::node::Node) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::component::Component> for ::transform::Transform {
  fn static_cast_mut(&mut self) -> &mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::transform::Transform)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::transform::Transform as *mut ::transform::Transform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::node::Node> for ::transform::Transform {
  fn static_cast_mut(&mut self) -> &mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::transform::Transform) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::transform::Transform as *mut ::transform::Transform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::transform::Transform {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QTransform_G_static_cast_QObject_ptr(self as *mut ::transform::Transform) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_static_cast_QObject_ptr(self as *const ::transform::Transform as *mut ::transform::Transform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::transform::Transform> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::transform::Transform {
    let ffi_result = ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::transform::Transform {
    let ffi_result = ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::transform::Transform> for ::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::transform::Transform {
    let ffi_result = ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(self as *mut ::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::transform::Transform {
    let ffi_result = ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(self as *const ::component::Component as *mut ::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::transform::Transform> for ::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::transform::Transform {
    let ffi_result =
      ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(self as *mut ::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::transform::Transform {
    let ffi_result = ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(self as *const ::node::Node as *mut ::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::transform::Transform {
  type Target = ::component::Component;
  fn deref(&self) -> &::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::transform::Transform as *mut ::transform::Transform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::transform::Transform {
  fn deref_mut(&mut self) -> &mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::transform::Transform)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Transform::from_axes_and_angles](../struct.Transform.html#method.from_axes_and_angles) method.
  pub trait TransformFromAxesAndAnglesArgs {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion;
  }
  impl<'a> TransformFromAxesAndAnglesArgs
    for (&'a ::qt_gui::vector_3d::Vector3D, ::libc::c_float, &'a ::qt_gui::vector_3d::Vector3D, ::libc::c_float) {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion {
      let axis1 = self.0;
      let angle1 = self.1;
      let axis2 = self.2;
      let angle2 = self.3;
      {
        let mut object: ::qt_gui::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_Qt3DCore_QTransform_fromAxesAndAngles_to_output_axis1_angle1_axis2_angle2(axis1 as *const ::qt_gui::vector_3d::Vector3D, angle1, axis2 as *const ::qt_gui::vector_3d::Vector3D, angle2, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TransformFromAxesAndAnglesArgs
    for (&'a ::qt_gui::vector_3d::Vector3D,
                                                   ::libc::c_float,
                                                   &'a ::qt_gui::vector_3d::Vector3D,
                                                   ::libc::c_float,
                                                   &'a ::qt_gui::vector_3d::Vector3D,
                                                   ::libc::c_float) {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion {
      let axis1 = self.0;
      let angle1 = self.1;
      let axis2 = self.2;
      let angle2 = self.3;
      let axis3 = self.4;
      let angle3 = self.5;
      {
        let mut object: ::qt_gui::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_Qt3DCore_QTransform_fromAxesAndAngles_to_output_axis1_angle1_axis2_angle2_axis3_angle3(axis1 as *const ::qt_gui::vector_3d::Vector3D, angle1, axis2 as *const ::qt_gui::vector_3d::Vector3D, angle2, axis3 as *const ::qt_gui::vector_3d::Vector3D, angle3, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::from_axis_and_angle](../struct.Transform.html#method.from_axis_and_angle) method.
  pub trait TransformFromAxisAndAngleArgs {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion;
  }
  impl<'a> TransformFromAxisAndAngleArgs for (&'a ::qt_gui::vector_3d::Vector3D, ::libc::c_float) {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion {
      let axis = self.0;
      let angle = self.1;
      {
        let mut object: ::qt_gui::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_Qt3DCore_QTransform_fromAxisAndAngle_to_output_axis_angle(axis as *const ::qt_gui::vector_3d::Vector3D, angle, &mut object);
        }
        object
      }
    }
  }
  impl TransformFromAxisAndAngleArgs for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      let angle = self.3;
      {
        let mut object: ::qt_gui::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_Qt3DCore_QTransform_fromAxisAndAngle_to_output_x_y_z_angle(x, y, z, angle, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Transform::from_euler_angles](../struct.Transform.html#method.from_euler_angles) method.
  pub trait TransformFromEulerAnglesArgs {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion;
  }
  impl<'a> TransformFromEulerAnglesArgs for &'a ::qt_gui::vector_3d::Vector3D {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion {
      let euler_angles = self;
      {
        let mut object: ::qt_gui::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_Qt3DCore_QTransform_fromEulerAngles_to_output_eulerAngles(euler_angles as *const ::qt_gui::vector_3d::Vector3D, &mut object);
        }
        object
      }
    }
  }
  impl TransformFromEulerAnglesArgs for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::qt_gui::quaternion::Quaternion {
      let pitch = self.0;
      let yaw = self.1;
      let roll = self.2;
      {
        let mut object: ::qt_gui::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_Qt3DCore_QTransform_fromEulerAngles_to_output_pitch_yaw_roll(pitch,
                                                                                           yaw,
                                                                                           roll,
                                                                                           &mut object);
        }
        object
      }
    }
  }
}
