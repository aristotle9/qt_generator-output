#include "qt_core_c_QPair.h"

void qt_core_c_QPair_QString_QString_constructor_no_args(QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >();
}

void qt_core_c_QPair_QString_QString_constructor_t1_t2(const QString* t1, const QString* t2, QPair< QString, QString >* output) {
  new(output) QPair< QString, QString >(*t1, *t2);
}

void qt_core_c_QPair_QString_QString_destructor(QPair< QString, QString >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QPair_QString_QString_swap(QPair< QString, QString >* this_ptr, QPair< QString, QString >* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QPair_double_QVariant_constructor_no_args(QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >();
}

void qt_core_c_QPair_double_QVariant_constructor_t1_t2(const double* t1, const QVariant* t2, QPair< double, QVariant >* output) {
  new(output) QPair< double, QVariant >(*t1, *t2);
}

void qt_core_c_QPair_double_QVariant_destructor(QPair< double, QVariant >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QPair_double_QVariant_swap(QPair< double, QVariant >* this_ptr, QPair< double, QVariant >* other) {
  this_ptr->swap(*other);
}

