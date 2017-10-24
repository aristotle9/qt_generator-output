#include "qt_widgets_c_QSplitter.h"

QSplitter* qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QSplitter*>(ptr);
}

QSplitter* qt_widgets_c_QSplitter_G_dynamic_cast_QSplitter_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QSplitter*>(ptr);
}

QTextStream* qt_widgets_c_QSplitter_G_operator_shl(QTextStream* arg1, const QSplitter* arg2) {
  return &operator<<(*arg1, *arg2);
}

QTextStream* qt_widgets_c_QSplitter_G_operator_shr(QTextStream* arg1, QSplitter* arg2) {
  return &operator>>(*arg1, *arg2);
}

QFrame* qt_widgets_c_QSplitter_G_static_cast_QFrame_ptr(QSplitter* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QSplitter_G_static_cast_QObject_ptr(QSplitter* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QSplitter_G_static_cast_QPaintDevice_ptr(QSplitter* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QFrame(QFrame* ptr) {
  return static_cast<QSplitter*>(ptr);
}

QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QObject(QObject* ptr) {
  return static_cast<QSplitter*>(ptr);
}

QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QSplitter*>(ptr);
}

QSplitter* qt_widgets_c_QSplitter_G_static_cast_QSplitter_ptr_QWidget(QWidget* ptr) {
  return static_cast<QSplitter*>(ptr);
}

QWidget* qt_widgets_c_QSplitter_G_static_cast_QWidget_ptr(QSplitter* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QSplitter_addWidget(QSplitter* this_ptr, QWidget* widget) {
  this_ptr->addWidget(widget);
}

bool qt_widgets_c_QSplitter_childrenCollapsible(const QSplitter* this_ptr) {
  return this_ptr->childrenCollapsible();
}

int qt_widgets_c_QSplitter_count(const QSplitter* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QSplitter_delete(QSplitter* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QSplitter_getRange(const QSplitter* this_ptr, int index, int* arg2, int* arg3) {
  this_ptr->getRange(index, arg2, arg3);
}

QSplitterHandle* qt_widgets_c_QSplitter_handle(const QSplitter* this_ptr, int index) {
  return this_ptr->handle(index);
}

int qt_widgets_c_QSplitter_handleWidth(const QSplitter* this_ptr) {
  return this_ptr->handleWidth();
}

int qt_widgets_c_QSplitter_indexOf(const QSplitter* this_ptr, QWidget* w) {
  return this_ptr->indexOf(w);
}

void qt_widgets_c_QSplitter_insertWidget(QSplitter* this_ptr, int index, QWidget* widget) {
  this_ptr->insertWidget(index, widget);
}

bool qt_widgets_c_QSplitter_isCollapsible(const QSplitter* this_ptr, int index) {
  return this_ptr->isCollapsible(index);
}

const QMetaObject* qt_widgets_c_QSplitter_metaObject(const QSplitter* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QSplitter_minimumSizeHint_to_output(const QSplitter* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QSplitter* qt_widgets_c_QSplitter_new_arg1(const Qt::Orientation* arg1) {
  return new QSplitter(*arg1);
}

QSplitter* qt_widgets_c_QSplitter_new_arg1_parent(const Qt::Orientation* arg1, QWidget* parent) {
  return new QSplitter(*arg1, parent);
}

QSplitter* qt_widgets_c_QSplitter_new_no_args() {
  return new QSplitter();
}

QSplitter* qt_widgets_c_QSplitter_new_parent(QWidget* parent) {
  return new QSplitter(parent);
}

bool qt_widgets_c_QSplitter_opaqueResize(const QSplitter* this_ptr) {
  return this_ptr->opaqueResize();
}

int qt_widgets_c_QSplitter_qt_metacall(QSplitter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSplitter_qt_metacast(QSplitter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSplitter_refresh(QSplitter* this_ptr) {
  this_ptr->refresh();
}

QWidget* qt_widgets_c_QSplitter_replaceWidget(QSplitter* this_ptr, int index, QWidget* widget) {
  return this_ptr->replaceWidget(index, widget);
}

bool qt_widgets_c_QSplitter_restoreState(QSplitter* this_ptr, const QByteArray* state) {
  return this_ptr->restoreState(*state);
}

void qt_widgets_c_QSplitter_saveState_to_output(const QSplitter* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->saveState());
}

void qt_widgets_c_QSplitter_setChildrenCollapsible(QSplitter* this_ptr, bool arg1) {
  this_ptr->setChildrenCollapsible(arg1);
}

void qt_widgets_c_QSplitter_setCollapsible(QSplitter* this_ptr, int index, bool arg2) {
  this_ptr->setCollapsible(index, arg2);
}

void qt_widgets_c_QSplitter_setHandleWidth(QSplitter* this_ptr, int arg1) {
  this_ptr->setHandleWidth(arg1);
}

void qt_widgets_c_QSplitter_setOpaqueResize_no_args(QSplitter* this_ptr) {
  this_ptr->setOpaqueResize();
}

void qt_widgets_c_QSplitter_setOpaqueResize_opaque(QSplitter* this_ptr, bool opaque) {
  this_ptr->setOpaqueResize(opaque);
}

void qt_widgets_c_QSplitter_setOrientation(QSplitter* this_ptr, const Qt::Orientation* arg1) {
  this_ptr->setOrientation(*arg1);
}

void qt_widgets_c_QSplitter_setSizes(QSplitter* this_ptr, const QList< int >* list) {
  this_ptr->setSizes(*list);
}

void qt_widgets_c_QSplitter_setStretchFactor(QSplitter* this_ptr, int index, int stretch) {
  this_ptr->setStretchFactor(index, stretch);
}

void qt_widgets_c_QSplitter_sizeHint_to_output(const QSplitter* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QSplitter_sizes_to_output(const QSplitter* this_ptr, QList< int >* output) {
  new(output) QList< int >(this_ptr->sizes());
}

void qt_widgets_c_QSplitter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSplitter::trUtf8(s, c, n));
}

void qt_widgets_c_QSplitter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSplitter::tr(s, c, n));
}

QWidget* qt_widgets_c_QSplitter_widget(const QSplitter* this_ptr, int index) {
  return this_ptr->widget(index);
}

