#include "qt_3d_extras_c_QTorusMesh.h"

QObject* qt_3d_extras_c_QTorusMesh_G_static_cast_QObject_ptr(Qt3DExtras::QTorusMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QTorusMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QTorusMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QTorusMesh* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QTorusMesh*>(ptr);
}

Qt3DExtras::QTorusMesh* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QTorusMesh*>(ptr);
}

Qt3DExtras::QTorusMesh* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QTorusMesh*>(ptr);
}

Qt3DExtras::QTorusMesh* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DExtras::QTorusMesh*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DExtras::QTorusMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_delete(Qt3DExtras::QTorusMesh* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QTorusMesh_metaObject(const Qt3DExtras::QTorusMesh* this_ptr) {
  return this_ptr->metaObject();
}

float qt_3d_extras_c_Qt3DExtras_QTorusMesh_minorRadius(const Qt3DExtras::QTorusMesh* this_ptr) {
  return this_ptr->minorRadius();
}

Qt3DExtras::QTorusMesh* qt_3d_extras_c_Qt3DExtras_QTorusMesh_new_no_args() {
  return new Qt3DExtras::QTorusMesh();
}

Qt3DExtras::QTorusMesh* qt_3d_extras_c_Qt3DExtras_QTorusMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QTorusMesh(parent);
}

int qt_3d_extras_c_Qt3DExtras_QTorusMesh_qt_metacall(Qt3DExtras::QTorusMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QTorusMesh_qt_metacast(Qt3DExtras::QTorusMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_extras_c_Qt3DExtras_QTorusMesh_radius(const Qt3DExtras::QTorusMesh* this_ptr) {
  return this_ptr->radius();
}

int qt_3d_extras_c_Qt3DExtras_QTorusMesh_rings(const Qt3DExtras::QTorusMesh* this_ptr) {
  return this_ptr->rings();
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_setMinorRadius(Qt3DExtras::QTorusMesh* this_ptr, float minorRadius) {
  this_ptr->setMinorRadius(minorRadius);
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_setRadius(Qt3DExtras::QTorusMesh* this_ptr, float radius) {
  this_ptr->setRadius(radius);
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_setRings(Qt3DExtras::QTorusMesh* this_ptr, int rings) {
  this_ptr->setRings(rings);
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_setSlices(Qt3DExtras::QTorusMesh* this_ptr, int slices) {
  this_ptr->setSlices(slices);
}

int qt_3d_extras_c_Qt3DExtras_QTorusMesh_slices(const Qt3DExtras::QTorusMesh* this_ptr) {
  return this_ptr->slices();
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QTorusMesh::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QTorusMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QTorusMesh::tr(s, c, n));
}

