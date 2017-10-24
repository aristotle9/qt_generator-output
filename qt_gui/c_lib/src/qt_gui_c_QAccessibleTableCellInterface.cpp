#include "qt_gui_c_QAccessibleTableCellInterface.h"

int qt_gui_c_QAccessibleTableCellInterface_columnExtent(const QAccessibleTableCellInterface* this_ptr) {
  return this_ptr->columnExtent();
}

void qt_gui_c_QAccessibleTableCellInterface_columnHeaderCells_to_output(const QAccessibleTableCellInterface* this_ptr, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(this_ptr->columnHeaderCells());
}

int qt_gui_c_QAccessibleTableCellInterface_columnIndex(const QAccessibleTableCellInterface* this_ptr) {
  return this_ptr->columnIndex();
}

void qt_gui_c_QAccessibleTableCellInterface_delete(QAccessibleTableCellInterface* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QAccessibleTableCellInterface_isSelected(const QAccessibleTableCellInterface* this_ptr) {
  return this_ptr->isSelected();
}

int qt_gui_c_QAccessibleTableCellInterface_rowExtent(const QAccessibleTableCellInterface* this_ptr) {
  return this_ptr->rowExtent();
}

void qt_gui_c_QAccessibleTableCellInterface_rowHeaderCells_to_output(const QAccessibleTableCellInterface* this_ptr, QList< QAccessibleInterface* >* output) {
  new(output) QList< QAccessibleInterface* >(this_ptr->rowHeaderCells());
}

int qt_gui_c_QAccessibleTableCellInterface_rowIndex(const QAccessibleTableCellInterface* this_ptr) {
  return this_ptr->rowIndex();
}

QAccessibleInterface* qt_gui_c_QAccessibleTableCellInterface_table(const QAccessibleTableCellInterface* this_ptr) {
  return this_ptr->table();
}

