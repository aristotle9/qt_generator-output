#ifndef QT_WIDGETS_C_QWIDGETITEM_H
#define QT_WIDGETS_C_QWIDGETITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QWidgetItem* qt_widgets_c_QWidgetItem_G_dynamic_cast_QWidgetItem_ptr(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT QLayoutItem* qt_widgets_c_QWidgetItem_G_static_cast_QLayoutItem_ptr(QWidgetItem* ptr);
QT_WIDGETS_C_EXPORT QWidgetItem* qt_widgets_c_QWidgetItem_G_static_cast_QWidgetItem_ptr(QLayoutItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetItem_delete(QWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetItem_geometry_to_output(const QWidgetItem* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidgetItem_hasHeightForWidth(const QWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidgetItem_heightForWidth(const QWidgetItem* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWidgetItem_isEmpty(const QWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetItem_maximumSize_to_output(const QWidgetItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetItem_minimumSize_to_output(const QWidgetItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QWidgetItem* qt_widgets_c_QWidgetItem_new(QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetItem_setGeometry(QWidgetItem* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetItem_sizeHint_to_output(const QWidgetItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidgetItem_widget(QWidgetItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QWIDGETITEM_H
