#include "qt_widgets_c_QTreeWidgetItemIterator.h"

void qt_widgets_c_QTreeWidgetItemIterator_constructor_it(const QTreeWidgetItemIterator* it, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(*it);
}

void qt_widgets_c_QTreeWidgetItemIterator_constructor_item(QTreeWidgetItem* item, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(item);
}

void qt_widgets_c_QTreeWidgetItemIterator_constructor_item_flags(QTreeWidgetItem* item, unsigned int flags, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(item, QFlags< QTreeWidgetItemIterator::IteratorFlag >(flags));
}

void qt_widgets_c_QTreeWidgetItemIterator_constructor_widget(QTreeWidget* widget, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(widget);
}

void qt_widgets_c_QTreeWidgetItemIterator_constructor_widget_flags(QTreeWidget* widget, unsigned int flags, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(widget, QFlags< QTreeWidgetItemIterator::IteratorFlag >(flags));
}

void qt_widgets_c_QTreeWidgetItemIterator_destructor(QTreeWidgetItemIterator* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

QTreeWidgetItemIterator* qt_widgets_c_QTreeWidgetItemIterator_operator_add_assign(QTreeWidgetItemIterator* this_ptr, int n) {
  return &this_ptr->operator+=(n);
}

QTreeWidgetItemIterator* qt_widgets_c_QTreeWidgetItemIterator_operator_assign(QTreeWidgetItemIterator* this_ptr, const QTreeWidgetItemIterator* it) {
  return &this_ptr->operator=(*it);
}

QTreeWidgetItemIterator* qt_widgets_c_QTreeWidgetItemIterator_operator_dec(QTreeWidgetItemIterator* this_ptr) {
  return &this_ptr->operator--();
}

void qt_widgets_c_QTreeWidgetItemIterator_operator_dec_postfix_to_output(QTreeWidgetItemIterator* this_ptr, int arg1, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(this_ptr->operator--(arg1));
}

QTreeWidgetItemIterator* qt_widgets_c_QTreeWidgetItemIterator_operator_inc(QTreeWidgetItemIterator* this_ptr) {
  return &this_ptr->operator++();
}

void qt_widgets_c_QTreeWidgetItemIterator_operator_inc_postfix_to_output(QTreeWidgetItemIterator* this_ptr, int arg1, QTreeWidgetItemIterator* output) {
  new(output) QTreeWidgetItemIterator(this_ptr->operator++(arg1));
}

QTreeWidgetItem* qt_widgets_c_QTreeWidgetItemIterator_operator_indirection(const QTreeWidgetItemIterator* this_ptr) {
  return this_ptr->operator*();
}

QTreeWidgetItemIterator* qt_widgets_c_QTreeWidgetItemIterator_operator_sub_assign(QTreeWidgetItemIterator* this_ptr, int n) {
  return &this_ptr->operator-=(n);
}

