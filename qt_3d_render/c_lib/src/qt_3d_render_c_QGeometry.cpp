#include "qt_3d_render_c_QGeometry.h"

QObject* qt_3d_render_c_QGeometry_G_static_cast_QObject_ptr(Qt3DRender::QGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QGeometry_addAttribute(Qt3DRender::QGeometry* this_ptr, Qt3DRender::QAttribute* attribute) {
  this_ptr->addAttribute(attribute);
}

void qt_3d_render_c_Qt3DRender_QGeometry_attributes_to_output(const Qt3DRender::QGeometry* this_ptr, QVector< Qt3DRender::QAttribute* >* output) {
  new(output) QVector< Qt3DRender::QAttribute* >(this_ptr->attributes());
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QGeometry_boundingVolumePositionAttribute(const Qt3DRender::QGeometry* this_ptr) {
  return this_ptr->boundingVolumePositionAttribute();
}

void qt_3d_render_c_Qt3DRender_QGeometry_delete(Qt3DRender::QGeometry* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QGeometry_metaObject(const Qt3DRender::QGeometry* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QGeometry* qt_3d_render_c_Qt3DRender_QGeometry_new_no_args() {
  return new Qt3DRender::QGeometry();
}

Qt3DRender::QGeometry* qt_3d_render_c_Qt3DRender_QGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QGeometry(parent);
}

int qt_3d_render_c_Qt3DRender_QGeometry_qt_metacall(Qt3DRender::QGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QGeometry_qt_metacast(Qt3DRender::QGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QGeometry_removeAttribute(Qt3DRender::QGeometry* this_ptr, Qt3DRender::QAttribute* attribute) {
  this_ptr->removeAttribute(attribute);
}

void qt_3d_render_c_Qt3DRender_QGeometry_setBoundingVolumePositionAttribute(Qt3DRender::QGeometry* this_ptr, Qt3DRender::QAttribute* boundingVolumePositionAttribute) {
  this_ptr->setBoundingVolumePositionAttribute(boundingVolumePositionAttribute);
}

void qt_3d_render_c_Qt3DRender_QGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QGeometry::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QGeometry::tr(s, c, n));
}

