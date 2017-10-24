#include "qt_widgets_c_QGridLayout.h"

QGridLayout* qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayout(QLayout* ptr) {
  return dynamic_cast<QGridLayout*>(ptr);
}

QGridLayout* qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return dynamic_cast<QGridLayout*>(ptr);
}

QGridLayout* qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayout(QLayout* ptr) {
  return static_cast<QGridLayout*>(ptr);
}

QGridLayout* qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QGridLayout*>(ptr);
}

QGridLayout* qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QGridLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QGridLayout_G_static_cast_QLayoutItem_ptr(QGridLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QGridLayout_G_static_cast_QLayout_ptr(QGridLayout* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QGridLayout_G_static_cast_QObject_ptr(QGridLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGridLayout_addWidget(QGridLayout* this_ptr, QWidget* w) {
  this_ptr->addWidget(w);
}

void qt_widgets_c_QGridLayout_cellRect_to_output(const QGridLayout* this_ptr, int row, int column, QRect* output) {
  new(output) QRect(this_ptr->cellRect(row, column));
}

int qt_widgets_c_QGridLayout_columnCount(const QGridLayout* this_ptr) {
  return this_ptr->columnCount();
}

int qt_widgets_c_QGridLayout_columnMinimumWidth(const QGridLayout* this_ptr, int column) {
  return this_ptr->columnMinimumWidth(column);
}

int qt_widgets_c_QGridLayout_columnStretch(const QGridLayout* this_ptr, int column) {
  return this_ptr->columnStretch(column);
}

int qt_widgets_c_QGridLayout_count(const QGridLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QGridLayout_delete(QGridLayout* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGridLayout_getItemPosition(const QGridLayout* this_ptr, int idx, int* row, int* column, int* rowSpan, int* columnSpan) {
  this_ptr->getItemPosition(idx, row, column, rowSpan, columnSpan);
}

bool qt_widgets_c_QGridLayout_hasHeightForWidth(const QGridLayout* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QGridLayout_heightForWidth(const QGridLayout* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

int qt_widgets_c_QGridLayout_horizontalSpacing(const QGridLayout* this_ptr) {
  return this_ptr->horizontalSpacing();
}

void qt_widgets_c_QGridLayout_invalidate(QGridLayout* this_ptr) {
  this_ptr->invalidate();
}

QLayoutItem* qt_widgets_c_QGridLayout_itemAt(const QGridLayout* this_ptr, int index) {
  return this_ptr->itemAt(index);
}

QLayoutItem* qt_widgets_c_QGridLayout_itemAtPosition(const QGridLayout* this_ptr, int row, int column) {
  return this_ptr->itemAtPosition(row, column);
}

void qt_widgets_c_QGridLayout_maximumSize_to_output(const QGridLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

const QMetaObject* qt_widgets_c_QGridLayout_metaObject(const QGridLayout* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QGridLayout_minimumHeightForWidth(const QGridLayout* this_ptr, int arg1) {
  return this_ptr->minimumHeightForWidth(arg1);
}

void qt_widgets_c_QGridLayout_minimumSize_to_output(const QGridLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QGridLayout* qt_widgets_c_QGridLayout_new_no_args() {
  return new QGridLayout();
}

QGridLayout* qt_widgets_c_QGridLayout_new_parent(QWidget* parent) {
  return new QGridLayout(parent);
}

int qt_widgets_c_QGridLayout_qt_metacall(QGridLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGridLayout_qt_metacast(QGridLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_widgets_c_QGridLayout_rowCount(const QGridLayout* this_ptr) {
  return this_ptr->rowCount();
}

int qt_widgets_c_QGridLayout_rowMinimumHeight(const QGridLayout* this_ptr, int row) {
  return this_ptr->rowMinimumHeight(row);
}

int qt_widgets_c_QGridLayout_rowStretch(const QGridLayout* this_ptr, int row) {
  return this_ptr->rowStretch(row);
}

void qt_widgets_c_QGridLayout_setColumnMinimumWidth(QGridLayout* this_ptr, int column, int minSize) {
  this_ptr->setColumnMinimumWidth(column, minSize);
}

void qt_widgets_c_QGridLayout_setColumnStretch(QGridLayout* this_ptr, int column, int stretch) {
  this_ptr->setColumnStretch(column, stretch);
}

void qt_widgets_c_QGridLayout_setDefaultPositioning(QGridLayout* this_ptr, int n, const Qt::Orientation* orient) {
  this_ptr->setDefaultPositioning(n, *orient);
}

void qt_widgets_c_QGridLayout_setGeometry(QGridLayout* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QGridLayout_setHorizontalSpacing(QGridLayout* this_ptr, int spacing) {
  this_ptr->setHorizontalSpacing(spacing);
}

void qt_widgets_c_QGridLayout_setOriginCorner(QGridLayout* this_ptr, const Qt::Corner* arg1) {
  this_ptr->setOriginCorner(*arg1);
}

void qt_widgets_c_QGridLayout_setRowMinimumHeight(QGridLayout* this_ptr, int row, int minSize) {
  this_ptr->setRowMinimumHeight(row, minSize);
}

void qt_widgets_c_QGridLayout_setRowStretch(QGridLayout* this_ptr, int row, int stretch) {
  this_ptr->setRowStretch(row, stretch);
}

void qt_widgets_c_QGridLayout_setSpacing(QGridLayout* this_ptr, int spacing) {
  this_ptr->setSpacing(spacing);
}

void qt_widgets_c_QGridLayout_setVerticalSpacing(QGridLayout* this_ptr, int spacing) {
  this_ptr->setVerticalSpacing(spacing);
}

void qt_widgets_c_QGridLayout_sizeHint_to_output(const QGridLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QGridLayout_spacing(const QGridLayout* this_ptr) {
  return this_ptr->spacing();
}

QLayoutItem* qt_widgets_c_QGridLayout_takeAt(QGridLayout* this_ptr, int index) {
  return this_ptr->takeAt(index);
}

void qt_widgets_c_QGridLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGridLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QGridLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGridLayout::tr(s, c, n));
}

int qt_widgets_c_QGridLayout_verticalSpacing(const QGridLayout* this_ptr) {
  return this_ptr->verticalSpacing();
}

