#include "qt_3d_input_c_QKeyboardHandler.h"

QObject* qt_3d_input_c_QKeyboardHandler_G_static_cast_QObject_ptr(Qt3DInput::QKeyboardHandler* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QKeyboardHandler* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QKeyboardHandler* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QKeyboardHandler* qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QKeyboardHandler*>(ptr);
}

Qt3DInput::QKeyboardHandler* qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DInput::QKeyboardHandler*>(ptr);
}

Qt3DInput::QKeyboardHandler* qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QKeyboardHandler*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QKeyboardHandler_delete(Qt3DInput::QKeyboardHandler* this_ptr) {
  delete this_ptr;
}

bool qt_3d_input_c_Qt3DInput_QKeyboardHandler_focus(const Qt3DInput::QKeyboardHandler* this_ptr) {
  return this_ptr->focus();
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QKeyboardHandler_metaObject(const Qt3DInput::QKeyboardHandler* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QKeyboardHandler* qt_3d_input_c_Qt3DInput_QKeyboardHandler_new_no_args() {
  return new Qt3DInput::QKeyboardHandler();
}

Qt3DInput::QKeyboardHandler* qt_3d_input_c_Qt3DInput_QKeyboardHandler_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QKeyboardHandler(parent);
}

int qt_3d_input_c_Qt3DInput_QKeyboardHandler_qt_metacall(Qt3DInput::QKeyboardHandler* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QKeyboardHandler_qt_metacast(Qt3DInput::QKeyboardHandler* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QKeyboardHandler_setFocus(Qt3DInput::QKeyboardHandler* this_ptr, bool focus) {
  this_ptr->setFocus(focus);
}

void qt_3d_input_c_Qt3DInput_QKeyboardHandler_setSourceDevice(Qt3DInput::QKeyboardHandler* this_ptr, Qt3DInput::QKeyboardDevice* keyboardDevice) {
  this_ptr->setSourceDevice(keyboardDevice);
}

Qt3DInput::QKeyboardDevice* qt_3d_input_c_Qt3DInput_QKeyboardHandler_sourceDevice(const Qt3DInput::QKeyboardHandler* this_ptr) {
  return this_ptr->sourceDevice();
}

void qt_3d_input_c_Qt3DInput_QKeyboardHandler_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QKeyboardHandler::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QKeyboardHandler_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QKeyboardHandler::tr(s, c, n));
}

