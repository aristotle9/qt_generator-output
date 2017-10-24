#include "qt_gui_c_QFileOpenEvent.h"

QEvent* qt_gui_c_QFileOpenEvent_G_static_cast_QEvent_ptr(QFileOpenEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QFileOpenEvent* qt_gui_c_QFileOpenEvent_G_static_cast_QFileOpenEvent_ptr(QEvent* ptr) {
  return static_cast<QFileOpenEvent*>(ptr);
}

void qt_gui_c_QFileOpenEvent_delete(QFileOpenEvent* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QFileOpenEvent_file_to_output(const QFileOpenEvent* this_ptr, QString* output) {
  new(output) QString(this_ptr->file());
}

QFileOpenEvent* qt_gui_c_QFileOpenEvent_new_file(const QString* file) {
  return new QFileOpenEvent(*file);
}

QFileOpenEvent* qt_gui_c_QFileOpenEvent_new_url(const QUrl* url) {
  return new QFileOpenEvent(*url);
}

void qt_gui_c_QFileOpenEvent_url_to_output(const QFileOpenEvent* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->url());
}

