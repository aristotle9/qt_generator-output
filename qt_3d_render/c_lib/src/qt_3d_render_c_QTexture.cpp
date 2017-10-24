#include "qt_3d_render_c_QTexture.h"

Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_dynamic_cast_Qt3DRender_QTextureLoader_ptr(Qt3DRender::QAbstractTexture* ptr) {
  return dynamic_cast<Qt3DRender::QTextureLoader*>(ptr);
}

QObject* qt_3d_render_c_QTexture_G_static_cast_QObject_ptr(Qt3DRender::QTextureLoader* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QTexture_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTextureLoader* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractTexture* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr(Qt3DRender::QTextureLoader* ptr) {
  return static_cast<Qt3DRender::QAbstractTexture*>(ptr);
}

Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QTextureLoader*>(ptr);
}

Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QTextureLoader*>(ptr);
}

Qt3DRender::QTextureLoader* qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DRender_QAbstractTexture(Qt3DRender::QAbstractTexture* ptr) {
  return static_cast<Qt3DRender::QTextureLoader*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTextureLoader_delete(Qt3DRender::QTextureLoader* this_ptr) {
  delete this_ptr;
}

bool qt_3d_render_c_Qt3DRender_QTextureLoader_isMirrored(const Qt3DRender::QTextureLoader* this_ptr) {
  return this_ptr->isMirrored();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QTextureLoader_metaObject(const Qt3DRender::QTextureLoader* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QTextureLoader* qt_3d_render_c_Qt3DRender_QTextureLoader_new_no_args() {
  return new Qt3DRender::QTextureLoader();
}

Qt3DRender::QTextureLoader* qt_3d_render_c_Qt3DRender_QTextureLoader_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QTextureLoader(parent);
}

int qt_3d_render_c_Qt3DRender_QTextureLoader_qt_metacall(Qt3DRender::QTextureLoader* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QTextureLoader_qt_metacast(Qt3DRender::QTextureLoader* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QTextureLoader_setMirrored(Qt3DRender::QTextureLoader* this_ptr, bool mirrored) {
  this_ptr->setMirrored(mirrored);
}

void qt_3d_render_c_Qt3DRender_QTextureLoader_setSource(Qt3DRender::QTextureLoader* this_ptr, const QUrl* source) {
  this_ptr->setSource(*source);
}

void qt_3d_render_c_Qt3DRender_QTextureLoader_source_to_output(const Qt3DRender::QTextureLoader* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->source());
}

void qt_3d_render_c_Qt3DRender_QTextureLoader_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTextureLoader::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QTextureLoader_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTextureLoader::tr(s, c, n));
}

