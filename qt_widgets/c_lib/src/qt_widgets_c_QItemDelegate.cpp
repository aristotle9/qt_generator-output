#include "qt_widgets_c_QItemDelegate.h"

QItemDelegate* qt_widgets_c_QItemDelegate_G_dynamic_cast_QItemDelegate_ptr(QAbstractItemDelegate* ptr) {
  return dynamic_cast<QItemDelegate*>(ptr);
}

QAbstractItemDelegate* qt_widgets_c_QItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(QItemDelegate* ptr) {
  return static_cast<QAbstractItemDelegate*>(ptr);
}

QItemDelegate* qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QAbstractItemDelegate(QAbstractItemDelegate* ptr) {
  return static_cast<QItemDelegate*>(ptr);
}

QItemDelegate* qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QObject(QObject* ptr) {
  return static_cast<QItemDelegate*>(ptr);
}

QObject* qt_widgets_c_QItemDelegate_G_static_cast_QObject_ptr(QItemDelegate* ptr) {
  return static_cast<QObject*>(ptr);
}

QWidget* qt_widgets_c_QItemDelegate_createEditor(const QItemDelegate* this_ptr, QWidget* parent, const QStyleOptionViewItem* option, const QModelIndex* index) {
  return this_ptr->createEditor(parent, *option, *index);
}

void qt_widgets_c_QItemDelegate_delete(QItemDelegate* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QItemDelegate_hasClipping(const QItemDelegate* this_ptr) {
  return this_ptr->hasClipping();
}

QItemEditorFactory* qt_widgets_c_QItemDelegate_itemEditorFactory(const QItemDelegate* this_ptr) {
  return this_ptr->itemEditorFactory();
}

const QMetaObject* qt_widgets_c_QItemDelegate_metaObject(const QItemDelegate* this_ptr) {
  return this_ptr->metaObject();
}

QItemDelegate* qt_widgets_c_QItemDelegate_new_no_args() {
  return new QItemDelegate();
}

QItemDelegate* qt_widgets_c_QItemDelegate_new_parent(QObject* parent) {
  return new QItemDelegate(parent);
}

void qt_widgets_c_QItemDelegate_paint(const QItemDelegate* this_ptr, QPainter* painter, const QStyleOptionViewItem* option, const QModelIndex* index) {
  this_ptr->paint(painter, *option, *index);
}

int qt_widgets_c_QItemDelegate_qt_metacall(QItemDelegate* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QItemDelegate_qt_metacast(QItemDelegate* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QItemDelegate_setClipping(QItemDelegate* this_ptr, bool clip) {
  this_ptr->setClipping(clip);
}

void qt_widgets_c_QItemDelegate_setEditorData(const QItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index) {
  this_ptr->setEditorData(editor, *index);
}

void qt_widgets_c_QItemDelegate_setItemEditorFactory(QItemDelegate* this_ptr, QItemEditorFactory* factory) {
  this_ptr->setItemEditorFactory(factory);
}

void qt_widgets_c_QItemDelegate_setModelData(const QItemDelegate* this_ptr, QWidget* editor, QAbstractItemModel* model, const QModelIndex* index) {
  this_ptr->setModelData(editor, model, *index);
}

void qt_widgets_c_QItemDelegate_sizeHint_to_output(const QItemDelegate* this_ptr, const QStyleOptionViewItem* option, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->sizeHint(*option, *index));
}

void qt_widgets_c_QItemDelegate_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QItemDelegate::trUtf8(s, c, n));
}

void qt_widgets_c_QItemDelegate_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QItemDelegate::tr(s, c, n));
}

void qt_widgets_c_QItemDelegate_updateEditorGeometry(const QItemDelegate* this_ptr, QWidget* editor, const QStyleOptionViewItem* option, const QModelIndex* index) {
  this_ptr->updateEditorGeometry(editor, *option, *index);
}

