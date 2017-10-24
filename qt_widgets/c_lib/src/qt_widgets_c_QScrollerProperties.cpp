#include "qt_widgets_c_QScrollerProperties.h"

void qt_widgets_c_QScrollerProperties_delete(QScrollerProperties* this_ptr) {
  delete this_ptr;
}

QScrollerProperties* qt_widgets_c_QScrollerProperties_new_no_args() {
  return new QScrollerProperties();
}

QScrollerProperties* qt_widgets_c_QScrollerProperties_new_sp(const QScrollerProperties* sp) {
  return new QScrollerProperties(*sp);
}

QScrollerProperties* qt_widgets_c_QScrollerProperties_operator_assign(QScrollerProperties* this_ptr, const QScrollerProperties* sp) {
  return &this_ptr->operator=(*sp);
}

bool qt_widgets_c_QScrollerProperties_operator_eq(const QScrollerProperties* this_ptr, const QScrollerProperties* sp) {
  return this_ptr->operator==(*sp);
}

bool qt_widgets_c_QScrollerProperties_operator_neq(const QScrollerProperties* this_ptr, const QScrollerProperties* sp) {
  return this_ptr->operator!=(*sp);
}

void qt_widgets_c_QScrollerProperties_scrollMetric_to_output(const QScrollerProperties* this_ptr, QScrollerProperties::ScrollMetric metric, QVariant* output) {
  new(output) QVariant(this_ptr->scrollMetric(metric));
}

void qt_widgets_c_QScrollerProperties_setDefaultScrollerProperties(const QScrollerProperties* sp) {
  QScrollerProperties::setDefaultScrollerProperties(*sp);
}

void qt_widgets_c_QScrollerProperties_setScrollMetric(QScrollerProperties* this_ptr, QScrollerProperties::ScrollMetric metric, const QVariant* value) {
  this_ptr->setScrollMetric(metric, *value);
}

void qt_widgets_c_QScrollerProperties_unsetDefaultScrollerProperties() {
  QScrollerProperties::unsetDefaultScrollerProperties();
}

