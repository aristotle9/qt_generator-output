#include "qt_widgets_c_QTableWidgetSelectionRange.h"

int qt_widgets_c_QTableWidgetSelectionRange_bottomRow(const QTableWidgetSelectionRange* this_ptr) {
  return this_ptr->bottomRow();
}

int qt_widgets_c_QTableWidgetSelectionRange_columnCount(const QTableWidgetSelectionRange* this_ptr) {
  return this_ptr->columnCount();
}

void qt_widgets_c_QTableWidgetSelectionRange_constructor_no_args(QTableWidgetSelectionRange* output) {
  new(output) QTableWidgetSelectionRange();
}

void qt_widgets_c_QTableWidgetSelectionRange_constructor_other(const QTableWidgetSelectionRange* other, QTableWidgetSelectionRange* output) {
  new(output) QTableWidgetSelectionRange(*other);
}

void qt_widgets_c_QTableWidgetSelectionRange_constructor_top_left_bottom_right(int top, int left, int bottom, int right, QTableWidgetSelectionRange* output) {
  new(output) QTableWidgetSelectionRange(top, left, bottom, right);
}

void qt_widgets_c_QTableWidgetSelectionRange_destructor(QTableWidgetSelectionRange* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

int qt_widgets_c_QTableWidgetSelectionRange_leftColumn(const QTableWidgetSelectionRange* this_ptr) {
  return this_ptr->leftColumn();
}

int qt_widgets_c_QTableWidgetSelectionRange_rightColumn(const QTableWidgetSelectionRange* this_ptr) {
  return this_ptr->rightColumn();
}

int qt_widgets_c_QTableWidgetSelectionRange_rowCount(const QTableWidgetSelectionRange* this_ptr) {
  return this_ptr->rowCount();
}

int qt_widgets_c_QTableWidgetSelectionRange_topRow(const QTableWidgetSelectionRange* this_ptr) {
  return this_ptr->topRow();
}

