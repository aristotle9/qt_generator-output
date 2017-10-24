#include "qt_widgets_c_QAbstractItemDelegate.h"

QAbstractItemDelegate* qt_widgets_c_QAbstractItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(QObject* ptr) {
  return static_cast<QAbstractItemDelegate*>(ptr);
}

QObject* qt_widgets_c_QAbstractItemDelegate_G_static_cast_QObject_ptr(QAbstractItemDelegate* ptr) {
  return static_cast<QObject*>(ptr);
}

QWidget* qt_widgets_c_QAbstractItemDelegate_createEditor(const QAbstractItemDelegate* this_ptr, QWidget* parent, const QStyleOptionViewItem* option, const QModelIndex* index) {
  return this_ptr->createEditor(parent, *option, *index);
}

void qt_widgets_c_QAbstractItemDelegate_delete(QAbstractItemDelegate* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QAbstractItemDelegate_destroyEditor(const QAbstractItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index) {
  this_ptr->destroyEditor(editor, *index);
}

bool qt_widgets_c_QAbstractItemDelegate_editorEvent(QAbstractItemDelegate* this_ptr, QEvent* event, QAbstractItemModel* model, const QStyleOptionViewItem* option, const QModelIndex* index) {
  return this_ptr->editorEvent(event, model, *option, *index);
}

void qt_widgets_c_QAbstractItemDelegate_elidedText_to_output(const QFontMetrics* fontMetrics, int width, const Qt::TextElideMode* mode, const QString* text, QString* output) {
  new(output) QString(QAbstractItemDelegate::elidedText(*fontMetrics, width, *mode, *text));
}

bool qt_widgets_c_QAbstractItemDelegate_helpEvent(QAbstractItemDelegate* this_ptr, QHelpEvent* event, QAbstractItemView* view, const QStyleOptionViewItem* option, const QModelIndex* index) {
  return this_ptr->helpEvent(event, view, *option, *index);
}

const QMetaObject* qt_widgets_c_QAbstractItemDelegate_metaObject(const QAbstractItemDelegate* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QAbstractItemDelegate_paint(const QAbstractItemDelegate* this_ptr, QPainter* painter, const QStyleOptionViewItem* option, const QModelIndex* index) {
  this_ptr->paint(painter, *option, *index);
}

void qt_widgets_c_QAbstractItemDelegate_paintingRoles_to_output(const QAbstractItemDelegate* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->paintingRoles());
}

int qt_widgets_c_QAbstractItemDelegate_qt_metacall(QAbstractItemDelegate* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAbstractItemDelegate_qt_metacast(QAbstractItemDelegate* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAbstractItemDelegate_setEditorData(const QAbstractItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index) {
  this_ptr->setEditorData(editor, *index);
}

void qt_widgets_c_QAbstractItemDelegate_setModelData(const QAbstractItemDelegate* this_ptr, QWidget* editor, QAbstractItemModel* model, const QModelIndex* index) {
  this_ptr->setModelData(editor, model, *index);
}

void qt_widgets_c_QAbstractItemDelegate_sizeHint_to_output(const QAbstractItemDelegate* this_ptr, const QStyleOptionViewItem* option, const QModelIndex* index, QSize* output) {
  new(output) QSize(this_ptr->sizeHint(*option, *index));
}

void qt_widgets_c_QAbstractItemDelegate_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractItemDelegate::trUtf8(s, c, n));
}

void qt_widgets_c_QAbstractItemDelegate_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractItemDelegate::tr(s, c, n));
}

void qt_widgets_c_QAbstractItemDelegate_updateEditorGeometry(const QAbstractItemDelegate* this_ptr, QWidget* editor, const QStyleOptionViewItem* option, const QModelIndex* index) {
  this_ptr->updateEditorGeometry(editor, *option, *index);
}

