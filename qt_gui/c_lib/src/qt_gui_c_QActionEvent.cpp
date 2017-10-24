#include "qt_gui_c_QActionEvent.h"

bool qt_gui_c_QActionEvent_G_operator_eq_e_key(QKeyEvent* e, const QKeySequence::StandardKey* key) {
  return operator==(e, *key);
}

bool qt_gui_c_QActionEvent_G_operator_eq_key_e(const QKeySequence::StandardKey* key, QKeyEvent* e) {
  return operator==(*key, e);
}

bool qt_gui_c_QActionEvent_G_operator_eq_lhs_rhs(const QPointingDeviceUniqueId* lhs, const QPointingDeviceUniqueId* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_gui_c_QActionEvent_G_operator_neq(const QPointingDeviceUniqueId* lhs, const QPointingDeviceUniqueId* rhs) {
  return operator!=(*lhs, *rhs);
}

void qt_gui_c_QActionEvent_G_operator_shl_to_output_QDebug_QEvent(const QDebug* arg1, const QEvent* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

void qt_gui_c_QActionEvent_G_operator_shl_to_output_QDebug_QTouchEvent_TouchPoint(const QDebug* arg1, const QTouchEvent::TouchPoint* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

unsigned int qt_gui_c_QActionEvent_G_qHash_key(const QPointingDeviceUniqueId* key) {
  return qHash(*key);
}

unsigned int qt_gui_c_QActionEvent_G_qHash_key_seed(const QPointingDeviceUniqueId* key, unsigned int seed) {
  return qHash(*key, seed);
}

QActionEvent* qt_gui_c_QActionEvent_G_static_cast_QActionEvent_ptr(QEvent* ptr) {
  return static_cast<QActionEvent*>(ptr);
}

QEvent* qt_gui_c_QActionEvent_G_static_cast_QEvent_ptr(QActionEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_gui_c_QActionEvent_delete(QActionEvent* this_ptr) {
  delete this_ptr;
}

