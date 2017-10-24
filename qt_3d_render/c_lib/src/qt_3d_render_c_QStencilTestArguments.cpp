#include "qt_3d_render_c_QStencilTestArguments.h"

QObject* qt_3d_render_c_QStencilTestArguments_G_static_cast_QObject_ptr(Qt3DRender::QStencilTestArguments* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DRender::QStencilTestArguments* qt_3d_render_c_QStencilTestArguments_G_static_cast_Qt3DRender_QStencilTestArguments_ptr(QObject* ptr) {
  return static_cast<Qt3DRender::QStencilTestArguments*>(ptr);
}

unsigned int qt_3d_render_c_Qt3DRender_QStencilTestArguments_comparisonMask(const Qt3DRender::QStencilTestArguments* this_ptr) {
  return this_ptr->comparisonMask();
}

void qt_3d_render_c_Qt3DRender_QStencilTestArguments_delete(Qt3DRender::QStencilTestArguments* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QStencilTestArguments::StencilFaceMode qt_3d_render_c_Qt3DRender_QStencilTestArguments_faceMode(const Qt3DRender::QStencilTestArguments* this_ptr) {
  return this_ptr->faceMode();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QStencilTestArguments_metaObject(const Qt3DRender::QStencilTestArguments* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QStencilTestArguments_qt_metacall(Qt3DRender::QStencilTestArguments* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QStencilTestArguments_qt_metacast(Qt3DRender::QStencilTestArguments* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_3d_render_c_Qt3DRender_QStencilTestArguments_referenceValue(const Qt3DRender::QStencilTestArguments* this_ptr) {
  return this_ptr->referenceValue();
}

void qt_3d_render_c_Qt3DRender_QStencilTestArguments_setComparisonMask(Qt3DRender::QStencilTestArguments* this_ptr, unsigned int comparisonMask) {
  this_ptr->setComparisonMask(comparisonMask);
}

void qt_3d_render_c_Qt3DRender_QStencilTestArguments_setReferenceValue(Qt3DRender::QStencilTestArguments* this_ptr, int referenceValue) {
  this_ptr->setReferenceValue(referenceValue);
}

void qt_3d_render_c_Qt3DRender_QStencilTestArguments_setStencilFunction(Qt3DRender::QStencilTestArguments* this_ptr, Qt3DRender::QStencilTestArguments::StencilFunction stencilFunction) {
  this_ptr->setStencilFunction(stencilFunction);
}

Qt3DRender::QStencilTestArguments::StencilFunction qt_3d_render_c_Qt3DRender_QStencilTestArguments_stencilFunction(const Qt3DRender::QStencilTestArguments* this_ptr) {
  return this_ptr->stencilFunction();
}

void qt_3d_render_c_Qt3DRender_QStencilTestArguments_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QStencilTestArguments::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QStencilTestArguments_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QStencilTestArguments::tr(s, c, n));
}

