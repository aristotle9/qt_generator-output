#ifndef QT_WIDGETS_C_QSTYLEDITEMDELEGATE_H
#define QT_WIDGETS_C_QSTYLEDITEMDELEGATE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_G_dynamic_cast_QStyledItemDelegate_ptr(QAbstractItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemDelegate* qt_widgets_c_QStyledItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(QStyledItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStyledItemDelegate_G_static_cast_QObject_ptr(QStyledItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_G_static_cast_QStyledItemDelegate_ptr_QAbstractItemDelegate(QAbstractItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_G_static_cast_QStyledItemDelegate_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStyledItemDelegate_createEditor(const QStyledItemDelegate* this_ptr, QWidget* parent, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_delete(QStyledItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_displayText_to_output(const QStyledItemDelegate* this_ptr, const QVariant* value, const QLocale* locale, QString* output);
QT_WIDGETS_C_EXPORT QItemEditorFactory* qt_widgets_c_QStyledItemDelegate_itemEditorFactory(const QStyledItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QStyledItemDelegate_metaObject(const QStyledItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_new_no_args();
QT_WIDGETS_C_EXPORT QStyledItemDelegate* qt_widgets_c_QStyledItemDelegate_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_paint(const QStyledItemDelegate* this_ptr, QPainter* painter, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyledItemDelegate_qt_metacall(QStyledItemDelegate* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QStyledItemDelegate_qt_metacast(QStyledItemDelegate* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_setEditorData(const QStyledItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_setItemEditorFactory(QStyledItemDelegate* this_ptr, QItemEditorFactory* factory);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_setModelData(const QStyledItemDelegate* this_ptr, QWidget* editor, QAbstractItemModel* model, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_sizeHint_to_output(const QStyledItemDelegate* this_ptr, const QStyleOptionViewItem* option, const QModelIndex* index, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyledItemDelegate_updateEditorGeometry(const QStyledItemDelegate* this_ptr, QWidget* editor, const QStyleOptionViewItem* option, const QModelIndex* index);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEDITEMDELEGATE_H
