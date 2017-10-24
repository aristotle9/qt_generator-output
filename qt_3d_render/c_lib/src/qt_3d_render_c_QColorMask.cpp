#include "qt_3d_render_c_QColorMask.h"

Qt3DRender::QColorMask* qt_3d_render_c_QColorMask_G_dynamic_cast_Qt3DRender_QColorMask_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QColorMask*>(ptr);
}

QObject* qt_3d_render_c_QColorMask_G_static_cast_QObject_ptr(Qt3DRender::QColorMask* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QColorMask_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QColorMask* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QColorMask* qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QColorMask*>(ptr);
}

Qt3DRender::QColorMask* qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QColorMask*>(ptr);
}

Qt3DRender::QColorMask* qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QColorMask_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QColorMask*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QColorMask_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QColorMask* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QColorMask_delete(Qt3DRender::QColorMask* this_ptr) {
  delete this_ptr;
}

bool qt_3d_render_c_Qt3DRender_QColorMask_isAlphaMasked(const Qt3DRender::QColorMask* this_ptr) {
  return this_ptr->isAlphaMasked();
}

bool qt_3d_render_c_Qt3DRender_QColorMask_isBlueMasked(const Qt3DRender::QColorMask* this_ptr) {
  return this_ptr->isBlueMasked();
}

bool qt_3d_render_c_Qt3DRender_QColorMask_isGreenMasked(const Qt3DRender::QColorMask* this_ptr) {
  return this_ptr->isGreenMasked();
}

bool qt_3d_render_c_Qt3DRender_QColorMask_isRedMasked(const Qt3DRender::QColorMask* this_ptr) {
  return this_ptr->isRedMasked();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QColorMask_metaObject(const Qt3DRender::QColorMask* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QColorMask* qt_3d_render_c_Qt3DRender_QColorMask_new_no_args() {
  return new Qt3DRender::QColorMask();
}

Qt3DRender::QColorMask* qt_3d_render_c_Qt3DRender_QColorMask_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QColorMask(parent);
}

int qt_3d_render_c_Qt3DRender_QColorMask_qt_metacall(Qt3DRender::QColorMask* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QColorMask_qt_metacast(Qt3DRender::QColorMask* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QColorMask_setAlphaMasked(Qt3DRender::QColorMask* this_ptr, bool alphaMasked) {
  this_ptr->setAlphaMasked(alphaMasked);
}

void qt_3d_render_c_Qt3DRender_QColorMask_setBlueMasked(Qt3DRender::QColorMask* this_ptr, bool blueMasked) {
  this_ptr->setBlueMasked(blueMasked);
}

void qt_3d_render_c_Qt3DRender_QColorMask_setGreenMasked(Qt3DRender::QColorMask* this_ptr, bool greenMasked) {
  this_ptr->setGreenMasked(greenMasked);
}

void qt_3d_render_c_Qt3DRender_QColorMask_setRedMasked(Qt3DRender::QColorMask* this_ptr, bool redMasked) {
  this_ptr->setRedMasked(redMasked);
}

void qt_3d_render_c_Qt3DRender_QColorMask_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QColorMask::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QColorMask_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QColorMask::tr(s, c, n));
}

