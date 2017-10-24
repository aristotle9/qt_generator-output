#ifndef QT_GUI_C_QGUIAPPLICATION_H
#define QT_GUI_C_QGUIAPPLICATION_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QCoreApplication* qt_gui_c_QGuiApplication_G_static_cast_QCoreApplication_ptr(QGuiApplication* ptr);
QT_GUI_C_EXPORT QGuiApplication* qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QCoreApplication(QCoreApplication* ptr);
QT_GUI_C_EXPORT QGuiApplication* qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QGuiApplication_G_static_cast_QObject_ptr(QGuiApplication* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_allWindows_to_output(QList< QWindow* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_applicationDisplayName_to_output(QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_changeOverrideCursor(const QCursor* arg1);
QT_GUI_C_EXPORT QClipboard* qt_gui_c_QGuiApplication_clipboard();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_delete(QGuiApplication* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_desktopFileName_to_output(QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_desktopSettingsAware();
QT_GUI_C_EXPORT double qt_gui_c_QGuiApplication_devicePixelRatio(const QGuiApplication* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QGuiApplication_exec();
QT_GUI_C_EXPORT QObject* qt_gui_c_QGuiApplication_focusObject();
QT_GUI_C_EXPORT QWindow* qt_gui_c_QGuiApplication_focusWindow();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_font_to_output(QFont* output);
QT_GUI_C_EXPORT QInputMethod* qt_gui_c_QGuiApplication_inputMethod();
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_isFallbackSessionManagementEnabled();
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_isLeftToRight();
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_isRightToLeft();
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_isSavingSession(const QGuiApplication* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_isSessionRestored(const QGuiApplication* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QGuiApplication_metaObject(const QGuiApplication* this_ptr);
QT_GUI_C_EXPORT QWindow* qt_gui_c_QGuiApplication_modalWindow();
QT_GUI_C_EXPORT QGuiApplication* qt_gui_c_QGuiApplication_new_argc_argv(int* argc, char** argv);
QT_GUI_C_EXPORT QGuiApplication* qt_gui_c_QGuiApplication_new_argc_argv_arg3(int* argc, char** argv, int arg3);
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_notify(QGuiApplication* this_ptr, QObject* arg1, QEvent* arg2);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QGuiApplication_overrideCursor();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_palette_to_output(QPalette* output);
QT_GUI_C_EXPORT void (*qt_gui_c_QGuiApplication_platformFunction(const QByteArray* function))();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_platformName_to_output(QString* output);
QT_GUI_C_EXPORT QScreen* qt_gui_c_QGuiApplication_primaryScreen();
QT_GUI_C_EXPORT int qt_gui_c_QGuiApplication_qt_metacall(QGuiApplication* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QGuiApplication_qt_metacast(QGuiApplication* this_ptr, const char* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QGuiApplication_quitOnLastWindowClosed();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_restoreOverrideCursor();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_screens_to_output(QList< QScreen* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_sessionId_to_output(const QGuiApplication* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_sessionKey_to_output(const QGuiApplication* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setApplicationDisplayName(const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setDesktopFileName(const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setDesktopSettingsAware(bool on);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setFallbackSessionManagementEnabled(bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setFont(const QFont* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setLayoutDirection(const Qt::LayoutDirection* direction);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setOverrideCursor(const QCursor* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setPalette(const QPalette* pal);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setQuitOnLastWindowClosed(bool quit);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_setWindowIcon(const QIcon* icon);
QT_GUI_C_EXPORT QStyleHints* qt_gui_c_QGuiApplication_styleHints();
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_sync();
QT_GUI_C_EXPORT QWindow* qt_gui_c_QGuiApplication_topLevelAt(const QPoint* pos);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_topLevelWindows_to_output(QList< QWindow* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QGuiApplication_windowIcon_to_output(QIcon* output);

} // extern "C"

#endif // QT_GUI_C_QGUIAPPLICATION_H
