#ifndef QT_CORE_C_QSIGNALTRANSITION_H
#define QT_CORE_C_QSIGNALTRANSITION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QAbstractTransition(QAbstractTransition* ptr);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractTransition* qt_core_c_QSignalTransition_G_static_cast_QAbstractTransition_ptr(QSignalTransition* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSignalTransition_G_static_cast_QObject_ptr(QSignalTransition* ptr);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QAbstractTransition(QAbstractTransition* ptr);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalTransition_delete(QSignalTransition* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSignalTransition_metaObject(const QSignalTransition* this_ptr);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_new_no_args();
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_new_sender_signal(const QObject* sender, const char* signal);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_new_sender_signal_sourceState(const QObject* sender, const char* signal, QState* sourceState);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QSignalTransition_new_sourceState(QState* sourceState);
QT_CORE_C_EXPORT QObject* qt_core_c_QSignalTransition_senderObject(const QSignalTransition* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalTransition_setSenderObject(QSignalTransition* this_ptr, const QObject* sender);
QT_CORE_C_EXPORT void qt_core_c_QSignalTransition_setSignal(QSignalTransition* this_ptr, const QByteArray* signal);
QT_CORE_C_EXPORT void qt_core_c_QSignalTransition_signal_to_output(const QSignalTransition* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QSignalTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSignalTransition_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSIGNALTRANSITION_H
