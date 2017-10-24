#include "qt_widgets_c_QPair.h"

void qt_widgets_c_QPair_double_QPointF_constructor_no_args(QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >();
}

void qt_widgets_c_QPair_double_QPointF_constructor_t1_t2(const double* t1, const QPointF* t2, QPair< double, QPointF >* output) {
  new(output) QPair< double, QPointF >(*t1, *t2);
}

void qt_widgets_c_QPair_double_QPointF_destructor(QPair< double, QPointF >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

void qt_widgets_c_QPair_double_QPointF_swap(QPair< double, QPointF >* this_ptr, QPair< double, QPointF >* other) {
  this_ptr->swap(*other);
}

void qt_widgets_c_QPair_double_double_constructor_no_args(QPair< double, double >* output) {
  new(output) QPair< double, double >();
}

void qt_widgets_c_QPair_double_double_constructor_t1_t2(const double* t1, const double* t2, QPair< double, double >* output) {
  new(output) QPair< double, double >(*t1, *t2);
}

void qt_widgets_c_QPair_double_double_destructor(QPair< double, double >* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

void qt_widgets_c_QPair_double_double_swap(QPair< double, double >* this_ptr, QPair< double, double >* other) {
  this_ptr->swap(*other);
}

