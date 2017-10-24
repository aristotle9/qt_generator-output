#include "qt_3d_input_c_QActionInput.h"

Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_dynamic_cast_Qt3DInput_QActionInput_ptr(Qt3DInput::QAbstractActionInput* ptr) {
  return dynamic_cast<Qt3DInput::QActionInput*>(ptr);
}

QObject* qt_3d_input_c_QActionInput_G_static_cast_QObject_ptr(Qt3DInput::QActionInput* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QActionInput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QActionInput* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(Qt3DInput::QActionInput* ptr) {
  return static_cast<Qt3DInput::QAbstractActionInput*>(ptr);
}

Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QActionInput*>(ptr);
}

Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QActionInput*>(ptr);
}

Qt3DInput::QActionInput* qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DInput_QAbstractActionInput(Qt3DInput::QAbstractActionInput* ptr) {
  return static_cast<Qt3DInput::QActionInput*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QActionInput_buttons_to_output(const Qt3DInput::QActionInput* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->buttons());
}

void qt_3d_input_c_Qt3DInput_QActionInput_delete(Qt3DInput::QActionInput* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QActionInput_metaObject(const Qt3DInput::QActionInput* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QActionInput* qt_3d_input_c_Qt3DInput_QActionInput_new_no_args() {
  return new Qt3DInput::QActionInput();
}

Qt3DInput::QActionInput* qt_3d_input_c_Qt3DInput_QActionInput_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QActionInput(parent);
}

int qt_3d_input_c_Qt3DInput_QActionInput_qt_metacall(Qt3DInput::QActionInput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QActionInput_qt_metacast(Qt3DInput::QActionInput* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QActionInput_setButtons(Qt3DInput::QActionInput* this_ptr, const QVector< int >* buttons) {
  this_ptr->setButtons(*buttons);
}

void qt_3d_input_c_Qt3DInput_QActionInput_setSourceDevice(Qt3DInput::QActionInput* this_ptr, Qt3DInput::QAbstractPhysicalDevice* sourceDevice) {
  this_ptr->setSourceDevice(sourceDevice);
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QActionInput_sourceDevice(const Qt3DInput::QActionInput* this_ptr) {
  return this_ptr->sourceDevice();
}

void qt_3d_input_c_Qt3DInput_QActionInput_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QActionInput::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QActionInput_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QActionInput::tr(s, c, n));
}

