#include "qt_3d_render_c_QTextureGenerator.h"

Qt3DRender::QTextureGenerator* qt_3d_render_c_QTextureGenerator_G_dynamic_cast_Qt3DRender_QTextureGenerator_ptr(Qt3DRender::QAbstractFunctor* ptr) {
  return dynamic_cast<Qt3DRender::QTextureGenerator*>(ptr);
}

Qt3DRender::QAbstractFunctor* qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(Qt3DRender::QTextureGenerator* ptr) {
  return static_cast<Qt3DRender::QAbstractFunctor*>(ptr);
}

Qt3DRender::QTextureGenerator* qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QTextureGenerator_ptr(Qt3DRender::QAbstractFunctor* ptr) {
  return static_cast<Qt3DRender::QTextureGenerator*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTextureGenerator_delete(Qt3DRender::QTextureGenerator* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QTextureGenerator_operator_call_to_output(Qt3DRender::QTextureGenerator* this_ptr, QSharedPointer< Qt3DRender::QTextureData >* output) {
  new(output) QSharedPointer< Qt3DRender::QTextureData >(this_ptr->operator()());
}

bool qt_3d_render_c_Qt3DRender_QTextureGenerator_operator_eq(const Qt3DRender::QTextureGenerator* this_ptr, const Qt3DRender::QTextureGenerator* other) {
  return this_ptr->operator==(*other);
}

