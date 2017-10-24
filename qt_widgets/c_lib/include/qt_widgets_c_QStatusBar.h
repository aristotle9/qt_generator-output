#ifndef QT_WIDGETS_C_QSTATUSBAR_H
#define QT_WIDGETS_C_QSTATUSBAR_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QStatusBar_G_dynamic_cast_QStatusBar_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QStatusBar_G_static_cast_QObject_ptr(QStatusBar* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QStatusBar_G_static_cast_QPaintDevice_ptr(QStatusBar* ptr);
QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QStatusBar_G_static_cast_QWidget_ptr(QStatusBar* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_addPermanentWidget_widget(QStatusBar* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_addPermanentWidget_widget_stretch(QStatusBar* this_ptr, QWidget* widget, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_addWidget_widget(QStatusBar* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_addWidget_widget_stretch(QStatusBar* this_ptr, QWidget* widget, int stretch);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_clearMessage(QStatusBar* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_currentMessage_to_output(const QStatusBar* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_delete(QStatusBar* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStatusBar_insertPermanentWidget_index_widget(QStatusBar* this_ptr, int index, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStatusBar_insertPermanentWidget_index_widget_stretch(QStatusBar* this_ptr, int index, QWidget* widget, int stretch);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStatusBar_insertWidget_index_widget(QStatusBar* this_ptr, int index, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStatusBar_insertWidget_index_widget_stretch(QStatusBar* this_ptr, int index, QWidget* widget, int stretch);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QStatusBar_isSizeGripEnabled(const QStatusBar* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QStatusBar_metaObject(const QStatusBar* this_ptr);
QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QStatusBar_new_no_args();
QT_WIDGETS_C_EXPORT QStatusBar* qt_widgets_c_QStatusBar_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStatusBar_qt_metacall(QStatusBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QStatusBar_qt_metacast(QStatusBar* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_removeWidget(QStatusBar* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_setSizeGripEnabled(QStatusBar* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_showMessage_text(QStatusBar* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_showMessage_text_timeout(QStatusBar* this_ptr, const QString* text, int timeout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStatusBar_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSTATUSBAR_H
