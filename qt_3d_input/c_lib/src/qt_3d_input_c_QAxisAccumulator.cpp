#include "qt_3d_input_c_QAxisAccumulator.h"

QObject* qt_3d_input_c_QAxisAccumulator_G_static_cast_QObject_ptr(Qt3DInput::QAxisAccumulator* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DInput::QAxisAccumulator* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QAxisAccumulator* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAxisAccumulator* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QAxisAccumulator*>(ptr);
}

Qt3DInput::QAxisAccumulator* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DInput::QAxisAccumulator*>(ptr);
}

Qt3DInput::QAxisAccumulator* qt_3d_input_c_QAxisAccumulator_G_static_cast_Qt3DInput_QAxisAccumulator_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QAxisAccumulator*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QAxisAccumulator_delete(Qt3DInput::QAxisAccumulator* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QAxisAccumulator_metaObject(const Qt3DInput::QAxisAccumulator* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QAxisAccumulator* qt_3d_input_c_Qt3DInput_QAxisAccumulator_new_no_args() {
  return new Qt3DInput::QAxisAccumulator();
}

Qt3DInput::QAxisAccumulator* qt_3d_input_c_Qt3DInput_QAxisAccumulator_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QAxisAccumulator(parent);
}

int qt_3d_input_c_Qt3DInput_QAxisAccumulator_qt_metacall(Qt3DInput::QAxisAccumulator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QAxisAccumulator_qt_metacast(Qt3DInput::QAxisAccumulator* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_input_c_Qt3DInput_QAxisAccumulator_scale(const Qt3DInput::QAxisAccumulator* this_ptr) {
  return this_ptr->scale();
}

void qt_3d_input_c_Qt3DInput_QAxisAccumulator_setScale(Qt3DInput::QAxisAccumulator* this_ptr, float scale) {
  this_ptr->setScale(scale);
}

void qt_3d_input_c_Qt3DInput_QAxisAccumulator_setSourceAxis(Qt3DInput::QAxisAccumulator* this_ptr, Qt3DInput::QAxis* sourceAxis) {
  this_ptr->setSourceAxis(sourceAxis);
}

void qt_3d_input_c_Qt3DInput_QAxisAccumulator_setSourceAxisType(Qt3DInput::QAxisAccumulator* this_ptr, const Qt3DInput::QAxisAccumulator::SourceAxisType* sourceAxisType) {
  this_ptr->setSourceAxisType(*sourceAxisType);
}

Qt3DInput::QAxis* qt_3d_input_c_Qt3DInput_QAxisAccumulator_sourceAxis(const Qt3DInput::QAxisAccumulator* this_ptr) {
  return this_ptr->sourceAxis();
}

Qt3DInput::QAxisAccumulator::SourceAxisType qt_3d_input_c_Qt3DInput_QAxisAccumulator_sourceAxisType(const Qt3DInput::QAxisAccumulator* this_ptr) {
  return this_ptr->sourceAxisType();
}

void qt_3d_input_c_Qt3DInput_QAxisAccumulator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAxisAccumulator::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QAxisAccumulator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QAxisAccumulator::tr(s, c, n));
}

float qt_3d_input_c_Qt3DInput_QAxisAccumulator_value(const Qt3DInput::QAxisAccumulator* this_ptr) {
  return this_ptr->value();
}

float qt_3d_input_c_Qt3DInput_QAxisAccumulator_velocity(const Qt3DInput::QAxisAccumulator* this_ptr) {
  return this_ptr->velocity();
}

