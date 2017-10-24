#include "qt_gui_c_QInputMethodQueryEvent.h"

QEvent* qt_gui_c_QInputMethodQueryEvent_G_static_cast_QEvent_ptr(QInputMethodQueryEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputMethodQueryEvent* qt_gui_c_QInputMethodQueryEvent_G_static_cast_QInputMethodQueryEvent_ptr(QEvent* ptr) {
  return static_cast<QInputMethodQueryEvent*>(ptr);
}

void qt_gui_c_QInputMethodQueryEvent_delete(QInputMethodQueryEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QInputMethodQueryEvent_setValue(QInputMethodQueryEvent* this_ptr, const Qt::InputMethodQuery* query, const QVariant* value) {
  this_ptr->setValue(*query, *value);
}

void qt_gui_c_QInputMethodQueryEvent_value_to_output(const QInputMethodQueryEvent* this_ptr, const Qt::InputMethodQuery* query, QVariant* output) {
  new(output) QVariant(this_ptr->value(*query));
}

