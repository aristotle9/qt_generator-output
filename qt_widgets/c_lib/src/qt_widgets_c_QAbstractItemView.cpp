#include "qt_widgets_c_QAbstractItemView.h"

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_dynamic_cast_QAbstractItemView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractItemView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QAbstractItemView_G_static_cast_QAbstractScrollArea_ptr(QAbstractItemView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QAbstractItemView_G_static_cast_QFrame_ptr(QAbstractItemView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QAbstractItemView_G_static_cast_QObject_ptr(QAbstractItemView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QAbstractItemView_G_static_cast_QPaintDevice_ptr(QAbstractItemView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QAbstractItemView_G_static_cast_QWidget_ptr(QAbstractItemView* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QAbstractItemView_alternatingRowColors(const QAbstractItemView* this_ptr) {
  return this_ptr->alternatingRowColors();
}

int qt_widgets_c_QAbstractItemView_autoScrollMargin(const QAbstractItemView* this_ptr) {
  return this_ptr->autoScrollMargin();
}

void qt_widgets_c_QAbstractItemView_clearSelection(QAbstractItemView* this_ptr) {
  this_ptr->clearSelection();
}

void qt_widgets_c_QAbstractItemView_closePersistentEditor(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->closePersistentEditor(*index);
}

void qt_widgets_c_QAbstractItemView_currentIndex_to_output(const QAbstractItemView* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->currentIndex());
}

void qt_widgets_c_QAbstractItemView_delete(QAbstractItemView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QAbstractItemView_doItemsLayout(QAbstractItemView* this_ptr) {
  this_ptr->doItemsLayout();
}

QAbstractItemView::DragDropMode qt_widgets_c_QAbstractItemView_dragDropMode(const QAbstractItemView* this_ptr) {
  return this_ptr->dragDropMode();
}

bool qt_widgets_c_QAbstractItemView_dragDropOverwriteMode(const QAbstractItemView* this_ptr) {
  return this_ptr->dragDropOverwriteMode();
}

bool qt_widgets_c_QAbstractItemView_dragEnabled(const QAbstractItemView* this_ptr) {
  return this_ptr->dragEnabled();
}

void qt_widgets_c_QAbstractItemView_edit(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->edit(*index);
}

unsigned int qt_widgets_c_QAbstractItemView_editTriggers(const QAbstractItemView* this_ptr) {
  return uint(this_ptr->editTriggers());
}

bool qt_widgets_c_QAbstractItemView_hasAutoScroll(const QAbstractItemView* this_ptr) {
  return this_ptr->hasAutoScroll();
}

QAbstractItemView::ScrollMode qt_widgets_c_QAbstractItemView_horizontalScrollMode(const QAbstractItemView* this_ptr) {
  return this_ptr->horizontalScrollMode();
}

void qt_widgets_c_QAbstractItemView_iconSize_to_output(const QAbstractItemView* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->iconSize());
}

void qt_widgets_c_QAbstractItemView_indexAt_to_output(const QAbstractItemView* this_ptr, const QPoint* point, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexAt(*point));
}

QWidget* qt_widgets_c_QAbstractItemView_indexWidget(const QAbstractItemView* this_ptr, const QModelIndex* index) {
  return this_ptr->indexWidget(*index);
}

void qt_widgets_c_QAbstractItemView_inputMethodQuery_to_output(const QAbstractItemView* this_ptr, const Qt::InputMethodQuery* query, QVariant* output) {
  new(output) QVariant(this_ptr->inputMethodQuery(*query));
}

QAbstractItemDelegate* qt_widgets_c_QAbstractItemView_itemDelegateForColumn(const QAbstractItemView* this_ptr, int column) {
  return this_ptr->itemDelegateForColumn(column);
}

QAbstractItemDelegate* qt_widgets_c_QAbstractItemView_itemDelegateForRow(const QAbstractItemView* this_ptr, int row) {
  return this_ptr->itemDelegateForRow(row);
}

QAbstractItemDelegate* qt_widgets_c_QAbstractItemView_itemDelegate_index(const QAbstractItemView* this_ptr, const QModelIndex* index) {
  return this_ptr->itemDelegate(*index);
}

QAbstractItemDelegate* qt_widgets_c_QAbstractItemView_itemDelegate_no_args(const QAbstractItemView* this_ptr) {
  return this_ptr->itemDelegate();
}

void qt_widgets_c_QAbstractItemView_keyboardSearch(QAbstractItemView* this_ptr, const QString* search) {
  this_ptr->keyboardSearch(*search);
}

const QMetaObject* qt_widgets_c_QAbstractItemView_metaObject(const QAbstractItemView* this_ptr) {
  return this_ptr->metaObject();
}

QAbstractItemModel* qt_widgets_c_QAbstractItemView_model(const QAbstractItemView* this_ptr) {
  return this_ptr->model();
}

void qt_widgets_c_QAbstractItemView_openPersistentEditor(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->openPersistentEditor(*index);
}

int qt_widgets_c_QAbstractItemView_qt_metacall(QAbstractItemView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAbstractItemView_qt_metacast(QAbstractItemView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAbstractItemView_reset(QAbstractItemView* this_ptr) {
  this_ptr->reset();
}

void qt_widgets_c_QAbstractItemView_resetHorizontalScrollMode(QAbstractItemView* this_ptr) {
  this_ptr->resetHorizontalScrollMode();
}

void qt_widgets_c_QAbstractItemView_resetVerticalScrollMode(QAbstractItemView* this_ptr) {
  this_ptr->resetVerticalScrollMode();
}

void qt_widgets_c_QAbstractItemView_rootIndex_to_output(const QAbstractItemView* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->rootIndex());
}

void qt_widgets_c_QAbstractItemView_scrollToBottom(QAbstractItemView* this_ptr) {
  this_ptr->scrollToBottom();
}

void qt_widgets_c_QAbstractItemView_scrollToTop(QAbstractItemView* this_ptr) {
  this_ptr->scrollToTop();
}

void qt_widgets_c_QAbstractItemView_scrollTo_index(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->scrollTo(*index);
}

void qt_widgets_c_QAbstractItemView_scrollTo_index_hint(QAbstractItemView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint) {
  this_ptr->scrollTo(*index, hint);
}

void qt_widgets_c_QAbstractItemView_selectAll(QAbstractItemView* this_ptr) {
  this_ptr->selectAll();
}

QItemSelectionModel* qt_widgets_c_QAbstractItemView_selectionModel(const QAbstractItemView* this_ptr) {
  return this_ptr->selectionModel();
}

void qt_widgets_c_QAbstractItemView_setAlternatingRowColors(QAbstractItemView* this_ptr, bool enable) {
  this_ptr->setAlternatingRowColors(enable);
}

void qt_widgets_c_QAbstractItemView_setAutoScroll(QAbstractItemView* this_ptr, bool enable) {
  this_ptr->setAutoScroll(enable);
}

void qt_widgets_c_QAbstractItemView_setAutoScrollMargin(QAbstractItemView* this_ptr, int margin) {
  this_ptr->setAutoScrollMargin(margin);
}

void qt_widgets_c_QAbstractItemView_setCurrentIndex(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->setCurrentIndex(*index);
}

void qt_widgets_c_QAbstractItemView_setDefaultDropAction(QAbstractItemView* this_ptr, const Qt::DropAction* dropAction) {
  this_ptr->setDefaultDropAction(*dropAction);
}

void qt_widgets_c_QAbstractItemView_setDragDropMode(QAbstractItemView* this_ptr, QAbstractItemView::DragDropMode behavior) {
  this_ptr->setDragDropMode(behavior);
}

void qt_widgets_c_QAbstractItemView_setDragDropOverwriteMode(QAbstractItemView* this_ptr, bool overwrite) {
  this_ptr->setDragDropOverwriteMode(overwrite);
}

void qt_widgets_c_QAbstractItemView_setDragEnabled(QAbstractItemView* this_ptr, bool enable) {
  this_ptr->setDragEnabled(enable);
}

void qt_widgets_c_QAbstractItemView_setDropIndicatorShown(QAbstractItemView* this_ptr, bool enable) {
  this_ptr->setDropIndicatorShown(enable);
}

void qt_widgets_c_QAbstractItemView_setEditTriggers(QAbstractItemView* this_ptr, unsigned int triggers) {
  this_ptr->setEditTriggers(QFlags< QAbstractItemView::EditTrigger >(triggers));
}

void qt_widgets_c_QAbstractItemView_setHorizontalScrollMode(QAbstractItemView* this_ptr, QAbstractItemView::ScrollMode mode) {
  this_ptr->setHorizontalScrollMode(mode);
}

void qt_widgets_c_QAbstractItemView_setIconSize(QAbstractItemView* this_ptr, const QSize* size) {
  this_ptr->setIconSize(*size);
}

void qt_widgets_c_QAbstractItemView_setIndexWidget(QAbstractItemView* this_ptr, const QModelIndex* index, QWidget* widget) {
  this_ptr->setIndexWidget(*index, widget);
}

void qt_widgets_c_QAbstractItemView_setItemDelegate(QAbstractItemView* this_ptr, QAbstractItemDelegate* delegate) {
  this_ptr->setItemDelegate(delegate);
}

void qt_widgets_c_QAbstractItemView_setItemDelegateForColumn(QAbstractItemView* this_ptr, int column, QAbstractItemDelegate* delegate) {
  this_ptr->setItemDelegateForColumn(column, delegate);
}

void qt_widgets_c_QAbstractItemView_setItemDelegateForRow(QAbstractItemView* this_ptr, int row, QAbstractItemDelegate* delegate) {
  this_ptr->setItemDelegateForRow(row, delegate);
}

void qt_widgets_c_QAbstractItemView_setModel(QAbstractItemView* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QAbstractItemView_setRootIndex(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->setRootIndex(*index);
}

void qt_widgets_c_QAbstractItemView_setSelectionBehavior(QAbstractItemView* this_ptr, const QAbstractItemView::SelectionBehavior* behavior) {
  this_ptr->setSelectionBehavior(*behavior);
}

void qt_widgets_c_QAbstractItemView_setSelectionMode(QAbstractItemView* this_ptr, const QAbstractItemView::SelectionMode* mode) {
  this_ptr->setSelectionMode(*mode);
}

void qt_widgets_c_QAbstractItemView_setSelectionModel(QAbstractItemView* this_ptr, QItemSelectionModel* selectionModel) {
  this_ptr->setSelectionModel(selectionModel);
}

void qt_widgets_c_QAbstractItemView_setTabKeyNavigation(QAbstractItemView* this_ptr, bool enable) {
  this_ptr->setTabKeyNavigation(enable);
}

void qt_widgets_c_QAbstractItemView_setTextElideMode(QAbstractItemView* this_ptr, const Qt::TextElideMode* mode) {
  this_ptr->setTextElideMode(*mode);
}

void qt_widgets_c_QAbstractItemView_setVerticalScrollMode(QAbstractItemView* this_ptr, QAbstractItemView::ScrollMode mode) {
  this_ptr->setVerticalScrollMode(mode);
}

bool qt_widgets_c_QAbstractItemView_showDropIndicator(const QAbstractItemView* this_ptr) {
  return this_ptr->showDropIndicator();
}

int qt_widgets_c_QAbstractItemView_sizeHintForColumn(const QAbstractItemView* this_ptr, int column) {
  return this_ptr->sizeHintForColumn(column);
}

void qt_widgets_c_QAbstractItemView_sizeHintForIndex_to_output(const QAbstractItemView* this_ptr, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->sizeHintForIndex(*index));
}

int qt_widgets_c_QAbstractItemView_sizeHintForRow(const QAbstractItemView* this_ptr, int row) {
  return this_ptr->sizeHintForRow(row);
}

bool qt_widgets_c_QAbstractItemView_tabKeyNavigation(const QAbstractItemView* this_ptr) {
  return this_ptr->tabKeyNavigation();
}

void qt_widgets_c_QAbstractItemView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractItemView::trUtf8(s, c, n));
}

void qt_widgets_c_QAbstractItemView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractItemView::tr(s, c, n));
}

void qt_widgets_c_QAbstractItemView_update(QAbstractItemView* this_ptr, const QModelIndex* index) {
  this_ptr->update(*index);
}

QAbstractItemView::ScrollMode qt_widgets_c_QAbstractItemView_verticalScrollMode(const QAbstractItemView* this_ptr) {
  return this_ptr->verticalScrollMode();
}

void qt_widgets_c_QAbstractItemView_visualRect_to_output(const QAbstractItemView* this_ptr, const QModelIndex* index, QRect* output) {
  new(output) QRect(this_ptr->visualRect(*index));
}

