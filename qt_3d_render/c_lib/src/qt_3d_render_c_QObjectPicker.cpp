#include "qt_3d_render_c_QObjectPicker.h"

QObject* qt_3d_render_c_QObjectPicker_G_static_cast_QObject_ptr(Qt3DRender::QObjectPicker* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QObjectPicker* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QObjectPicker* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QObjectPicker* qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QObjectPicker*>(ptr);
}

Qt3DRender::QObjectPicker* qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QObjectPicker*>(ptr);
}

Qt3DRender::QObjectPicker* qt_3d_render_c_QObjectPicker_G_static_cast_Qt3DRender_QObjectPicker_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QObjectPicker*>(ptr);
}

bool qt_3d_render_c_Qt3DRender_QObjectPicker_containsMouse(const Qt3DRender::QObjectPicker* this_ptr) {
  return this_ptr->containsMouse();
}

void qt_3d_render_c_Qt3DRender_QObjectPicker_delete(Qt3DRender::QObjectPicker* this_ptr) {
  delete this_ptr;
}

bool qt_3d_render_c_Qt3DRender_QObjectPicker_isDragEnabled(const Qt3DRender::QObjectPicker* this_ptr) {
  return this_ptr->isDragEnabled();
}

bool qt_3d_render_c_Qt3DRender_QObjectPicker_isHoverEnabled(const Qt3DRender::QObjectPicker* this_ptr) {
  return this_ptr->isHoverEnabled();
}

bool qt_3d_render_c_Qt3DRender_QObjectPicker_isPressed(const Qt3DRender::QObjectPicker* this_ptr) {
  return this_ptr->isPressed();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QObjectPicker_metaObject(const Qt3DRender::QObjectPicker* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QObjectPicker* qt_3d_render_c_Qt3DRender_QObjectPicker_new_no_args() {
  return new Qt3DRender::QObjectPicker();
}

Qt3DRender::QObjectPicker* qt_3d_render_c_Qt3DRender_QObjectPicker_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QObjectPicker(parent);
}

int qt_3d_render_c_Qt3DRender_QObjectPicker_qt_metacall(Qt3DRender::QObjectPicker* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QObjectPicker_qt_metacast(Qt3DRender::QObjectPicker* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QObjectPicker_setDragEnabled(Qt3DRender::QObjectPicker* this_ptr, bool dragEnabled) {
  this_ptr->setDragEnabled(dragEnabled);
}

void qt_3d_render_c_Qt3DRender_QObjectPicker_setHoverEnabled(Qt3DRender::QObjectPicker* this_ptr, bool hoverEnabled) {
  this_ptr->setHoverEnabled(hoverEnabled);
}

void qt_3d_render_c_Qt3DRender_QObjectPicker_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QObjectPicker::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QObjectPicker_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QObjectPicker::tr(s, c, n));
}

