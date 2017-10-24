#include "qt_3d_extras_c_QTorusGeometry.h"

QObject* qt_3d_extras_c_QTorusGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QTorusGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QTorusGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QTorusGeometry* qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QTorusGeometry*>(ptr);
}

Qt3DExtras::QTorusGeometry* qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QTorusGeometry*>(ptr);
}

Qt3DExtras::QTorusGeometry* qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DExtras::QTorusGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QTorusGeometry* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_delete(Qt3DExtras::QTorusGeometry* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_indexAttribute(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->indexAttribute();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_metaObject(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->metaObject();
}

float qt_3d_extras_c_Qt3DExtras_QTorusGeometry_minorRadius(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->minorRadius();
}

Qt3DExtras::QTorusGeometry* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_new_no_args() {
  return new Qt3DExtras::QTorusGeometry();
}

Qt3DExtras::QTorusGeometry* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QTorusGeometry(parent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_normalAttribute(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->normalAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_positionAttribute(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->positionAttribute();
}

int qt_3d_extras_c_Qt3DExtras_QTorusGeometry_qt_metacall(Qt3DExtras::QTorusGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_qt_metacast(Qt3DExtras::QTorusGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_extras_c_Qt3DExtras_QTorusGeometry_radius(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->radius();
}

int qt_3d_extras_c_Qt3DExtras_QTorusGeometry_rings(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->rings();
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setMinorRadius(Qt3DExtras::QTorusGeometry* this_ptr, float minorRadius) {
  this_ptr->setMinorRadius(minorRadius);
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setRadius(Qt3DExtras::QTorusGeometry* this_ptr, float radius) {
  this_ptr->setRadius(radius);
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setRings(Qt3DExtras::QTorusGeometry* this_ptr, int rings) {
  this_ptr->setRings(rings);
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setSlices(Qt3DExtras::QTorusGeometry* this_ptr, int slices) {
  this_ptr->setSlices(slices);
}

int qt_3d_extras_c_Qt3DExtras_QTorusGeometry_slices(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->slices();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QTorusGeometry_texCoordAttribute(const Qt3DExtras::QTorusGeometry* this_ptr) {
  return this_ptr->texCoordAttribute();
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QTorusGeometry::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QTorusGeometry::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_updateIndices(Qt3DExtras::QTorusGeometry* this_ptr) {
  this_ptr->updateIndices();
}

void qt_3d_extras_c_Qt3DExtras_QTorusGeometry_updateVertices(Qt3DExtras::QTorusGeometry* this_ptr) {
  this_ptr->updateVertices();
}

