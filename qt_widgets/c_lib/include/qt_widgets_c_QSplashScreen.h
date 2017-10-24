#ifndef QT_WIDGETS_C_QSPLASHSCREEN_H
#define QT_WIDGETS_C_QSPLASHSCREEN_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSplashScreen* qt_widgets_c_QSplashScreen_G_dynamic_cast_QSplashScreen_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSplashScreen_G_static_cast_QObject_ptr(QSplashScreen* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QSplashScreen_G_static_cast_QPaintDevice_ptr(QSplashScreen* ptr);
QT_WIDGETS_C_EXPORT QSplashScreen* qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QSplashScreen* qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QSplashScreen* qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSplashScreen_G_static_cast_QWidget_ptr(QSplashScreen* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_clearMessage(QSplashScreen* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_delete(QSplashScreen* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_finish(QSplashScreen* this_ptr, QWidget* w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_message_to_output(const QSplashScreen* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSplashScreen_metaObject(const QSplashScreen* this_ptr);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QSplashScreen_pixmap_as_ptr(const QSplashScreen* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSplashScreen_qt_metacall(QSplashScreen* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSplashScreen_qt_metacast(QSplashScreen* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_repaint(QSplashScreen* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_setPixmap(QSplashScreen* this_ptr, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_showMessage_message(QSplashScreen* this_ptr, const QString* message);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_showMessage_message_alignment(QSplashScreen* this_ptr, const QString* message, int alignment);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_showMessage_message_alignment_color(QSplashScreen* this_ptr, const QString* message, int alignment, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplashScreen_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSPLASHSCREEN_H
