#ifndef QT_CORE_C_QCOREAPPLICATION_H
#define QT_CORE_C_QCOREAPPLICATION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QCoreApplication* qt_core_c_QCoreApplication_G_dynamic_cast_QCoreApplication_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_G_qAddPostRoutine(void (*arg1)());
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_G_qAddPreRoutine(void (*arg1)());
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_G_qRemovePostRoutine(void (*arg1)());
QT_CORE_C_EXPORT QCoreApplication* qt_core_c_QCoreApplication_G_static_cast_QCoreApplication_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QCoreApplication_G_static_cast_QObject_ptr(QCoreApplication* ptr);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_addLibraryPath(const QString* arg1);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_applicationDirPath_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_applicationFilePath_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_applicationName_to_output(QString* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QCoreApplication_applicationPid();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_applicationVersion_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_arguments_to_output(QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_closingDown();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_delete(QCoreApplication* this_ptr);
QT_CORE_C_EXPORT QAbstractEventDispatcher* qt_core_c_QCoreApplication_eventDispatcher();
QT_CORE_C_EXPORT int qt_core_c_QCoreApplication_exec();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_exit_no_args();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_exit_retcode(int retcode);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_flush();
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_hasPendingEvents();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_installNativeEventFilter(QCoreApplication* this_ptr, QAbstractNativeEventFilter* filterObj);
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_installTranslator(QTranslator* messageFile);
QT_CORE_C_EXPORT QCoreApplication* qt_core_c_QCoreApplication_instance();
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_isQuitLockEnabled();
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_isSetuidAllowed();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_libraryPaths_to_output(QStringList* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QCoreApplication_metaObject(const QCoreApplication* this_ptr);
QT_CORE_C_EXPORT QCoreApplication* qt_core_c_QCoreApplication_new_argc_argv(int* argc, char** argv);
QT_CORE_C_EXPORT QCoreApplication* qt_core_c_QCoreApplication_new_argc_argv_arg3(int* argc, char** argv, int arg3);
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_notify(QCoreApplication* this_ptr, QObject* arg1, QEvent* arg2);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_organizationDomain_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_organizationName_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_postEvent_receiver_event(QObject* receiver, QEvent* event);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_postEvent_receiver_event_priority(QObject* receiver, QEvent* event, int priority);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_quit();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_removeLibraryPath(const QString* arg1);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_removeNativeEventFilter(QCoreApplication* this_ptr, QAbstractNativeEventFilter* filterObj);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_removePostedEvents_receiver(QObject* receiver);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_removePostedEvents_receiver_eventType(QObject* receiver, int eventType);
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_removeTranslator(QTranslator* messageFile);
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_sendEvent(QObject* receiver, QEvent* event);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_sendPostedEvents_no_args();
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_sendPostedEvents_receiver(QObject* receiver);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_sendPostedEvents_receiver_event_type(QObject* receiver, int event_type);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setApplicationName(const QString* application);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setApplicationVersion(const QString* version);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setAttribute_attribute(const Qt::ApplicationAttribute* attribute);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setAttribute_attribute_on(const Qt::ApplicationAttribute* attribute, bool on);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setEventDispatcher(QAbstractEventDispatcher* eventDispatcher);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setLibraryPaths(const QStringList* arg1);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setOrganizationDomain(const QString* orgDomain);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setOrganizationName(const QString* orgName);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setQuitLockEnabled(bool enabled);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_setSetuidAllowed(bool allow);
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_startingUp();
QT_CORE_C_EXPORT bool qt_core_c_QCoreApplication_testAttribute(const Qt::ApplicationAttribute* attribute);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_translate_to_output_context_key(const char* context, const char* key, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_translate_to_output_context_key_disambiguation(const char* context, const char* key, const char* disambiguation, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCoreApplication_translate_to_output_context_key_disambiguation_n(const char* context, const char* key, const char* disambiguation, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QCOREAPPLICATION_H
