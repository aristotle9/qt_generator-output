#include "qt_widgets_c_QStyledItemDelegate.h"

QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_G_dynamic_cast_QStyledItemDelegate_ptr(QAbstractItemDelegate* ptr) {
  return dynamic_cast<QStyledItemDelegate*>(ptr);
}

QAbstractItemDelegate* qt_widgets_c_QStyledItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(QStyledItemDelegate* ptr) {
  return static_cast<QAbstractItemDelegate*>(ptr);
}

QObject* qt_widgets_c_QStyledItemDelegate_G_static_cast_QObject_ptr(QStyledItemDelegate* ptr) {
  return static_cast<QObject*>(ptr);
}

QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_G_static_cast_QStyledItemDelegate_ptr_QAbstractItemDelegate(QAbstractItemDelegate* ptr) {
  return static_cast<QStyledItemDelegate*>(ptr);
}

QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_G_static_cast_QStyledItemDelegate_ptr_QObject(QObject* ptr) {
  return static_cast<QStyledItemDelegate*>(ptr);
}

QWidget* qt_widgets_c_QStyledItemDelegate_createEditor(const QStyledItemDelegate* this_ptr, QWidget* parent, const QStyleOptionViewItem* option, const QModelIndex* index) {
  return this_ptr->createEditor(parent, *option, *index);
}

void qt_widgets_c_QStyledItemDelegate_delete(QStyledItemDelegate* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QStyledItemDelegate_displayText_to_output(const QStyledItemDelegate* this_ptr, const QVariant* value, const QLocale* locale, QString* output) {
  new(output) QString(this_ptr->displayText(*value, *locale));
}

QItemEditorFactory* qt_widgets_c_QStyledItemDelegate_itemEditorFactory(const QStyledItemDelegate* this_ptr) {
  return this_ptr->itemEditorFactory();
}

const QMetaObject* qt_widgets_c_QStyledItemDelegate_metaObject(const QStyledItemDelegate* this_ptr) {
  return this_ptr->metaObject();
}

QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_new_no_args() {
  return new QStyledItemDelegate();
}

QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_new_parent(QObject* parent) {
  return new QStyledItemDelegate(parent);
}

void qt_widgets_c_QStyledItemDelegate_paint(const QStyledItemDelegate* this_ptr, QPainter* painter, const QStyleOptionViewItem* option, const QModelIndex* index) {
  this_ptr->paint(painter, *option, *index);
}

int qt_widgets_c_QStyledItemDelegate_qt_metacall(QStyledItemDelegate* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QStyledItemDelegate_qt_metacast(QStyledItemDelegate* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QStyledItemDelegate_setEditorData(const QStyledItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index) {
  this_ptr->setEditorData(editor, *index);
}

void qt_widgets_c_QStyledItemDelegate_setItemEditorFactory(QStyledItemDelegate* this_ptr, QItemEditorFactory* factory) {
  this_ptr->setItemEditorFactory(factory);
}

void qt_widgets_c_QStyledItemDelegate_setModelData(const QStyledItemDelegate* this_ptr, QWidget* editor, QAbstractItemModel* model, const QModelIndex* index) {
  this_ptr->setModelData(editor, model, *index);
}

void qt_widgets_c_QStyledItemDelegate_sizeHint_to_output(const QStyledItemDelegate* this_ptr, const QStyleOptionViewItem* option, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->sizeHint(*option, *index));
}

void qt_widgets_c_QStyledItemDelegate_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStyledItemDelegate::trUtf8(s, c, n));
}

void qt_widgets_c_QStyledItemDelegate_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStyledItemDelegate::tr(s, c, n));
}

void qt_widgets_c_QStyledItemDelegate_updateEditorGeometry(const QStyledItemDelegate* this_ptr, QWidget* editor, const QStyleOptionViewItem* option, const QModelIndex* index) {
  this_ptr->updateEditorGeometry(editor, *option, *index);
}

