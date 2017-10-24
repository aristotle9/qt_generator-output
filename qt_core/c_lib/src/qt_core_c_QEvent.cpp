#include "qt_core_c_QEvent.h"

void qt_core_c_QEvent_accept(QEvent* this_ptr) {
  this_ptr->accept();
}

void qt_core_c_QEvent_delete(QEvent* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QEvent_ignore(QEvent* this_ptr) {
  this_ptr->ignore();
}

bool qt_core_c_QEvent_isAccepted(const QEvent* this_ptr) {
  return this_ptr->isAccepted();
}

QEvent* qt_core_c_QEvent_new_other(const QEvent* other) {
  return new QEvent(*other);
}

QEvent* qt_core_c_QEvent_new_type(QEvent::Type type) {
  return new QEvent(type);
}

QEvent* qt_core_c_QEvent_operator_assign(QEvent* this_ptr, const QEvent* other) {
  return &this_ptr->operator=(*other);
}

int qt_core_c_QEvent_registerEventType_hint(int hint) {
  return QEvent::registerEventType(hint);
}

int qt_core_c_QEvent_registerEventType_no_args() {
  return QEvent::registerEventType();
}

void qt_core_c_QEvent_setAccepted(QEvent* this_ptr, bool accepted) {
  this_ptr->setAccepted(accepted);
}

bool qt_core_c_QEvent_spontaneous(const QEvent* this_ptr) {
  return this_ptr->spontaneous();
}

QEvent::Type qt_core_c_QEvent_type(const QEvent* this_ptr) {
  return this_ptr->type();
}

