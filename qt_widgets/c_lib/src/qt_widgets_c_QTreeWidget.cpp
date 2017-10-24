#include "qt_widgets_c_QTreeWidget.h"

QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QTreeView(QTreeView* ptr) {
  return dynamic_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTreeWidget*>(ptr);
}

QDataStream* qt_widgets_c_QTreeWidget_G_operator_shl(QDataStream* out, const QTreeWidgetItem* item) {
  return &operator<<(*out, *item);
}

QDataStream* qt_widgets_c_QTreeWidget_G_operator_shr(QDataStream* in, QTreeWidgetItem* item) {
  return &operator>>(*in, *item);
}

QAbstractItemView* qt_widgets_c_QTreeWidget_G_static_cast_QAbstractItemView_ptr(QTreeWidget* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QTreeWidget_G_static_cast_QAbstractScrollArea_ptr(QTreeWidget* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QTreeWidget_G_static_cast_QFrame_ptr(QTreeWidget* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QTreeWidget_G_static_cast_QObject_ptr(QTreeWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTreeWidget_G_static_cast_QPaintDevice_ptr(QTreeWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTreeView* qt_widgets_c_QTreeWidget_G_static_cast_QTreeView_ptr(QTreeWidget* ptr) {
  return static_cast<QTreeView*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QFrame(QFrame* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QTreeView(QTreeView* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QTreeWidget* qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTreeWidget*>(ptr);
}

QWidget* qt_widgets_c_QTreeWidget_G_static_cast_QWidget_ptr(QTreeWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QTreeWidget_addTopLevelItem(QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  this_ptr->addTopLevelItem(item);
}

void qt_widgets_c_QTreeWidget_addTopLevelItems(QTreeWidget* this_ptr, const QList< QTreeWidgetItem* >* items) {
  this_ptr->addTopLevelItems(*items);
}

void qt_widgets_c_QTreeWidget_clear(QTreeWidget* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QTreeWidget_closePersistentEditor_item(QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  this_ptr->closePersistentEditor(item);
}

void qt_widgets_c_QTreeWidget_closePersistentEditor_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column) {
  this_ptr->closePersistentEditor(item, column);
}

void qt_widgets_c_QTreeWidget_collapseItem(QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  this_ptr->collapseItem(item);
}

int qt_widgets_c_QTreeWidget_columnCount(const QTreeWidget* this_ptr) {
  return this_ptr->columnCount();
}

int qt_widgets_c_QTreeWidget_currentColumn(const QTreeWidget* this_ptr) {
  return this_ptr->currentColumn();
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_currentItem(const QTreeWidget* this_ptr) {
  return this_ptr->currentItem();
}

void qt_widgets_c_QTreeWidget_delete(QTreeWidget* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QTreeWidget_editItem_item(QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  this_ptr->editItem(item);
}

void qt_widgets_c_QTreeWidget_editItem_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column) {
  this_ptr->editItem(item, column);
}

void qt_widgets_c_QTreeWidget_expandItem(QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  this_ptr->expandItem(item);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_headerItem(const QTreeWidget* this_ptr) {
  return this_ptr->headerItem();
}

int qt_widgets_c_QTreeWidget_indexOfTopLevelItem(const QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  return this_ptr->indexOfTopLevelItem(item);
}

void qt_widgets_c_QTreeWidget_insertTopLevelItem(QTreeWidget* this_ptr, int index, QTreeWidgetItem* item) {
  this_ptr->insertTopLevelItem(index, item);
}

void qt_widgets_c_QTreeWidget_insertTopLevelItems(QTreeWidget* this_ptr, int index, const QList< QTreeWidgetItem* >* items) {
  this_ptr->insertTopLevelItems(index, *items);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_invisibleRootItem(const QTreeWidget* this_ptr) {
  return this_ptr->invisibleRootItem();
}

bool qt_widgets_c_QTreeWidget_isFirstItemColumnSpanned(const QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  return this_ptr->isFirstItemColumnSpanned(item);
}

bool qt_widgets_c_QTreeWidget_isItemExpanded(const QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  return this_ptr->isItemExpanded(item);
}

bool qt_widgets_c_QTreeWidget_isItemHidden(const QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  return this_ptr->isItemHidden(item);
}

bool qt_widgets_c_QTreeWidget_isItemSelected(const QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  return this_ptr->isItemSelected(item);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemAbove(const QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  return this_ptr->itemAbove(item);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemAt_p(const QTreeWidget* this_ptr, const QPoint* p) {
  return this_ptr->itemAt(*p);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemAt_x_y(const QTreeWidget* this_ptr, int x, int y) {
  return this_ptr->itemAt(x, y);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_itemBelow(const QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  return this_ptr->itemBelow(item);
}

QWidget* qt_widgets_c_QTreeWidget_itemWidget(const QTreeWidget* this_ptr, QTreeWidgetItem* item, int column) {
  return this_ptr->itemWidget(item, column);
}

const QMetaObject* qt_widgets_c_QTreeWidget_metaObject(const QTreeWidget* this_ptr) {
  return this_ptr->metaObject();
}

QTreeWidget* qt_widgets_c_QTreeWidget_new_no_args() {
  return new QTreeWidget();
}

QTreeWidget* qt_widgets_c_QTreeWidget_new_parent(QWidget* parent) {
  return new QTreeWidget(parent);
}

void qt_widgets_c_QTreeWidget_openPersistentEditor_item(QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  this_ptr->openPersistentEditor(item);
}

void qt_widgets_c_QTreeWidget_openPersistentEditor_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column) {
  this_ptr->openPersistentEditor(item, column);
}

int qt_widgets_c_QTreeWidget_qt_metacall(QTreeWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTreeWidget_qt_metacast(QTreeWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTreeWidget_removeItemWidget(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column) {
  this_ptr->removeItemWidget(item, column);
}

void qt_widgets_c_QTreeWidget_scrollToItem_item(QTreeWidget* this_ptr, const QTreeWidgetItem* item) {
  this_ptr->scrollToItem(item);
}

void qt_widgets_c_QTreeWidget_scrollToItem_item_hint(QTreeWidget* this_ptr, const QTreeWidgetItem* item, const QAbstractItemView::ScrollHint* hint) {
  this_ptr->scrollToItem(item, *hint);
}

void qt_widgets_c_QTreeWidget_selectedItems_to_output(const QTreeWidget* this_ptr, QList< QTreeWidgetItem* >* output) {
  new(output) QList< QTreeWidgetItem* >(this_ptr->selectedItems());
}

void qt_widgets_c_QTreeWidget_setColumnCount(QTreeWidget* this_ptr, int columns) {
  this_ptr->setColumnCount(columns);
}

void qt_widgets_c_QTreeWidget_setCurrentItem_item(QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  this_ptr->setCurrentItem(item);
}

void qt_widgets_c_QTreeWidget_setCurrentItem_item_column(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column) {
  this_ptr->setCurrentItem(item, column);
}

void qt_widgets_c_QTreeWidget_setFirstItemColumnSpanned(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool span) {
  this_ptr->setFirstItemColumnSpanned(item, span);
}

void qt_widgets_c_QTreeWidget_setHeaderItem(QTreeWidget* this_ptr, QTreeWidgetItem* item) {
  this_ptr->setHeaderItem(item);
}

void qt_widgets_c_QTreeWidget_setHeaderLabel(QTreeWidget* this_ptr, const QString* label) {
  this_ptr->setHeaderLabel(*label);
}

void qt_widgets_c_QTreeWidget_setHeaderLabels(QTreeWidget* this_ptr, const QStringList* labels) {
  this_ptr->setHeaderLabels(*labels);
}

void qt_widgets_c_QTreeWidget_setItemExpanded(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool expand) {
  this_ptr->setItemExpanded(item, expand);
}

void qt_widgets_c_QTreeWidget_setItemHidden(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool hide) {
  this_ptr->setItemHidden(item, hide);
}

void qt_widgets_c_QTreeWidget_setItemSelected(QTreeWidget* this_ptr, const QTreeWidgetItem* item, bool select) {
  this_ptr->setItemSelected(item, select);
}

void qt_widgets_c_QTreeWidget_setItemWidget(QTreeWidget* this_ptr, QTreeWidgetItem* item, int column, QWidget* widget) {
  this_ptr->setItemWidget(item, column, widget);
}

void qt_widgets_c_QTreeWidget_setSelectionModel(QTreeWidget* this_ptr, QItemSelectionModel* selectionModel) {
  this_ptr->setSelectionModel(selectionModel);
}

int qt_widgets_c_QTreeWidget_sortColumn(const QTreeWidget* this_ptr) {
  return this_ptr->sortColumn();
}

void qt_widgets_c_QTreeWidget_sortItems(QTreeWidget* this_ptr, int column, const Qt::SortOrder* order) {
  this_ptr->sortItems(column, *order);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_takeTopLevelItem(QTreeWidget* this_ptr, int index) {
  return this_ptr->takeTopLevelItem(index);
}

QTreeWidgetItem* qt_widgets_c_QTreeWidget_topLevelItem(const QTreeWidget* this_ptr, int index) {
  return this_ptr->topLevelItem(index);
}

int qt_widgets_c_QTreeWidget_topLevelItemCount(const QTreeWidget* this_ptr) {
  return this_ptr->topLevelItemCount();
}

void qt_widgets_c_QTreeWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTreeWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QTreeWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTreeWidget::tr(s, c, n));
}

void qt_widgets_c_QTreeWidget_visualItemRect_to_output(const QTreeWidget* this_ptr, const QTreeWidgetItem* item, QRect* output) {
  new(output) QRect(this_ptr->visualItemRect(item));
}

