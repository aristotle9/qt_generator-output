#ifndef QT_WIDGETS_C_QSCROLLAREA_H
#define QT_WIDGETS_C_QSCROLLAREA_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QAbstractScrollArea_ptr(QScrollArea* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QScrollArea_G_static_cast_QFrame_ptr(QScrollArea* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QScrollArea_G_static_cast_QObject_ptr(QScrollArea* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QScrollArea_G_static_cast_QPaintDevice_ptr(QScrollArea* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QScrollArea_G_static_cast_QWidget_ptr(QScrollArea* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_delete(QScrollArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_ensureVisible_x_y(QScrollArea* this_ptr, int x, int y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_ensureVisible_x_y_xmargin(QScrollArea* this_ptr, int x, int y, int xmargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_ensureVisible_x_y_xmargin_ymargin(QScrollArea* this_ptr, int x, int y, int xmargin, int ymargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget(QScrollArea* this_ptr, QWidget* childWidget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget_xmargin(QScrollArea* this_ptr, QWidget* childWidget, int xmargin);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget_xmargin_ymargin(QScrollArea* this_ptr, QWidget* childWidget, int xmargin, int ymargin);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScrollArea_focusNextPrevChild(QScrollArea* this_ptr, bool next);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QScrollArea_metaObject(const QScrollArea* this_ptr);
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_new_no_args();
QT_WIDGETS_C_EXPORT QScrollArea* qt_widgets_c_QScrollArea_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QScrollArea_qt_metacall(QScrollArea* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QScrollArea_qt_metacast(QScrollArea* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_setWidget(QScrollArea* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_setWidgetResizable(QScrollArea* this_ptr, bool resizable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_sizeHint_to_output(const QScrollArea* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QScrollArea_takeWidget(QScrollArea* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollArea_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QScrollArea_widget(const QScrollArea* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScrollArea_widgetResizable(const QScrollArea* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSCROLLAREA_H
