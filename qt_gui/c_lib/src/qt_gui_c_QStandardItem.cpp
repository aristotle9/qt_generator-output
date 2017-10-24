#include "qt_gui_c_QStandardItem.h"

void qt_gui_c_QStandardItem_accessibleDescription_to_output(const QStandardItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->accessibleDescription());
}

void qt_gui_c_QStandardItem_accessibleText_to_output(const QStandardItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->accessibleText());
}

void qt_gui_c_QStandardItem_appendColumn(QStandardItem* this_ptr, const QList< QStandardItem* >* items) {
  this_ptr->appendColumn(*items);
}

void qt_gui_c_QStandardItem_appendRow_item(QStandardItem* this_ptr, QStandardItem* item) {
  this_ptr->appendRow(item);
}

void qt_gui_c_QStandardItem_appendRow_items(QStandardItem* this_ptr, const QList< QStandardItem* >* items) {
  this_ptr->appendRow(*items);
}

void qt_gui_c_QStandardItem_appendRows(QStandardItem* this_ptr, const QList< QStandardItem* >* items) {
  this_ptr->appendRows(*items);
}

void qt_gui_c_QStandardItem_background_to_output(const QStandardItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->background());
}

QStandardItem* qt_gui_c_QStandardItem_child_row(const QStandardItem* this_ptr, int row) {
  return this_ptr->child(row);
}

QStandardItem* qt_gui_c_QStandardItem_child_row_column(const QStandardItem* this_ptr, int row, int column) {
  return this_ptr->child(row, column);
}

QStandardItem* qt_gui_c_QStandardItem_clone(const QStandardItem* this_ptr) {
  return this_ptr->clone();
}

int qt_gui_c_QStandardItem_column(const QStandardItem* this_ptr) {
  return this_ptr->column();
}

int qt_gui_c_QStandardItem_columnCount(const QStandardItem* this_ptr) {
  return this_ptr->columnCount();
}

void qt_gui_c_QStandardItem_data_to_output_no_args(const QStandardItem* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->data());
}

void qt_gui_c_QStandardItem_data_to_output_role(const QStandardItem* this_ptr, int role, QVariant* output) {
  new(output) QVariant(this_ptr->data(role));
}

void qt_gui_c_QStandardItem_delete(QStandardItem* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QStandardItem_font_to_output(const QStandardItem* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_gui_c_QStandardItem_foreground_to_output(const QStandardItem* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->foreground());
}

bool qt_gui_c_QStandardItem_hasChildren(const QStandardItem* this_ptr) {
  return this_ptr->hasChildren();
}

void qt_gui_c_QStandardItem_icon_to_output(const QStandardItem* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

void qt_gui_c_QStandardItem_index_to_output(const QStandardItem* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->index());
}

void qt_gui_c_QStandardItem_insertColumn(QStandardItem* this_ptr, int column, const QList< QStandardItem* >* items) {
  this_ptr->insertColumn(column, *items);
}

void qt_gui_c_QStandardItem_insertColumns(QStandardItem* this_ptr, int column, int count) {
  this_ptr->insertColumns(column, count);
}

void qt_gui_c_QStandardItem_insertRow_row_item(QStandardItem* this_ptr, int row, QStandardItem* item) {
  this_ptr->insertRow(row, item);
}

void qt_gui_c_QStandardItem_insertRow_row_items(QStandardItem* this_ptr, int row, const QList< QStandardItem* >* items) {
  this_ptr->insertRow(row, *items);
}

void qt_gui_c_QStandardItem_insertRows_row_count(QStandardItem* this_ptr, int row, int count) {
  this_ptr->insertRows(row, count);
}

void qt_gui_c_QStandardItem_insertRows_row_items(QStandardItem* this_ptr, int row, const QList< QStandardItem* >* items) {
  this_ptr->insertRows(row, *items);
}

bool qt_gui_c_QStandardItem_isAutoTristate(const QStandardItem* this_ptr) {
  return this_ptr->isAutoTristate();
}

bool qt_gui_c_QStandardItem_isCheckable(const QStandardItem* this_ptr) {
  return this_ptr->isCheckable();
}

bool qt_gui_c_QStandardItem_isDragEnabled(const QStandardItem* this_ptr) {
  return this_ptr->isDragEnabled();
}

bool qt_gui_c_QStandardItem_isDropEnabled(const QStandardItem* this_ptr) {
  return this_ptr->isDropEnabled();
}

bool qt_gui_c_QStandardItem_isEditable(const QStandardItem* this_ptr) {
  return this_ptr->isEditable();
}

bool qt_gui_c_QStandardItem_isEnabled(const QStandardItem* this_ptr) {
  return this_ptr->isEnabled();
}

bool qt_gui_c_QStandardItem_isSelectable(const QStandardItem* this_ptr) {
  return this_ptr->isSelectable();
}

bool qt_gui_c_QStandardItem_isTristate(const QStandardItem* this_ptr) {
  return this_ptr->isTristate();
}

bool qt_gui_c_QStandardItem_isUserTristate(const QStandardItem* this_ptr) {
  return this_ptr->isUserTristate();
}

QStandardItemModel* qt_gui_c_QStandardItem_model(const QStandardItem* this_ptr) {
  return this_ptr->model();
}

QStandardItem* qt_gui_c_QStandardItem_new_icon_text(const QIcon* icon, const QString* text) {
  return new QStandardItem(*icon, *text);
}

QStandardItem* qt_gui_c_QStandardItem_new_no_args() {
  return new QStandardItem();
}

QStandardItem* qt_gui_c_QStandardItem_new_rows(int rows) {
  return new QStandardItem(rows);
}

QStandardItem* qt_gui_c_QStandardItem_new_rows_columns(int rows, int columns) {
  return new QStandardItem(rows, columns);
}

QStandardItem* qt_gui_c_QStandardItem_new_text(const QString* text) {
  return new QStandardItem(*text);
}

bool qt_gui_c_QStandardItem_operator_lt(const QStandardItem* this_ptr, const QStandardItem* other) {
  return this_ptr->operator<(*other);
}

QStandardItem* qt_gui_c_QStandardItem_parent(const QStandardItem* this_ptr) {
  return this_ptr->parent();
}

void qt_gui_c_QStandardItem_read(QStandardItem* this_ptr, QDataStream* in) {
  this_ptr->read(*in);
}

void qt_gui_c_QStandardItem_removeColumn(QStandardItem* this_ptr, int column) {
  this_ptr->removeColumn(column);
}

void qt_gui_c_QStandardItem_removeColumns(QStandardItem* this_ptr, int column, int count) {
  this_ptr->removeColumns(column, count);
}

void qt_gui_c_QStandardItem_removeRow(QStandardItem* this_ptr, int row) {
  this_ptr->removeRow(row);
}

void qt_gui_c_QStandardItem_removeRows(QStandardItem* this_ptr, int row, int count) {
  this_ptr->removeRows(row, count);
}

int qt_gui_c_QStandardItem_row(const QStandardItem* this_ptr) {
  return this_ptr->row();
}

int qt_gui_c_QStandardItem_rowCount(const QStandardItem* this_ptr) {
  return this_ptr->rowCount();
}

void qt_gui_c_QStandardItem_setAccessibleDescription(QStandardItem* this_ptr, const QString* accessibleDescription) {
  this_ptr->setAccessibleDescription(*accessibleDescription);
}

void qt_gui_c_QStandardItem_setAccessibleText(QStandardItem* this_ptr, const QString* accessibleText) {
  this_ptr->setAccessibleText(*accessibleText);
}

void qt_gui_c_QStandardItem_setAutoTristate(QStandardItem* this_ptr, bool tristate) {
  this_ptr->setAutoTristate(tristate);
}

void qt_gui_c_QStandardItem_setBackground(QStandardItem* this_ptr, const QBrush* brush) {
  this_ptr->setBackground(*brush);
}

void qt_gui_c_QStandardItem_setCheckState(QStandardItem* this_ptr, const Qt::CheckState* checkState) {
  this_ptr->setCheckState(*checkState);
}

void qt_gui_c_QStandardItem_setCheckable(QStandardItem* this_ptr, bool checkable) {
  this_ptr->setCheckable(checkable);
}

void qt_gui_c_QStandardItem_setChild_row_column_item(QStandardItem* this_ptr, int row, int column, QStandardItem* item) {
  this_ptr->setChild(row, column, item);
}

void qt_gui_c_QStandardItem_setChild_row_item(QStandardItem* this_ptr, int row, QStandardItem* item) {
  this_ptr->setChild(row, item);
}

void qt_gui_c_QStandardItem_setColumnCount(QStandardItem* this_ptr, int columns) {
  this_ptr->setColumnCount(columns);
}

void qt_gui_c_QStandardItem_setData_value(QStandardItem* this_ptr, const QVariant* value) {
  this_ptr->setData(*value);
}

void qt_gui_c_QStandardItem_setData_value_role(QStandardItem* this_ptr, const QVariant* value, int role) {
  this_ptr->setData(*value, role);
}

void qt_gui_c_QStandardItem_setDragEnabled(QStandardItem* this_ptr, bool dragEnabled) {
  this_ptr->setDragEnabled(dragEnabled);
}

void qt_gui_c_QStandardItem_setDropEnabled(QStandardItem* this_ptr, bool dropEnabled) {
  this_ptr->setDropEnabled(dropEnabled);
}

void qt_gui_c_QStandardItem_setEditable(QStandardItem* this_ptr, bool editable) {
  this_ptr->setEditable(editable);
}

void qt_gui_c_QStandardItem_setEnabled(QStandardItem* this_ptr, bool enabled) {
  this_ptr->setEnabled(enabled);
}

void qt_gui_c_QStandardItem_setFont(QStandardItem* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_gui_c_QStandardItem_setForeground(QStandardItem* this_ptr, const QBrush* brush) {
  this_ptr->setForeground(*brush);
}

void qt_gui_c_QStandardItem_setIcon(QStandardItem* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_gui_c_QStandardItem_setRowCount(QStandardItem* this_ptr, int rows) {
  this_ptr->setRowCount(rows);
}

void qt_gui_c_QStandardItem_setSelectable(QStandardItem* this_ptr, bool selectable) {
  this_ptr->setSelectable(selectable);
}

void qt_gui_c_QStandardItem_setSizeHint(QStandardItem* this_ptr, const QSize* sizeHint) {
  this_ptr->setSizeHint(*sizeHint);
}

void qt_gui_c_QStandardItem_setStatusTip(QStandardItem* this_ptr, const QString* statusTip) {
  this_ptr->setStatusTip(*statusTip);
}

void qt_gui_c_QStandardItem_setText(QStandardItem* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_gui_c_QStandardItem_setToolTip(QStandardItem* this_ptr, const QString* toolTip) {
  this_ptr->setToolTip(*toolTip);
}

void qt_gui_c_QStandardItem_setTristate(QStandardItem* this_ptr, bool tristate) {
  this_ptr->setTristate(tristate);
}

void qt_gui_c_QStandardItem_setUserTristate(QStandardItem* this_ptr, bool tristate) {
  this_ptr->setUserTristate(tristate);
}

void qt_gui_c_QStandardItem_setWhatsThis(QStandardItem* this_ptr, const QString* whatsThis) {
  this_ptr->setWhatsThis(*whatsThis);
}

void qt_gui_c_QStandardItem_sizeHint_to_output(const QStandardItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_gui_c_QStandardItem_sortChildren_column(QStandardItem* this_ptr, int column) {
  this_ptr->sortChildren(column);
}

void qt_gui_c_QStandardItem_sortChildren_column_order(QStandardItem* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sortChildren(column, *order);
}

void qt_gui_c_QStandardItem_statusTip_to_output(const QStandardItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->statusTip());
}

QStandardItem* qt_gui_c_QStandardItem_takeChild_row(QStandardItem* this_ptr, int row) {
  return this_ptr->takeChild(row);
}

QStandardItem* qt_gui_c_QStandardItem_takeChild_row_column(QStandardItem* this_ptr, int row, int column) {
  return this_ptr->takeChild(row, column);
}

void qt_gui_c_QStandardItem_takeColumn_to_output(QStandardItem* this_ptr, int column, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->takeColumn(column));
}

void qt_gui_c_QStandardItem_takeRow_to_output(QStandardItem* this_ptr, int row, QList< QStandardItem* >* output) {
  new(output) QList< QStandardItem* >(this_ptr->takeRow(row));
}

void qt_gui_c_QStandardItem_text_to_output(const QStandardItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_gui_c_QStandardItem_toolTip_to_output(const QStandardItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

int qt_gui_c_QStandardItem_type(const QStandardItem* this_ptr) {
  return this_ptr->type();
}

void qt_gui_c_QStandardItem_whatsThis_to_output(const QStandardItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->whatsThis());
}

void qt_gui_c_QStandardItem_write(const QStandardItem* this_ptr, QDataStream* out) {
  this_ptr->write(*out);
}

