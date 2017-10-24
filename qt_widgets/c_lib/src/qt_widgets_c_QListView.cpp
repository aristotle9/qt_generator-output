#include "qt_widgets_c_QListView.h"

QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_dynamic_cast_QListView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QListView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QListView_G_static_cast_QAbstractItemView_ptr(QListView* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QListView_G_static_cast_QAbstractScrollArea_ptr(QListView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QListView_G_static_cast_QFrame_ptr(QListView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QObject(QObject* ptr) {
  return static_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QListView*>(ptr);
}

QListView* qt_widgets_c_QListView_G_static_cast_QListView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QListView*>(ptr);
}

QObject* qt_widgets_c_QListView_G_static_cast_QObject_ptr(QListView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QListView_G_static_cast_QPaintDevice_ptr(QListView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QListView_G_static_cast_QWidget_ptr(QListView* ptr) {
  return static_cast<QWidget*>(ptr);
}

int qt_widgets_c_QListView_batchSize(const QListView* this_ptr) {
  return this_ptr->batchSize();
}

void qt_widgets_c_QListView_clearPropertyFlags(QListView* this_ptr) {
  this_ptr->clearPropertyFlags();
}

void qt_widgets_c_QListView_delete(QListView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QListView_doItemsLayout(QListView* this_ptr) {
  this_ptr->doItemsLayout();
}

QListView::Flow qt_widgets_c_QListView_flow(const QListView* this_ptr) {
  return this_ptr->flow();
}

void qt_widgets_c_QListView_gridSize_to_output(const QListView* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->gridSize());
}

void qt_widgets_c_QListView_indexAt_to_output(const QListView* this_ptr, const QPoint* p, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexAt(*p));
}

bool qt_widgets_c_QListView_isRowHidden(const QListView* this_ptr, int row) {
  return this_ptr->isRowHidden(row);
}

bool qt_widgets_c_QListView_isSelectionRectVisible(const QListView* this_ptr) {
  return this_ptr->isSelectionRectVisible();
}

bool qt_widgets_c_QListView_isWrapping(const QListView* this_ptr) {
  return this_ptr->isWrapping();
}

QListView::LayoutMode qt_widgets_c_QListView_layoutMode(const QListView* this_ptr) {
  return this_ptr->layoutMode();
}

const QMetaObject* qt_widgets_c_QListView_metaObject(const QListView* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QListView_modelColumn(const QListView* this_ptr) {
  return this_ptr->modelColumn();
}

QListView::Movement qt_widgets_c_QListView_movement(const QListView* this_ptr) {
  return this_ptr->movement();
}

QListView* qt_widgets_c_QListView_new_no_args() {
  return new QListView();
}

QListView* qt_widgets_c_QListView_new_parent(QWidget* parent) {
  return new QListView(parent);
}

int qt_widgets_c_QListView_qt_metacall(QListView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QListView_qt_metacast(QListView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QListView_reset(QListView* this_ptr) {
  this_ptr->reset();
}

QListView::ResizeMode qt_widgets_c_QListView_resizeMode(const QListView* this_ptr) {
  return this_ptr->resizeMode();
}

void qt_widgets_c_QListView_scrollTo_index(QListView* this_ptr, const QModelIndex* index) {
  this_ptr->scrollTo(*index);
}

void qt_widgets_c_QListView_scrollTo_index_hint(QListView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint) {
  this_ptr->scrollTo(*index, hint);
}

void qt_widgets_c_QListView_setBatchSize(QListView* this_ptr, int batchSize) {
  this_ptr->setBatchSize(batchSize);
}

void qt_widgets_c_QListView_setFlow(QListView* this_ptr, QListView::Flow flow) {
  this_ptr->setFlow(flow);
}

void qt_widgets_c_QListView_setGridSize(QListView* this_ptr, const QSize* size) {
  this_ptr->setGridSize(*size);
}

void qt_widgets_c_QListView_setLayoutMode(QListView* this_ptr, QListView::LayoutMode mode) {
  this_ptr->setLayoutMode(mode);
}

void qt_widgets_c_QListView_setModelColumn(QListView* this_ptr, int column) {
  this_ptr->setModelColumn(column);
}

void qt_widgets_c_QListView_setMovement(QListView* this_ptr, QListView::Movement movement) {
  this_ptr->setMovement(movement);
}

void qt_widgets_c_QListView_setResizeMode(QListView* this_ptr, QListView::ResizeMode mode) {
  this_ptr->setResizeMode(mode);
}

void qt_widgets_c_QListView_setRootIndex(QListView* this_ptr, const QModelIndex* index) {
  this_ptr->setRootIndex(*index);
}

void qt_widgets_c_QListView_setRowHidden(QListView* this_ptr, int row, bool hide) {
  this_ptr->setRowHidden(row, hide);
}

void qt_widgets_c_QListView_setSelectionRectVisible(QListView* this_ptr, bool show) {
  this_ptr->setSelectionRectVisible(show);
}

void qt_widgets_c_QListView_setSpacing(QListView* this_ptr, int space) {
  this_ptr->setSpacing(space);
}

void qt_widgets_c_QListView_setUniformItemSizes(QListView* this_ptr, bool enable) {
  this_ptr->setUniformItemSizes(enable);
}

void qt_widgets_c_QListView_setViewMode(QListView* this_ptr, QListView::ViewMode mode) {
  this_ptr->setViewMode(mode);
}

void qt_widgets_c_QListView_setWordWrap(QListView* this_ptr, bool on) {
  this_ptr->setWordWrap(on);
}

void qt_widgets_c_QListView_setWrapping(QListView* this_ptr, bool enable) {
  this_ptr->setWrapping(enable);
}

int qt_widgets_c_QListView_spacing(const QListView* this_ptr) {
  return this_ptr->spacing();
}

void qt_widgets_c_QListView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QListView::trUtf8(s, c, n));
}

void qt_widgets_c_QListView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QListView::tr(s, c, n));
}

bool qt_widgets_c_QListView_uniformItemSizes(const QListView* this_ptr) {
  return this_ptr->uniformItemSizes();
}

QListView::ViewMode qt_widgets_c_QListView_viewMode(const QListView* this_ptr) {
  return this_ptr->viewMode();
}

void qt_widgets_c_QListView_visualRect_to_output(const QListView* this_ptr, const QModelIndex* index, QRect* output) {
  new(output) QRect(this_ptr->visualRect(*index));
}

bool qt_widgets_c_QListView_wordWrap(const QListView* this_ptr) {
  return this_ptr->wordWrap();
}

