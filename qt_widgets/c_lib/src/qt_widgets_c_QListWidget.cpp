#include "qt_widgets_c_QListWidget.h"

QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QListView(QListView* ptr) {
  return dynamic_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QListWidget*>(ptr);
}

QDataStream* qt_widgets_c_QListWidget_G_operator_shl(QDataStream* out, const QListWidgetItem* item) {
  return &operator<<(*out, *item);
}

QDataStream* qt_widgets_c_QListWidget_G_operator_shr(QDataStream* in, QListWidgetItem* item) {
  return &operator>>(*in, *item);
}

QAbstractItemView* qt_widgets_c_QListWidget_G_static_cast_QAbstractItemView_ptr(QListWidget* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QListWidget_G_static_cast_QAbstractScrollArea_ptr(QListWidget* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QListWidget_G_static_cast_QFrame_ptr(QListWidget* ptr) {
  return static_cast<QFrame*>(ptr);
}

QListView* qt_widgets_c_QListWidget_G_static_cast_QListView_ptr(QListWidget* ptr) {
  return static_cast<QListView*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QFrame(QFrame* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QListView(QListView* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QListWidget* qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QListWidget*>(ptr);
}

QObject* qt_widgets_c_QListWidget_G_static_cast_QObject_ptr(QListWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QListWidget_G_static_cast_QPaintDevice_ptr(QListWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QListWidget_G_static_cast_QWidget_ptr(QListWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QListWidget_addItem_item(QListWidget* this_ptr, QListWidgetItem* item) {
  this_ptr->addItem(item);
}

void qt_widgets_c_QListWidget_addItem_label(QListWidget* this_ptr, const QString* label) {
  this_ptr->addItem(*label);
}

void qt_widgets_c_QListWidget_addItems(QListWidget* this_ptr, const QStringList* labels) {
  this_ptr->addItems(*labels);
}

void qt_widgets_c_QListWidget_clear(QListWidget* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QListWidget_closePersistentEditor(QListWidget* this_ptr, QListWidgetItem* item) {
  this_ptr->closePersistentEditor(item);
}

int qt_widgets_c_QListWidget_count(const QListWidget* this_ptr) {
  return this_ptr->count();
}

QListWidgetItem* qt_widgets_c_QListWidget_currentItem(const QListWidget* this_ptr) {
  return this_ptr->currentItem();
}

int qt_widgets_c_QListWidget_currentRow(const QListWidget* this_ptr) {
  return this_ptr->currentRow();
}

void qt_widgets_c_QListWidget_delete(QListWidget* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QListWidget_dropEvent(QListWidget* this_ptr, QDropEvent* event) {
  this_ptr->dropEvent(event);
}

void qt_widgets_c_QListWidget_editItem(QListWidget* this_ptr, QListWidgetItem* item) {
  this_ptr->editItem(item);
}

void qt_widgets_c_QListWidget_insertItem_row_item(QListWidget* this_ptr, int row, QListWidgetItem* item) {
  this_ptr->insertItem(row, item);
}

void qt_widgets_c_QListWidget_insertItem_row_label(QListWidget* this_ptr, int row, const QString* label) {
  this_ptr->insertItem(row, *label);
}

void qt_widgets_c_QListWidget_insertItems(QListWidget* this_ptr, int row, const QStringList* labels) {
  this_ptr->insertItems(row, *labels);
}

bool qt_widgets_c_QListWidget_isItemHidden(const QListWidget* this_ptr, const QListWidgetItem* item) {
  return this_ptr->isItemHidden(item);
}

bool qt_widgets_c_QListWidget_isItemSelected(const QListWidget* this_ptr, const QListWidgetItem* item) {
  return this_ptr->isItemSelected(item);
}

bool qt_widgets_c_QListWidget_isSortingEnabled(const QListWidget* this_ptr) {
  return this_ptr->isSortingEnabled();
}

QListWidgetItem* qt_widgets_c_QListWidget_item(const QListWidget* this_ptr, int row) {
  return this_ptr->item(row);
}

QListWidgetItem* qt_widgets_c_QListWidget_itemAt_p(const QListWidget* this_ptr, const QPoint* p) {
  return this_ptr->itemAt(*p);
}

QListWidgetItem* qt_widgets_c_QListWidget_itemAt_x_y(const QListWidget* this_ptr, int x, int y) {
  return this_ptr->itemAt(x, y);
}

QWidget* qt_widgets_c_QListWidget_itemWidget(const QListWidget* this_ptr, QListWidgetItem* item) {
  return this_ptr->itemWidget(item);
}

const QMetaObject* qt_widgets_c_QListWidget_metaObject(const QListWidget* this_ptr) {
  return this_ptr->metaObject();
}

QListWidget* qt_widgets_c_QListWidget_new_no_args() {
  return new QListWidget();
}

QListWidget* qt_widgets_c_QListWidget_new_parent(QWidget* parent) {
  return new QListWidget(parent);
}

void qt_widgets_c_QListWidget_openPersistentEditor(QListWidget* this_ptr, QListWidgetItem* item) {
  this_ptr->openPersistentEditor(item);
}

int qt_widgets_c_QListWidget_qt_metacall(QListWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QListWidget_qt_metacast(QListWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QListWidget_removeItemWidget(QListWidget* this_ptr, QListWidgetItem* item) {
  this_ptr->removeItemWidget(item);
}

int qt_widgets_c_QListWidget_row(const QListWidget* this_ptr, const QListWidgetItem* item) {
  return this_ptr->row(item);
}

void qt_widgets_c_QListWidget_scrollToItem_item(QListWidget* this_ptr, const QListWidgetItem* item) {
  this_ptr->scrollToItem(item);
}

void qt_widgets_c_QListWidget_scrollToItem_item_hint(QListWidget* this_ptr, const QListWidgetItem* item, const QAbstractItemView::ScrollHint* hint) {
  this_ptr->scrollToItem(item, *hint);
}

void qt_widgets_c_QListWidget_selectedItems_to_output(const QListWidget* this_ptr, QList< QListWidgetItem* >* output) {
  new(output) QList< QListWidgetItem* >(this_ptr->selectedItems());
}

void qt_widgets_c_QListWidget_setCurrentItem(QListWidget* this_ptr, QListWidgetItem* item) {
  this_ptr->setCurrentItem(item);
}

void qt_widgets_c_QListWidget_setCurrentRow(QListWidget* this_ptr, int row) {
  this_ptr->setCurrentRow(row);
}

void qt_widgets_c_QListWidget_setItemHidden(QListWidget* this_ptr, const QListWidgetItem* item, bool hide) {
  this_ptr->setItemHidden(item, hide);
}

void qt_widgets_c_QListWidget_setItemSelected(QListWidget* this_ptr, const QListWidgetItem* item, bool select) {
  this_ptr->setItemSelected(item, select);
}

void qt_widgets_c_QListWidget_setItemWidget(QListWidget* this_ptr, QListWidgetItem* item, QWidget* widget) {
  this_ptr->setItemWidget(item, widget);
}

void qt_widgets_c_QListWidget_setSelectionModel(QListWidget* this_ptr, QItemSelectionModel* selectionModel) {
  this_ptr->setSelectionModel(selectionModel);
}

void qt_widgets_c_QListWidget_setSortingEnabled(QListWidget* this_ptr, bool enable) {
  this_ptr->setSortingEnabled(enable);
}

void qt_widgets_c_QListWidget_sortItems_no_args(QListWidget* this_ptr) {
  this_ptr->sortItems();
}

void qt_widgets_c_QListWidget_sortItems_order(QListWidget* this_ptr, const Qt::SortOrder* order) {
  this_ptr->sortItems(*order);
}

QListWidgetItem* qt_widgets_c_QListWidget_takeItem(QListWidget* this_ptr, int row) {
  return this_ptr->takeItem(row);
}

void qt_widgets_c_QListWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QListWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QListWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QListWidget::tr(s, c, n));
}

void qt_widgets_c_QListWidget_visualItemRect_to_output(const QListWidget* this_ptr, const QListWidgetItem* item, QRect* output) {
  new(output) QRect(this_ptr->visualItemRect(item));
}

