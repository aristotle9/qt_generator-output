#include "qt_gui_c_QStatusTipEvent.h"

QEvent* qt_gui_c_QStatusTipEvent_G_static_cast_QEvent_ptr(QStatusTipEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QStatusTipEvent* qt_gui_c_QStatusTipEvent_G_static_cast_QStatusTipEvent_ptr(QEvent* ptr) {
  return static_cast<QStatusTipEvent*>(ptr);
}

void qt_gui_c_QStatusTipEvent_delete(QStatusTipEvent* this_ptr) {
  delete this_ptr;
}

QStatusTipEvent* qt_gui_c_QStatusTipEvent_new(const QString* tip) {
  return new QStatusTipEvent(*tip);
}

void qt_gui_c_QStatusTipEvent_tip_to_output(const QStatusTipEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->tip());
}

