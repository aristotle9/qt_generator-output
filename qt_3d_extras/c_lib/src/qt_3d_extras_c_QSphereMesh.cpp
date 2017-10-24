#include "qt_3d_extras_c_QSphereMesh.h"

QObject* qt_3d_extras_c_QSphereMesh_G_static_cast_QObject_ptr(Qt3DExtras::QSphereMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QSphereMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QSphereMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QSphereMesh* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DExtras_QSphereMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QSphereMesh*>(ptr);
}

Qt3DExtras::QSphereMesh* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DExtras_QSphereMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QSphereMesh*>(ptr);
}

Qt3DExtras::QSphereMesh* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DExtras_QSphereMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QSphereMesh*>(ptr);
}

Qt3DExtras::QSphereMesh* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DExtras_QSphereMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DExtras::QSphereMesh*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_extras_c_QSphereMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DExtras::QSphereMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_delete(Qt3DExtras::QSphereMesh* this_ptr) {
  delete this_ptr;
}

bool qt_3d_extras_c_Qt3DExtras_QSphereMesh_generateTangents(const Qt3DExtras::QSphereMesh* this_ptr) {
  return this_ptr->generateTangents();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QSphereMesh_metaObject(const Qt3DExtras::QSphereMesh* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QSphereMesh* qt_3d_extras_c_Qt3DExtras_QSphereMesh_new_no_args() {
  return new Qt3DExtras::QSphereMesh();
}

Qt3DExtras::QSphereMesh* qt_3d_extras_c_Qt3DExtras_QSphereMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QSphereMesh(parent);
}

int qt_3d_extras_c_Qt3DExtras_QSphereMesh_qt_metacall(Qt3DExtras::QSphereMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QSphereMesh_qt_metacast(Qt3DExtras::QSphereMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_extras_c_Qt3DExtras_QSphereMesh_radius(const Qt3DExtras::QSphereMesh* this_ptr) {
  return this_ptr->radius();
}

int qt_3d_extras_c_Qt3DExtras_QSphereMesh_rings(const Qt3DExtras::QSphereMesh* this_ptr) {
  return this_ptr->rings();
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_setGenerateTangents(Qt3DExtras::QSphereMesh* this_ptr, bool gen) {
  this_ptr->setGenerateTangents(gen);
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_setRadius(Qt3DExtras::QSphereMesh* this_ptr, float radius) {
  this_ptr->setRadius(radius);
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_setRings(Qt3DExtras::QSphereMesh* this_ptr, int rings) {
  this_ptr->setRings(rings);
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_setSlices(Qt3DExtras::QSphereMesh* this_ptr, int slices) {
  this_ptr->setSlices(slices);
}

int qt_3d_extras_c_Qt3DExtras_QSphereMesh_slices(const Qt3DExtras::QSphereMesh* this_ptr) {
  return this_ptr->slices();
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QSphereMesh::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QSphereMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QSphereMesh::tr(s, c, n));
}

