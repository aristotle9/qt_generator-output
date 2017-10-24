#include "qt_gui_c_QAccessibleTableInterface.h"

QAccessibleInterface* qt_gui_c_QAccessibleTableInterface_caption(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->caption();
}

QAccessibleInterface* qt_gui_c_QAccessibleTableInterface_cellAt(const QAccessibleTableInterface* this_ptr, int row, int column) {
  return this_ptr->cellAt(row, column);
}

int qt_gui_c_QAccessibleTableInterface_columnCount(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->columnCount();
}

void qt_gui_c_QAccessibleTableInterface_columnDescription_to_output(const QAccessibleTableInterface* this_ptr, int column, QString* output) {
  new(output) QString(this_ptr->columnDescription(column));
}

void qt_gui_c_QAccessibleTableInterface_delete(QAccessibleTableInterface* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QAccessibleTableInterface_isColumnSelected(const QAccessibleTableInterface* this_ptr, int column) {
  return this_ptr->isColumnSelected(column);
}

bool qt_gui_c_QAccessibleTableInterface_isRowSelected(const QAccessibleTableInterface* this_ptr, int row) {
  return this_ptr->isRowSelected(row);
}

void qt_gui_c_QAccessibleTableInterface_modelChange(QAccessibleTableInterface* this_ptr, QAccessibleTableModelChangeEvent* event) {
  this_ptr->modelChange(event);
}

int qt_gui_c_QAccessibleTableInterface_rowCount(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->rowCount();
}

void qt_gui_c_QAccessibleTableInterface_rowDescription_to_output(const QAccessibleTableInterface* this_ptr, int row, QString* output) {
  new(output) QString(this_ptr->rowDescription(row));
}

bool qt_gui_c_QAccessibleTableInterface_selectColumn(QAccessibleTableInterface* this_ptr, int column) {
  return this_ptr->selectColumn(column);
}

bool qt_gui_c_QAccessibleTableInterface_selectRow(QAccessibleTableInterface* this_ptr, int row) {
  return this_ptr->selectRow(row);
}

int qt_gui_c_QAccessibleTableInterface_selectedCellCount(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->selectedCellCount();
}

void qt_gui_c_QAccessibleTableInterface_selectedCells_to_output(const QAccessibleTableInterface* this_ptr, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(this_ptr->selectedCells());
}

int qt_gui_c_QAccessibleTableInterface_selectedColumnCount(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->selectedColumnCount();
}

void qt_gui_c_QAccessibleTableInterface_selectedColumns_to_output(const QAccessibleTableInterface* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->selectedColumns());
}

int qt_gui_c_QAccessibleTableInterface_selectedRowCount(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->selectedRowCount();
}

void qt_gui_c_QAccessibleTableInterface_selectedRows_to_output(const QAccessibleTableInterface* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->selectedRows());
}

QAccessibleInterface* qt_gui_c_QAccessibleTableInterface_summary(const QAccessibleTableInterface* this_ptr) {
  return this_ptr->summary();
}

bool qt_gui_c_QAccessibleTableInterface_unselectColumn(QAccessibleTableInterface* this_ptr, int column) {
  return this_ptr->unselectColumn(column);
}

bool qt_gui_c_QAccessibleTableInterface_unselectRow(QAccessibleTableInterface* this_ptr, int row) {
  return this_ptr->unselectRow(row);
}

