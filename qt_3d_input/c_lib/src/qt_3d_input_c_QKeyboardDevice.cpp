#include "qt_3d_input_c_QKeyboardDevice.h"

Qt3DInput::QKeyboardDevice* qt_3d_input_c_QKeyboardDevice_G_dynamic_cast_Qt3DInput_QKeyboardDevice_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr) {
  return dynamic_cast<Qt3DInput::QKeyboardDevice*>(ptr);
}

QObject* qt_3d_input_c_QKeyboardDevice_G_static_cast_QObject_ptr(Qt3DInput::QKeyboardDevice* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QKeyboardDevice* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(Qt3DInput::QKeyboardDevice* ptr) {
  return static_cast<Qt3DInput::QAbstractPhysicalDevice*>(ptr);
}

Qt3DInput::QKeyboardDevice* qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QKeyboardDevice*>(ptr);
}

Qt3DInput::QKeyboardDevice* qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QKeyboardDevice*>(ptr);
}

Qt3DInput::QKeyboardDevice* qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(Qt3DInput::QAbstractPhysicalDevice* ptr) {
  return static_cast<Qt3DInput::QKeyboardDevice*>(ptr);
}

Qt3DInput::QKeyboardHandler* qt_3d_input_c_Qt3DInput_QKeyboardDevice_activeInput(const Qt3DInput::QKeyboardDevice* this_ptr) {
  return this_ptr->activeInput();
}

int qt_3d_input_c_Qt3DInput_QKeyboardDevice_axisCount(const Qt3DInput::QKeyboardDevice* this_ptr) {
  return this_ptr->axisCount();
}

int qt_3d_input_c_Qt3DInput_QKeyboardDevice_axisIdentifier(const Qt3DInput::QKeyboardDevice* this_ptr, const QString* name) {
  return this_ptr->axisIdentifier(*name);
}

void qt_3d_input_c_Qt3DInput_QKeyboardDevice_axisNames_to_output(const Qt3DInput::QKeyboardDevice* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->axisNames());
}

int qt_3d_input_c_Qt3DInput_QKeyboardDevice_buttonCount(const Qt3DInput::QKeyboardDevice* this_ptr) {
  return this_ptr->buttonCount();
}

int qt_3d_input_c_Qt3DInput_QKeyboardDevice_buttonIdentifier(const Qt3DInput::QKeyboardDevice* this_ptr, const QString* name) {
  return this_ptr->buttonIdentifier(*name);
}

void qt_3d_input_c_Qt3DInput_QKeyboardDevice_buttonNames_to_output(const Qt3DInput::QKeyboardDevice* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->buttonNames());
}

void qt_3d_input_c_Qt3DInput_QKeyboardDevice_delete(Qt3DInput::QKeyboardDevice* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QKeyboardDevice_metaObject(const Qt3DInput::QKeyboardDevice* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QKeyboardDevice* qt_3d_input_c_Qt3DInput_QKeyboardDevice_new_no_args() {
  return new Qt3DInput::QKeyboardDevice();
}

Qt3DInput::QKeyboardDevice* qt_3d_input_c_Qt3DInput_QKeyboardDevice_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QKeyboardDevice(parent);
}

int qt_3d_input_c_Qt3DInput_QKeyboardDevice_qt_metacall(Qt3DInput::QKeyboardDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QKeyboardDevice_qt_metacast(Qt3DInput::QKeyboardDevice* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QKeyboardDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QKeyboardDevice::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QKeyboardDevice_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QKeyboardDevice::tr(s, c, n));
}

