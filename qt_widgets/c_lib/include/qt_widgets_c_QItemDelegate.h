#ifndef QT_WIDGETS_C_QITEMDELEGATE_H
#define QT_WIDGETS_C_QITEMDELEGATE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QItemDelegate* qt_widgets_c_QItemDelegate_G_dynamic_cast_QItemDelegate_ptr(QAbstractItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemDelegate* qt_widgets_c_QItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(QItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QItemDelegate* qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QAbstractItemDelegate(QAbstractItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QItemDelegate* qt_widgets_c_QItemDelegate_G_static_cast_QItemDelegate_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QItemDelegate_G_static_cast_QObject_ptr(QItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QItemDelegate_createEditor(const QItemDelegate* this_ptr, QWidget* parent, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_delete(QItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QItemDelegate_hasClipping(const QItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT QItemEditorFactory* qt_widgets_c_QItemDelegate_itemEditorFactory(const QItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QItemDelegate_metaObject(const QItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT QItemDelegate* qt_widgets_c_QItemDelegate_new_no_args();
QT_WIDGETS_C_EXPORT QItemDelegate* qt_widgets_c_QItemDelegate_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_paint(const QItemDelegate* this_ptr, QPainter* painter, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QItemDelegate_qt_metacall(QItemDelegate* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QItemDelegate_qt_metacast(QItemDelegate* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_setClipping(QItemDelegate* this_ptr, bool clip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_setEditorData(const QItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_setItemEditorFactory(QItemDelegate* this_ptr, QItemEditorFactory* factory);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_setModelData(const QItemDelegate* this_ptr, QWidget* editor, QAbstractItemModel* model, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_sizeHint_to_output(const QItemDelegate* this_ptr, const QStyleOptionViewItem* option, const QModelIndex* index, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QItemDelegate_updateEditorGeometry(const QItemDelegate* this_ptr, QWidget* editor, const QStyleOptionViewItem* option, const QModelIndex* index);

} // extern "C"

#endif // QT_WIDGETS_C_QITEMDELEGATE_H
