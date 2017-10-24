#include "qt_3d_render_c_QBufferDataGenerator.h"

Qt3DRender::QBufferDataGenerator* qt_3d_render_c_QBufferDataGenerator_G_dynamic_cast_Qt3DRender_QBufferDataGenerator_ptr(Qt3DRender::QAbstractFunctor* ptr) {
  return dynamic_cast<Qt3DRender::QBufferDataGenerator*>(ptr);
}

Qt3DRender::QAbstractFunctor* qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(Qt3DRender::QBufferDataGenerator* ptr) {
  return static_cast<Qt3DRender::QAbstractFunctor*>(ptr);
}

Qt3DRender::QBufferDataGenerator* qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QBufferDataGenerator_ptr(Qt3DRender::QAbstractFunctor* ptr) {
  return static_cast<Qt3DRender::QBufferDataGenerator*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QBufferDataGenerator_delete(Qt3DRender::QBufferDataGenerator* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QBufferDataGenerator_operator_call_to_output(Qt3DRender::QBufferDataGenerator* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->operator()());
}

bool qt_3d_render_c_Qt3DRender_QBufferDataGenerator_operator_eq(const Qt3DRender::QBufferDataGenerator* this_ptr, const Qt3DRender::QBufferDataGenerator* other) {
  return this_ptr->operator==(*other);
}

