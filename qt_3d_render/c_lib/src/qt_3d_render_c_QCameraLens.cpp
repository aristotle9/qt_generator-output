#include "qt_3d_render_c_QCameraLens.h"

QObject* qt_3d_render_c_QCameraLens_G_static_cast_QObject_ptr(Qt3DRender::QCameraLens* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QCameraLens* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QCameraLens* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QCameraLens* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QCameraLens*>(ptr);
}

Qt3DRender::QCameraLens* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QCameraLens*>(ptr);
}

Qt3DRender::QCameraLens* qt_3d_render_c_QCameraLens_G_static_cast_Qt3DRender_QCameraLens_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QCameraLens*>(ptr);
}

float qt_3d_render_c_Qt3DRender_QCameraLens_aspectRatio(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->aspectRatio();
}

float qt_3d_render_c_Qt3DRender_QCameraLens_bottom(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->bottom();
}

void qt_3d_render_c_Qt3DRender_QCameraLens_delete(Qt3DRender::QCameraLens* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QCameraLens_exposure(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->exposure();
}

float qt_3d_render_c_Qt3DRender_QCameraLens_farPlane(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->farPlane();
}

float qt_3d_render_c_Qt3DRender_QCameraLens_fieldOfView(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->fieldOfView();
}

float qt_3d_render_c_Qt3DRender_QCameraLens_left(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->left();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QCameraLens_metaObject(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->metaObject();
}

float qt_3d_render_c_Qt3DRender_QCameraLens_nearPlane(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->nearPlane();
}

Qt3DRender::QCameraLens* qt_3d_render_c_Qt3DRender_QCameraLens_new_no_args() {
  return new Qt3DRender::QCameraLens();
}

Qt3DRender::QCameraLens* qt_3d_render_c_Qt3DRender_QCameraLens_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QCameraLens(parent);
}

QMatrix4x4* qt_3d_render_c_Qt3DRender_QCameraLens_projectionMatrix_as_ptr(const Qt3DRender::QCameraLens* this_ptr) {
  return new QMatrix4x4(this_ptr->projectionMatrix());
}

Qt3DRender::QCameraLens::ProjectionType qt_3d_render_c_Qt3DRender_QCameraLens_projectionType(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->projectionType();
}

int qt_3d_render_c_Qt3DRender_QCameraLens_qt_metacall(Qt3DRender::QCameraLens* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QCameraLens_qt_metacast(Qt3DRender::QCameraLens* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_render_c_Qt3DRender_QCameraLens_right(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->right();
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setAspectRatio(Qt3DRender::QCameraLens* this_ptr, float aspectRatio) {
  this_ptr->setAspectRatio(aspectRatio);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setBottom(Qt3DRender::QCameraLens* this_ptr, float bottom) {
  this_ptr->setBottom(bottom);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setExposure(Qt3DRender::QCameraLens* this_ptr, float exposure) {
  this_ptr->setExposure(exposure);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setFarPlane(Qt3DRender::QCameraLens* this_ptr, float farPlane) {
  this_ptr->setFarPlane(farPlane);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setFieldOfView(Qt3DRender::QCameraLens* this_ptr, float fieldOfView) {
  this_ptr->setFieldOfView(fieldOfView);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setFrustumProjection(Qt3DRender::QCameraLens* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane) {
  this_ptr->setFrustumProjection(left, right, bottom, top, nearPlane, farPlane);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setLeft(Qt3DRender::QCameraLens* this_ptr, float left) {
  this_ptr->setLeft(left);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setNearPlane(Qt3DRender::QCameraLens* this_ptr, float nearPlane) {
  this_ptr->setNearPlane(nearPlane);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setOrthographicProjection(Qt3DRender::QCameraLens* this_ptr, float left, float right, float bottom, float top, float nearPlane, float farPlane) {
  this_ptr->setOrthographicProjection(left, right, bottom, top, nearPlane, farPlane);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setPerspectiveProjection(Qt3DRender::QCameraLens* this_ptr, float fieldOfView, float aspect, float nearPlane, float farPlane) {
  this_ptr->setPerspectiveProjection(fieldOfView, aspect, nearPlane, farPlane);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setProjectionMatrix(Qt3DRender::QCameraLens* this_ptr, const QMatrix4x4* projectionMatrix) {
  this_ptr->setProjectionMatrix(*projectionMatrix);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setProjectionType(Qt3DRender::QCameraLens* this_ptr, Qt3DRender::QCameraLens::ProjectionType projectionType) {
  this_ptr->setProjectionType(projectionType);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setRight(Qt3DRender::QCameraLens* this_ptr, float right) {
  this_ptr->setRight(right);
}

void qt_3d_render_c_Qt3DRender_QCameraLens_setTop(Qt3DRender::QCameraLens* this_ptr, float top) {
  this_ptr->setTop(top);
}

float qt_3d_render_c_Qt3DRender_QCameraLens_top(const Qt3DRender::QCameraLens* this_ptr) {
  return this_ptr->top();
}

void qt_3d_render_c_Qt3DRender_QCameraLens_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCameraLens::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QCameraLens_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCameraLens::tr(s, c, n));
}

