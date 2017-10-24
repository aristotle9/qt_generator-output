#ifndef QT_GUI_C_QSESSIONMANAGER_H
#define QT_GUI_C_QSESSIONMANAGER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QSessionManager_G_static_cast_QObject_ptr(QSessionManager* ptr);
QT_GUI_C_EXPORT QSessionManager* qt_gui_c_QSessionManager_G_static_cast_QSessionManager_ptr(QObject* ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QSessionManager_allowsErrorInteraction(QSessionManager* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QSessionManager_allowsInteraction(QSessionManager* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_cancel(QSessionManager* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_discardCommand_to_output(const QSessionManager* this_ptr, QStringList* output);
QT_GUI_C_EXPORT bool qt_gui_c_QSessionManager_isPhase2(const QSessionManager* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QSessionManager_metaObject(const QSessionManager* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QSessionManager_qt_metacall(QSessionManager* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QSessionManager_qt_metacast(QSessionManager* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_release(QSessionManager* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_requestPhase2(QSessionManager* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_restartCommand_to_output(const QSessionManager* this_ptr, QStringList* output);
QT_GUI_C_EXPORT QSessionManager::RestartHint qt_gui_c_QSessionManager_restartHint(const QSessionManager* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_sessionId_to_output(const QSessionManager* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_sessionKey_to_output(const QSessionManager* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_setDiscardCommand(QSessionManager* this_ptr, const QStringList* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_setManagerProperty_QString_QString(QSessionManager* this_ptr, const QString* name, const QString* value);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_setManagerProperty_QString_QStringList(QSessionManager* this_ptr, const QString* name, const QStringList* value);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_setRestartCommand(QSessionManager* this_ptr, const QStringList* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_setRestartHint(QSessionManager* this_ptr, QSessionManager::RestartHint arg1);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QSessionManager_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QSESSIONMANAGER_H
