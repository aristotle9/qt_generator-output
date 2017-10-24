#include "qt_3d_render_c_QParameter.h"

QObject* qt_3d_render_c_QParameter_G_static_cast_QObject_ptr(Qt3DRender::QParameter* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QParameter_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QParameter* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QParameter* qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QParameter*>(ptr);
}

Qt3DRender::QParameter* qt_3d_render_c_QParameter_G_static_cast_Qt3DRender_QParameter_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QParameter*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QParameter_delete(Qt3DRender::QParameter* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QParameter_metaObject(const Qt3DRender::QParameter* this_ptr) {
  return this_ptr->metaObject();
}

void qt_3d_render_c_Qt3DRender_QParameter_name_to_output(const Qt3DRender::QParameter* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_texture(const QString* name, Qt3DRender::QAbstractTexture* texture) {
  return new Qt3DRender::QParameter(*name, texture);
}

Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_texture_parent(const QString* name, Qt3DRender::QAbstractTexture* texture, Qt3DCore::QNode* parent) {
  return new Qt3DRender::QParameter(*name, texture, parent);
}

Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_value(const QString* name, const QVariant* value) {
  return new Qt3DRender::QParameter(*name, *value);
}

Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_name_value_parent(const QString* name, const QVariant* value, Qt3DCore::QNode* parent) {
  return new Qt3DRender::QParameter(*name, *value, parent);
}

Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_no_args() {
  return new Qt3DRender::QParameter();
}

Qt3DRender::QParameter* qt_3d_render_c_Qt3DRender_QParameter_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QParameter(parent);
}

int qt_3d_render_c_Qt3DRender_QParameter_qt_metacall(Qt3DRender::QParameter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QParameter_qt_metacast(Qt3DRender::QParameter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QParameter_setName(Qt3DRender::QParameter* this_ptr, const QString* name) {
  this_ptr->setName(*name);
}

void qt_3d_render_c_Qt3DRender_QParameter_setValue(Qt3DRender::QParameter* this_ptr, const QVariant* dv) {
  this_ptr->setValue(*dv);
}

void qt_3d_render_c_Qt3DRender_QParameter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QParameter::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QParameter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QParameter::tr(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QParameter_value_to_output(const Qt3DRender::QParameter* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->value());
}

