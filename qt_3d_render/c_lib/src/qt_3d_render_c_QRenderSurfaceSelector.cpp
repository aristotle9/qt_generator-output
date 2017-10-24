#include "qt_3d_render_c_QRenderSurfaceSelector.h"

Qt3DRender::QRenderSurfaceSelector* qt_3d_render_c_QRenderSurfaceSelector_G_dynamic_cast_Qt3DRender_QRenderSurfaceSelector_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QRenderSurfaceSelector*>(ptr);
}

QObject* qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_QObject_ptr(Qt3DRender::QRenderSurfaceSelector* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderSurfaceSelector* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QRenderSurfaceSelector* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QRenderSurfaceSelector* qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderSurfaceSelector*>(ptr);
}

Qt3DRender::QRenderSurfaceSelector* qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderSurfaceSelector*>(ptr);
}

Qt3DRender::QRenderSurfaceSelector* qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QRenderSurfaceSelector*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_delete(Qt3DRender::QRenderSurfaceSelector* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_externalRenderTargetSize_to_output(const Qt3DRender::QRenderSurfaceSelector* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->externalRenderTargetSize());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_metaObject(const Qt3DRender::QRenderSurfaceSelector* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderSurfaceSelector* qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_new_no_args() {
  return new Qt3DRender::QRenderSurfaceSelector();
}

Qt3DRender::QRenderSurfaceSelector* qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderSurfaceSelector(parent);
}

int qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_qt_metacall(Qt3DRender::QRenderSurfaceSelector* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_qt_metacast(Qt3DRender::QRenderSurfaceSelector* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_setExternalRenderTargetSize(Qt3DRender::QRenderSurfaceSelector* this_ptr, const QSize* size) {
  this_ptr->setExternalRenderTargetSize(*size);
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_setSurface(Qt3DRender::QRenderSurfaceSelector* this_ptr, QObject* surfaceObject) {
  this_ptr->setSurface(surfaceObject);
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_setSurfacePixelRatio(Qt3DRender::QRenderSurfaceSelector* this_ptr, float ratio) {
  this_ptr->setSurfacePixelRatio(ratio);
}

QObject* qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_surface(const Qt3DRender::QRenderSurfaceSelector* this_ptr) {
  return this_ptr->surface();
}

float qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_surfacePixelRatio(const Qt3DRender::QRenderSurfaceSelector* this_ptr) {
  return this_ptr->surfacePixelRatio();
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderSurfaceSelector::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderSurfaceSelector::tr(s, c, n));
}

