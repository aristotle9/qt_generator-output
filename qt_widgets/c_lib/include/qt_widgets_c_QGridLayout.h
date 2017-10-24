#ifndef QT_WIDGETS_C_QGRIDLAYOUT_H
#define QT_WIDGETS_C_QGRIDLAYOUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QGridLayout_G_static_cast_QLayoutItem_ptr(QGridLayout* ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QGridLayout_G_static_cast_QLayout_ptr(QGridLayout* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGridLayout_G_static_cast_QObject_ptr(QGridLayout* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_addWidget(QGridLayout* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_cellRect_to_output(const QGridLayout* this_ptr, int row, int column, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_columnCount(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_columnMinimumWidth(const QGridLayout* this_ptr, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_columnStretch(const QGridLayout* this_ptr, int column);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_count(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_delete(QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_getItemPosition(const QGridLayout* this_ptr, int idx, int* row, int* column, int* rowSpan, int* columnSpan);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGridLayout_hasHeightForWidth(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_heightForWidth(const QGridLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_horizontalSpacing(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_invalidate(QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QGridLayout_itemAt(const QGridLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QGridLayout_itemAtPosition(const QGridLayout* this_ptr, int row, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_maximumSize_to_output(const QGridLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGridLayout_metaObject(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_minimumHeightForWidth(const QGridLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_minimumSize_to_output(const QGridLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_new_no_args();
QT_WIDGETS_C_EXPORT QGridLayout* qt_widgets_c_QGridLayout_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_qt_metacall(QGridLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGridLayout_qt_metacast(QGridLayout* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_rowCount(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_rowMinimumHeight(const QGridLayout* this_ptr, int row);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_rowStretch(const QGridLayout* this_ptr, int row);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setColumnMinimumWidth(QGridLayout* this_ptr, int column, int minSize);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setColumnStretch(QGridLayout* this_ptr, int column, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setDefaultPositioning(QGridLayout* this_ptr, int n, const Qt::Orientation* orient);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setGeometry(QGridLayout* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setHorizontalSpacing(QGridLayout* this_ptr, int spacing);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setOriginCorner(QGridLayout* this_ptr, const Qt::Corner* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setRowMinimumHeight(QGridLayout* this_ptr, int row, int minSize);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setRowStretch(QGridLayout* this_ptr, int row, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setSpacing(QGridLayout* this_ptr, int spacing);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_setVerticalSpacing(QGridLayout* this_ptr, int spacing);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_sizeHint_to_output(const QGridLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_spacing(const QGridLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QGridLayout_takeAt(QGridLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGridLayout_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGridLayout_verticalSpacing(const QGridLayout* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRIDLAYOUT_H
