#include "qt_3d_render_c_QAbstractLight.h"

QObject* qt_3d_render_c_QAbstractLight_G_static_cast_QObject_ptr(Qt3DRender::QAbstractLight* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QAbstractLight* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAbstractLight* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractLight* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QAbstractLight*>(ptr);
}

Qt3DRender::QAbstractLight* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QAbstractLight*>(ptr);
}

Qt3DRender::QAbstractLight* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QAbstractLight*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QAbstractLight_color_to_output(const Qt3DRender::QAbstractLight* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->color());
}

void qt_3d_render_c_Qt3DRender_QAbstractLight_delete(Qt3DRender::QAbstractLight* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QAbstractLight_intensity(const Qt3DRender::QAbstractLight* this_ptr) {
  return this_ptr->intensity();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QAbstractLight_metaObject(const Qt3DRender::QAbstractLight* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QAbstractLight_qt_metacall(Qt3DRender::QAbstractLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QAbstractLight_qt_metacast(Qt3DRender::QAbstractLight* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QAbstractLight_setColor(Qt3DRender::QAbstractLight* this_ptr, const QColor* color) {
  this_ptr->setColor(*color);
}

void qt_3d_render_c_Qt3DRender_QAbstractLight_setIntensity(Qt3DRender::QAbstractLight* this_ptr, float intensity) {
  this_ptr->setIntensity(intensity);
}

void qt_3d_render_c_Qt3DRender_QAbstractLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAbstractLight::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QAbstractLight_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAbstractLight::tr(s, c, n));
}

Qt3DRender::QAbstractLight::Type qt_3d_render_c_Qt3DRender_QAbstractLight_type(const Qt3DRender::QAbstractLight* this_ptr) {
  return this_ptr->type();
}

