#include "qt_widgets_c_QStyleHintReturn.h"

void qt_widgets_c_QStyleHintReturn_delete(QStyleHintReturn* this_ptr) {
  delete this_ptr;
}

QStyleHintReturn* qt_widgets_c_QStyleHintReturn_new_no_args() {
  return new QStyleHintReturn();
}

QStyleHintReturn* qt_widgets_c_QStyleHintReturn_new_version(int version) {
  return new QStyleHintReturn(version);
}

QStyleHintReturn* qt_widgets_c_QStyleHintReturn_new_version_type(int version, int type) {
  return new QStyleHintReturn(version, type);
}

void qt_widgets_c_QStyleHintReturn_set_type(QStyleHintReturn* this_ptr, int value) {
  this_ptr->type = value;
}

void qt_widgets_c_QStyleHintReturn_set_version(QStyleHintReturn* this_ptr, int value) {
  this_ptr->version = value;
}

int qt_widgets_c_QStyleHintReturn_type(const QStyleHintReturn* this_ptr) {
  return this_ptr->type;
}

int qt_widgets_c_QStyleHintReturn_version(const QStyleHintReturn* this_ptr) {
  return this_ptr->version;
}

