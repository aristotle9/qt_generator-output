#include "qt_3d_extras_c_QCuboidMesh.h"

QObject* qt_3d_extras_c_QCuboidMesh_G_static_cast_QObject_ptr(Qt3DExtras::QCuboidMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QCuboidMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QCuboidMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QCuboidMesh* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QCuboidMesh*>(ptr);
}

Qt3DExtras::QCuboidMesh* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QCuboidMesh*>(ptr);
}

Qt3DExtras::QCuboidMesh* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QCuboidMesh*>(ptr);
}

Qt3DExtras::QCuboidMesh* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DExtras::QCuboidMesh*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DExtras::QCuboidMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_delete(Qt3DExtras::QCuboidMesh* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QCuboidMesh_metaObject(const Qt3DExtras::QCuboidMesh* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QCuboidMesh* qt_3d_extras_c_Qt3DExtras_QCuboidMesh_new_no_args() {
  return new Qt3DExtras::QCuboidMesh();
}

Qt3DExtras::QCuboidMesh* qt_3d_extras_c_Qt3DExtras_QCuboidMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QCuboidMesh(parent);
}

int qt_3d_extras_c_Qt3DExtras_QCuboidMesh_qt_metacall(Qt3DExtras::QCuboidMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QCuboidMesh_qt_metacast(Qt3DExtras::QCuboidMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setXExtent(Qt3DExtras::QCuboidMesh* this_ptr, float xExtent) {
  this_ptr->setXExtent(xExtent);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setXYMeshResolution(Qt3DExtras::QCuboidMesh* this_ptr, const QSize* resolution) {
  this_ptr->setXYMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setXZMeshResolution(Qt3DExtras::QCuboidMesh* this_ptr, const QSize* resolution) {
  this_ptr->setXZMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setYExtent(Qt3DExtras::QCuboidMesh* this_ptr, float yExtent) {
  this_ptr->setYExtent(yExtent);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setYZMeshResolution(Qt3DExtras::QCuboidMesh* this_ptr, const QSize* resolution) {
  this_ptr->setYZMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setZExtent(Qt3DExtras::QCuboidMesh* this_ptr, float zExtent) {
  this_ptr->setZExtent(zExtent);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QCuboidMesh::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QCuboidMesh::tr(s, c, n));
}

float qt_3d_extras_c_Qt3DExtras_QCuboidMesh_xExtent(const Qt3DExtras::QCuboidMesh* this_ptr) {
  return this_ptr->xExtent();
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_xyMeshResolution_to_output(const Qt3DExtras::QCuboidMesh* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->xyMeshResolution());
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_xzMeshResolution_to_output(const Qt3DExtras::QCuboidMesh* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->xzMeshResolution());
}

float qt_3d_extras_c_Qt3DExtras_QCuboidMesh_yExtent(const Qt3DExtras::QCuboidMesh* this_ptr) {
  return this_ptr->yExtent();
}

void qt_3d_extras_c_Qt3DExtras_QCuboidMesh_yzMeshResolution_to_output(const Qt3DExtras::QCuboidMesh* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->yzMeshResolution());
}

float qt_3d_extras_c_Qt3DExtras_QCuboidMesh_zExtent(const Qt3DExtras::QCuboidMesh* this_ptr) {
  return this_ptr->zExtent();
}

