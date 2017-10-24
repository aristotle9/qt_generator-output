#include "qt_3d_input_c_QAction.h"

QObject* qt_3d_input_c_QAction_G_static_cast_QObject_ptr(Qt3DInput::QAction* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAction_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAction* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAction* qt_3d_input_c_QAction_G_static_cast_Qt3DInput_QAction_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAction*>(ptr);
}

Qt3DInput::QAction* qt_3d_input_c_QAction_G_static_cast_Qt3DInput_QAction_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAction*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QAction_addInput(Qt3DInput::QAction* this_ptr, Qt3DInput::QAbstractActionInput* input) {
  this_ptr->addInput(input);
}

void qt_3d_input_c_Qt3DInput_QAction_delete(Qt3DInput::QAction* this_ptr) {
  delete this_ptr;
}

void qt_3d_input_c_Qt3DInput_QAction_inputs_to_output(const Qt3DInput::QAction* this_ptr, QVector< Qt3DInput::QAbstractActionInput* >* output) {
  new(output) QVector< Qt3DInput::QAbstractActionInput* >(this_ptr->inputs());
}

bool qt_3d_input_c_Qt3DInput_QAction_isActive(const Qt3DInput::QAction* this_ptr) {
  return this_ptr->isActive();
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAction_metaObject(const Qt3DInput::QAction* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QAction* qt_3d_input_c_Qt3DInput_QAction_new_no_args() {
  return new Qt3DInput::QAction();
}

Qt3DInput::QAction* qt_3d_input_c_Qt3DInput_QAction_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QAction(parent);
}

int qt_3d_input_c_Qt3DInput_QAction_qt_metacall(Qt3DInput::QAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAction_qt_metacast(Qt3DInput::QAction* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QAction_removeInput(Qt3DInput::QAction* this_ptr, Qt3DInput::QAbstractActionInput* input) {
  this_ptr->removeInput(input);
}

void qt_3d_input_c_Qt3DInput_QAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAction::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAction_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAction::tr(s, c, n));
}

