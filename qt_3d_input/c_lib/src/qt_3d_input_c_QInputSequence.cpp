#include "qt_3d_input_c_QInputSequence.h"

Qt3DInput::QInputSequence* qt_3d_input_c_QInputSequence_G_dynamic_cast_Qt3DInput_QInputSequence_ptr(Qt3DInput::QAbstractActionInput* ptr) {
  return dynamic_cast<Qt3DInput::QInputSequence*>(ptr);
}

QObject* qt_3d_input_c_QInputSequence_G_static_cast_QObject_ptr(Qt3DInput::QInputSequence* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QInputSequence_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QInputSequence* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractActionInput* qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(Qt3DInput::QInputSequence* ptr) {
  return static_cast<Qt3DInput::QAbstractActionInput*>(ptr);
}

Qt3DInput::QInputSequence* qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QInputSequence*>(ptr);
}

Qt3DInput::QInputSequence* qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QInputSequence*>(ptr);
}

Qt3DInput::QInputSequence* qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_Qt3DInput_QAbstractActionInput(Qt3DInput::QAbstractActionInput* ptr) {
  return static_cast<Qt3DInput::QInputSequence*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QInputSequence_addSequence(Qt3DInput::QInputSequence* this_ptr, Qt3DInput::QAbstractActionInput* input) {
  this_ptr->addSequence(input);
}

int qt_3d_input_c_Qt3DInput_QInputSequence_buttonInterval(const Qt3DInput::QInputSequence* this_ptr) {
  return this_ptr->buttonInterval();
}

void qt_3d_input_c_Qt3DInput_QInputSequence_delete(Qt3DInput::QInputSequence* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QInputSequence_metaObject(const Qt3DInput::QInputSequence* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QInputSequence* qt_3d_input_c_Qt3DInput_QInputSequence_new_no_args() {
  return new Qt3DInput::QInputSequence();
}

Qt3DInput::QInputSequence* qt_3d_input_c_Qt3DInput_QInputSequence_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QInputSequence(parent);
}

int qt_3d_input_c_Qt3DInput_QInputSequence_qt_metacall(Qt3DInput::QInputSequence* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QInputSequence_qt_metacast(Qt3DInput::QInputSequence* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QInputSequence_removeSequence(Qt3DInput::QInputSequence* this_ptr, Qt3DInput::QAbstractActionInput* input) {
  this_ptr->removeSequence(input);
}

void qt_3d_input_c_Qt3DInput_QInputSequence_sequences_to_output(const Qt3DInput::QInputSequence* this_ptr, QVector< Qt3DInput::QAbstractActionInput* >* output) {
  new(output) QVector< Qt3DInput::QAbstractActionInput* >(this_ptr->sequences());
}

void qt_3d_input_c_Qt3DInput_QInputSequence_setButtonInterval(Qt3DInput::QInputSequence* this_ptr, int buttonInterval) {
  this_ptr->setButtonInterval(buttonInterval);
}

void qt_3d_input_c_Qt3DInput_QInputSequence_setTimeout(Qt3DInput::QInputSequence* this_ptr, int timeout) {
  this_ptr->setTimeout(timeout);
}

int qt_3d_input_c_Qt3DInput_QInputSequence_timeout(const Qt3DInput::QInputSequence* this_ptr) {
  return this_ptr->timeout();
}

void qt_3d_input_c_Qt3DInput_QInputSequence_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputSequence::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QInputSequence_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputSequence::tr(s, c, n));
}

