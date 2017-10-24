#include "qt_3d_input_c_QAbstractAxisInput.h"

QObject* qt_3d_input_c_QAbstractAxisInput_G_static_cast_QObject_ptr(Qt3DInput::QAbstractAxisInput* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAbstractAxisInput* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractAxisInput* qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAbstractAxisInput*>(ptr);
}

Qt3DInput::QAbstractAxisInput* qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAbstractAxisInput*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QAbstractAxisInput_delete(Qt3DInput::QAbstractAxisInput* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAbstractAxisInput_metaObject(const Qt3DInput::QAbstractAxisInput* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_input_c_Qt3DInput_QAbstractAxisInput_qt_metacall(Qt3DInput::QAbstractAxisInput* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAbstractAxisInput_qt_metacast(Qt3DInput::QAbstractAxisInput* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QAbstractAxisInput_setSourceDevice(Qt3DInput::QAbstractAxisInput* this_ptr, Qt3DInput::QAbstractPhysicalDevice* sourceDevice) {
  this_ptr->setSourceDevice(sourceDevice);
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QAbstractAxisInput_sourceDevice(const Qt3DInput::QAbstractAxisInput* this_ptr) {
  return this_ptr->sourceDevice();
}

void qt_3d_input_c_Qt3DInput_QAbstractAxisInput_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAbstractAxisInput::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAbstractAxisInput_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAbstractAxisInput::tr(s, c, n));
}

