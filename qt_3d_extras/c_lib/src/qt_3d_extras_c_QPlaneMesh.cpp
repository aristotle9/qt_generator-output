#include "qt_3d_extras_c_QPlaneMesh.h"

QObject* qt_3d_extras_c_QPlaneMesh_G_static_cast_QObject_ptr(Qt3DExtras::QPlaneMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QPlaneMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QPlaneMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QPlaneMesh* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QPlaneMesh*>(ptr);
}

Qt3DExtras::QPlaneMesh* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QPlaneMesh*>(ptr);
}

Qt3DExtras::QPlaneMesh* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QPlaneMesh*>(ptr);
}

Qt3DExtras::QPlaneMesh* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DExtras::QPlaneMesh*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DExtras::QPlaneMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_delete(Qt3DExtras::QPlaneMesh* this_ptr) {
  delete this_ptr;
}

float qt_3d_extras_c_Qt3DExtras_QPlaneMesh_height(const Qt3DExtras::QPlaneMesh* this_ptr) {
  return this_ptr->height();
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_meshResolution_to_output(const Qt3DExtras::QPlaneMesh* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->meshResolution());
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QPlaneMesh_metaObject(const Qt3DExtras::QPlaneMesh* this_ptr) {
  return this_ptr->metaObject();
}

bool qt_3d_extras_c_Qt3DExtras_QPlaneMesh_mirrored(const Qt3DExtras::QPlaneMesh* this_ptr) {
  return this_ptr->mirrored();
}

Qt3DExtras::QPlaneMesh* qt_3d_extras_c_Qt3DExtras_QPlaneMesh_new_no_args() {
  return new Qt3DExtras::QPlaneMesh();
}

Qt3DExtras::QPlaneMesh* qt_3d_extras_c_Qt3DExtras_QPlaneMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QPlaneMesh(parent);
}

int qt_3d_extras_c_Qt3DExtras_QPlaneMesh_qt_metacall(Qt3DExtras::QPlaneMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QPlaneMesh_qt_metacast(Qt3DExtras::QPlaneMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setHeight(Qt3DExtras::QPlaneMesh* this_ptr, float height) {
  this_ptr->setHeight(height);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setMeshResolution(Qt3DExtras::QPlaneMesh* this_ptr, const QSize* resolution) {
  this_ptr->setMeshResolution(*resolution);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setMirrored(Qt3DExtras::QPlaneMesh* this_ptr, bool mirrored) {
  this_ptr->setMirrored(mirrored);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setWidth(Qt3DExtras::QPlaneMesh* this_ptr, float width) {
  this_ptr->setWidth(width);
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QPlaneMesh::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QPlaneMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QPlaneMesh::tr(s, c, n));
}

float qt_3d_extras_c_Qt3DExtras_QPlaneMesh_width(const Qt3DExtras::QPlaneMesh* this_ptr) {
  return this_ptr->width();
}

