#include "qt_3d_extras_c_QCuboidGeometry.h"

QObject* qt_3d_extras_c_QCuboidGeometry_G_static_cast_QObject_ptr(Qt3DExtras::QCuboidGeometry* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QCuboidGeometry* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QCuboidGeometry* qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QCuboidGeometry*>(ptr);
}

Qt3DExtras::QCuboidGeometry* qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QCuboidGeometry*>(ptr);
}

Qt3DExtras::QCuboidGeometry* qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_Qt3DRender_QGeometry(Qt3DRender::QGeometry* ptr) {
  return static_cast<Qt3DExtras::QCuboidGeometry*>(ptr);
}

Qt3DRender::QGeometry* qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(Qt3DExtras::QCuboidGeometry* ptr) {
  return static_cast<Qt3DRender::QGeometry*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_delete(Qt3DExtras::QCuboidGeometry* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_indexAttribute(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->indexAttribute();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_metaObject(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QCuboidGeometry* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_new_no_args() {
  return new Qt3DExtras::QCuboidGeometry();
}

Qt3DExtras::QCuboidGeometry* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QCuboidGeometry(parent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_normalAttribute(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->normalAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_positionAttribute(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->positionAttribute();
}

int qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_qt_metacall(Qt3DExtras::QCuboidGeometry* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_qt_metacast(Qt3DExtras::QCuboidGeometry* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setXExtent(Qt3DExtras::QCuboidGeometry* this_ptr, float xExtent) {
  this_ptr->setXExtent(xExtent);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setXYMeshResolution(Qt3DExtras::QCuboidGeometry* this_ptr, const QSize* resolution) {
  this_ptr->setXYMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setXZMeshResolution(Qt3DExtras::QCuboidGeometry* this_ptr, const QSize* resolution) {
  this_ptr->setXZMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setYExtent(Qt3DExtras::QCuboidGeometry* this_ptr, float yExtent) {
  this_ptr->setYExtent(yExtent);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setYZMeshResolution(Qt3DExtras::QCuboidGeometry* this_ptr, const QSize* resolution) {
  this_ptr->setYZMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setZExtent(Qt3DExtras::QCuboidGeometry* this_ptr, float zExtent) {
  this_ptr->setZExtent(zExtent);
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_tangentAttribute(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->tangentAttribute();
}

Qt3DRender::QAttribute* qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_texCoordAttribute(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->texCoordAttribute();
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QCuboidGeometry::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QCuboidGeometry::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_updateIndices(Qt3DExtras::QCuboidGeometry* this_ptr) {
  this_ptr->updateIndices();
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_updateVertices(Qt3DExtras::QCuboidGeometry* this_ptr) {
  this_ptr->updateVertices();
}

float qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_xExtent(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->xExtent();
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_xyMeshResolution_to_output(const Qt3DExtras::QCuboidGeometry* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->xyMeshResolution());
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_xzMeshResolution_to_output(const Qt3DExtras::QCuboidGeometry* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->xzMeshResolution());
}

float qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_yExtent(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->yExtent();
}

void qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_yzMeshResolution_to_output(const Qt3DExtras::QCuboidGeometry* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->yzMeshResolution());
}

float qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_zExtent(const Qt3DExtras::QCuboidGeometry* this_ptr) {
  return this_ptr->zExtent();
}

