#ifndef QT_WIDGETS_C_QSTACKEDLAYOUT_H
#define QT_WIDGETS_C_QSTACKEDLAYOUT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_G_dynamic_cast_QStackedLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QStackedLayout_G_static_cast_QLayoutItem_ptr(QStackedLayout* ptr);
QT_WIDGETS_C_EXPORT QLayout* qt_widgets_c_QStackedLayout_G_static_cast_QLayout_ptr(QStackedLayout* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStackedLayout_G_static_cast_QObject_ptr(QStackedLayout* ptr);
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayout(QLayout* ptr);
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QLayoutItem(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_G_static_cast_QStackedLayout_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_addItem(QStackedLayout* this_ptr, QLayoutItem* item);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedLayout_addWidget(QStackedLayout* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedLayout_count(const QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedLayout_currentIndex(const QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStackedLayout_currentWidget(const QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_delete(QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStackedLayout_hasHeightForWidth(const QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedLayout_heightForWidth(const QStackedLayout* this_ptr, int width);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedLayout_insertWidget(QStackedLayout* this_ptr, int index, QWidget* w);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QStackedLayout_itemAt(const QStackedLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QStackedLayout_metaObject(const QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_minimumSize_to_output(const QStackedLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_new_no_args();
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QStackedLayout* qt_widgets_c_QStackedLayout_new_parentLayout(QLayout* parentLayout);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStackedLayout_qt_metacall(QStackedLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QStackedLayout_qt_metacast(QStackedLayout* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_setCurrentIndex(QStackedLayout* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_setCurrentWidget(QStackedLayout* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_setGeometry(QStackedLayout* this_ptr, const QRect* rect);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_setStackingMode(QStackedLayout* this_ptr, QStackedLayout::StackingMode stackingMode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_sizeHint_to_output(const QStackedLayout* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QStackedLayout::StackingMode qt_widgets_c_QStackedLayout_stackingMode(const QStackedLayout* this_ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QStackedLayout_takeAt(QStackedLayout* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStackedLayout_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStackedLayout_widget(const QStackedLayout* this_ptr, int arg1);

} // extern "C"

#endif // QT_WIDGETS_C_QSTACKEDLAYOUT_H
