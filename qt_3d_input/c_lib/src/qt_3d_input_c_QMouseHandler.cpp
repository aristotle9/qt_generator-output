#include "qt_3d_input_c_QMouseHandler.h"

QObject* qt_3d_input_c_QMouseHandler_G_static_cast_QObject_ptr(Qt3DInput::QMouseHandler* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QMouseHandler* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QMouseHandler* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QMouseHandler* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QMouseHandler*>(ptr);
}

Qt3DInput::QMouseHandler* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DInput::QMouseHandler*>(ptr);
}

Qt3DInput::QMouseHandler* qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QMouseHandler*>(ptr);
}

bool qt_3d_input_c_Qt3DInput_QMouseHandler_containsMouse(const Qt3DInput::QMouseHandler* this_ptr) {
  return this_ptr->containsMouse();
}

void qt_3d_input_c_Qt3DInput_QMouseHandler_delete(Qt3DInput::QMouseHandler* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QMouseHandler_metaObject(const Qt3DInput::QMouseHandler* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QMouseHandler* qt_3d_input_c_Qt3DInput_QMouseHandler_new_no_args() {
  return new Qt3DInput::QMouseHandler();
}

Qt3DInput::QMouseHandler* qt_3d_input_c_Qt3DInput_QMouseHandler_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QMouseHandler(parent);
}

int qt_3d_input_c_Qt3DInput_QMouseHandler_qt_metacall(Qt3DInput::QMouseHandler* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QMouseHandler_qt_metacast(Qt3DInput::QMouseHandler* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QMouseHandler_setSourceDevice(Qt3DInput::QMouseHandler* this_ptr, Qt3DInput::QMouseDevice* mouseDevice) {
  this_ptr->setSourceDevice(mouseDevice);
}

Qt3DInput::QMouseDevice* qt_3d_input_c_Qt3DInput_QMouseHandler_sourceDevice(const Qt3DInput::QMouseHandler* this_ptr) {
  return this_ptr->sourceDevice();
}

void qt_3d_input_c_Qt3DInput_QMouseHandler_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QMouseHandler::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QMouseHandler_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QMouseHandler::tr(s, c, n));
}

