#include "qt_3d_render_c_QMesh.h"

Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_dynamic_cast_Qt3DRender_QMesh_ptr(Qt3DRender::QGeometryRenderer* ptr) {
  return dynamic_cast<Qt3DRender::QMesh*>(ptr);
}

QObject* qt_3d_render_c_QMesh_G_static_cast_QObject_ptr(Qt3DRender::QMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DRender::QMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QMesh*>(ptr);
}

Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QMesh*>(ptr);
}

Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QMesh*>(ptr);
}

Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DRender::QMesh*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QMesh_delete(Qt3DRender::QMesh* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QMesh_meshName_to_output(const Qt3DRender::QMesh* this_ptr, QString* output) {
  new(output) QString(this_ptr->meshName());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QMesh_metaObject(const Qt3DRender::QMesh* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QMesh* qt_3d_render_c_Qt3DRender_QMesh_new_no_args() {
  return new Qt3DRender::QMesh();
}

Qt3DRender::QMesh* qt_3d_render_c_Qt3DRender_QMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QMesh(parent);
}

int qt_3d_render_c_Qt3DRender_QMesh_qt_metacall(Qt3DRender::QMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QMesh_qt_metacast(Qt3DRender::QMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QMesh_setMeshName(Qt3DRender::QMesh* this_ptr, const QString* meshName) {
  this_ptr->setMeshName(*meshName);
}

void qt_3d_render_c_Qt3DRender_QMesh_setSource(Qt3DRender::QMesh* this_ptr, const QUrl* source) {
  this_ptr->setSource(*source);
}

void qt_3d_render_c_Qt3DRender_QMesh_source_to_output(const Qt3DRender::QMesh* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->source());
}

void qt_3d_render_c_Qt3DRender_QMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QMesh::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QMesh::tr(s, c, n));
}

