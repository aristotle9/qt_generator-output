#include "qt_3d_render_c_QAbstractTextureImage.h"

QObject* qt_3d_render_c_QAbstractTextureImage_G_static_cast_QObject_ptr(Qt3DRender::QAbstractTextureImage* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAbstractTextureImage* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractTextureImage* qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QAbstractTextureImage*>(ptr);
}

Qt3DRender::QAbstractTextureImage* qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QAbstractTextureImage*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QAbstractTextureImage_delete(Qt3DRender::QAbstractTextureImage* this_ptr) {
  delete this_ptr;
}

int qt_3d_render_c_Qt3DRender_QAbstractTextureImage_layer(const Qt3DRender::QAbstractTextureImage* this_ptr) {
  return this_ptr->layer();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QAbstractTextureImage_metaObject(const Qt3DRender::QAbstractTextureImage* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QAbstractTextureImage_mipLevel(const Qt3DRender::QAbstractTextureImage* this_ptr) {
  return this_ptr->mipLevel();
}

int qt_3d_render_c_Qt3DRender_QAbstractTextureImage_qt_metacall(Qt3DRender::QAbstractTextureImage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QAbstractTextureImage_qt_metacast(Qt3DRender::QAbstractTextureImage* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QAbstractTextureImage_setFace(Qt3DRender::QAbstractTextureImage* this_ptr, const Qt3DRender::QAbstractTexture::CubeMapFace* face) {
  this_ptr->setFace(*face);
}

void qt_3d_render_c_Qt3DRender_QAbstractTextureImage_setLayer(Qt3DRender::QAbstractTextureImage* this_ptr, int layer) {
  this_ptr->setLayer(layer);
}

void qt_3d_render_c_Qt3DRender_QAbstractTextureImage_setMipLevel(Qt3DRender::QAbstractTextureImage* this_ptr, int level) {
  this_ptr->setMipLevel(level);
}

void qt_3d_render_c_Qt3DRender_QAbstractTextureImage_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAbstractTextureImage::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QAbstractTextureImage_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAbstractTextureImage::tr(s, c, n));
}

