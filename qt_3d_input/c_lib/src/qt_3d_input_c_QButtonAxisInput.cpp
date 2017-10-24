#include "qt_3d_input_c_QButtonAxisInput.h"

Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_dynamic_cast_Qt3DInput_QButtonAxisInput_ptr(Qt3DInput::QAbstractAxisInput* ptr) {
  return dynamic_cast<Qt3DInput::QButtonAxisInput*>(ptr);
}

QObject* qt_3d_input_c_QButtonAxisInput_G_static_cast_QObject_ptr(Qt3DInput::QButtonAxisInput* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QButtonAxisInput* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(Qt3DInput::QButtonAxisInput* ptr) {
  return static_cast<Qt3DInput::QAbstractAxisInput*>(ptr);
}

Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QButtonAxisInput*>(ptr);
}

Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QButtonAxisInput*>(ptr);
}

Qt3DInput::QButtonAxisInput* qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DInput_QAbstractAxisInput(Qt3DInput::QAbstractAxisInput* ptr) {
  return static_cast<Qt3DInput::QButtonAxisInput*>(ptr);
}

float qt_3d_input_c_Qt3DInput_QButtonAxisInput_acceleration(const Qt3DInput::QButtonAxisInput* this_ptr) {
  return this_ptr->acceleration();
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_buttons_to_output(const Qt3DInput::QButtonAxisInput* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->buttons());
}

float qt_3d_input_c_Qt3DInput_QButtonAxisInput_deceleration(const Qt3DInput::QButtonAxisInput* this_ptr) {
  return this_ptr->deceleration();
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_delete(Qt3DInput::QButtonAxisInput* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QButtonAxisInput_metaObject(const Qt3DInput::QButtonAxisInput* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QButtonAxisInput* qt_3d_input_c_Qt3DInput_QButtonAxisInput_new_no_args() {
  return new Qt3DInput::QButtonAxisInput();
}

Qt3DInput::QButtonAxisInput* qt_3d_input_c_Qt3DInput_QButtonAxisInput_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QButtonAxisInput(parent);
}

int qt_3d_input_c_Qt3DInput_QButtonAxisInput_qt_metacall(Qt3DInput::QButtonAxisInput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QButtonAxisInput_qt_metacast(Qt3DInput::QButtonAxisInput* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_input_c_Qt3DInput_QButtonAxisInput_scale(const Qt3DInput::QButtonAxisInput* this_ptr) {
  return this_ptr->scale();
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setAcceleration(Qt3DInput::QButtonAxisInput* this_ptr, float acceleration) {
  this_ptr->setAcceleration(acceleration);
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setButtons(Qt3DInput::QButtonAxisInput* this_ptr, const QVector< int >* buttons) {
  this_ptr->setButtons(*buttons);
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setDeceleration(Qt3DInput::QButtonAxisInput* this_ptr, float deceleration) {
  this_ptr->setDeceleration(deceleration);
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_setScale(Qt3DInput::QButtonAxisInput* this_ptr, float scale) {
  this_ptr->setScale(scale);
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QButtonAxisInput::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QButtonAxisInput_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QButtonAxisInput::tr(s, c, n));
}

