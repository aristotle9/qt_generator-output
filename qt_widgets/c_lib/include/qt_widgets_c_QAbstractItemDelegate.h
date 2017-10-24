#ifndef QT_WIDGETS_C_QABSTRACTITEMDELEGATE_H
#define QT_WIDGETS_C_QABSTRACTITEMDELEGATE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractItemDelegate* qt_widgets_c_QAbstractItemDelegate_G_static_cast_QAbstractItemDelegate_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QAbstractItemDelegate_G_static_cast_QObject_ptr(QAbstractItemDelegate* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QAbstractItemDelegate_createEditor(const QAbstractItemDelegate* this_ptr, QWidget* parent, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_delete(QAbstractItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_destroyEditor(const QAbstractItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractItemDelegate_editorEvent(QAbstractItemDelegate* this_ptr, QEvent* event, QAbstractItemModel* model, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_elidedText_to_output(const QFontMetrics* fontMetrics, int width, const Qt::TextElideMode* mode, const QString* text, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractItemDelegate_helpEvent(QAbstractItemDelegate* this_ptr, QHelpEvent* event, QAbstractItemView* view, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QAbstractItemDelegate_metaObject(const QAbstractItemDelegate* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_paint(const QAbstractItemDelegate* this_ptr, QPainter* painter, const QStyleOptionViewItem* option, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_paintingRoles_to_output(const QAbstractItemDelegate* this_ptr, QVector< int >* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractItemDelegate_qt_metacall(QAbstractItemDelegate* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QAbstractItemDelegate_qt_metacast(QAbstractItemDelegate* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_setEditorData(const QAbstractItemDelegate* this_ptr, QWidget* editor, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_setModelData(const QAbstractItemDelegate* this_ptr, QWidget* editor, QAbstractItemModel* model, const QModelIndex* index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_sizeHint_to_output(const QAbstractItemDelegate* this_ptr, const QStyleOptionViewItem* option, const QModelIndex* index, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractItemDelegate_updateEditorGeometry(const QAbstractItemDelegate* this_ptr, QWidget* editor, const QStyleOptionViewItem* option, const QModelIndex* index);

} // extern "C"

#endif // QT_WIDGETS_C_QABSTRACTITEMDELEGATE_H
