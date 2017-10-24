#include "qt_widgets_c_QStackedLayout.h"

QStackedLayout* qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayout(QLayout* ptr) {
  return dynamic_cast<QStackedLayout*>(ptr);
}

QStackedLayout* qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return dynamic_cast<QStackedLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QStackedLayout_G_static_cast_QLayoutItem_ptr(QStackedLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QStackedLayout_G_static_cast_QLayout_ptr(QStackedLayout* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QStackedLayout_G_static_cast_QObject_ptr(QStackedLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

QStackedLayout* qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayout(QLayout* ptr) {
  return static_cast<QStackedLayout*>(ptr);
}

QStackedLayout* qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QStackedLayout*>(ptr);
}

QStackedLayout* qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QStackedLayout*>(ptr);
}

void qt_widgets_c_QStackedLayout_addItem(QStackedLayout* this_ptr, QLayoutItem* item) {
  this_ptr->addItem(item);
}

int qt_widgets_c_QStackedLayout_addWidget(QStackedLayout* this_ptr, QWidget* w) {
  return this_ptr->addWidget(w);
}

int qt_widgets_c_QStackedLayout_count(const QStackedLayout* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QStackedLayout_currentIndex(const QStackedLayout* this_ptr) {
  return this_ptr->currentIndex();
}

QWidget* qt_widgets_c_QStackedLayout_currentWidget(const QStackedLayout* this_ptr) {
  return this_ptr->currentWidget();
}

void qt_widgets_c_QStackedLayout_delete(QStackedLayout* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QStackedLayout_hasHeightForWidth(const QStackedLayout* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QStackedLayout_heightForWidth(const QStackedLayout* this_ptr, int width) {
  return this_ptr->heightForWidth(width);
}

int qt_widgets_c_QStackedLayout_insertWidget(QStackedLayout* this_ptr, int index, QWidget* w) {
  return this_ptr->insertWidget(index, w);
}

QLayoutItem* qt_widgets_c_QStackedLayout_itemAt(const QStackedLayout* this_ptr, int arg1) {
  return this_ptr->itemAt(arg1);
}

const QMetaObject* qt_widgets_c_QStackedLayout_metaObject(const QStackedLayout* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QStackedLayout_minimumSize_to_output(const QStackedLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QStackedLayout* qt_widgets_c_QStackedLayout_new_no_args() {
  return new QStackedLayout();
}

QStackedLayout* qt_widgets_c_QStackedLayout_new_parent(QWidget* parent) {
  return new QStackedLayout(parent);
}

QStackedLayout* qt_widgets_c_QStackedLayout_new_parentLayout(QLayout* parentLayout) {
  return new QStackedLayout(parentLayout);
}

int qt_widgets_c_QStackedLayout_qt_metacall(QStackedLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QStackedLayout_qt_metacast(QStackedLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QStackedLayout_setCurrentIndex(QStackedLayout* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QStackedLayout_setCurrentWidget(QStackedLayout* this_ptr, QWidget* w) {
  this_ptr->setCurrentWidget(w);
}

void qt_widgets_c_QStackedLayout_setGeometry(QStackedLayout* this_ptr, const QRect* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QStackedLayout_setStackingMode(QStackedLayout* this_ptr, QStackedLayout::StackingMode stackingMode) {
  this_ptr->setStackingMode(stackingMode);
}

void qt_widgets_c_QStackedLayout_sizeHint_to_output(const QStackedLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QStackedLayout::StackingMode qt_widgets_c_QStackedLayout_stackingMode(const QStackedLayout* this_ptr) {
  return this_ptr->stackingMode();
}

QLayoutItem* qt_widgets_c_QStackedLayout_takeAt(QStackedLayout* this_ptr, int arg1) {
  return this_ptr->takeAt(arg1);
}

void qt_widgets_c_QStackedLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStackedLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QStackedLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStackedLayout::tr(s, c, n));
}

QWidget* qt_widgets_c_QStackedLayout_widget(const QStackedLayout* this_ptr, int arg1) {
  return this_ptr->widget(arg1);
}

