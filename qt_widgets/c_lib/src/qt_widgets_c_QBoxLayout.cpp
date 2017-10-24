#include "qt_widgets_c_QBoxLayout.h"

QBoxLayout* qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayout(QLayout* ptr) {
  return dynamic_cast<QBoxLayout*>(ptr);
}

QBoxLayout* qt_widgets_c_QBoxLayout_G_dynamic_cast_QBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return dynamic_cast<QBoxLayout*>(ptr);
}

QBoxLayout* qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayout(QLayout* ptr) {
  return static_cast<QBoxLayout*>(ptr);
}

QBoxLayout* qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QBoxLayout*>(ptr);
}

QBoxLayout* qt_widgets_c_QBoxLayout_G_static_cast_QBoxLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QBoxLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QBoxLayout_G_static_cast_QLayoutItem_ptr(QBoxLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QBoxLayout_G_static_cast_QLayout_ptr(QBoxLayout* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QBoxLayout_G_static_cast_QObject_ptr(QBoxLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QBoxLayout_addItem(QBoxLayout* this_ptr, QLayoutItem* arg1) {
  this_ptr->addItem(arg1);
}

void qt_widgets_c_QBoxLayout_addLayout_layout(QBoxLayout* this_ptr, QLayout* layout) {
  this_ptr->addLayout(layout);
}

void qt_widgets_c_QBoxLayout_addLayout_layout_stretch(QBoxLayout* this_ptr, QLayout* layout, int stretch) {
  this_ptr->addLayout(layout, stretch);
}

void qt_widgets_c_QBoxLayout_addSpacerItem(QBoxLayout* this_ptr, QSpacerItem* spacerItem) {
  this_ptr->addSpacerItem(spacerItem);
}

void qt_widgets_c_QBoxLayout_addSpacing(QBoxLayout* this_ptr, int size) {
  this_ptr->addSpacing(size);
}

void qt_widgets_c_QBoxLayout_addStretch_no_args(QBoxLayout* this_ptr) {
  this_ptr->addStretch();
}

void qt_widgets_c_QBoxLayout_addStretch_stretch(QBoxLayout* this_ptr, int stretch) {
  this_ptr->addStretch(stretch);
}

void qt_widgets_c_QBoxLayout_addStrut(QBoxLayout* this_ptr, int arg1) {
  this_ptr->addStrut(arg1);
}

int qt_widgets_c_QBoxLayout_count(const QBoxLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QBoxLayout_delete(QBoxLayout* this_ptr) {
  delete this_ptr;
}

QBoxLayout::Direction qt_widgets_c_QBoxLayout_direction(const QBoxLayout* this_ptr) {
  return this_ptr->direction();
}

bool qt_widgets_c_QBoxLayout_hasHeightForWidth(const QBoxLayout* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QBoxLayout_heightForWidth(const QBoxLayout* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

void qt_widgets_c_QBoxLayout_insertItem(QBoxLayout* this_ptr, int index, QLayoutItem* arg2) {
  this_ptr->insertItem(index, arg2);
}

void qt_widgets_c_QBoxLayout_insertLayout_index_layout(QBoxLayout* this_ptr, int index, QLayout* layout) {
  this_ptr->insertLayout(index, layout);
}

void qt_widgets_c_QBoxLayout_insertLayout_index_layout_stretch(QBoxLayout* this_ptr, int index, QLayout* layout, int stretch) {
  this_ptr->insertLayout(index, layout, stretch);
}

void qt_widgets_c_QBoxLayout_insertSpacerItem(QBoxLayout* this_ptr, int index, QSpacerItem* spacerItem) {
  this_ptr->insertSpacerItem(index, spacerItem);
}

void qt_widgets_c_QBoxLayout_insertSpacing(QBoxLayout* this_ptr, int index, int size) {
  this_ptr->insertSpacing(index, size);
}

void qt_widgets_c_QBoxLayout_insertStretch_index(QBoxLayout* this_ptr, int index) {
  this_ptr->insertStretch(index);
}

void qt_widgets_c_QBoxLayout_insertStretch_index_stretch(QBoxLayout* this_ptr, int index, int stretch) {
  this_ptr->insertStretch(index, stretch);
}

void qt_widgets_c_QBoxLayout_invalidate(QBoxLayout* this_ptr) {
  this_ptr->invalidate();
}

QLayoutItem* qt_widgets_c_QBoxLayout_itemAt(const QBoxLayout* this_ptr, int arg1) {
  return this_ptr->itemAt(arg1);
}

void qt_widgets_c_QBoxLayout_maximumSize_to_output(const QBoxLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

const QMetaObject* qt_widgets_c_QBoxLayout_metaObject(const QBoxLayout* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QBoxLayout_minimumHeightForWidth(const QBoxLayout* this_ptr, int arg1) {
  return this_ptr->minimumHeightForWidth(arg1);
}

void qt_widgets_c_QBoxLayout_minimumSize_to_output(const QBoxLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QBoxLayout* qt_widgets_c_QBoxLayout_new_arg1(QBoxLayout::Direction arg1) {
  return new QBoxLayout(arg1);
}

QBoxLayout* qt_widgets_c_QBoxLayout_new_arg1_parent(QBoxLayout::Direction arg1, QWidget* parent) {
  return new QBoxLayout(arg1, parent);
}

int qt_widgets_c_QBoxLayout_qt_metacall(QBoxLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QBoxLayout_qt_metacast(QBoxLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QBoxLayout_setDirection(QBoxLayout* this_ptr, QBoxLayout::Direction arg1) {
  this_ptr->setDirection(arg1);
}

void qt_widgets_c_QBoxLayout_setGeometry(QBoxLayout* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QBoxLayout_setSpacing(QBoxLayout* this_ptr, int spacing) {
  this_ptr->setSpacing(spacing);
}

void qt_widgets_c_QBoxLayout_setStretch(QBoxLayout* this_ptr, int index, int stretch) {
  this_ptr->setStretch(index, stretch);
}

bool qt_widgets_c_QBoxLayout_setStretchFactor_l_stretch(QBoxLayout* this_ptr, QLayout* l, int stretch) {
  return this_ptr->setStretchFactor(l, stretch);
}

bool qt_widgets_c_QBoxLayout_setStretchFactor_w_stretch(QBoxLayout* this_ptr, QWidget* w, int stretch) {
  return this_ptr->setStretchFactor(w, stretch);
}

void qt_widgets_c_QBoxLayout_sizeHint_to_output(const QBoxLayout* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

int qt_widgets_c_QBoxLayout_spacing(const QBoxLayout* this_ptr) {
  return this_ptr->spacing();
}

int qt_widgets_c_QBoxLayout_stretch(const QBoxLayout* this_ptr, int index) {
  return this_ptr->stretch(index);
}

QLayoutItem* qt_widgets_c_QBoxLayout_takeAt(QBoxLayout* this_ptr, int arg1) {
  return this_ptr->takeAt(arg1);
}

void qt_widgets_c_QBoxLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QBoxLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QBoxLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QBoxLayout::tr(s, c, n));
}

