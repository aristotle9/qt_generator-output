#include "qt_3d_core_c_QTransform.h"

Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return dynamic_cast<Qt3DCore::QTransform*>(ptr);
}

Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_dynamic_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return dynamic_cast<Qt3DCore::QTransform*>(ptr);
}

QObject* qt_3d_core_c_QTransform_G_static_cast_QObject_ptr(Qt3DCore::QTransform* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DCore::QTransform* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QNode_ptr(Qt3DCore::QTransform* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DCore::QTransform*>(ptr);
}

Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DCore::QTransform*>(ptr);
}

Qt3DCore::QTransform* qt_3d_core_c_QTransform_G_static_cast_Qt3DCore_QTransform_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DCore::QTransform*>(ptr);
}

void qt_3d_core_c_Qt3DCore_QTransform_delete(Qt3DCore::QTransform* this_ptr) {
  delete this_ptr;
}

void qt_3d_core_c_Qt3DCore_QTransform_fromAxesAndAngles_to_output_axis1_angle1_axis2_angle2(const QVector3D* axis1, float angle1, const QVector3D* axis2, float angle2, QQuaternion* output) {
  new(output) QQuaternion(Qt3DCore::QTransform::fromAxesAndAngles(*axis1, angle1, *axis2, angle2));
}

void qt_3d_core_c_Qt3DCore_QTransform_fromAxesAndAngles_to_output_axis1_angle1_axis2_angle2_axis3_angle3(const QVector3D* axis1, float angle1, const QVector3D* axis2, float angle2, const QVector3D* axis3, float angle3, QQuaternion* output) {
  new(output) QQuaternion(Qt3DCore::QTransform::fromAxesAndAngles(*axis1, angle1, *axis2, angle2, *axis3, angle3));
}

void qt_3d_core_c_Qt3DCore_QTransform_fromAxisAndAngle_to_output_axis_angle(const QVector3D* axis, float angle, QQuaternion* output) {
  new(output) QQuaternion(Qt3DCore::QTransform::fromAxisAndAngle(*axis, angle));
}

void qt_3d_core_c_Qt3DCore_QTransform_fromAxisAndAngle_to_output_x_y_z_angle(float x, float y, float z, float angle, QQuaternion* output) {
  new(output) QQuaternion(Qt3DCore::QTransform::fromAxisAndAngle(x, y, z, angle));
}

void qt_3d_core_c_Qt3DCore_QTransform_fromEulerAngles_to_output_eulerAngles(const QVector3D* eulerAngles, QQuaternion* output) {
  new(output) QQuaternion(Qt3DCore::QTransform::fromEulerAngles(*eulerAngles));
}

void qt_3d_core_c_Qt3DCore_QTransform_fromEulerAngles_to_output_pitch_yaw_roll(float pitch, float yaw, float roll, QQuaternion* output) {
  new(output) QQuaternion(Qt3DCore::QTransform::fromEulerAngles(pitch, yaw, roll));
}

QMatrix4x4* qt_3d_core_c_Qt3DCore_QTransform_matrix_as_ptr(const Qt3DCore::QTransform* this_ptr) {
  return new QMatrix4x4(this_ptr->matrix());
}

const QMetaObject* qt_3d_core_c_Qt3DCore_QTransform_metaObject(const Qt3DCore::QTransform* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DCore::QTransform* qt_3d_core_c_Qt3DCore_QTransform_new_no_args() {
  return new Qt3DCore::QTransform();
}

Qt3DCore::QTransform* qt_3d_core_c_Qt3DCore_QTransform_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DCore::QTransform(parent);
}

int qt_3d_core_c_Qt3DCore_QTransform_qt_metacall(Qt3DCore::QTransform* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_core_c_Qt3DCore_QTransform_qt_metacast(Qt3DCore::QTransform* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

QMatrix4x4* qt_3d_core_c_Qt3DCore_QTransform_rotateAround_as_ptr(const QVector3D* point, float angle, const QVector3D* axis) {
  return new QMatrix4x4(Qt3DCore::QTransform::rotateAround(*point, angle, *axis));
}

float qt_3d_core_c_Qt3DCore_QTransform_rotationX(const Qt3DCore::QTransform* this_ptr) {
  return this_ptr->rotationX();
}

float qt_3d_core_c_Qt3DCore_QTransform_rotationY(const Qt3DCore::QTransform* this_ptr) {
  return this_ptr->rotationY();
}

float qt_3d_core_c_Qt3DCore_QTransform_rotationZ(const Qt3DCore::QTransform* this_ptr) {
  return this_ptr->rotationZ();
}

void qt_3d_core_c_Qt3DCore_QTransform_rotation_to_output(const Qt3DCore::QTransform* this_ptr, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->rotation());
}

float qt_3d_core_c_Qt3DCore_QTransform_scale(const Qt3DCore::QTransform* this_ptr) {
  return this_ptr->scale();
}

QVector3D* qt_3d_core_c_Qt3DCore_QTransform_scale3D_as_ptr(const Qt3DCore::QTransform* this_ptr) {
  return new QVector3D(this_ptr->scale3D());
}

void qt_3d_core_c_Qt3DCore_QTransform_setMatrix(Qt3DCore::QTransform* this_ptr, const QMatrix4x4* matrix) {
  this_ptr->setMatrix(*matrix);
}

void qt_3d_core_c_Qt3DCore_QTransform_setRotation(Qt3DCore::QTransform* this_ptr, const QQuaternion* rotation) {
  this_ptr->setRotation(*rotation);
}

void qt_3d_core_c_Qt3DCore_QTransform_setRotationX(Qt3DCore::QTransform* this_ptr, float rotationX) {
  this_ptr->setRotationX(rotationX);
}

void qt_3d_core_c_Qt3DCore_QTransform_setRotationY(Qt3DCore::QTransform* this_ptr, float rotationY) {
  this_ptr->setRotationY(rotationY);
}

void qt_3d_core_c_Qt3DCore_QTransform_setRotationZ(Qt3DCore::QTransform* this_ptr, float rotationZ) {
  this_ptr->setRotationZ(rotationZ);
}

void qt_3d_core_c_Qt3DCore_QTransform_setScale(Qt3DCore::QTransform* this_ptr, float scale) {
  this_ptr->setScale(scale);
}

void qt_3d_core_c_Qt3DCore_QTransform_setScale3D(Qt3DCore::QTransform* this_ptr, const QVector3D* scale) {
  this_ptr->setScale3D(*scale);
}

void qt_3d_core_c_Qt3DCore_QTransform_setTranslation(Qt3DCore::QTransform* this_ptr, const QVector3D* translation) {
  this_ptr->setTranslation(*translation);
}

void qt_3d_core_c_Qt3DCore_QTransform_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QTransform::trUtf8(s, c, n));
}

void qt_3d_core_c_Qt3DCore_QTransform_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DCore::QTransform::tr(s, c, n));
}

QVector3D* qt_3d_core_c_Qt3DCore_QTransform_translation_as_ptr(const Qt3DCore::QTransform* this_ptr) {
  return new QVector3D(this_ptr->translation());
}

