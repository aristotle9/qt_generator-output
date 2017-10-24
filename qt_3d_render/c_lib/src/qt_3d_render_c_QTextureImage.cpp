#include "qt_3d_render_c_QTextureImage.h"

Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_dynamic_cast_Qt3DRender_QTextureImage_ptr(Qt3DRender::QAbstractTextureImage* ptr) {
  return dynamic_cast<Qt3DRender::QTextureImage*>(ptr);
}

QObject* qt_3d_render_c_QTextureImage_G_static_cast_QObject_ptr(Qt3DRender::QTextureImage* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTextureImage* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(Qt3DRender::QTextureImage* ptr) {
  return static_cast<Qt3DRender::QAbstractTextureImage*>(ptr);
}

Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QTextureImage*>(ptr);
}

Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QTextureImage*>(ptr);
}

Qt3DRender::QTextureImage* qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DRender_QAbstractTextureImage(Qt3DRender::QAbstractTextureImage* ptr) {
  return static_cast<Qt3DRender::QTextureImage*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTextureImage_delete(Qt3DRender::QTextureImage* this_ptr) {
  delete this_ptr;
}

bool qt_3d_render_c_Qt3DRender_QTextureImage_isMirrored(const Qt3DRender::QTextureImage* this_ptr) {
  return this_ptr->isMirrored();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QTextureImage_metaObject(const Qt3DRender::QTextureImage* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QTextureImage* qt_3d_render_c_Qt3DRender_QTextureImage_new_no_args() {
  return new Qt3DRender::QTextureImage();
}

Qt3DRender::QTextureImage* qt_3d_render_c_Qt3DRender_QTextureImage_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QTextureImage(parent);
}

int qt_3d_render_c_Qt3DRender_QTextureImage_qt_metacall(Qt3DRender::QTextureImage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QTextureImage_qt_metacast(Qt3DRender::QTextureImage* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QTextureImage_setMirrored(Qt3DRender::QTextureImage* this_ptr, bool mirrored) {
  this_ptr->setMirrored(mirrored);
}

void qt_3d_render_c_Qt3DRender_QTextureImage_setSource(Qt3DRender::QTextureImage* this_ptr, const QUrl* source) {
  this_ptr->setSource(*source);
}

void qt_3d_render_c_Qt3DRender_QTextureImage_source_to_output(const Qt3DRender::QTextureImage* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->source());
}

Qt3DRender::QTextureImage::Status qt_3d_render_c_Qt3DRender_QTextureImage_status(const Qt3DRender::QTextureImage* this_ptr) {
  return this_ptr->status();
}

void qt_3d_render_c_Qt3DRender_QTextureImage_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTextureImage::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QTextureImage_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTextureImage::tr(s, c, n));
}

