#include "qt_widgets_c_QGraphicsGridLayout.h"

QGraphicsGridLayout* qt_widgets_c_QGraphicsGridLayout_G_dynamic_cast_QGraphicsGridLayout_ptr_QGraphicsLayout(QGraphicsLayout* ptr) {
  return dynamic_cast<QGraphicsGridLayout*>(ptr);
}

QGraphicsGridLayout* qt_widgets_c_QGraphicsGridLayout_G_dynamic_cast_QGraphicsGridLayout_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return dynamic_cast<QGraphicsGridLayout*>(ptr);
}

QGraphicsGridLayout* qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsGridLayout_ptr_QGraphicsLayout(QGraphicsLayout* ptr) {
  return static_cast<QGraphicsGridLayout*>(ptr);
}

QGraphicsGridLayout* qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsGridLayout_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return static_cast<QGraphicsGridLayout*>(ptr);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsGridLayout* ptr) {
  return static_cast<QGraphicsLayoutItem*>(ptr);
}

QGraphicsLayout* qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayout_ptr(QGraphicsGridLayout* ptr) {
  return static_cast<QGraphicsLayout*>(ptr);
}

int qt_widgets_c_QGraphicsGridLayout_columnCount(const QGraphicsGridLayout* this_ptr) {
  return this_ptr->columnCount();
}

double qt_widgets_c_QGraphicsGridLayout_columnMaximumWidth(const QGraphicsGridLayout* this_ptr, int column) {
  return this_ptr->columnMaximumWidth(column);
}

double qt_widgets_c_QGraphicsGridLayout_columnMinimumWidth(const QGraphicsGridLayout* this_ptr, int column) {
  return this_ptr->columnMinimumWidth(column);
}

double qt_widgets_c_QGraphicsGridLayout_columnPreferredWidth(const QGraphicsGridLayout* this_ptr, int column) {
  return this_ptr->columnPreferredWidth(column);
}

double qt_widgets_c_QGraphicsGridLayout_columnSpacing(const QGraphicsGridLayout* this_ptr, int column) {
  return this_ptr->columnSpacing(column);
}

int qt_widgets_c_QGraphicsGridLayout_columnStretchFactor(const QGraphicsGridLayout* this_ptr, int column) {
  return this_ptr->columnStretchFactor(column);
}

int qt_widgets_c_QGraphicsGridLayout_count(const QGraphicsGridLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QGraphicsGridLayout_delete(QGraphicsGridLayout* this_ptr) {
  delete this_ptr;
}

double qt_widgets_c_QGraphicsGridLayout_horizontalSpacing(const QGraphicsGridLayout* this_ptr) {
  return this_ptr->horizontalSpacing();
}

void qt_widgets_c_QGraphicsGridLayout_invalidate(QGraphicsGridLayout* this_ptr) {
  this_ptr->invalidate();
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsGridLayout_itemAt_index(const QGraphicsGridLayout* this_ptr, int index) {
  return this_ptr->itemAt(index);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsGridLayout_itemAt_row_column(const QGraphicsGridLayout* this_ptr, int row, int column) {
  return this_ptr->itemAt(row, column);
}

QGraphicsGridLayout* qt_widgets_c_QGraphicsGridLayout_new_no_args() {
  return new QGraphicsGridLayout();
}

QGraphicsGridLayout* qt_widgets_c_QGraphicsGridLayout_new_parent(QGraphicsLayoutItem* parent) {
  return new QGraphicsGridLayout(parent);
}

void qt_widgets_c_QGraphicsGridLayout_removeAt(QGraphicsGridLayout* this_ptr, int index) {
  this_ptr->removeAt(index);
}

void qt_widgets_c_QGraphicsGridLayout_removeItem(QGraphicsGridLayout* this_ptr, QGraphicsLayoutItem* item) {
  this_ptr->removeItem(item);
}

int qt_widgets_c_QGraphicsGridLayout_rowCount(const QGraphicsGridLayout* this_ptr) {
  return this_ptr->rowCount();
}

double qt_widgets_c_QGraphicsGridLayout_rowMaximumHeight(const QGraphicsGridLayout* this_ptr, int row) {
  return this_ptr->rowMaximumHeight(row);
}

double qt_widgets_c_QGraphicsGridLayout_rowMinimumHeight(const QGraphicsGridLayout* this_ptr, int row) {
  return this_ptr->rowMinimumHeight(row);
}

double qt_widgets_c_QGraphicsGridLayout_rowPreferredHeight(const QGraphicsGridLayout* this_ptr, int row) {
  return this_ptr->rowPreferredHeight(row);
}

double qt_widgets_c_QGraphicsGridLayout_rowSpacing(const QGraphicsGridLayout* this_ptr, int row) {
  return this_ptr->rowSpacing(row);
}

int qt_widgets_c_QGraphicsGridLayout_rowStretchFactor(const QGraphicsGridLayout* this_ptr, int row) {
  return this_ptr->rowStretchFactor(row);
}

void qt_widgets_c_QGraphicsGridLayout_setColumnFixedWidth(QGraphicsGridLayout* this_ptr, int column, double width) {
  this_ptr->setColumnFixedWidth(column, width);
}

void qt_widgets_c_QGraphicsGridLayout_setColumnMaximumWidth(QGraphicsGridLayout* this_ptr, int column, double width) {
  this_ptr->setColumnMaximumWidth(column, width);
}

void qt_widgets_c_QGraphicsGridLayout_setColumnMinimumWidth(QGraphicsGridLayout* this_ptr, int column, double width) {
  this_ptr->setColumnMinimumWidth(column, width);
}

void qt_widgets_c_QGraphicsGridLayout_setColumnPreferredWidth(QGraphicsGridLayout* this_ptr, int column, double width) {
  this_ptr->setColumnPreferredWidth(column, width);
}

void qt_widgets_c_QGraphicsGridLayout_setColumnSpacing(QGraphicsGridLayout* this_ptr, int column, double spacing) {
  this_ptr->setColumnSpacing(column, spacing);
}

void qt_widgets_c_QGraphicsGridLayout_setColumnStretchFactor(QGraphicsGridLayout* this_ptr, int column, int stretch) {
  this_ptr->setColumnStretchFactor(column, stretch);
}

void qt_widgets_c_QGraphicsGridLayout_setGeometry(QGraphicsGridLayout* this_ptr, const QRectF* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QGraphicsGridLayout_setHorizontalSpacing(QGraphicsGridLayout* this_ptr, double spacing) {
  this_ptr->setHorizontalSpacing(spacing);
}

void qt_widgets_c_QGraphicsGridLayout_setRowFixedHeight(QGraphicsGridLayout* this_ptr, int row, double height) {
  this_ptr->setRowFixedHeight(row, height);
}

void qt_widgets_c_QGraphicsGridLayout_setRowMaximumHeight(QGraphicsGridLayout* this_ptr, int row, double height) {
  this_ptr->setRowMaximumHeight(row, height);
}

void qt_widgets_c_QGraphicsGridLayout_setRowMinimumHeight(QGraphicsGridLayout* this_ptr, int row, double height) {
  this_ptr->setRowMinimumHeight(row, height);
}

void qt_widgets_c_QGraphicsGridLayout_setRowPreferredHeight(QGraphicsGridLayout* this_ptr, int row, double height) {
  this_ptr->setRowPreferredHeight(row, height);
}

void qt_widgets_c_QGraphicsGridLayout_setRowSpacing(QGraphicsGridLayout* this_ptr, int row, double spacing) {
  this_ptr->setRowSpacing(row, spacing);
}

void qt_widgets_c_QGraphicsGridLayout_setRowStretchFactor(QGraphicsGridLayout* this_ptr, int row, int stretch) {
  this_ptr->setRowStretchFactor(row, stretch);
}

void qt_widgets_c_QGraphicsGridLayout_setSpacing(QGraphicsGridLayout* this_ptr, double spacing) {
  this_ptr->setSpacing(spacing);
}

void qt_widgets_c_QGraphicsGridLayout_setVerticalSpacing(QGraphicsGridLayout* this_ptr, double spacing) {
  this_ptr->setVerticalSpacing(spacing);
}

void qt_widgets_c_QGraphicsGridLayout_sizeHint_to_output_which(const QGraphicsGridLayout* this_ptr, const Qt::SizeHint* which, QSizeF* output) {
  new(output) QSizeF(this_ptr->sizeHint(*which));
}

void qt_widgets_c_QGraphicsGridLayout_sizeHint_to_output_which_constraint(const QGraphicsGridLayout* this_ptr, const Qt::SizeHint* which, const QSizeF* constraint, QSizeF* output) {
  new(output) QSizeF(this_ptr->sizeHint(*which, *constraint));
}

double qt_widgets_c_QGraphicsGridLayout_verticalSpacing(const QGraphicsGridLayout* this_ptr) {
  return this_ptr->verticalSpacing();
}

