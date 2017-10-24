#ifndef QT_WIDGETS_C_QAPPLICATION_H
#define QT_WIDGETS_C_QAPPLICATION_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QApplication* qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QCoreApplication(QCoreApplication* ptr);
QT_WIDGETS_C_EXPORT QApplication* qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QGuiApplication(QGuiApplication* ptr);
QT_WIDGETS_C_EXPORT QApplication* qt_widgets_c_QApplication_G_static_cast_QApplication_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QCoreApplication* qt_widgets_c_QApplication_G_static_cast_QCoreApplication_ptr(QApplication* ptr);
QT_WIDGETS_C_EXPORT QGuiApplication* qt_widgets_c_QApplication_G_static_cast_QGuiApplication_ptr(QApplication* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QApplication_G_static_cast_QObject_ptr(QApplication* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_aboutQt();
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_activeModalWidget();
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_activePopupWidget();
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_activeWindow();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_alert_widget(QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_alert_widget_duration(QWidget* widget, int duration);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_allWidgets_to_output(QList< QWidget* >* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QApplication_autoSipEnabled(const QApplication* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_beep();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_closeAllWindows();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_colorSpec();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_cursorFlashTime();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_delete(QApplication* this_ptr);
QT_WIDGETS_C_EXPORT QDesktopWidget* qt_widgets_c_QApplication_desktop();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_doubleClickInterval();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_exec();
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_focusWidget();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_fontMetrics_to_output(QFontMetrics* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_font_to_output_arg1(const QWidget* arg1, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_font_to_output_className(const char* className, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_font_to_output_no_args(QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_globalStrut_to_output(QSize* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QApplication_isEffectEnabled(const Qt::UIEffect* arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_keyboardInputInterval();
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QApplication_metaObject(const QApplication* this_ptr);
QT_WIDGETS_C_EXPORT QApplication* qt_widgets_c_QApplication_new_argc_argv(int* argc, char** argv);
QT_WIDGETS_C_EXPORT QApplication* qt_widgets_c_QApplication_new_argc_argv_arg3(int* argc, char** argv, int arg3);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QApplication_notify(QApplication* this_ptr, QObject* arg1, QEvent* arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_palette_to_output_arg1(const QWidget* arg1, QPalette* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_palette_to_output_className(const char* className, QPalette* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_qt_metacall(QApplication* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QApplication_qt_metacast(QApplication* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setActiveWindow(QWidget* act);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setAutoSipEnabled(QApplication* this_ptr, const bool enabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setColorSpec(int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setCursorFlashTime(int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setDoubleClickInterval(int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setEffectEnabled_arg1(const Qt::UIEffect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setEffectEnabled_arg1_enable(const Qt::UIEffect* arg1, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setFont_arg1(const QFont* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setFont_arg1_className(const QFont* arg1, const char* className);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setGlobalStrut(const QSize* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setKeyboardInputInterval(int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setPalette_arg1(const QPalette* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setPalette_arg1_className(const QPalette* arg1, const char* className);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setStartDragDistance(int l);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setStartDragTime(int ms);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setStyleSheet(QApplication* this_ptr, const QString* sheet);
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QApplication_setStyle_QString(const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setStyle_QStyle(QStyle* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setWheelScrollLines(int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_setWindowIcon(const QIcon* icon);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_startDragDistance();
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_startDragTime();
QT_WIDGETS_C_EXPORT QStyle* qt_widgets_c_QApplication_style();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_styleSheet_to_output(const QApplication* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_topLevelAt_p(const QPoint* p);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_topLevelAt_x_y(int x, int y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_topLevelWidgets_to_output(QList< QWidget* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QApplication_wheelScrollLines();
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_widgetAt_p(const QPoint* p);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QApplication_widgetAt_x_y(int x, int y);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QApplication_windowIcon_to_output(QIcon* output);

} // extern "C"

#endif // QT_WIDGETS_C_QAPPLICATION_H
