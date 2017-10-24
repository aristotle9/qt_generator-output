#include "qt_3d_render_c_QCamera.h"

QObject* qt_3d_render_c_QCamera_G_static_cast_QObject_ptr(Qt3DRender::QCamera* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QEntity* qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QEntity_ptr(Qt3DRender::QCamera* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QCamera_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QCamera* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QCamera* qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QCamera*>(ptr);
}

Qt3DRender::QCamera* qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QEntity(Qt3DCore::QEntity* ptr) {
  return static_cast<Qt3DRender::QCamera*>(ptr);
}

Qt3DRender::QCamera* qt_3d_render_c_QCamera_G_static_cast_Qt3DRender_QCamera_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QCamera*>(ptr);
}

float qt_3d_render_c_Qt3DRender_QCamera_aspectRatio(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->aspectRatio();
}

float qt_3d_render_c_Qt3DRender_QCamera_bottom(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->bottom();
}

void qt_3d_render_c_Qt3DRender_QCamera_delete(Qt3DRender::QCamera* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QCamera_exposure(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->exposure();
}

float qt_3d_render_c_Qt3DRender_QCamera_farPlane(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->farPlane();
}

float qt_3d_render_c_Qt3DRender_QCamera_fieldOfView(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->fieldOfView();
}

float qt_3d_render_c_Qt3DRender_QCamera_left(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->left();
}

Qt3DRender::QCameraLens* qt_3d_render_c_Qt3DRender_QCamera_lens(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->lens();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QCamera_metaObject(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->metaObject();
}

float qt_3d_render_c_Qt3DRender_QCamera_nearPlane(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->nearPlane();
}

Qt3DRender::QCamera* qt_3d_render_c_Qt3DRender_QCamera_new_no_args() {
  return new Qt3DRender::QCamera();
}

Qt3DRender::QCamera* qt_3d_render_c_Qt3DRender_QCamera_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QCamera(parent);
}

void qt_3d_render_c_Qt3DRender_QCamera_panAboutViewCenter_angle(Qt3DRender::QCamera* this_ptr, float angle) {
  this_ptr->panAboutViewCenter(angle);
}

void qt_3d_render_c_Qt3DRender_QCamera_panAboutViewCenter_angle_axis(Qt3DRender::QCamera* this_ptr, float angle, const QVector3D* axis) {
  this_ptr->panAboutViewCenter(angle, *axis);
}

void qt_3d_render_c_Qt3DRender_QCamera_panRotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->panRotation(angle));
}

void qt_3d_render_c_Qt3DRender_QCamera_pan_angle(Qt3DRender::QCamera* this_ptr, float angle) {
  this_ptr->pan(angle);
}

void qt_3d_render_c_Qt3DRender_QCamera_pan_angle_axis(Qt3DRender::QCamera* this_ptr, float angle, const QVector3D* axis) {
  this_ptr->pan(angle, *axis);
}

QVector3D* qt_3d_render_c_Qt3DRender_QCamera_position_as_ptr(const Qt3DRender::QCamera* this_ptr) {
  return new QVector3D(this_ptr->position());
}

QMatrix4x4* qt_3d_render_c_Qt3DRender_QCamera_projectionMatrix_as_ptr(const Qt3DRender::QCamera* this_ptr) {
  return new QMatrix4x4(this_ptr->projectionMatrix());
}

int qt_3d_render_c_Qt3DRender_QCamera_qt_metacall(Qt3DRender::QCamera* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QCamera_qt_metacast(Qt3DRender::QCamera* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_render_c_Qt3DRender_QCamera_right(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->right();
}

void qt_3d_render_c_Qt3DRender_QCamera_roll(Qt3DRender::QCamera* this_ptr, float angle) {
  this_ptr->roll(angle);
}

void qt_3d_render_c_Qt3DRender_QCamera_rollAboutViewCenter(Qt3DRender::QCamera* this_ptr, float angle) {
  this_ptr->rollAboutViewCenter(angle);
}

void qt_3d_render_c_Qt3DRender_QCamera_rollRotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->rollRotation(angle));
}

void qt_3d_render_c_Qt3DRender_QCamera_rotate(Qt3DRender::QCamera* this_ptr, const QQuaternion* q) {
  this_ptr->rotate(*q);
}

void qt_3d_render_c_Qt3DRender_QCamera_rotateAboutViewCenter(Qt3DRender::QCamera* this_ptr, const QQuaternion* q) {
  this_ptr->rotateAboutViewCenter(*q);
}

void qt_3d_render_c_Qt3DRender_QCamera_rotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, const QVector3D* axis, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->rotation(angle, *axis));
}

void qt_3d_render_c_Qt3DRender_QCamera_setAspectRatio(Qt3DRender::QCamera* this_ptr, float aspectRatio) {
  this_ptr->setAspectRatio(aspectRatio);
}

void qt_3d_render_c_Qt3DRender_QCamera_setBottom(Qt3DRender::QCamera* this_ptr, float bottom) {
  this_ptr->setBottom(bottom);
}

void qt_3d_render_c_Qt3DRender_QCamera_setExposure(Qt3DRender::QCamera* this_ptr, float exposure) {
  this_ptr->setExposure(exposure);
}

void qt_3d_render_c_Qt3DRender_QCamera_setFarPlane(Qt3DRender::QCamera* this_ptr, float farPlane) {
  this_ptr->setFarPlane(farPlane);
}

void qt_3d_render_c_Qt3DRender_QCamera_setFieldOfView(Qt3DRender::QCamera* this_ptr, float fieldOfView) {
  this_ptr->setFieldOfView(fieldOfView);
}

void qt_3d_render_c_Qt3DRender_QCamera_setLeft(Qt3DRender::QCamera* this_ptr, float left) {
  this_ptr->setLeft(left);
}

void qt_3d_render_c_Qt3DRender_QCamera_setNearPlane(Qt3DRender::QCamera* this_ptr, float nearPlane) {
  this_ptr->setNearPlane(nearPlane);
}

void qt_3d_render_c_Qt3DRender_QCamera_setPosition(Qt3DRender::QCamera* this_ptr, const QVector3D* position) {
  this_ptr->setPosition(*position);
}

void qt_3d_render_c_Qt3DRender_QCamera_setProjectionMatrix(Qt3DRender::QCamera* this_ptr, const QMatrix4x4* projectionMatrix) {
  this_ptr->setProjectionMatrix(*projectionMatrix);
}

void qt_3d_render_c_Qt3DRender_QCamera_setProjectionType(Qt3DRender::QCamera* this_ptr, const Qt3DRender::QCameraLens::ProjectionType* type) {
  this_ptr->setProjectionType(*type);
}

void qt_3d_render_c_Qt3DRender_QCamera_setRight(Qt3DRender::QCamera* this_ptr, float right) {
  this_ptr->setRight(right);
}

void qt_3d_render_c_Qt3DRender_QCamera_setTop(Qt3DRender::QCamera* this_ptr, float top) {
  this_ptr->setTop(top);
}

void qt_3d_render_c_Qt3DRender_QCamera_setUpVector(Qt3DRender::QCamera* this_ptr, const QVector3D* upVector) {
  this_ptr->setUpVector(*upVector);
}

void qt_3d_render_c_Qt3DRender_QCamera_setViewCenter(Qt3DRender::QCamera* this_ptr, const QVector3D* viewCenter) {
  this_ptr->setViewCenter(*viewCenter);
}

void qt_3d_render_c_Qt3DRender_QCamera_tilt(Qt3DRender::QCamera* this_ptr, float angle) {
  this_ptr->tilt(angle);
}

void qt_3d_render_c_Qt3DRender_QCamera_tiltAboutViewCenter(Qt3DRender::QCamera* this_ptr, float angle) {
  this_ptr->tiltAboutViewCenter(angle);
}

void qt_3d_render_c_Qt3DRender_QCamera_tiltRotation_to_output(const Qt3DRender::QCamera* this_ptr, float angle, QQuaternion* output) {
  new(output) QQuaternion(this_ptr->tiltRotation(angle));
}

float qt_3d_render_c_Qt3DRender_QCamera_top(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->top();
}

void qt_3d_render_c_Qt3DRender_QCamera_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCamera::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QCamera_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCamera::tr(s, c, n));
}

Qt3DCore::QTransform* qt_3d_render_c_Qt3DRender_QCamera_transform(const Qt3DRender::QCamera* this_ptr) {
  return this_ptr->transform();
}

void qt_3d_render_c_Qt3DRender_QCamera_translateWorld_vWorld(Qt3DRender::QCamera* this_ptr, const QVector3D* vWorld) {
  this_ptr->translateWorld(*vWorld);
}

void qt_3d_render_c_Qt3DRender_QCamera_translateWorld_vWorld_option(Qt3DRender::QCamera* this_ptr, const QVector3D* vWorld, Qt3DRender::QCamera::CameraTranslationOption option) {
  this_ptr->translateWorld(*vWorld, option);
}

void qt_3d_render_c_Qt3DRender_QCamera_translate_vLocal(Qt3DRender::QCamera* this_ptr, const QVector3D* vLocal) {
  this_ptr->translate(*vLocal);
}

void qt_3d_render_c_Qt3DRender_QCamera_translate_vLocal_option(Qt3DRender::QCamera* this_ptr, const QVector3D* vLocal, Qt3DRender::QCamera::CameraTranslationOption option) {
  this_ptr->translate(*vLocal, option);
}

QVector3D* qt_3d_render_c_Qt3DRender_QCamera_upVector_as_ptr(const Qt3DRender::QCamera* this_ptr) {
  return new QVector3D(this_ptr->upVector());
}

QVector3D* qt_3d_render_c_Qt3DRender_QCamera_viewCenter_as_ptr(const Qt3DRender::QCamera* this_ptr) {
  return new QVector3D(this_ptr->viewCenter());
}

QMatrix4x4* qt_3d_render_c_Qt3DRender_QCamera_viewMatrix_as_ptr(const Qt3DRender::QCamera* this_ptr) {
  return new QMatrix4x4(this_ptr->viewMatrix());
}

QVector3D* qt_3d_render_c_Qt3DRender_QCamera_viewVector_as_ptr(const Qt3DRender::QCamera* this_ptr) {
  return new QVector3D(this_ptr->viewVector());
}

