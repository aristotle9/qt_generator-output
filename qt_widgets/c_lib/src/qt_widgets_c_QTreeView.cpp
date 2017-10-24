#include "qt_widgets_c_QTreeView.h"

QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_dynamic_cast_QTreeView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTreeView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QTreeView_G_static_cast_QAbstractItemView_ptr(QTreeView* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QTreeView_G_static_cast_QAbstractScrollArea_ptr(QTreeView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QTreeView_G_static_cast_QFrame_ptr(QTreeView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QTreeView_G_static_cast_QObject_ptr(QTreeView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTreeView_G_static_cast_QPaintDevice_ptr(QTreeView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QObject(QObject* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QTreeView* qt_widgets_c_QTreeView_G_static_cast_QTreeView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QWidget* qt_widgets_c_QTreeView_G_static_cast_QWidget_ptr(QTreeView* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QTreeView_allColumnsShowFocus(const QTreeView* this_ptr) {
  return this_ptr->allColumnsShowFocus();
}

int qt_widgets_c_QTreeView_autoExpandDelay(const QTreeView* this_ptr) {
  return this_ptr->autoExpandDelay();
}

void qt_widgets_c_QTreeView_collapse(QTreeView* this_ptr, const QModelIndex* index) {
  this_ptr->collapse(*index);
}

void qt_widgets_c_QTreeView_collapseAll(QTreeView* this_ptr) {
  this_ptr->collapseAll();
}

int qt_widgets_c_QTreeView_columnAt(const QTreeView* this_ptr, int x) {
  return this_ptr->columnAt(x);
}

int qt_widgets_c_QTreeView_columnViewportPosition(const QTreeView* this_ptr, int column) {
  return this_ptr->columnViewportPosition(column);
}

int qt_widgets_c_QTreeView_columnWidth(const QTreeView* this_ptr, int column) {
  return this_ptr->columnWidth(column);
}

void qt_widgets_c_QTreeView_dataChanged_topLeft_bottomRight(QTreeView* this_ptr, const QModelIndex* topLeft, const QModelIndex* bottomRight) {
  this_ptr->dataChanged(*topLeft, *bottomRight);
}

void qt_widgets_c_QTreeView_dataChanged_topLeft_bottomRight_roles(QTreeView* this_ptr, const QModelIndex* topLeft, const QModelIndex* bottomRight, const QVector< int >* roles) {
  this_ptr->dataChanged(*topLeft, *bottomRight, *roles);
}

void qt_widgets_c_QTreeView_delete(QTreeView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTreeView_doItemsLayout(QTreeView* this_ptr) {
  this_ptr->doItemsLayout();
}

void qt_widgets_c_QTreeView_expand(QTreeView* this_ptr, const QModelIndex* index) {
  this_ptr->expand(*index);
}

void qt_widgets_c_QTreeView_expandAll(QTreeView* this_ptr) {
  this_ptr->expandAll();
}

void qt_widgets_c_QTreeView_expandToDepth(QTreeView* this_ptr, int depth) {
  this_ptr->expandToDepth(depth);
}

bool qt_widgets_c_QTreeView_expandsOnDoubleClick(const QTreeView* this_ptr) {
  return this_ptr->expandsOnDoubleClick();
}

QHeaderView* qt_widgets_c_QTreeView_header(const QTreeView* this_ptr) {
  return this_ptr->header();
}

void qt_widgets_c_QTreeView_hideColumn(QTreeView* this_ptr, int column) {
  this_ptr->hideColumn(column);
}

int qt_widgets_c_QTreeView_indentation(const QTreeView* this_ptr) {
  return this_ptr->indentation();
}

void qt_widgets_c_QTreeView_indexAbove_to_output(const QTreeView* this_ptr, const QModelIndex* index, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexAbove(*index));
}

void qt_widgets_c_QTreeView_indexAt_to_output(const QTreeView* this_ptr, const QPoint* p, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexAt(*p));
}

void qt_widgets_c_QTreeView_indexBelow_to_output(const QTreeView* this_ptr, const QModelIndex* index, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexBelow(*index));
}

bool qt_widgets_c_QTreeView_isAnimated(const QTreeView* this_ptr) {
  return this_ptr->isAnimated();
}

bool qt_widgets_c_QTreeView_isColumnHidden(const QTreeView* this_ptr, int column) {
  return this_ptr->isColumnHidden(column);
}

bool qt_widgets_c_QTreeView_isExpanded(const QTreeView* this_ptr, const QModelIndex* index) {
  return this_ptr->isExpanded(*index);
}

bool qt_widgets_c_QTreeView_isFirstColumnSpanned(const QTreeView* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->isFirstColumnSpanned(row, *parent);
}

bool qt_widgets_c_QTreeView_isHeaderHidden(const QTreeView* this_ptr) {
  return this_ptr->isHeaderHidden();
}

bool qt_widgets_c_QTreeView_isRowHidden(const QTreeView* this_ptr, int row, const QModelIndex* parent) {
  return this_ptr->isRowHidden(row, *parent);
}

bool qt_widgets_c_QTreeView_isSortingEnabled(const QTreeView* this_ptr) {
  return this_ptr->isSortingEnabled();
}

bool qt_widgets_c_QTreeView_itemsExpandable(const QTreeView* this_ptr) {
  return this_ptr->itemsExpandable();
}

void qt_widgets_c_QTreeView_keyboardSearch(QTreeView* this_ptr, const QString* search) {
  this_ptr->keyboardSearch(*search);
}

const QMetaObject* qt_widgets_c_QTreeView_metaObject(const QTreeView* this_ptr) {
  return this_ptr->metaObject();
}

QTreeView* qt_widgets_c_QTreeView_new_no_args() {
  return new QTreeView();
}

QTreeView* qt_widgets_c_QTreeView_new_parent(QWidget* parent) {
  return new QTreeView(parent);
}

int qt_widgets_c_QTreeView_qt_metacall(QTreeView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTreeView_qt_metacast(QTreeView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTreeView_reset(QTreeView* this_ptr) {
  this_ptr->reset();
}

void qt_widgets_c_QTreeView_resetIndentation(QTreeView* this_ptr) {
  this_ptr->resetIndentation();
}

void qt_widgets_c_QTreeView_resizeColumnToContents(QTreeView* this_ptr, int column) {
  this_ptr->resizeColumnToContents(column);
}

bool qt_widgets_c_QTreeView_rootIsDecorated(const QTreeView* this_ptr) {
  return this_ptr->rootIsDecorated();
}

void qt_widgets_c_QTreeView_scrollTo_index(QTreeView* this_ptr, const QModelIndex* index) {
  this_ptr->scrollTo(*index);
}

void qt_widgets_c_QTreeView_scrollTo_index_hint(QTreeView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint) {
  this_ptr->scrollTo(*index, hint);
}

void qt_widgets_c_QTreeView_selectAll(QTreeView* this_ptr) {
  this_ptr->selectAll();
}

void qt_widgets_c_QTreeView_setAllColumnsShowFocus(QTreeView* this_ptr, bool enable) {
  this_ptr->setAllColumnsShowFocus(enable);
}

void qt_widgets_c_QTreeView_setAnimated(QTreeView* this_ptr, bool enable) {
  this_ptr->setAnimated(enable);
}

void qt_widgets_c_QTreeView_setAutoExpandDelay(QTreeView* this_ptr, int delay) {
  this_ptr->setAutoExpandDelay(delay);
}

void qt_widgets_c_QTreeView_setColumnHidden(QTreeView* this_ptr, int column, bool hide) {
  this_ptr->setColumnHidden(column, hide);
}

void qt_widgets_c_QTreeView_setColumnWidth(QTreeView* this_ptr, int column, int width) {
  this_ptr->setColumnWidth(column, width);
}

void qt_widgets_c_QTreeView_setExpanded(QTreeView* this_ptr, const QModelIndex* index, bool expand) {
  this_ptr->setExpanded(*index, expand);
}

void qt_widgets_c_QTreeView_setExpandsOnDoubleClick(QTreeView* this_ptr, bool enable) {
  this_ptr->setExpandsOnDoubleClick(enable);
}

void qt_widgets_c_QTreeView_setFirstColumnSpanned(QTreeView* this_ptr, int row, const QModelIndex* parent, bool span) {
  this_ptr->setFirstColumnSpanned(row, *parent, span);
}

void qt_widgets_c_QTreeView_setHeader(QTreeView* this_ptr, QHeaderView* header) {
  this_ptr->setHeader(header);
}

void qt_widgets_c_QTreeView_setHeaderHidden(QTreeView* this_ptr, bool hide) {
  this_ptr->setHeaderHidden(hide);
}

void qt_widgets_c_QTreeView_setIndentation(QTreeView* this_ptr, int i) {
  this_ptr->setIndentation(i);
}

void qt_widgets_c_QTreeView_setItemsExpandable(QTreeView* this_ptr, bool enable) {
  this_ptr->setItemsExpandable(enable);
}

void qt_widgets_c_QTreeView_setModel(QTreeView* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QTreeView_setRootIndex(QTreeView* this_ptr, const QModelIndex* index) {
  this_ptr->setRootIndex(*index);
}

void qt_widgets_c_QTreeView_setRootIsDecorated(QTreeView* this_ptr, bool show) {
  this_ptr->setRootIsDecorated(show);
}

void qt_widgets_c_QTreeView_setRowHidden(QTreeView* this_ptr, int row, const QModelIndex* parent, bool hide) {
  this_ptr->setRowHidden(row, *parent, hide);
}

void qt_widgets_c_QTreeView_setSelectionModel(QTreeView* this_ptr, QItemSelectionModel* selectionModel) {
  this_ptr->setSelectionModel(selectionModel);
}

void qt_widgets_c_QTreeView_setSortingEnabled(QTreeView* this_ptr, bool enable) {
  this_ptr->setSortingEnabled(enable);
}

void qt_widgets_c_QTreeView_setTreePosition(QTreeView* this_ptr, int logicalIndex) {
  this_ptr->setTreePosition(logicalIndex);
}

void qt_widgets_c_QTreeView_setUniformRowHeights(QTreeView* this_ptr, bool uniform) {
  this_ptr->setUniformRowHeights(uniform);
}

void qt_widgets_c_QTreeView_setWordWrap(QTreeView* this_ptr, bool on) {
  this_ptr->setWordWrap(on);
}

void qt_widgets_c_QTreeView_showColumn(QTreeView* this_ptr, int column) {
  this_ptr->showColumn(column);
}

void qt_widgets_c_QTreeView_sortByColumn_column(QTreeView* this_ptr, int column) {
  this_ptr->sortByColumn(column);
}

void qt_widgets_c_QTreeView_sortByColumn_column_order(QTreeView* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sortByColumn(column, *order);
}

void qt_widgets_c_QTreeView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTreeView::trUtf8(s, c, n));
}

void qt_widgets_c_QTreeView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTreeView::tr(s, c, n));
}

int qt_widgets_c_QTreeView_treePosition(const QTreeView* this_ptr) {
  return this_ptr->treePosition();
}

bool qt_widgets_c_QTreeView_uniformRowHeights(const QTreeView* this_ptr) {
  return this_ptr->uniformRowHeights();
}

void qt_widgets_c_QTreeView_visualRect_to_output(const QTreeView* this_ptr, const QModelIndex* index, QRect* output) {
  new(output) QRect(this_ptr->visualRect(*index));
}

bool qt_widgets_c_QTreeView_wordWrap(const QTreeView* this_ptr) {
  return this_ptr->wordWrap();
}

