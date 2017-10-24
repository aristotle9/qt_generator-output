#include "qt_3d_input_c_QAnalogAxisInput.h"

Qt3DInput::QAnalogAxisInput* qt_3d_input_c_QAnalogAxisInput_G_dynamic_cast_Qt3DInput_QAnalogAxisInput_ptr(Qt3DInput::QAbstractAxisInput* ptr) {
  return dynamic_cast<Qt3DInput::QAnalogAxisInput*>(ptr);
}

QObject* qt_3d_input_c_QAnalogAxisInput_G_static_cast_QObject_ptr(Qt3DInput::QAnalogAxisInput* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAnalogAxisInput* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractAxisInput* qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(Qt3DInput::QAnalogAxisInput* ptr) {
  return static_cast<Qt3DInput::QAbstractAxisInput*>(ptr);
}

Qt3DInput::QAnalogAxisInput* qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAnalogAxisInput*>(ptr);
}

Qt3DInput::QAnalogAxisInput* qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAnalogAxisInput*>(ptr);
}

Qt3DInput::QAnalogAxisInput* qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_Qt3DInput_QAbstractAxisInput(Qt3DInput::QAbstractAxisInput* ptr) {
  return static_cast<Qt3DInput::QAnalogAxisInput*>(ptr);
}

int qt_3d_input_c_Qt3DInput_QAnalogAxisInput_axis(const Qt3DInput::QAnalogAxisInput* this_ptr) {
  return this_ptr->axis();
}

void qt_3d_input_c_Qt3DInput_QAnalogAxisInput_delete(Qt3DInput::QAnalogAxisInput* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAnalogAxisInput_metaObject(const Qt3DInput::QAnalogAxisInput* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QAnalogAxisInput* qt_3d_input_c_Qt3DInput_QAnalogAxisInput_new_no_args() {
  return new Qt3DInput::QAnalogAxisInput();
}

Qt3DInput::QAnalogAxisInput* qt_3d_input_c_Qt3DInput_QAnalogAxisInput_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QAnalogAxisInput(parent);
}

int qt_3d_input_c_Qt3DInput_QAnalogAxisInput_qt_metacall(Qt3DInput::QAnalogAxisInput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAnalogAxisInput_qt_metacast(Qt3DInput::QAnalogAxisInput* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QAnalogAxisInput_setAxis(Qt3DInput::QAnalogAxisInput* this_ptr, int axis) {
  this_ptr->setAxis(axis);
}

void qt_3d_input_c_Qt3DInput_QAnalogAxisInput_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAnalogAxisInput::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAnalogAxisInput_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAnalogAxisInput::tr(s, c, n));
}

