#include "qt_3d_input_c_QAxis.h"

QObject* qt_3d_input_c_QAxis_G_static_cast_QObject_ptr(Qt3DInput::QAxis* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAxis_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAxis* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAxis* qt_3d_input_c_QAxis_G_static_cast_Qt3DInput_QAxis_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAxis*>(ptr);
}

Qt3DInput::QAxis* qt_3d_input_c_QAxis_G_static_cast_Qt3DInput_QAxis_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAxis*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QAxis_addInput(Qt3DInput::QAxis* this_ptr, Qt3DInput::QAbstractAxisInput* input) {
  this_ptr->addInput(input);
}

void qt_3d_input_c_Qt3DInput_QAxis_delete(Qt3DInput::QAxis* this_ptr) {
  delete this_ptr;
}

void qt_3d_input_c_Qt3DInput_QAxis_inputs_to_output(const Qt3DInput::QAxis* this_ptr, QVector< Qt3DInput::QAbstractAxisInput* >* output) {
  new(output) QVector< Qt3DInput::QAbstractAxisInput* >(this_ptr->inputs());
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAxis_metaObject(const Qt3DInput::QAxis* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QAxis* qt_3d_input_c_Qt3DInput_QAxis_new_no_args() {
  return new Qt3DInput::QAxis();
}

Qt3DInput::QAxis* qt_3d_input_c_Qt3DInput_QAxis_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QAxis(parent);
}

int qt_3d_input_c_Qt3DInput_QAxis_qt_metacall(Qt3DInput::QAxis* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAxis_qt_metacast(Qt3DInput::QAxis* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QAxis_removeInput(Qt3DInput::QAxis* this_ptr, Qt3DInput::QAbstractAxisInput* input) {
  this_ptr->removeInput(input);
}

void qt_3d_input_c_Qt3DInput_QAxis_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAxis::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAxis_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAxis::tr(s, c, n));
}

float qt_3d_input_c_Qt3DInput_QAxis_value(const Qt3DInput::QAxis* this_ptr) {
  return this_ptr->value();
}

