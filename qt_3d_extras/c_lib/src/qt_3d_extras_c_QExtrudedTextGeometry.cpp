#include "qt_3d_extras_c_QExtrudedTextGeometry.h"

QObject* qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QExtrudedTextGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QExtrudedTextGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QExtrudedTextGeometry* qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextGeometry*>(ptr);
}

Qt3DExtras::QExtrudedTextGeometry* qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextGeometry*>(ptr);
}

Qt3DExtras::QExtrudedTextGeometry* qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QExtrudedTextGeometry* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_delete(Qt3DExtras::QExtrudedTextGeometry* this_ptr) {
  delete this_ptr;
}

float qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_extrusionLength(const Qt3DExtras::QExtrudedTextGeometry* this_ptr) {
  return this_ptr->extrusionLength();
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_font_to_output(const Qt3DExtras::QExtrudedTextGeometry* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_indexAttribute(const Qt3DExtras::QExtrudedTextGeometry* this_ptr) {
  return this_ptr->indexAttribute();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_metaObject(const Qt3DExtras::QExtrudedTextGeometry* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QExtrudedTextGeometry* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_new_no_args() {
  return new Qt3DExtras::QExtrudedTextGeometry();
}

Qt3DExtras::QExtrudedTextGeometry* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QExtrudedTextGeometry(parent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_normalAttribute(const Qt3DExtras::QExtrudedTextGeometry* this_ptr) {
  return this_ptr->normalAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_positionAttribute(const Qt3DExtras::QExtrudedTextGeometry* this_ptr) {
  return this_ptr->positionAttribute();
}

int qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_qt_metacall(Qt3DExtras::QExtrudedTextGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_qt_metacast(Qt3DExtras::QExtrudedTextGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_setDepth(Qt3DExtras::QExtrudedTextGeometry* this_ptr, float extrusionLength) {
  this_ptr->setDepth(extrusionLength);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_setFont(Qt3DExtras::QExtrudedTextGeometry* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_setText(Qt3DExtras::QExtrudedTextGeometry* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_text_to_output(const Qt3DExtras::QExtrudedTextGeometry* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QExtrudedTextGeometry::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QExtrudedTextGeometry::tr(s, c, n));
}

