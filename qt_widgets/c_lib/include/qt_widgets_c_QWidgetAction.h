#ifndef QT_WIDGETS_C_QWIDGETACTION_H
#define QT_WIDGETS_C_QWIDGETACTION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QWidgetAction* qt_widgets_c_QWidgetAction_G_dynamic_cast_QWidgetAction_ptr(QAction* ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QWidgetAction_G_static_cast_QAction_ptr(QWidgetAction* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QWidgetAction_G_static_cast_QObject_ptr(QWidgetAction* ptr);
QT_WIDGETS_C_EXPORT QWidgetAction* qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QAction(QAction* ptr);
QT_WIDGETS_C_EXPORT QWidgetAction* qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidgetAction_defaultWidget(const QWidgetAction* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetAction_delete(QWidgetAction* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QWidgetAction_metaObject(const QWidgetAction* this_ptr);
QT_WIDGETS_C_EXPORT QWidgetAction* qt_widgets_c_QWidgetAction_new(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWidgetAction_qt_metacall(QWidgetAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QWidgetAction_qt_metacast(QWidgetAction* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetAction_releaseWidget(QWidgetAction* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWidgetAction_requestWidget(QWidgetAction* this_ptr, QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetAction_setDefaultWidget(QWidgetAction* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWidgetAction_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QWIDGETACTION_H
