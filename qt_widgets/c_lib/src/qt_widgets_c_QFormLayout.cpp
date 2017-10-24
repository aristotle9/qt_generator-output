#include "qt_widgets_c_QFormLayout.h"

QFormLayout* qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayout(QLayout* ptr) {
  return dynamic_cast<QFormLayout*>(ptr);
}

QFormLayout* qt_widgets_c_QFormLayout_G_dynamic_cast_QFormLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return dynamic_cast<QFormLayout*>(ptr);
}

QFormLayout* qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayout(QLayout* ptr) {
  return static_cast<QFormLayout*>(ptr);
}

QFormLayout* qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QFormLayout*>(ptr);
}

QFormLayout* qt_widgets_c_QFormLayout_G_static_cast_QFormLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QFormLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QFormLayout_G_static_cast_QLayoutItem_ptr(QFormLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QFormLayout_G_static_cast_QLayout_ptr(QFormLayout* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QFormLayout_G_static_cast_QObject_ptr(QFormLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QFormLayout_TakeRowResult_destructor(QFormLayout::TakeRowResult* this_ptr) {
  qt_widgets_c_call_destructor(this_ptr);
}

QLayoutItem* qt_widgets_c_QFormLayout_TakeRowResult_fieldItem(const QFormLayout::TakeRowResult* this_ptr) {
  return this_ptr->fieldItem;
}

QLayoutItem* qt_widgets_c_QFormLayout_TakeRowResult_labelItem(const QFormLayout::TakeRowResult* this_ptr) {
  return this_ptr->labelItem;
}

void qt_widgets_c_QFormLayout_TakeRowResult_set_fieldItem(QFormLayout::TakeRowResult* this_ptr, QLayoutItem* value) {
  this_ptr->fieldItem = value;
}

void qt_widgets_c_QFormLayout_TakeRowResult_set_labelItem(QFormLayout::TakeRowResult* this_ptr, QLayoutItem* value) {
  this_ptr->labelItem = value;
}

void qt_widgets_c_QFormLayout_addItem(QFormLayout* this_ptr, QLayoutItem* item) {
  this_ptr->addItem(item);
}

void qt_widgets_c_QFormLayout_addRow_QLayout(QFormLayout* this_ptr, QLayout* layout) {
  this_ptr->addRow(layout);
}

void qt_widgets_c_QFormLayout_addRow_QString_QLayout(QFormLayout* this_ptr, const QString* labelText, QLayout* field) {
  this_ptr->addRow(*labelText, field);
}

void qt_widgets_c_QFormLayout_addRow_QString_QWidget(QFormLayout* this_ptr, const QString* labelText, QWidget* field) {
  this_ptr->addRow(*labelText, field);
}

void qt_widgets_c_QFormLayout_addRow_QWidget(QFormLayout* this_ptr, QWidget* widget) {
  this_ptr->addRow(widget);
}

void qt_widgets_c_QFormLayout_addRow_QWidget_QLayout(QFormLayout* this_ptr, QWidget* label, QLayout* field) {
  this_ptr->addRow(label, field);
}

void qt_widgets_c_QFormLayout_addRow_QWidget_QWidget(QFormLayout* this_ptr, QWidget* label, QWidget* field) {
  this_ptr->addRow(label, field);
}

int qt_widgets_c_QFormLayout_count(const QFormLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QFormLayout_delete(QFormLayout* this_ptr) {
  delete this_ptr;
}

QFormLayout::FieldGrowthPolicy qt_widgets_c_QFormLayout_fieldGrowthPolicy(const QFormLayout* this_ptr) {
  return this_ptr->fieldGrowthPolicy();
}

void qt_widgets_c_QFormLayout_getItemPosition(const QFormLayout* this_ptr, int index, int* rowPtr, QFormLayout::ItemRole* rolePtr) {
  this_ptr->getItemPosition(index, rowPtr, rolePtr);
}

void qt_widgets_c_QFormLayout_getLayoutPosition(const QFormLayout* this_ptr, QLayout* layout, int* rowPtr, QFormLayout::ItemRole* rolePtr) {
  this_ptr->getLayoutPosition(layout, rowPtr, rolePtr);
}

void qt_widgets_c_QFormLayout_getWidgetPosition(const QFormLayout* this_ptr, QWidget* widget, int* rowPtr, QFormLayout::ItemRole* rolePtr) {
  this_ptr->getWidgetPosition(widget, rowPtr, rolePtr);
}

bool qt_widgets_c_QFormLayout_hasHeightForWidth(const QFormLayout* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QFormLayout_heightForWidth(const QFormLayout* this_ptr, int width) {
  return this_ptr->heightForWidth(width);
}

int qt_widgets_c_QFormLayout_horizontalSpacing(const QFormLayout* this_ptr) {
  return this_ptr->horizontalSpacing();
}

void qt_widgets_c_QFormLayout_insertRow_int_QLayout(QFormLayout* this_ptr, int row, QLayout* layout) {
  this_ptr->insertRow(row, layout);
}

void qt_widgets_c_QFormLayout_insertRow_int_QString_QLayout(QFormLayout* this_ptr, int row, const QString* labelText, QLayout* field) {
  this_ptr->insertRow(row, *labelText, field);
}

void qt_widgets_c_QFormLayout_insertRow_int_QString_QWidget(QFormLayout* this_ptr, int row, const QString* labelText, QWidget* field) {
  this_ptr->insertRow(row, *labelText, field);
}

void qt_widgets_c_QFormLayout_insertRow_int_QWidget(QFormLayout* this_ptr, int row, QWidget* widget) {
  this_ptr->insertRow(row, widget);
}

void qt_widgets_c_QFormLayout_insertRow_int_QWidget_QLayout(QFormLayout* this_ptr, int row, QWidget* label, QLayout* field) {
  this_ptr->insertRow(row, label, field);
}

void qt_widgets_c_QFormLayout_insertRow_int_QWidget_QWidget(QFormLayout* this_ptr, int row, QWidget* label, QWidget* field) {
  this_ptr->insertRow(row, label, field);
}

void qt_widgets_c_QFormLayout_invalidate(QFormLayout* this_ptr) {
  this_ptr->invalidate();
}

QLayoutItem* qt_widgets_c_QFormLayout_itemAt_index(const QFormLayout* this_ptr, int index) {
  return this_ptr->itemAt(index);
}

QLayoutItem* qt_widgets_c_QFormLayout_itemAt_row_role(const QFormLayout* this_ptr, int row, QFormLayout::ItemRole role) {
  return this_ptr->itemAt(row, role);
}

QWidget* qt_widgets_c_QFormLayout_labelForField_QLayout(const QFormLayout* this_ptr, QLayout* field) {
  return this_ptr->labelForField(field);
}

QWidget* qt_widgets_c_QFormLayout_labelForField_QWidget(const QFormLayout* this_ptr, QWidget* field) {
  return this_ptr->labelForField(field);
}

const QMetaObject* qt_widgets_c_QFormLayout_metaObject(const QFormLayout* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QFormLayout_minimumSize_to_output(const QFormLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QFormLayout* qt_widgets_c_QFormLayout_new_no_args() {
  return new QFormLayout();
}

QFormLayout* qt_widgets_c_QFormLayout_new_parent(QWidget* parent) {
  return new QFormLayout(parent);
}

int qt_widgets_c_QFormLayout_qt_metacall(QFormLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFormLayout_qt_metacast(QFormLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QFormLayout_removeRow_layout(QFormLayout* this_ptr, QLayout* layout) {
  this_ptr->removeRow(layout);
}

void qt_widgets_c_QFormLayout_removeRow_row(QFormLayout* this_ptr, int row) {
  this_ptr->removeRow(row);
}

void qt_widgets_c_QFormLayout_removeRow_widget(QFormLayout* this_ptr, QWidget* widget) {
  this_ptr->removeRow(widget);
}

int qt_widgets_c_QFormLayout_rowCount(const QFormLayout* this_ptr) {
  return this_ptr->rowCount();
}

QFormLayout::RowWrapPolicy qt_widgets_c_QFormLayout_rowWrapPolicy(const QFormLayout* this_ptr) {
  return this_ptr->rowWrapPolicy();
}

void qt_widgets_c_QFormLayout_setFieldGrowthPolicy(QFormLayout* this_ptr, QFormLayout::FieldGrowthPolicy policy) {
  this_ptr->setFieldGrowthPolicy(policy);
}

void qt_widgets_c_QFormLayout_setGeometry(QFormLayout* this_ptr, const QRect* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QFormLayout_setHorizontalSpacing(QFormLayout* this_ptr, int spacing) {
  this_ptr->setHorizontalSpacing(spacing);
}

void qt_widgets_c_QFormLayout_setItem(QFormLayout* this_ptr, int row, QFormLayout::ItemRole role, QLayoutItem* item) {
  this_ptr->setItem(row, role, item);
}

void qt_widgets_c_QFormLayout_setLayout(QFormLayout* this_ptr, int row, QFormLayout::ItemRole role, QLayout* layout) {
  this_ptr->setLayout(row, role, layout);
}

void qt_widgets_c_QFormLayout_setRowWrapPolicy(QFormLayout* this_ptr, QFormLayout::RowWrapPolicy policy) {
  this_ptr->setRowWrapPolicy(policy);
}

void qt_widgets_c_QFormLayout_setSpacing(QFormLayout* this_ptr, int arg1) {
  this_ptr->setSpacing(arg1);
}

void qt_widgets_c_QFormLayout_setVerticalSpacing(QFormLayout* this_ptr, int spacing) {
  this_ptr->setVerticalSpacing(spacing);
}

void qt_widgets_c_QFormLayout_setWidget(QFormLayout* this_ptr, int row, QFormLayout::ItemRole role, QWidget* widget) {
  this_ptr->setWidget(row, role, widget);
}

void qt_widgets_c_QFormLayout_sizeHint_to_output(const QFormLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QFormLayout_spacing(const QFormLayout* this_ptr) {
  return this_ptr->spacing();
}

QLayoutItem* qt_widgets_c_QFormLayout_takeAt(QFormLayout* this_ptr, int index) {
  return this_ptr->takeAt(index);
}

void qt_widgets_c_QFormLayout_takeRow_to_output_layout(QFormLayout* this_ptr, QLayout* layout, QFormLayout::TakeRowResult* output) {
  new(output) QFormLayout::TakeRowResult(this_ptr->takeRow(layout));
}

void qt_widgets_c_QFormLayout_takeRow_to_output_row(QFormLayout* this_ptr, int row, QFormLayout::TakeRowResult* output) {
  new(output) QFormLayout::TakeRowResult(this_ptr->takeRow(row));
}

void qt_widgets_c_QFormLayout_takeRow_to_output_widget(QFormLayout* this_ptr, QWidget* widget, QFormLayout::TakeRowResult* output) {
  new(output) QFormLayout::TakeRowResult(this_ptr->takeRow(widget));
}

void qt_widgets_c_QFormLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFormLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QFormLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFormLayout::tr(s, c, n));
}

int qt_widgets_c_QFormLayout_verticalSpacing(const QFormLayout* this_ptr) {
  return this_ptr->verticalSpacing();
}

