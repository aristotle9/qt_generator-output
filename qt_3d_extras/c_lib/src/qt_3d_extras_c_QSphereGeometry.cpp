#include "qt_3d_extras_c_QSphereGeometry.h"

QObject* qt_3d_extras_c_QSphereGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QSphereGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QSphereGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QSphereGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QSphereGeometry*>(ptr);
}

Qt3DExtras::QSphereGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QSphereGeometry*>(ptr);
}

Qt3DExtras::QSphereGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DExtras::QSphereGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QSphereGeometry* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_delete(Qt3DExtras::QSphereGeometry* this_ptr) {
  delete this_ptr;
}

bool qt_3d_extras_c_Qt3DExtras_QSphereGeometry_generateTangents(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->generateTangents();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_indexAttribute(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->indexAttribute();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_metaObject(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QSphereGeometry* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_new_no_args() {
  return new Qt3DExtras::QSphereGeometry();
}

Qt3DExtras::QSphereGeometry* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QSphereGeometry(parent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_normalAttribute(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->normalAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_positionAttribute(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->positionAttribute();
}

int qt_3d_extras_c_Qt3DExtras_QSphereGeometry_qt_metacall(Qt3DExtras::QSphereGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_qt_metacast(Qt3DExtras::QSphereGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_extras_c_Qt3DExtras_QSphereGeometry_radius(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->radius();
}

int qt_3d_extras_c_Qt3DExtras_QSphereGeometry_rings(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->rings();
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setGenerateTangents(Qt3DExtras::QSphereGeometry* this_ptr, bool gen) {
  this_ptr->setGenerateTangents(gen);
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setRadius(Qt3DExtras::QSphereGeometry* this_ptr, float radius) {
  this_ptr->setRadius(radius);
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setRings(Qt3DExtras::QSphereGeometry* this_ptr, int rings) {
  this_ptr->setRings(rings);
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setSlices(Qt3DExtras::QSphereGeometry* this_ptr, int slices) {
  this_ptr->setSlices(slices);
}

int qt_3d_extras_c_Qt3DExtras_QSphereGeometry_slices(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->slices();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_tangentAttribute(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->tangentAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QSphereGeometry_texCoordAttribute(const Qt3DExtras::QSphereGeometry* this_ptr) {
  return this_ptr->texCoordAttribute();
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QSphereGeometry::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QSphereGeometry::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_updateIndices(Qt3DExtras::QSphereGeometry* this_ptr) {
  this_ptr->updateIndices();
}

void qt_3d_extras_c_Qt3DExtras_QSphereGeometry_updateVertices(Qt3DExtras::QSphereGeometry* this_ptr) {
  this_ptr->updateVertices();
}

