#include "qt_3d_input_c_QInputChord.h"

Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_dynamic_cast_Qt3DInput_QInputChord_ptr(Qt3DInput::QAbstractActionInput* ptr) {
  return dynamic_cast<Qt3DInput::QInputChord*>(ptr);
}

QObject* qt_3d_input_c_QInputChord_G_static_cast_QObject_ptr(Qt3DInput::QInputChord* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_input_c_QInputChord_G_static_cast_Qt3DCore_QNode_ptr(Qt3DInput::QInputChord* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DInput::QAbstractActionInput* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(Qt3DInput::QInputChord* ptr) {
  return static_cast<Qt3DInput::QAbstractActionInput*>(ptr);
}

Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QInputChord_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DInput::QInputChord*>(ptr);
}

Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QInputChord_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DInput::QInputChord*>(ptr);
}

Qt3DInput::QInputChord* qt_3d_input_c_QInputChord_G_static_cast_Qt3DInput_QInputChord_ptr_Qt3DInput_QAbstractActionInput(Qt3DInput::QAbstractActionInput* ptr) {
  return static_cast<Qt3DInput::QInputChord*>(ptr);
}

void qt_3d_input_c_Qt3DInput_QInputChord_addChord(Qt3DInput::QInputChord* this_ptr, Qt3DInput::QAbstractActionInput* input) {
  this_ptr->addChord(input);
}

void qt_3d_input_c_Qt3DInput_QInputChord_chords_to_output(const Qt3DInput::QInputChord* this_ptr, QVector< Qt3DInput::QAbstractActionInput* >* output) {
  new(output) QVector< Qt3DInput::QAbstractActionInput* >(this_ptr->chords());
}

void qt_3d_input_c_Qt3DInput_QInputChord_delete(Qt3DInput::QInputChord* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QInputChord_metaObject(const Qt3DInput::QInputChord* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QInputChord* qt_3d_input_c_Qt3DInput_QInputChord_new_no_args() {
  return new Qt3DInput::QInputChord();
}

Qt3DInput::QInputChord* qt_3d_input_c_Qt3DInput_QInputChord_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DInput::QInputChord(parent);
}

int qt_3d_input_c_Qt3DInput_QInputChord_qt_metacall(Qt3DInput::QInputChord* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QInputChord_qt_metacast(Qt3DInput::QInputChord* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QInputChord_removeChord(Qt3DInput::QInputChord* this_ptr, Qt3DInput::QAbstractActionInput* input) {
  this_ptr->removeChord(input);
}

void qt_3d_input_c_Qt3DInput_QInputChord_setTimeout(Qt3DInput::QInputChord* this_ptr, int timeout) {
  this_ptr->setTimeout(timeout);
}

int qt_3d_input_c_Qt3DInput_QInputChord_timeout(const Qt3DInput::QInputChord* this_ptr) {
  return this_ptr->timeout();
}

void qt_3d_input_c_Qt3DInput_QInputChord_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputChord::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QInputChord_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QInputChord::tr(s, c, n));
}

