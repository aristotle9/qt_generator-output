#ifndef QT_CORE_C_QABSTRACTSTATE_H
#define QT_CORE_C_QABSTRACTSTATE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractState* qt_core_c_QAbstractState_G_dynamic_cast_QAbstractState_ptr(QObject* ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QAbstractState_G_static_cast_QAbstractState_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractState_G_static_cast_QObject_ptr(QAbstractState* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QAbstractState_active(const QAbstractState* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractState_delete(QAbstractState* this_ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QAbstractState_machine(const QAbstractState* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractState_metaObject(const QAbstractState* this_ptr);
QT_CORE_C_EXPORT QState* qt_core_c_QAbstractState_parentState(const QAbstractState* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractState_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractState_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTSTATE_H
