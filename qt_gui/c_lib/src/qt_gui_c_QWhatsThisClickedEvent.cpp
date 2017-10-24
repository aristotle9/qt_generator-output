#include "qt_gui_c_QWhatsThisClickedEvent.h"

QEvent* qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QEvent_ptr(QWhatsThisClickedEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QWhatsThisClickedEvent* qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QWhatsThisClickedEvent_ptr(QEvent* ptr) {
  return static_cast<QWhatsThisClickedEvent*>(ptr);
}

void qt_gui_c_QWhatsThisClickedEvent_delete(QWhatsThisClickedEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QWhatsThisClickedEvent_href_to_output(const QWhatsThisClickedEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->href());
}

QWhatsThisClickedEvent* qt_gui_c_QWhatsThisClickedEvent_new(const QString* href) {
  return new QWhatsThisClickedEvent(*href);
}

