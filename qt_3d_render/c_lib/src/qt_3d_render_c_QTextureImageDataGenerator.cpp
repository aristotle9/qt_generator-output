#include "qt_3d_render_c_QTextureImageDataGenerator.h"

Qt3DRender::QTextureImageDataGenerator* qt_3d_render_c_QTextureImageDataGenerator_G_dynamic_cast_Qt3DRender_QTextureImageDataGenerator_ptr(Qt3DRender::QAbstractFunctor* ptr) {
  return dynamic_cast<Qt3DRender::QTextureImageDataGenerator*>(ptr);
}

Qt3DRender::QAbstractFunctor* qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(Qt3DRender::QTextureImageDataGenerator* ptr) {
  return static_cast<Qt3DRender::QAbstractFunctor*>(ptr);
}

Qt3DRender::QTextureImageDataGenerator* qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QTextureImageDataGenerator_ptr(Qt3DRender::QAbstractFunctor* ptr) {
  return static_cast<Qt3DRender::QTextureImageDataGenerator*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTextureImageDataGenerator_delete(Qt3DRender::QTextureImageDataGenerator* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QTextureImageDataGenerator_operator_call_to_output(Qt3DRender::QTextureImageDataGenerator* this_ptr, QSharedPointer< Qt3DRender::QTextureImageData >* output) {
  new(output) QSharedPointer< Qt3DRender::QTextureImageData >(this_ptr->operator()());
}

bool qt_3d_render_c_Qt3DRender_QTextureImageDataGenerator_operator_eq(const Qt3DRender::QTextureImageDataGenerator* this_ptr, const Qt3DRender::QTextureImageDataGenerator* other) {
  return this_ptr->operator==(*other);
}

