#include "qt_3d_extras_c_QForwardRenderer.h"

QObject* qt_3d_extras_c_QForwardRenderer_G_static_cast_QObject_ptr(Qt3DExtras::QForwardRenderer* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QForwardRenderer* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QForwardRenderer*>(ptr);
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QForwardRenderer*>(ptr);
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DExtras::QForwardRenderer*>(ptr);
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DRender_QTechniqueFilter(Qt3DRender::QTechniqueFilter* ptr) {
  return static_cast<Qt3DExtras::QForwardRenderer*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DExtras::QForwardRenderer* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QTechniqueFilter* qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QTechniqueFilter_ptr(Qt3DExtras::QForwardRenderer* ptr) {
  return static_cast<Qt3DRender::QTechniqueFilter*>(ptr);
}

Qt3DCore::QEntity* qt_3d_extras_c_Qt3DExtras_QForwardRenderer_camera(const Qt3DExtras::QForwardRenderer* this_ptr) {
  return this_ptr->camera();
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_clearColor_to_output(const Qt3DExtras::QForwardRenderer* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->clearColor());
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_delete(Qt3DExtras::QForwardRenderer* this_ptr) {
  delete this_ptr;
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_externalRenderTargetSize_to_output(const Qt3DExtras::QForwardRenderer* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->externalRenderTargetSize());
}

float qt_3d_extras_c_Qt3DExtras_QForwardRenderer_gamma(const Qt3DExtras::QForwardRenderer* this_ptr) {
  return this_ptr->gamma();
}

bool qt_3d_extras_c_Qt3DExtras_QForwardRenderer_isFrustumCullingEnabled(const Qt3DExtras::QForwardRenderer* this_ptr) {
  return this_ptr->isFrustumCullingEnabled();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QForwardRenderer_metaObject(const Qt3DExtras::QForwardRenderer* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_Qt3DExtras_QForwardRenderer_new_no_args() {
  return new Qt3DExtras::QForwardRenderer();
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_Qt3DExtras_QForwardRenderer_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QForwardRenderer(parent);
}

int qt_3d_extras_c_Qt3DExtras_QForwardRenderer_qt_metacall(Qt3DExtras::QForwardRenderer* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QForwardRenderer_qt_metacast(Qt3DExtras::QForwardRenderer* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setCamera(Qt3DExtras::QForwardRenderer* this_ptr, Qt3DCore::QEntity* camera) {
  this_ptr->setCamera(camera);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setClearColor(Qt3DExtras::QForwardRenderer* this_ptr, const QColor* clearColor) {
  this_ptr->setClearColor(*clearColor);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setExternalRenderTargetSize(Qt3DExtras::QForwardRenderer* this_ptr, const QSize* size) {
  this_ptr->setExternalRenderTargetSize(*size);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setFrustumCullingEnabled(Qt3DExtras::QForwardRenderer* this_ptr, bool enabled) {
  this_ptr->setFrustumCullingEnabled(enabled);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setGamma(Qt3DExtras::QForwardRenderer* this_ptr, float gamma) {
  this_ptr->setGamma(gamma);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setSurface(Qt3DExtras::QForwardRenderer* this_ptr, QObject* surface) {
  this_ptr->setSurface(surface);
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setViewportRect(Qt3DExtras::QForwardRenderer* this_ptr, const QRectF* viewportRect) {
  this_ptr->setViewportRect(*viewportRect);
}

QObject* qt_3d_extras_c_Qt3DExtras_QForwardRenderer_surface(const Qt3DExtras::QForwardRenderer* this_ptr) {
  return this_ptr->surface();
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QForwardRenderer::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QForwardRenderer::tr(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QForwardRenderer_viewportRect_to_output(const Qt3DExtras::QForwardRenderer* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->viewportRect());
}

