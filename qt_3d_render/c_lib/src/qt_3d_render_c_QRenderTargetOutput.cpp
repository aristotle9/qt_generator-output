#include "qt_3d_render_c_QRenderTargetOutput.h"

QObject* qt_3d_render_c_QRenderTargetOutput_G_static_cast_QObject_ptr(Qt3DRender::QRenderTargetOutput* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderTargetOutput* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QRenderTargetOutput* qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderTargetOutput*>(ptr);
}

Qt3DRender::QRenderTargetOutput* qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderTargetOutput*>(ptr);
}

Qt3DRender::QRenderTargetOutput::AttachmentPoint qt_3d_render_c_Qt3DRender_QRenderTargetOutput_attachmentPoint(const Qt3DRender::QRenderTargetOutput* this_ptr) {
  return this_ptr->attachmentPoint();
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_delete(Qt3DRender::QRenderTargetOutput* this_ptr) {
  delete this_ptr;
}

int qt_3d_render_c_Qt3DRender_QRenderTargetOutput_layer(const Qt3DRender::QRenderTargetOutput* this_ptr) {
  return this_ptr->layer();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_metaObject(const Qt3DRender::QRenderTargetOutput* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QRenderTargetOutput_mipLevel(const Qt3DRender::QRenderTargetOutput* this_ptr) {
  return this_ptr->mipLevel();
}

Qt3DRender::QRenderTargetOutput* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_new_no_args() {
  return new Qt3DRender::QRenderTargetOutput();
}

Qt3DRender::QRenderTargetOutput* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderTargetOutput(parent);
}

int qt_3d_render_c_Qt3DRender_QRenderTargetOutput_qt_metacall(Qt3DRender::QRenderTargetOutput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_qt_metacast(Qt3DRender::QRenderTargetOutput* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setAttachmentPoint(Qt3DRender::QRenderTargetOutput* this_ptr, Qt3DRender::QRenderTargetOutput::AttachmentPoint attachmentPoint) {
  this_ptr->setAttachmentPoint(attachmentPoint);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setFace(Qt3DRender::QRenderTargetOutput* this_ptr, const Qt3DRender::QAbstractTexture::CubeMapFace* face) {
  this_ptr->setFace(*face);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setLayer(Qt3DRender::QRenderTargetOutput* this_ptr, int layer) {
  this_ptr->setLayer(layer);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setMipLevel(Qt3DRender::QRenderTargetOutput* this_ptr, int level) {
  this_ptr->setMipLevel(level);
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setTexture(Qt3DRender::QRenderTargetOutput* this_ptr, Qt3DRender::QAbstractTexture* texture) {
  this_ptr->setTexture(texture);
}

Qt3DRender::QAbstractTexture* qt_3d_render_c_Qt3DRender_QRenderTargetOutput_texture(const Qt3DRender::QRenderTargetOutput* this_ptr) {
  return this_ptr->texture();
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderTargetOutput::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderTargetOutput_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderTargetOutput::tr(s, c, n));
}

