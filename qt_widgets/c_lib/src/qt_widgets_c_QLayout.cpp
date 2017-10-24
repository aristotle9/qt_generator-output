#include "qt_widgets_c_QLayout.h"

QLayout* qt_widgets_c_QLayout_G_dynamic_cast_QLayout_ptr(QLayoutItem* ptr) {
  return dynamic_cast<QLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QLayout_G_static_cast_QLayoutItem_ptr(QLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QLayout*>(ptr);
}

QLayout* qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QLayout_G_static_cast_QObject_ptr(QLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_widgets_c_QLayout_activate(QLayout* this_ptr) {
  return this_ptr->activate();
}

void qt_widgets_c_QLayout_addItem(QLayout* this_ptr, QLayoutItem* arg1) {
  this_ptr->addItem(arg1);
}

void qt_widgets_c_QLayout_addWidget(QLayout* this_ptr, QWidget* w) {
  this_ptr->addWidget(w);
}

void qt_widgets_c_QLayout_closestAcceptableSize_to_output(const QWidget* w, const QSize* s, QSize* output) {
  new(output) QSize(QLayout::closestAcceptableSize(w, *s));
}

void qt_widgets_c_QLayout_contentsMargins_to_output(const QLayout* this_ptr, QMargins* output) {
  new(output) QMargins(this_ptr->contentsMargins());
}

void qt_widgets_c_QLayout_contentsRect_to_output(const QLayout* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->contentsRect());
}

int qt_widgets_c_QLayout_count(const QLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QLayout_delete(QLayout* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QLayout_geometry_to_output(const QLayout* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

void qt_widgets_c_QLayout_getContentsMargins(const QLayout* this_ptr, int* left, int* top, int* right, int* bottom) {
  this_ptr->getContentsMargins(left, top, right, bottom);
}

int qt_widgets_c_QLayout_indexOf(const QLayout* this_ptr, QWidget* arg1) {
  return this_ptr->indexOf(arg1);
}

void qt_widgets_c_QLayout_invalidate(QLayout* this_ptr) {
  this_ptr->invalidate();
}

bool qt_widgets_c_QLayout_isEmpty(const QLayout* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_widgets_c_QLayout_isEnabled(const QLayout* this_ptr) {
  return this_ptr->isEnabled();
}

QLayoutItem* qt_widgets_c_QLayout_itemAt(const QLayout* this_ptr, int index) {
  return this_ptr->itemAt(index);
}

QLayout* qt_widgets_c_QLayout_layout(QLayout* this_ptr) {
  return this_ptr->layout();
}

int qt_widgets_c_QLayout_margin(const QLayout* this_ptr) {
  return this_ptr->margin();
}

void qt_widgets_c_QLayout_maximumSize_to_output(const QLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

QWidget* qt_widgets_c_QLayout_menuBar(const QLayout* this_ptr) {
  return this_ptr->menuBar();
}

const QMetaObject* qt_widgets_c_QLayout_metaObject(const QLayout* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QLayout_minimumSize_to_output(const QLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QWidget* qt_widgets_c_QLayout_parentWidget(const QLayout* this_ptr) {
  return this_ptr->parentWidget();
}

int qt_widgets_c_QLayout_qt_metacall(QLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QLayout_qt_metacast(QLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QLayout_removeItem(QLayout* this_ptr, QLayoutItem* arg1) {
  this_ptr->removeItem(arg1);
}

void qt_widgets_c_QLayout_removeWidget(QLayout* this_ptr, QWidget* w) {
  this_ptr->removeWidget(w);
}

void qt_widgets_c_QLayout_setContentsMargins_left_top_right_bottom(QLayout* this_ptr, int left, int top, int right, int bottom) {
  this_ptr->setContentsMargins(left, top, right, bottom);
}

void qt_widgets_c_QLayout_setContentsMargins_margins(QLayout* this_ptr, const QMargins* margins) {
  this_ptr->setContentsMargins(*margins);
}

void qt_widgets_c_QLayout_setEnabled(QLayout* this_ptr, bool arg1) {
  this_ptr->setEnabled(arg1);
}

void qt_widgets_c_QLayout_setGeometry(QLayout* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QLayout_setMargin(QLayout* this_ptr, int arg1) {
  this_ptr->setMargin(arg1);
}

void qt_widgets_c_QLayout_setMenuBar(QLayout* this_ptr, QWidget* w) {
  this_ptr->setMenuBar(w);
}

void qt_widgets_c_QLayout_setSizeConstraint(QLayout* this_ptr, QLayout::SizeConstraint arg1) {
  this_ptr->setSizeConstraint(arg1);
}

void qt_widgets_c_QLayout_setSpacing(QLayout* this_ptr, int arg1) {
  this_ptr->setSpacing(arg1);
}

QLayout::SizeConstraint qt_widgets_c_QLayout_sizeConstraint(const QLayout* this_ptr) {
  return this_ptr->sizeConstraint();
}

int qt_widgets_c_QLayout_spacing(const QLayout* this_ptr) {
  return this_ptr->spacing();
}

QLayoutItem* qt_widgets_c_QLayout_takeAt(QLayout* this_ptr, int index) {
  return this_ptr->takeAt(index);
}

int qt_widgets_c_QLayout_totalHeightForWidth(const QLayout* this_ptr, int w) {
  return this_ptr->totalHeightForWidth(w);
}

void qt_widgets_c_QLayout_totalMaximumSize_to_output(const QLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->totalMaximumSize());
}

void qt_widgets_c_QLayout_totalMinimumSize_to_output(const QLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->totalMinimumSize());
}

void qt_widgets_c_QLayout_totalSizeHint_to_output(const QLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->totalSizeHint());
}

void qt_widgets_c_QLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLayout::tr(s, c, n));
}

void qt_widgets_c_QLayout_update(QLayout* this_ptr) {
  this_ptr->update();
}

