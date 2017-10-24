#include "qt_3d_extras_c_Qt3DWindow.h"

Qt3DRender::QFrameGraphNode* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_activeFrameGraph(const Qt3DExtras::Qt3DWindow* this_ptr) {
  return this_ptr->activeFrameGraph();
}

Qt3DRender::QCamera* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_camera(const Qt3DExtras::Qt3DWindow* this_ptr) {
  return this_ptr->camera();
}

Qt3DExtras::QForwardRenderer* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_defaultFrameGraph(const Qt3DExtras::Qt3DWindow* this_ptr) {
  return this_ptr->defaultFrameGraph();
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_delete(Qt3DExtras::Qt3DWindow* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_metaObject(const Qt3DExtras::Qt3DWindow* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_new_no_args() {
  return new Qt3DExtras::Qt3DWindow();
}

Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_new_screen(QScreen* screen) {
  return new Qt3DExtras::Qt3DWindow(screen);
}

int qt_3d_extras_c_Qt3DExtras_Qt3DWindow_qt_metacall(Qt3DExtras::Qt3DWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_qt_metacast(Qt3DExtras::Qt3DWindow* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_registerAspect_aspect(Qt3DExtras::Qt3DWindow* this_ptr, Qt3DCore::QAbstractAspect* aspect) {
  this_ptr->registerAspect(aspect);
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_registerAspect_name(Qt3DExtras::Qt3DWindow* this_ptr, const QString* name) {
  this_ptr->registerAspect(*name);
}

Qt3DRender::QRenderSettings* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_renderSettings(const Qt3DExtras::Qt3DWindow* this_ptr) {
  return this_ptr->renderSettings();
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_setActiveFrameGraph(Qt3DExtras::Qt3DWindow* this_ptr, Qt3DRender::QFrameGraphNode* activeFrameGraph) {
  this_ptr->setActiveFrameGraph(activeFrameGraph);
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_setRootEntity(Qt3DExtras::Qt3DWindow* this_ptr, Qt3DCore::QEntity* root) {
  this_ptr->setRootEntity(root);
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::Qt3DWindow::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::Qt3DWindow::tr(s, c, n));
}

QObject* qt_3d_extras_c_Qt3DWindow_G_static_cast_QObject_ptr(Qt3DExtras::Qt3DWindow* ptr) {
  return static_cast<QObject*>(ptr);
}

QSurface* qt_3d_extras_c_Qt3DWindow_G_static_cast_QSurface_ptr(Qt3DExtras::Qt3DWindow* ptr) {
  return static_cast<QSurface*>(ptr);
}

QWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_QWindow_ptr(Qt3DExtras::Qt3DWindow* ptr) {
  return static_cast<QWindow*>(ptr);
}

Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::Qt3DWindow*>(ptr);
}

Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QSurface(QSurface* ptr) {
  return static_cast<Qt3DExtras::Qt3DWindow*>(ptr);
}

Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QWindow(QWindow* ptr) {
  return static_cast<Qt3DExtras::Qt3DWindow*>(ptr);
}

