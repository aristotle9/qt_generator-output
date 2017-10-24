#include "qt_3d_extras_c_QCylinderGeometry.h"

QObject* qt_3d_extras_c_QCylinderGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QCylinderGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QCylinderGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QCylinderGeometry* qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QCylinderGeometry*>(ptr);
}

Qt3DExtras::QCylinderGeometry* qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QCylinderGeometry*>(ptr);
}

Qt3DExtras::QCylinderGeometry* qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DExtras::QCylinderGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QCylinderGeometry* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_delete(Qt3DExtras::QCylinderGeometry* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_indexAttribute(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->indexAttribute();
}

float qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_length(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->length();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_metaObject(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QCylinderGeometry* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_new_no_args() {
  return new Qt3DExtras::QCylinderGeometry();
}

Qt3DExtras::QCylinderGeometry* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QCylinderGeometry(parent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_normalAttribute(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->normalAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_positionAttribute(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->positionAttribute();
}

int qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_qt_metacall(Qt3DExtras::QCylinderGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_qt_metacast(Qt3DExtras::QCylinderGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_radius(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->radius();
}

int qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_rings(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->rings();
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setLength(Qt3DExtras::QCylinderGeometry* this_ptr, float length) {
  this_ptr->setLength(length);
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setRadius(Qt3DExtras::QCylinderGeometry* this_ptr, float radius) {
  this_ptr->setRadius(radius);
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setRings(Qt3DExtras::QCylinderGeometry* this_ptr, int rings) {
  this_ptr->setRings(rings);
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setSlices(Qt3DExtras::QCylinderGeometry* this_ptr, int slices) {
  this_ptr->setSlices(slices);
}

int qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_slices(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->slices();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_texCoordAttribute(const Qt3DExtras::QCylinderGeometry* this_ptr) {
  return this_ptr->texCoordAttribute();
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QCylinderGeometry::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QCylinderGeometry::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_updateIndices(Qt3DExtras::QCylinderGeometry* this_ptr) {
  this_ptr->updateIndices();
}

void qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_updateVertices(Qt3DExtras::QCylinderGeometry* this_ptr) {
  this_ptr->updateVertices();
}

