#include "qt_3d_input_c_QMouseDevice.h"

Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_dynamic_cast_Qt3DInput_QMouseDevice_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr) {
  return dynamic_cast<Qt3DInput::QMouseDevice*>(ptr);
}

QObject* qt_3d_input_c_QMouseDevice_G_static_cast_QObject_ptr(Qt3DInput::QMouseDevice* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QMouseDevice* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(Qt3DInput::QMouseDevice* ptr) {
  return static_cast<Qt3DInput::QAbstractPhysicalDevice*>(ptr);
}

Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QMouseDevice*>(ptr);
}

Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QMouseDevice*>(ptr);
}

Qt3DInput::QMouseDevice* qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(Qt3DInput::QAbstractPhysicalDevice* ptr) {
  return static_cast<Qt3DInput::QMouseDevice*>(ptr);
}

int qt_3d_input_c_Qt3DInput_QMouseDevice_axisCount(const Qt3DInput::QMouseDevice* this_ptr) {
  return this_ptr->axisCount();
}

int qt_3d_input_c_Qt3DInput_QMouseDevice_axisIdentifier(const Qt3DInput::QMouseDevice* this_ptr, const QString* name) {
  return this_ptr->axisIdentifier(*name);
}

void qt_3d_input_c_Qt3DInput_QMouseDevice_axisNames_to_output(const Qt3DInput::QMouseDevice* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->axisNames());
}

int qt_3d_input_c_Qt3DInput_QMouseDevice_buttonCount(const Qt3DInput::QMouseDevice* this_ptr) {
  return this_ptr->buttonCount();
}

int qt_3d_input_c_Qt3DInput_QMouseDevice_buttonIdentifier(const Qt3DInput::QMouseDevice* this_ptr, const QString* name) {
  return this_ptr->buttonIdentifier(*name);
}

void qt_3d_input_c_Qt3DInput_QMouseDevice_buttonNames_to_output(const Qt3DInput::QMouseDevice* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->buttonNames());
}

void qt_3d_input_c_Qt3DInput_QMouseDevice_delete(Qt3DInput::QMouseDevice* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QMouseDevice_metaObject(const Qt3DInput::QMouseDevice* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QMouseDevice* qt_3d_input_c_Qt3DInput_QMouseDevice_new_no_args() {
  return new Qt3DInput::QMouseDevice();
}

Qt3DInput::QMouseDevice* qt_3d_input_c_Qt3DInput_QMouseDevice_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QMouseDevice(parent);
}

int qt_3d_input_c_Qt3DInput_QMouseDevice_qt_metacall(Qt3DInput::QMouseDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QMouseDevice_qt_metacast(Qt3DInput::QMouseDevice* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_input_c_Qt3DInput_QMouseDevice_sensitivity(const Qt3DInput::QMouseDevice* this_ptr) {
  return this_ptr->sensitivity();
}

void qt_3d_input_c_Qt3DInput_QMouseDevice_setSensitivity(Qt3DInput::QMouseDevice* this_ptr, float value) {
  this_ptr->setSensitivity(value);
}

void qt_3d_input_c_Qt3DInput_QMouseDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QMouseDevice::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QMouseDevice_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QMouseDevice::tr(s, c, n));
}

