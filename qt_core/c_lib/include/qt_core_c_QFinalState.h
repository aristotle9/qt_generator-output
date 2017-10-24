#ifndef QT_CORE_C_QFINALSTATE_H
#define QT_CORE_C_QFINALSTATE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QFinalState* qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QFinalState* qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QFinalState_G_static_cast_QAbstractState_ptr(QFinalState* ptr);
QT_CORE_C_EXPORT QFinalState* qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QFinalState* qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QFinalState_G_static_cast_QObject_ptr(QFinalState* ptr);
QT_CORE_C_EXPORT void qt_core_c_QFinalState_delete(QFinalState* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QFinalState_metaObject(const QFinalState* this_ptr);
QT_CORE_C_EXPORT QFinalState* qt_core_c_QFinalState_new_no_args();
QT_CORE_C_EXPORT QFinalState* qt_core_c_QFinalState_new_parent(QState* parent);
QT_CORE_C_EXPORT void qt_core_c_QFinalState_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFinalState_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QFINALSTATE_H
