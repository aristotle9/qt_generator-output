#include "qt_3d_input_c_QMouseEvent.h"

QObject* qt_3d_input_c_QMouseEvent_G_static_cast_QObject_ptr(Qt3DInput::QMouseEvent* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DInput::QMouseEvent* qt_3d_input_c_QMouseEvent_G_static_cast_Qt3DInput_QMouseEvent_ptr(QObject* ptr) {
  return static_cast<Qt3DInput::QMouseEvent*>(ptr);
}

Qt3DInput::QMouseEvent::Buttons qt_3d_input_c_Qt3DInput_QMouseEvent_button(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->button();
}

int qt_3d_input_c_Qt3DInput_QMouseEvent_buttons(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->buttons();
}

void qt_3d_input_c_Qt3DInput_QMouseEvent_delete(Qt3DInput::QMouseEvent* this_ptr) {
  delete this_ptr;
}

bool qt_3d_input_c_Qt3DInput_QMouseEvent_isAccepted(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->isAccepted();
}

const QMetaObject* qt_3d_input_c_Qt3DInput_QMouseEvent_metaObject(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DInput::QMouseEvent::Modifiers qt_3d_input_c_Qt3DInput_QMouseEvent_modifiers(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->modifiers();
}

Qt3DInput::QMouseEvent* qt_3d_input_c_Qt3DInput_QMouseEvent_new(const QMouseEvent* e) {
  return new Qt3DInput::QMouseEvent(*e);
}

int qt_3d_input_c_Qt3DInput_QMouseEvent_qt_metacall(Qt3DInput::QMouseEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_input_c_Qt3DInput_QMouseEvent_qt_metacast(Qt3DInput::QMouseEvent* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_input_c_Qt3DInput_QMouseEvent_setAccepted(Qt3DInput::QMouseEvent* this_ptr, bool accepted) {
  this_ptr->setAccepted(accepted);
}

void qt_3d_input_c_Qt3DInput_QMouseEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QMouseEvent::trUtf8(s, c, n));
}

void qt_3d_input_c_Qt3DInput_QMouseEvent_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DInput::QMouseEvent::tr(s, c, n));
}

bool qt_3d_input_c_Qt3DInput_QMouseEvent_wasHeld(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->wasHeld();
}

int qt_3d_input_c_Qt3DInput_QMouseEvent_x(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->x();
}

int qt_3d_input_c_Qt3DInput_QMouseEvent_y(const Qt3DInput::QMouseEvent* this_ptr) {
  return this_ptr->y();
}

