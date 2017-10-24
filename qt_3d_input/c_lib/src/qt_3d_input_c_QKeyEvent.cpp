#include "qt_3d_input_c_QKeyEvent.h"

QObject* qt_3d_input_c_QKeyEvent_G_static_cast_QObject_ptr(Qt3DInput::QKeyEvent* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DInput::QKeyEvent* qt_3d_input_c_QKeyEvent_G_static_cast_Qt3DInput_QKeyEvent_ptr(QObject* ptr) {
  return static_cast<Qt3DInput::QKeyEvent*>(ptr);
}

int qt_3d_input_c_Qt3DInput_QKeyEvent_count(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->count();
}

void qt_3d_input_c_Qt3DInput_QKeyEvent_delete(Qt3DInput::QKeyEvent* this_ptr) {
  delete this_ptr;
}

bool qt_3d_input_c_Qt3DInput_QKeyEvent_isAccepted(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->isAccepted();
}

bool qt_3d_input_c_Qt3DInput_QKeyEvent_isAutoRepeat(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->isAutoRepeat();
}

int qt_3d_input_c_Qt3DInput_QKeyEvent_key(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->key();
}

bool qt_3d_input_c_Qt3DInput_QKeyEvent_matches(const Qt3DInput::QKeyEvent* this_ptr, const QKeySequence::StandardKey* key_) {
  return this_ptr->matches(*key_);
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QKeyEvent_metaObject(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_input_c_Qt3DInput_QKeyEvent_modifiers(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->modifiers();
}

quint32 qt_3d_input_c_Qt3DInput_QKeyEvent_nativeScanCode(const Qt3DInput::QKeyEvent* this_ptr) {
  return this_ptr->nativeScanCode();
}

Qt3DInput::QKeyEvent* qt_3d_input_c_Qt3DInput_QKeyEvent_new(const QKeyEvent* ke) {
  return new Qt3DInput::QKeyEvent(*ke);
}

int qt_3d_input_c_Qt3DInput_QKeyEvent_qt_metacall(Qt3DInput::QKeyEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QKeyEvent_qt_metacast(Qt3DInput::QKeyEvent* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QKeyEvent_setAccepted(Qt3DInput::QKeyEvent* this_ptr, bool accepted) {
  this_ptr->setAccepted(accepted);
}

void qt_3d_input_c_Qt3DInput_QKeyEvent_text_to_output(const Qt3DInput::QKeyEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_3d_input_c_Qt3DInput_QKeyEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QKeyEvent::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QKeyEvent_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QKeyEvent::tr(s, c, n));
}

