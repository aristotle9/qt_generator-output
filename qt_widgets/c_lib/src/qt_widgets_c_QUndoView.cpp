#include "qt_widgets_c_QUndoView.h"

QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return dynamic_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QListView(QListView* ptr) {
  return dynamic_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QUndoView*>(ptr);
}

QAbstractItemView* qt_widgets_c_QUndoView_G_static_cast_QAbstractItemView_ptr(QUndoView* ptr) {
  return static_cast<QAbstractItemView*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QUndoView_G_static_cast_QAbstractScrollArea_ptr(QUndoView* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QUndoView_G_static_cast_QFrame_ptr(QUndoView* ptr) {
  return static_cast<QFrame*>(ptr);
}

QListView* qt_widgets_c_QUndoView_G_static_cast_QListView_ptr(QUndoView* ptr) {
  return static_cast<QListView*>(ptr);
}

QObject* qt_widgets_c_QUndoView_G_static_cast_QObject_ptr(QUndoView* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QUndoView_G_static_cast_QPaintDevice_ptr(QUndoView* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractItemView(QAbstractItemView* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QFrame(QFrame* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QListView(QListView* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QObject(QObject* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QWidget(QWidget* ptr) {
  return static_cast<QUndoView*>(ptr);
}

QWidget* qt_widgets_c_QUndoView_G_static_cast_QWidget_ptr(QUndoView* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QUndoView_cleanIcon_to_output(const QUndoView* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->cleanIcon());
}

void qt_widgets_c_QUndoView_delete(QUndoView* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QUndoView_emptyLabel_to_output(const QUndoView* this_ptr, QString* output) {
  new(output) QString(this_ptr->emptyLabel());
}

QUndoGroup* qt_widgets_c_QUndoView_group(const QUndoView* this_ptr) {
  return this_ptr->group();
}

const QMetaObject* qt_widgets_c_QUndoView_metaObject(const QUndoView* this_ptr) {
  return this_ptr->metaObject();
}

QUndoView* qt_widgets_c_QUndoView_new_group(QUndoGroup* group) {
  return new QUndoView(group);
}

QUndoView* qt_widgets_c_QUndoView_new_group_parent(QUndoGroup* group, QWidget* parent) {
  return new QUndoView(group, parent);
}

QUndoView* qt_widgets_c_QUndoView_new_no_args() {
  return new QUndoView();
}

QUndoView* qt_widgets_c_QUndoView_new_parent(QWidget* parent) {
  return new QUndoView(parent);
}

QUndoView* qt_widgets_c_QUndoView_new_stack(QUndoStack* stack) {
  return new QUndoView(stack);
}

QUndoView* qt_widgets_c_QUndoView_new_stack_parent(QUndoStack* stack, QWidget* parent) {
  return new QUndoView(stack, parent);
}

int qt_widgets_c_QUndoView_qt_metacall(QUndoView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QUndoView_qt_metacast(QUndoView* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QUndoView_setCleanIcon(QUndoView* this_ptr, const QIcon* icon) {
  this_ptr->setCleanIcon(*icon);
}

void qt_widgets_c_QUndoView_setEmptyLabel(QUndoView* this_ptr, const QString* label) {
  this_ptr->setEmptyLabel(*label);
}

void qt_widgets_c_QUndoView_setGroup(QUndoView* this_ptr, QUndoGroup* group) {
  this_ptr->setGroup(group);
}

void qt_widgets_c_QUndoView_setStack(QUndoView* this_ptr, QUndoStack* stack) {
  this_ptr->setStack(stack);
}

QUndoStack* qt_widgets_c_QUndoView_stack(const QUndoView* this_ptr) {
  return this_ptr->stack();
}

void qt_widgets_c_QUndoView_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUndoView::trUtf8(s, c, n));
}

void qt_widgets_c_QUndoView_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QUndoView::tr(s, c, n));
}

