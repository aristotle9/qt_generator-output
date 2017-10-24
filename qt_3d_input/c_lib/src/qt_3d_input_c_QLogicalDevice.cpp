#include "qt_3d_input_c_QLogicalDevice.h"

QObject* qt_3d_input_c_QLogicalDevice_G_static_cast_QObject_ptr(Qt3DInput::QLogicalDevice* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QLogicalDevice* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QLogicalDevice* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QLogicalDevice* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QLogicalDevice*>(ptr);
}

Qt3DInput::QLogicalDevice* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DInput::QLogicalDevice*>(ptr);
}

Qt3DInput::QLogicalDevice* qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QLogicalDevice*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_actions_to_output(const Qt3DInput::QLogicalDevice* this_ptr, QVector< Qt3DInput::QAction* >* output) {
  new(output) QVector< Qt3DInput::QAction* >(this_ptr->actions());
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_addAction(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAction* action) {
  this_ptr->addAction(action);
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_addAxis(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAxis* axis) {
  this_ptr->addAxis(axis);
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_axes_to_output(const Qt3DInput::QLogicalDevice* this_ptr, QVector< Qt3DInput::QAxis* >* output) {
  new(output) QVector< Qt3DInput::QAxis* >(this_ptr->axes());
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_delete(Qt3DInput::QLogicalDevice* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QLogicalDevice_metaObject(const Qt3DInput::QLogicalDevice* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QLogicalDevice* qt_3d_input_c_Qt3DInput_QLogicalDevice_new_no_args() {
  return new Qt3DInput::QLogicalDevice();
}

Qt3DInput::QLogicalDevice* qt_3d_input_c_Qt3DInput_QLogicalDevice_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QLogicalDevice(parent);
}

int qt_3d_input_c_Qt3DInput_QLogicalDevice_qt_metacall(Qt3DInput::QLogicalDevice* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QLogicalDevice_qt_metacast(Qt3DInput::QLogicalDevice* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_removeAction(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAction* action) {
  this_ptr->removeAction(action);
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_removeAxis(Qt3DInput::QLogicalDevice* this_ptr, Qt3DInput::QAxis* axis) {
  this_ptr->removeAxis(axis);
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QLogicalDevice::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QLogicalDevice_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QLogicalDevice::tr(s, c, n));
}

