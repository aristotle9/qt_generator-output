#ifndef QT_WIDGETS_C_QMDISUBWINDOW_H
#define QT_WIDGETS_C_QMDISUBWINDOW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_dynamic_cast_QMdiSubWindow_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMdiSubWindow_G_static_cast_QObject_ptr(QMdiSubWindow* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QMdiSubWindow_G_static_cast_QPaintDevice_ptr(QMdiSubWindow* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMdiSubWindow_G_static_cast_QWidget_ptr(QMdiSubWindow* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_delete(QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMdiSubWindow_isShaded(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMdiSubWindow_keyboardPageStep(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMdiSubWindow_keyboardSingleStep(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMdiSubWindow_maximizedButtonsWidget(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMdiSubWindow_maximizedSystemMenuIconWidget(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT QMdiArea* qt_widgets_c_QMdiSubWindow_mdiArea(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMdiSubWindow_metaObject(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_minimumSizeHint_to_output(const QMdiSubWindow* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMdiSubWindow_qt_metacall(QMdiSubWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMdiSubWindow_qt_metacast(QMdiSubWindow* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_setKeyboardPageStep(QMdiSubWindow* this_ptr, int step);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_setKeyboardSingleStep(QMdiSubWindow* this_ptr, int step);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_setOption_option(QMdiSubWindow* this_ptr, QMdiSubWindow::SubWindowOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_setOption_option_on(QMdiSubWindow* this_ptr, QMdiSubWindow::SubWindowOption option, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_setSystemMenu(QMdiSubWindow* this_ptr, QMenu* systemMenu);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_setWidget(QMdiSubWindow* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_showShaded(QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_showSystemMenu(QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_sizeHint_to_output(const QMdiSubWindow* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QMdiSubWindow_systemMenu(const QMdiSubWindow* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QMdiSubWindow_testOption(const QMdiSubWindow* this_ptr, QMdiSubWindow::SubWindowOption arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMdiSubWindow_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMdiSubWindow_widget(const QMdiSubWindow* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QMDISUBWINDOW_H
