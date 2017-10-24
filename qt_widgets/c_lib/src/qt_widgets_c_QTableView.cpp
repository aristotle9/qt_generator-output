#include "qt_widgets_c_QTableView.h"

QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_dynamic_cast_QTableView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTableView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QTableView_G_static_cast_QAbstractItemView_ptr(QTableView* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QTableView_G_static_cast_QAbstractScrollArea_ptr(QTableView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QTableView_G_static_cast_QFrame_ptr(QTableView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QTableView_G_static_cast_QObject_ptr(QTableView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTableView_G_static_cast_QPaintDevice_ptr(QTableView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QObject(QObject* ptr) {
  return static_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTableView*>(ptr);
}

QTableView* qt_widgets_c_QTableView_G_static_cast_QTableView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTableView*>(ptr);
}

QWidget* qt_widgets_c_QTableView_G_static_cast_QWidget_ptr(QTableView* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QTableView_clearSpans(QTableView* this_ptr) {
  this_ptr->clearSpans();
}

int qt_widgets_c_QTableView_columnAt(const QTableView* this_ptr, int x) {
  return this_ptr->columnAt(x);
}

int qt_widgets_c_QTableView_columnSpan(const QTableView* this_ptr, int row, int column) {
  return this_ptr->columnSpan(row, column);
}

int qt_widgets_c_QTableView_columnViewportPosition(const QTableView* this_ptr, int column) {
  return this_ptr->columnViewportPosition(column);
}

int qt_widgets_c_QTableView_columnWidth(const QTableView* this_ptr, int column) {
  return this_ptr->columnWidth(column);
}

void qt_widgets_c_QTableView_delete(QTableView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTableView_doItemsLayout(QTableView* this_ptr) {
  this_ptr->doItemsLayout();
}

void qt_widgets_c_QTableView_hideColumn(QTableView* this_ptr, int column) {
  this_ptr->hideColumn(column);
}

void qt_widgets_c_QTableView_hideRow(QTableView* this_ptr, int row) {
  this_ptr->hideRow(row);
}

QHeaderView* qt_widgets_c_QTableView_horizontalHeader(const QTableView* this_ptr) {
  return this_ptr->horizontalHeader();
}

void qt_widgets_c_QTableView_indexAt_to_output(const QTableView* this_ptr, const QPoint* p, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexAt(*p));
}

bool qt_widgets_c_QTableView_isColumnHidden(const QTableView* this_ptr, int column) {
  return this_ptr->isColumnHidden(column);
}

bool qt_widgets_c_QTableView_isCornerButtonEnabled(const QTableView* this_ptr) {
  return this_ptr->isCornerButtonEnabled();
}

bool qt_widgets_c_QTableView_isRowHidden(const QTableView* this_ptr, int row) {
  return this_ptr->isRowHidden(row);
}

bool qt_widgets_c_QTableView_isSortingEnabled(const QTableView* this_ptr) {
  return this_ptr->isSortingEnabled();
}

const QMetaObject* qt_widgets_c_QTableView_metaObject(const QTableView* this_ptr) {
  return this_ptr->metaObject();
}

QTableView* qt_widgets_c_QTableView_new_no_args() {
  return new QTableView();
}

QTableView* qt_widgets_c_QTableView_new_parent(QWidget* parent) {
  return new QTableView(parent);
}

int qt_widgets_c_QTableView_qt_metacall(QTableView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTableView_qt_metacast(QTableView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTableView_resizeColumnToContents(QTableView* this_ptr, int column) {
  this_ptr->resizeColumnToContents(column);
}

void qt_widgets_c_QTableView_resizeColumnsToContents(QTableView* this_ptr) {
  this_ptr->resizeColumnsToContents();
}

void qt_widgets_c_QTableView_resizeRowToContents(QTableView* this_ptr, int row) {
  this_ptr->resizeRowToContents(row);
}

void qt_widgets_c_QTableView_resizeRowsToContents(QTableView* this_ptr) {
  this_ptr->resizeRowsToContents();
}

int qt_widgets_c_QTableView_rowAt(const QTableView* this_ptr, int y) {
  return this_ptr->rowAt(y);
}

int qt_widgets_c_QTableView_rowHeight(const QTableView* this_ptr, int row) {
  return this_ptr->rowHeight(row);
}

int qt_widgets_c_QTableView_rowSpan(const QTableView* this_ptr, int row, int column) {
  return this_ptr->rowSpan(row, column);
}

int qt_widgets_c_QTableView_rowViewportPosition(const QTableView* this_ptr, int row) {
  return this_ptr->rowViewportPosition(row);
}

void qt_widgets_c_QTableView_scrollTo_index(QTableView* this_ptr, const QModelIndex* index) {
  this_ptr->scrollTo(*index);
}

void qt_widgets_c_QTableView_scrollTo_index_hint(QTableView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint) {
  this_ptr->scrollTo(*index, hint);
}

void qt_widgets_c_QTableView_selectColumn(QTableView* this_ptr, int column) {
  this_ptr->selectColumn(column);
}

void qt_widgets_c_QTableView_selectRow(QTableView* this_ptr, int row) {
  this_ptr->selectRow(row);
}

void qt_widgets_c_QTableView_setColumnHidden(QTableView* this_ptr, int column, bool hide) {
  this_ptr->setColumnHidden(column, hide);
}

void qt_widgets_c_QTableView_setColumnWidth(QTableView* this_ptr, int column, int width) {
  this_ptr->setColumnWidth(column, width);
}

void qt_widgets_c_QTableView_setCornerButtonEnabled(QTableView* this_ptr, bool enable) {
  this_ptr->setCornerButtonEnabled(enable);
}

void qt_widgets_c_QTableView_setGridStyle(QTableView* this_ptr, const Qt::PenStyle* style) {
  this_ptr->setGridStyle(*style);
}

void qt_widgets_c_QTableView_setHorizontalHeader(QTableView* this_ptr, QHeaderView* header) {
  this_ptr->setHorizontalHeader(header);
}

void qt_widgets_c_QTableView_setModel(QTableView* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QTableView_setRootIndex(QTableView* this_ptr, const QModelIndex* index) {
  this_ptr->setRootIndex(*index);
}

void qt_widgets_c_QTableView_setRowHeight(QTableView* this_ptr, int row, int height) {
  this_ptr->setRowHeight(row, height);
}

void qt_widgets_c_QTableView_setRowHidden(QTableView* this_ptr, int row, bool hide) {
  this_ptr->setRowHidden(row, hide);
}

void qt_widgets_c_QTableView_setSelectionModel(QTableView* this_ptr, QItemSelectionModel* selectionModel) {
  this_ptr->setSelectionModel(selectionModel);
}

void qt_widgets_c_QTableView_setShowGrid(QTableView* this_ptr, bool show) {
  this_ptr->setShowGrid(show);
}

void qt_widgets_c_QTableView_setSortingEnabled(QTableView* this_ptr, bool enable) {
  this_ptr->setSortingEnabled(enable);
}

void qt_widgets_c_QTableView_setSpan(QTableView* this_ptr, int row, int column, int rowSpan, int columnSpan) {
  this_ptr->setSpan(row, column, rowSpan, columnSpan);
}

void qt_widgets_c_QTableView_setVerticalHeader(QTableView* this_ptr, QHeaderView* header) {
  this_ptr->setVerticalHeader(header);
}

void qt_widgets_c_QTableView_setWordWrap(QTableView* this_ptr, bool on) {
  this_ptr->setWordWrap(on);
}

void qt_widgets_c_QTableView_showColumn(QTableView* this_ptr, int column) {
  this_ptr->showColumn(column);
}

bool qt_widgets_c_QTableView_showGrid(const QTableView* this_ptr) {
  return this_ptr->showGrid();
}

void qt_widgets_c_QTableView_showRow(QTableView* this_ptr, int row) {
  this_ptr->showRow(row);
}

void qt_widgets_c_QTableView_sortByColumn_column(QTableView* this_ptr, int column) {
  this_ptr->sortByColumn(column);
}

void qt_widgets_c_QTableView_sortByColumn_column_order(QTableView* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sortByColumn(column, *order);
}

void qt_widgets_c_QTableView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTableView::trUtf8(s, c, n));
}

void qt_widgets_c_QTableView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTableView::tr(s, c, n));
}

QHeaderView* qt_widgets_c_QTableView_verticalHeader(const QTableView* this_ptr) {
  return this_ptr->verticalHeader();
}

void qt_widgets_c_QTableView_visualRect_to_output(const QTableView* this_ptr, const QModelIndex* index, QRect* output) {
  new(output) QRect(this_ptr->visualRect(*index));
}

bool qt_widgets_c_QTableView_wordWrap(const QTableView* this_ptr) {
  return this_ptr->wordWrap();
}

