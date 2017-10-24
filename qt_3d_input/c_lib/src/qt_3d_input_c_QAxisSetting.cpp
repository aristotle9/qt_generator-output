#include "qt_3d_input_c_QAxisSetting.h"

QObject* qt_3d_input_c_QAxisSetting_G_static_cast_QObject_ptr(Qt3DInput::QAxisSetting* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAxisSetting* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAxisSetting* qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAxisSetting*>(ptr);
}

Qt3DInput::QAxisSetting* qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAxisSetting*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_axes_to_output(const Qt3DInput::QAxisSetting* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->axes());
}

float qt_3d_input_c_Qt3DInput_QAxisSetting_deadZoneRadius(const Qt3DInput::QAxisSetting* this_ptr) {
  return this_ptr->deadZoneRadius();
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_delete(Qt3DInput::QAxisSetting* this_ptr) {
  delete this_ptr;
}

bool qt_3d_input_c_Qt3DInput_QAxisSetting_isSmoothEnabled(const Qt3DInput::QAxisSetting* this_ptr) {
  return this_ptr->isSmoothEnabled();
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAxisSetting_metaObject(const Qt3DInput::QAxisSetting* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QAxisSetting* qt_3d_input_c_Qt3DInput_QAxisSetting_new_no_args() {
  return new Qt3DInput::QAxisSetting();
}

Qt3DInput::QAxisSetting* qt_3d_input_c_Qt3DInput_QAxisSetting_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QAxisSetting(parent);
}

int qt_3d_input_c_Qt3DInput_QAxisSetting_qt_metacall(Qt3DInput::QAxisSetting* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAxisSetting_qt_metacast(Qt3DInput::QAxisSetting* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_setAxes(Qt3DInput::QAxisSetting* this_ptr, const QVector< int >* axes) {
  this_ptr->setAxes(*axes);
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_setDeadZoneRadius(Qt3DInput::QAxisSetting* this_ptr, float deadZoneRadius) {
  this_ptr->setDeadZoneRadius(deadZoneRadius);
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_setSmoothEnabled(Qt3DInput::QAxisSetting* this_ptr, bool enabled) {
  this_ptr->setSmoothEnabled(enabled);
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAxisSetting::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAxisSetting_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAxisSetting::tr(s, c, n));
}

