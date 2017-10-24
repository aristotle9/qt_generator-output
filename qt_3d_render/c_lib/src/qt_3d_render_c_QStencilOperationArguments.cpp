#include "qt_3d_render_c_QStencilOperationArguments.h"

QObject* qt_3d_render_c_QStencilOperationArguments_G_static_cast_QObject_ptr(Qt3DRender::QStencilOperationArguments* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DRender::QStencilOperationArguments* qt_3d_render_c_QStencilOperationArguments_G_static_cast_Qt3DRender_QStencilOperationArguments_ptr(QObject* ptr) {
  return static_cast<Qt3DRender::QStencilOperationArguments*>(ptr);
}

Qt3DRender::QStencilOperationArguments::Operation qt_3d_render_c_Qt3DRender_QStencilOperationArguments_allTestsPassOperation(const Qt3DRender::QStencilOperationArguments* this_ptr) {
  return this_ptr->allTestsPassOperation();
}

void qt_3d_render_c_Qt3DRender_QStencilOperationArguments_delete(Qt3DRender::QStencilOperationArguments* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QStencilOperationArguments::Operation qt_3d_render_c_Qt3DRender_QStencilOperationArguments_depthTestFailureOperation(const Qt3DRender::QStencilOperationArguments* this_ptr) {
  return this_ptr->depthTestFailureOperation();
}

Qt3DRender::QStencilOperationArguments::FaceMode qt_3d_render_c_Qt3DRender_QStencilOperationArguments_faceMode(const Qt3DRender::QStencilOperationArguments* this_ptr) {
  return this_ptr->faceMode();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QStencilOperationArguments_metaObject(const Qt3DRender::QStencilOperationArguments* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QStencilOperationArguments_qt_metacall(Qt3DRender::QStencilOperationArguments* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QStencilOperationArguments_qt_metacast(Qt3DRender::QStencilOperationArguments* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QStencilOperationArguments_setAllTestsPassOperation(Qt3DRender::QStencilOperationArguments* this_ptr, Qt3DRender::QStencilOperationArguments::Operation operation) {
  this_ptr->setAllTestsPassOperation(operation);
}

void qt_3d_render_c_Qt3DRender_QStencilOperationArguments_setDepthTestFailureOperation(Qt3DRender::QStencilOperationArguments* this_ptr, Qt3DRender::QStencilOperationArguments::Operation operation) {
  this_ptr->setDepthTestFailureOperation(operation);
}

void qt_3d_render_c_Qt3DRender_QStencilOperationArguments_setStencilTestFailureOperation(Qt3DRender::QStencilOperationArguments* this_ptr, Qt3DRender::QStencilOperationArguments::Operation operation) {
  this_ptr->setStencilTestFailureOperation(operation);
}

Qt3DRender::QStencilOperationArguments::Operation qt_3d_render_c_Qt3DRender_QStencilOperationArguments_stencilTestFailureOperation(const Qt3DRender::QStencilOperationArguments* this_ptr) {
  return this_ptr->stencilTestFailureOperation();
}

void qt_3d_render_c_Qt3DRender_QStencilOperationArguments_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QStencilOperationArguments::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QStencilOperationArguments_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QStencilOperationArguments::tr(s, c, n));
}

