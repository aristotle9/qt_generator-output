#include "qt_3d_extras_c_QConeMesh.h"

QObject* qt_3d_extras_c_QConeMesh_G_static_cast_QObject_ptr(Qt3DExtras::QConeMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QConeMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QConeMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QConeMesh* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QConeMesh*>(ptr);
}

Qt3DExtras::QConeMesh* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QConeMesh*>(ptr);
}

Qt3DExtras::QConeMesh* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QConeMesh*>(ptr);
}

Qt3DExtras::QConeMesh* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DExtras::QConeMesh*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DExtras::QConeMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

float qt_3d_extras_c_Qt3DExtras_QConeMesh_bottomRadius(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->bottomRadius();
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_delete(Qt3DExtras::QConeMesh* this_ptr) {
  delete this_ptr;
}

bool qt_3d_extras_c_Qt3DExtras_QConeMesh_hasBottomEndcap(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->hasBottomEndcap();
}

bool qt_3d_extras_c_Qt3DExtras_QConeMesh_hasTopEndcap(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->hasTopEndcap();
}

float qt_3d_extras_c_Qt3DExtras_QConeMesh_length(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->length();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QConeMesh_metaObject(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QConeMesh* qt_3d_extras_c_Qt3DExtras_QConeMesh_new_no_args() {
  return new Qt3DExtras::QConeMesh();
}

Qt3DExtras::QConeMesh* qt_3d_extras_c_Qt3DExtras_QConeMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QConeMesh(parent);
}

int qt_3d_extras_c_Qt3DExtras_QConeMesh_qt_metacall(Qt3DExtras::QConeMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QConeMesh_qt_metacast(Qt3DExtras::QConeMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_3d_extras_c_Qt3DExtras_QConeMesh_rings(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->rings();
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setBottomRadius(Qt3DExtras::QConeMesh* this_ptr, float bottomRadius) {
  this_ptr->setBottomRadius(bottomRadius);
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setHasBottomEndcap(Qt3DExtras::QConeMesh* this_ptr, bool hasBottomEndcap) {
  this_ptr->setHasBottomEndcap(hasBottomEndcap);
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setHasTopEndcap(Qt3DExtras::QConeMesh* this_ptr, bool hasTopEndcap) {
  this_ptr->setHasTopEndcap(hasTopEndcap);
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setLength(Qt3DExtras::QConeMesh* this_ptr, float length) {
  this_ptr->setLength(length);
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setRings(Qt3DExtras::QConeMesh* this_ptr, int rings) {
  this_ptr->setRings(rings);
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setSlices(Qt3DExtras::QConeMesh* this_ptr, int slices) {
  this_ptr->setSlices(slices);
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_setTopRadius(Qt3DExtras::QConeMesh* this_ptr, float topRadius) {
  this_ptr->setTopRadius(topRadius);
}

int qt_3d_extras_c_Qt3DExtras_QConeMesh_slices(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->slices();
}

float qt_3d_extras_c_Qt3DExtras_QConeMesh_topRadius(const Qt3DExtras::QConeMesh* this_ptr) {
  return this_ptr->topRadius();
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QConeMesh::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QConeMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QConeMesh::tr(s, c, n));
}

