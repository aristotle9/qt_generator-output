#include "qt_widgets_c_QTableWidget.h"

QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QTableView(QTableView* ptr) {
  return dynamic_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTableWidget*>(ptr);
}

QDataStream* qt_widgets_c_QTableWidget_G_operator_shl(QDataStream* out, const QTableWidgetItem* item) {
  return &operator<<(*out, *item);
}

QDataStream* qt_widgets_c_QTableWidget_G_operator_shr(QDataStream* in, QTableWidgetItem* item) {
  return &operator>>(*in, *item);
}

QAbstractItemView* qt_widgets_c_QTableWidget_G_static_cast_QAbstractItemView_ptr(QTableWidget* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QTableWidget_G_static_cast_QAbstractScrollArea_ptr(QTableWidget* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QTableWidget_G_static_cast_QFrame_ptr(QTableWidget* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QTableWidget_G_static_cast_QObject_ptr(QTableWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTableWidget_G_static_cast_QPaintDevice_ptr(QTableWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTableView* qt_widgets_c_QTableWidget_G_static_cast_QTableView_ptr(QTableWidget* ptr) {
  return static_cast<QTableView*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QFrame(QFrame* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QTableView(QTableView* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QTableWidget* qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTableWidget*>(ptr);
}

QWidget* qt_widgets_c_QTableWidget_G_static_cast_QWidget_ptr(QTableWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

QWidget* qt_widgets_c_QTableWidget_cellWidget(const QTableWidget* this_ptr, int row, int column) {
  return this_ptr->cellWidget(row, column);
}

void qt_widgets_c_QTableWidget_clear(QTableWidget* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QTableWidget_clearContents(QTableWidget* this_ptr) {
  this_ptr->clearContents();
}

void qt_widgets_c_QTableWidget_closePersistentEditor(QTableWidget* this_ptr, QTableWidgetItem* item) {
  this_ptr->closePersistentEditor(item);
}

int qt_widgets_c_QTableWidget_column(const QTableWidget* this_ptr, const QTableWidgetItem* item) {
  return this_ptr->column(item);
}

int qt_widgets_c_QTableWidget_columnCount(const QTableWidget* this_ptr) {
  return this_ptr->columnCount();
}

int qt_widgets_c_QTableWidget_currentColumn(const QTableWidget* this_ptr) {
  return this_ptr->currentColumn();
}

QTableWidgetItem* qt_widgets_c_QTableWidget_currentItem(const QTableWidget* this_ptr) {
  return this_ptr->currentItem();
}

int qt_widgets_c_QTableWidget_currentRow(const QTableWidget* this_ptr) {
  return this_ptr->currentRow();
}

void qt_widgets_c_QTableWidget_delete(QTableWidget* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTableWidget_editItem(QTableWidget* this_ptr, QTableWidgetItem* item) {
  this_ptr->editItem(item);
}

QTableWidgetItem* qt_widgets_c_QTableWidget_horizontalHeaderItem(const QTableWidget* this_ptr, int column) {
  return this_ptr->horizontalHeaderItem(column);
}

void qt_widgets_c_QTableWidget_insertColumn(QTableWidget* this_ptr, int column) {
  this_ptr->insertColumn(column);
}

void qt_widgets_c_QTableWidget_insertRow(QTableWidget* this_ptr, int row) {
  this_ptr->insertRow(row);
}

bool qt_widgets_c_QTableWidget_isItemSelected(const QTableWidget* this_ptr, const QTableWidgetItem* item) {
  return this_ptr->isItemSelected(item);
}

bool qt_widgets_c_QTableWidget_isSortingEnabled(const QTableWidget* this_ptr) {
  return this_ptr->isSortingEnabled();
}

QTableWidgetItem* qt_widgets_c_QTableWidget_item(const QTableWidget* this_ptr, int row, int column) {
  return this_ptr->item(row, column);
}

QTableWidgetItem* qt_widgets_c_QTableWidget_itemAt_p(const QTableWidget* this_ptr, const QPoint* p) {
  return this_ptr->itemAt(*p);
}

QTableWidgetItem* qt_widgets_c_QTableWidget_itemAt_x_y(const QTableWidget* this_ptr, int x, int y) {
  return this_ptr->itemAt(x, y);
}

const QTableWidgetItem* qt_widgets_c_QTableWidget_itemPrototype(const QTableWidget* this_ptr) {
  return this_ptr->itemPrototype();
}

const QMetaObject* qt_widgets_c_QTableWidget_metaObject(const QTableWidget* this_ptr) {
  return this_ptr->metaObject();
}

QTableWidget* qt_widgets_c_QTableWidget_new_no_args() {
  return new QTableWidget();
}

QTableWidget* qt_widgets_c_QTableWidget_new_parent(QWidget* parent) {
  return new QTableWidget(parent);
}

QTableWidget* qt_widgets_c_QTableWidget_new_rows_columns(int rows, int columns) {
  return new QTableWidget(rows, columns);
}

QTableWidget* qt_widgets_c_QTableWidget_new_rows_columns_parent(int rows, int columns, QWidget* parent) {
  return new QTableWidget(rows, columns, parent);
}

void qt_widgets_c_QTableWidget_openPersistentEditor(QTableWidget* this_ptr, QTableWidgetItem* item) {
  this_ptr->openPersistentEditor(item);
}

int qt_widgets_c_QTableWidget_qt_metacall(QTableWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTableWidget_qt_metacast(QTableWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTableWidget_removeCellWidget(QTableWidget* this_ptr, int row, int column) {
  this_ptr->removeCellWidget(row, column);
}

void qt_widgets_c_QTableWidget_removeColumn(QTableWidget* this_ptr, int column) {
  this_ptr->removeColumn(column);
}

void qt_widgets_c_QTableWidget_removeRow(QTableWidget* this_ptr, int row) {
  this_ptr->removeRow(row);
}

int qt_widgets_c_QTableWidget_row(const QTableWidget* this_ptr, const QTableWidgetItem* item) {
  return this_ptr->row(item);
}

int qt_widgets_c_QTableWidget_rowCount(const QTableWidget* this_ptr) {
  return this_ptr->rowCount();
}

void qt_widgets_c_QTableWidget_scrollToItem_item(QTableWidget* this_ptr, const QTableWidgetItem* item) {
  this_ptr->scrollToItem(item);
}

void qt_widgets_c_QTableWidget_scrollToItem_item_hint(QTableWidget* this_ptr, const QTableWidgetItem* item, const QAbstractItemView::ScrollHint* hint) {
  this_ptr->scrollToItem(item, *hint);
}

void qt_widgets_c_QTableWidget_selectedItems_to_output(const QTableWidget* this_ptr, QList< QTableWidgetItem* >* output) {
  new(output) QList< QTableWidgetItem* >(this_ptr->selectedItems());
}

void qt_widgets_c_QTableWidget_selectedRanges_to_output(const QTableWidget* this_ptr, QList< QTableWidgetSelectionRange >* output) {
  new(output) QList< QTableWidgetSelectionRange >(this_ptr->selectedRanges());
}

void qt_widgets_c_QTableWidget_setCellWidget(QTableWidget* this_ptr, int row, int column, QWidget* widget) {
  this_ptr->setCellWidget(row, column, widget);
}

void qt_widgets_c_QTableWidget_setColumnCount(QTableWidget* this_ptr, int columns) {
  this_ptr->setColumnCount(columns);
}

void qt_widgets_c_QTableWidget_setCurrentCell(QTableWidget* this_ptr, int row, int column) {
  this_ptr->setCurrentCell(row, column);
}

void qt_widgets_c_QTableWidget_setCurrentItem(QTableWidget* this_ptr, QTableWidgetItem* item) {
  this_ptr->setCurrentItem(item);
}

void qt_widgets_c_QTableWidget_setHorizontalHeaderItem(QTableWidget* this_ptr, int column, QTableWidgetItem* item) {
  this_ptr->setHorizontalHeaderItem(column, item);
}

void qt_widgets_c_QTableWidget_setHorizontalHeaderLabels(QTableWidget* this_ptr, const QStringList* labels) {
  this_ptr->setHorizontalHeaderLabels(*labels);
}

void qt_widgets_c_QTableWidget_setItem(QTableWidget* this_ptr, int row, int column, QTableWidgetItem* item) {
  this_ptr->setItem(row, column, item);
}

void qt_widgets_c_QTableWidget_setItemPrototype(QTableWidget* this_ptr, const QTableWidgetItem* item) {
  this_ptr->setItemPrototype(item);
}

void qt_widgets_c_QTableWidget_setItemSelected(QTableWidget* this_ptr, const QTableWidgetItem* item, bool select) {
  this_ptr->setItemSelected(item, select);
}

void qt_widgets_c_QTableWidget_setRangeSelected(QTableWidget* this_ptr, const QTableWidgetSelectionRange* range, bool select) {
  this_ptr->setRangeSelected(*range, select);
}

void qt_widgets_c_QTableWidget_setRowCount(QTableWidget* this_ptr, int rows) {
  this_ptr->setRowCount(rows);
}

void qt_widgets_c_QTableWidget_setSortingEnabled(QTableWidget* this_ptr, bool enable) {
  this_ptr->setSortingEnabled(enable);
}

void qt_widgets_c_QTableWidget_setVerticalHeaderItem(QTableWidget* this_ptr, int row, QTableWidgetItem* item) {
  this_ptr->setVerticalHeaderItem(row, item);
}

void qt_widgets_c_QTableWidget_setVerticalHeaderLabels(QTableWidget* this_ptr, const QStringList* labels) {
  this_ptr->setVerticalHeaderLabels(*labels);
}

void qt_widgets_c_QTableWidget_sortItems_column(QTableWidget* this_ptr, int column) {
  this_ptr->sortItems(column);
}

void qt_widgets_c_QTableWidget_sortItems_column_order(QTableWidget* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sortItems(column, *order);
}

QTableWidgetItem* qt_widgets_c_QTableWidget_takeHorizontalHeaderItem(QTableWidget* this_ptr, int column) {
  return this_ptr->takeHorizontalHeaderItem(column);
}

QTableWidgetItem* qt_widgets_c_QTableWidget_takeItem(QTableWidget* this_ptr, int row, int column) {
  return this_ptr->takeItem(row, column);
}

QTableWidgetItem* qt_widgets_c_QTableWidget_takeVerticalHeaderItem(QTableWidget* this_ptr, int row) {
  return this_ptr->takeVerticalHeaderItem(row);
}

void qt_widgets_c_QTableWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTableWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QTableWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTableWidget::tr(s, c, n));
}

QTableWidgetItem* qt_widgets_c_QTableWidget_verticalHeaderItem(const QTableWidget* this_ptr, int row) {
  return this_ptr->verticalHeaderItem(row);
}

int qt_widgets_c_QTableWidget_visualColumn(const QTableWidget* this_ptr, int logicalColumn) {
  return this_ptr->visualColumn(logicalColumn);
}

void qt_widgets_c_QTableWidget_visualItemRect_to_output(const QTableWidget* this_ptr, const QTableWidgetItem* item, QRect* output) {
  new(output) QRect(this_ptr->visualItemRect(item));
}

int qt_widgets_c_QTableWidget_visualRow(const QTableWidget* this_ptr, int logicalRow) {
  return this_ptr->visualRow(logicalRow);
}

