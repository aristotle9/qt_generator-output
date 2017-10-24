#include "qt_3d_render_c_QDirectionalLight.h"

Qt3DRender::QDirectionalLight* qt_3d_render_c_QDirectionalLight_G_dynamic_cast_Qt3DRender_QDirectionalLight_ptr(Qt3DRender::QAbstractLight* ptr) {
  return dynamic_cast<Qt3DRender::QDirectionalLight*>(ptr);
}

QObject* qt_3d_render_c_QDirectionalLight_G_static_cast_QObject_ptr(Qt3DRender::QDirectionalLight* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QDirectionalLight* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QDirectionalLight* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractLight* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(Qt3DRender::QDirectionalLight* ptr) {
  return static_cast<Qt3DRender::QAbstractLight*>(ptr);
}

Qt3DRender::QDirectionalLight* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QDirectionalLight*>(ptr);
}

Qt3DRender::QDirectionalLight* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QDirectionalLight*>(ptr);
}

Qt3DRender::QDirectionalLight* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QDirectionalLight*>(ptr);
}

Qt3DRender::QDirectionalLight* qt_3d_render_c_QDirectionalLight_G_static_cast_Qt3DRender_QDirectionalLight_ptr_Qt3DRender_QAbstractLight(Qt3DRender::QAbstractLight* ptr) {
  return static_cast<Qt3DRender::QDirectionalLight*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QDirectionalLight_delete(Qt3DRender::QDirectionalLight* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QDirectionalLight_metaObject(const Qt3DRender::QDirectionalLight* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QDirectionalLight* qt_3d_render_c_Qt3DRender_QDirectionalLight_new_no_args() {
  return new Qt3DRender::QDirectionalLight();
}

Qt3DRender::QDirectionalLight* qt_3d_render_c_Qt3DRender_QDirectionalLight_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QDirectionalLight(parent);
}

int qt_3d_render_c_Qt3DRender_QDirectionalLight_qt_metacall(Qt3DRender::QDirectionalLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QDirectionalLight_qt_metacast(Qt3DRender::QDirectionalLight* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QDirectionalLight_setWorldDirection(Qt3DRender::QDirectionalLight* this_ptr, const QVector3D* worldDirection) {
  this_ptr->setWorldDirection(*worldDirection);
}

void qt_3d_render_c_Qt3DRender_QDirectionalLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QDirectionalLight::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QDirectionalLight_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QDirectionalLight::tr(s, c, n));
}

QVector3D* qt_3d_render_c_Qt3DRender_QDirectionalLight_worldDirection_as_ptr(const Qt3DRender::QDirectionalLight* this_ptr) {
  return new QVector3D(this_ptr->worldDirection());
}

