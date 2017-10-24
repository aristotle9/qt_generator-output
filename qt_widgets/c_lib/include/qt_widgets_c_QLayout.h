#ifndef QT_WIDGETS_C_QLAYOUT_H
#define QT_WIDGETS_C_QLAYOUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QLayout_G_dynamic_cast_QLayout_ptr(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QLayout_G_static_cast_QLayoutItem_ptr(QLayout* ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QLayout_G_static_cast_QLayout_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QLayout_G_static_cast_QObject_ptr(QLayout* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLayout_activate(QLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_addItem(QLayout* this_ptr, QLayoutItem* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_addWidget(QLayout* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_closestAcceptableSize_to_output(const QWidget* w, const QSize* s, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_contentsMargins_to_output(const QLayout* this_ptr, QMargins* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_contentsRect_to_output(const QLayout* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayout_count(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_delete(QLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_geometry_to_output(const QLayout* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_getContentsMargins(const QLayout* this_ptr, int* left, int* top, int* right, int* bottom);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayout_indexOf(const QLayout* this_ptr, QWidget* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_invalidate(QLayout* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLayout_isEmpty(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLayout_isEnabled(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QLayout_itemAt(const QLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QLayout_layout(QLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayout_margin(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_maximumSize_to_output(const QLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLayout_menuBar(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QLayout_metaObject(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_minimumSize_to_output(const QLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLayout_parentWidget(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayout_qt_metacall(QLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QLayout_qt_metacast(QLayout* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_removeItem(QLayout* this_ptr, QLayoutItem* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_removeWidget(QLayout* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setContentsMargins_left_top_right_bottom(QLayout* this_ptr, int left, int top, int right, int bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setContentsMargins_margins(QLayout* this_ptr, const QMargins* margins);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setEnabled(QLayout* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setGeometry(QLayout* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setMargin(QLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setMenuBar(QLayout* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setSizeConstraint(QLayout* this_ptr, QLayout::SizeConstraint arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_setSpacing(QLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT QLayout::SizeConstraint qt_widgets_c_QLayout_sizeConstraint(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayout_spacing(const QLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QLayout_takeAt(QLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLayout_totalHeightForWidth(const QLayout* this_ptr, int w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_totalMaximumSize_to_output(const QLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_totalMinimumSize_to_output(const QLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_totalSizeHint_to_output(const QLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLayout_update(QLayout* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QLAYOUT_H
