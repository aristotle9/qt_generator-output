#include "qt_3d_render_c_QRenderSettings.h"

QObject* qt_3d_render_c_QRenderSettings_G_static_cast_QObject_ptr(Qt3DRender::QRenderSettings* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QRenderSettings* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QRenderSettings* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QRenderSettings* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderSettings*>(ptr);
}

Qt3DRender::QRenderSettings* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QRenderSettings*>(ptr);
}

Qt3DRender::QRenderSettings* qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QRenderSettings*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_Qt3DRender_QRenderSettings_activeFrameGraph(const Qt3DRender::QRenderSettings* this_ptr) {
  return this_ptr->activeFrameGraph();
}

void qt_3d_render_c_Qt3DRender_QRenderSettings_delete(Qt3DRender::QRenderSettings* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderSettings_metaObject(const Qt3DRender::QRenderSettings* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderSettings* qt_3d_render_c_Qt3DRender_QRenderSettings_new_no_args() {
  return new Qt3DRender::QRenderSettings();
}

Qt3DRender::QRenderSettings* qt_3d_render_c_Qt3DRender_QRenderSettings_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QRenderSettings(parent);
}

Qt3DRender::QPickingSettings* qt_3d_render_c_Qt3DRender_QRenderSettings_pickingSettings(Qt3DRender::QRenderSettings* this_ptr) {
  return this_ptr->pickingSettings();
}

int qt_3d_render_c_Qt3DRender_QRenderSettings_qt_metacall(Qt3DRender::QRenderSettings* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderSettings_qt_metacast(Qt3DRender::QRenderSettings* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

Qt3DRender::QRenderSettings::RenderPolicy qt_3d_render_c_Qt3DRender_QRenderSettings_renderPolicy(const Qt3DRender::QRenderSettings* this_ptr) {
  return this_ptr->renderPolicy();
}

void qt_3d_render_c_Qt3DRender_QRenderSettings_setActiveFrameGraph(Qt3DRender::QRenderSettings* this_ptr, Qt3DRender::QFrameGraphNode* activeFrameGraph) {
  this_ptr->setActiveFrameGraph(activeFrameGraph);
}

void qt_3d_render_c_Qt3DRender_QRenderSettings_setRenderPolicy(Qt3DRender::QRenderSettings* this_ptr, Qt3DRender::QRenderSettings::RenderPolicy renderPolicy) {
  this_ptr->setRenderPolicy(renderPolicy);
}

void qt_3d_render_c_Qt3DRender_QRenderSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderSettings::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderSettings_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderSettings::tr(s, c, n));
}

