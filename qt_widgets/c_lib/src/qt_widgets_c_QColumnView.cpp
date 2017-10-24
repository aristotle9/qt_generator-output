#include "qt_widgets_c_QColumnView.h"

QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_dynamic_cast_QColumnView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QColumnView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QColumnView_G_static_cast_QAbstractItemView_ptr(QColumnView* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QColumnView_G_static_cast_QAbstractScrollArea_ptr(QColumnView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QObject(QObject* ptr) {
  return static_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QColumnView*>(ptr);
}

QColumnView* qt_widgets_c_QColumnView_G_static_cast_QColumnView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QColumnView*>(ptr);
}

QFrame* qt_widgets_c_QColumnView_G_static_cast_QFrame_ptr(QColumnView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QColumnView_G_static_cast_QObject_ptr(QColumnView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QColumnView_G_static_cast_QPaintDevice_ptr(QColumnView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QColumnView_G_static_cast_QWidget_ptr(QColumnView* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QColumnView_columnWidths_to_output(const QColumnView* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->columnWidths());
}

void qt_widgets_c_QColumnView_delete(QColumnView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QColumnView_indexAt_to_output(const QColumnView* this_ptr, const QPoint* point, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->indexAt(*point));
}

const QMetaObject* qt_widgets_c_QColumnView_metaObject(const QColumnView* this_ptr) {
  return this_ptr->metaObject();
}

QColumnView* qt_widgets_c_QColumnView_new_no_args() {
  return new QColumnView();
}

QColumnView* qt_widgets_c_QColumnView_new_parent(QWidget* parent) {
  return new QColumnView(parent);
}

QWidget* qt_widgets_c_QColumnView_previewWidget(const QColumnView* this_ptr) {
  return this_ptr->previewWidget();
}

int qt_widgets_c_QColumnView_qt_metacall(QColumnView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QColumnView_qt_metacast(QColumnView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_widgets_c_QColumnView_resizeGripsVisible(const QColumnView* this_ptr) {
  return this_ptr->resizeGripsVisible();
}

void qt_widgets_c_QColumnView_scrollTo_index(QColumnView* this_ptr, const QModelIndex* index) {
  this_ptr->scrollTo(*index);
}

void qt_widgets_c_QColumnView_scrollTo_index_hint(QColumnView* this_ptr, const QModelIndex* index, QAbstractItemView::ScrollHint hint) {
  this_ptr->scrollTo(*index, hint);
}

void qt_widgets_c_QColumnView_selectAll(QColumnView* this_ptr) {
  this_ptr->selectAll();
}

void qt_widgets_c_QColumnView_setColumnWidths(QColumnView* this_ptr, const QList< int >* list) {
  this_ptr->setColumnWidths(*list);
}

void qt_widgets_c_QColumnView_setModel(QColumnView* this_ptr, QAbstractItemModel* model) {
  this_ptr->setModel(model);
}

void qt_widgets_c_QColumnView_setPreviewWidget(QColumnView* this_ptr, QWidget* widget) {
  this_ptr->setPreviewWidget(widget);
}

void qt_widgets_c_QColumnView_setResizeGripsVisible(QColumnView* this_ptr, bool visible) {
  this_ptr->setResizeGripsVisible(visible);
}

void qt_widgets_c_QColumnView_setRootIndex(QColumnView* this_ptr, const QModelIndex* index) {
  this_ptr->setRootIndex(*index);
}

void qt_widgets_c_QColumnView_setSelectionModel(QColumnView* this_ptr, QItemSelectionModel* selectionModel) {
  this_ptr->setSelectionModel(selectionModel);
}

void qt_widgets_c_QColumnView_sizeHint_to_output(const QColumnView* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QColumnView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QColumnView::trUtf8(s, c, n));
}

void qt_widgets_c_QColumnView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QColumnView::tr(s, c, n));
}

void qt_widgets_c_QColumnView_visualRect_to_output(const QColumnView* this_ptr, const QModelIndex* index, QRect* output) {
  new(output) QRect(this_ptr->visualRect(*index));
}

