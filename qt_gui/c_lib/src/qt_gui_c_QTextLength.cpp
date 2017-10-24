#include "qt_gui_c_QTextLength.h"

void qt_gui_c_QTextLength_constructor_no_args(QTextLength* output) {
  new(output) QTextLength();
}

void qt_gui_c_QTextLength_constructor_type_value(QTextLength::Type type, double value, QTextLength* output) {
  new(output) QTextLength(type, value);
}

void qt_gui_c_QTextLength_convert_to_QVariant_to_output(const QTextLength* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QTextLength_destructor(QTextLength* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QTextLength_operator_eq(const QTextLength* this_ptr, const QTextLength* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QTextLength_operator_neq(const QTextLength* this_ptr, const QTextLength* other) {
  return this_ptr->operator!=(*other);
}

double qt_gui_c_QTextLength_rawValue(const QTextLength* this_ptr) {
  return this_ptr->rawValue();
}

QTextLength::Type qt_gui_c_QTextLength_type(const QTextLength* this_ptr) {
  return this_ptr->type();
}

double qt_gui_c_QTextLength_value(const QTextLength* this_ptr, double maximumLength) {
  return this_ptr->value(maximumLength);
}

