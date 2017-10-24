#include "qt_3d_render_c_QFilterKey.h"

QObject* qt_3d_render_c_QFilterKey_G_static_cast_QObject_ptr(Qt3DRender::QFilterKey* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QFilterKey_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QFilterKey* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFilterKey* qt_3d_render_c_QFilterKey_G_static_cast_Qt3DRender_QFilterKey_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QFilterKey*>(ptr);
}

Qt3DRender::QFilterKey* qt_3d_render_c_QFilterKey_G_static_cast_Qt3DRender_QFilterKey_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QFilterKey*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QFilterKey_delete(Qt3DRender::QFilterKey* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QFilterKey_metaObject(const Qt3DRender::QFilterKey* this_ptr) {
  return this_ptr->metaObject();
}

void qt_3d_render_c_Qt3DRender_QFilterKey_name_to_output(const Qt3DRender::QFilterKey* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

Qt3DRender::QFilterKey* qt_3d_render_c_Qt3DRender_QFilterKey_new_no_args() {
  return new Qt3DRender::QFilterKey();
}

Qt3DRender::QFilterKey* qt_3d_render_c_Qt3DRender_QFilterKey_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QFilterKey(parent);
}

int qt_3d_render_c_Qt3DRender_QFilterKey_qt_metacall(Qt3DRender::QFilterKey* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QFilterKey_qt_metacast(Qt3DRender::QFilterKey* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QFilterKey_setName(Qt3DRender::QFilterKey* this_ptr, const QString* customType) {
  this_ptr->setName(*customType);
}

void qt_3d_render_c_Qt3DRender_QFilterKey_setValue(Qt3DRender::QFilterKey* this_ptr, const QVariant* value) {
  this_ptr->setValue(*value);
}

void qt_3d_render_c_Qt3DRender_QFilterKey_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QFilterKey::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QFilterKey_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QFilterKey::tr(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QFilterKey_value_to_output(const Qt3DRender::QFilterKey* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->value());
}

