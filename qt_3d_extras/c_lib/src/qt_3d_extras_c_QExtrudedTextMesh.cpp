#include "qt_3d_extras_c_QExtrudedTextMesh.h"

QObject* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_QObject_ptr(Qt3DExtras::QExtrudedTextMesh* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DExtras::QExtrudedTextMesh* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QExtrudedTextMesh* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QExtrudedTextMesh* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextMesh*>(ptr);
}

Qt3DExtras::QExtrudedTextMesh* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextMesh*>(ptr);
}

Qt3DExtras::QExtrudedTextMesh* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextMesh*>(ptr);
}

Qt3DExtras::QExtrudedTextMesh* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr) {
  return static_cast<Qt3DExtras::QExtrudedTextMesh*>(ptr);
}

Qt3DRender::QGeometryRenderer* qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DExtras::QExtrudedTextMesh* ptr) {
  return static_cast<Qt3DRender::QGeometryRenderer*>(ptr);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_delete(Qt3DExtras::QExtrudedTextMesh* this_ptr) {
  delete this_ptr;
}

float qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_depth(const Qt3DExtras::QExtrudedTextMesh* this_ptr) {
  return this_ptr->depth();
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_font_to_output(const Qt3DExtras::QExtrudedTextMesh* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_metaObject(const Qt3DExtras::QExtrudedTextMesh* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QExtrudedTextMesh* qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_new_no_args() {
  return new Qt3DExtras::QExtrudedTextMesh();
}

Qt3DExtras::QExtrudedTextMesh* qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QExtrudedTextMesh(parent);
}

int qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_qt_metacall(Qt3DExtras::QExtrudedTextMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_qt_metacast(Qt3DExtras::QExtrudedTextMesh* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_setDepth(Qt3DExtras::QExtrudedTextMesh* this_ptr, float depth) {
  this_ptr->setDepth(depth);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_setFont(Qt3DExtras::QExtrudedTextMesh* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_setText(Qt3DExtras::QExtrudedTextMesh* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_text_to_output(const Qt3DExtras::QExtrudedTextMesh* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QExtrudedTextMesh::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QExtrudedTextMesh::tr(s, c, n));
}

