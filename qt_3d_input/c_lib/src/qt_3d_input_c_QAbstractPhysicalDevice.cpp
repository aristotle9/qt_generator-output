#include "qt_3d_input_c_QAbstractPhysicalDevice.h"

QObject* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_QObject_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAbstractPhysicalDevice* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAbstractPhysicalDevice*>(ptr);
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAbstractPhysicalDevice*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_addAxisSetting(Qt3DInput::QAbstractPhysicalDevice* this_ptr, Qt3DInput::QAxisSetting* axisSetting) {
  this_ptr->addAxisSetting(axisSetting);
}

int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisCount(const Qt3DInput::QAbstractPhysicalDevice* this_ptr) {
  return this_ptr->axisCount();
}

int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisIdentifier(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, const QString* name) {
  return this_ptr->axisIdentifier(*name);
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisNames_to_output(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->axisNames());
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisSettings_to_output(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, QVector< Qt3DInput::QAxisSetting* >* output) {
  new(output) QVector< Qt3DInput::QAxisSetting* >(this_ptr->axisSettings());
}

int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonCount(const Qt3DInput::QAbstractPhysicalDevice* this_ptr) {
  return this_ptr->buttonCount();
}

int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonIdentifier(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, const QString* name) {
  return this_ptr->buttonIdentifier(*name);
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonNames_to_output(const Qt3DInput::QAbstractPhysicalDevice* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->buttonNames());
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_delete(Qt3DInput::QAbstractPhysicalDevice* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_metaObject(const Qt3DInput::QAbstractPhysicalDevice* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_new_no_args() {
  return new Qt3DInput::QAbstractPhysicalDevice();
}

Qt3DInput::QAbstractPhysicalDevice* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QAbstractPhysicalDevice(parent);
}

int qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_qt_metacall(Qt3DInput::QAbstractPhysicalDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_qt_metacast(Qt3DInput::QAbstractPhysicalDevice* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_removeAxisSetting(Qt3DInput::QAbstractPhysicalDevice* this_ptr, Qt3DInput::QAxisSetting* axisSetting) {
  this_ptr->removeAxisSetting(axisSetting);
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAbstractPhysicalDevice::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAbstractPhysicalDevice::tr(s, c, n));
}

