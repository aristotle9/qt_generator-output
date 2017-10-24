#ifndef QT_CORE_C_QSTATE_H
#define QT_CORE_C_QSTATE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QState* qt_core_c_QState_G_dynamic_cast_QState_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QState* qt_core_c_QState_G_dynamic_cast_QState_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QState_G_static_cast_QAbstractState_ptr(QState* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QState_G_static_cast_QObject_ptr(QState* ptr);
QT_CORE_C_EXPORT QState* qt_core_c_QState_G_static_cast_QState_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QState* qt_core_c_QState_G_static_cast_QState_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QSignalTransition* qt_core_c_QState_addTransition_sender_signal_target(QState* this_ptr, const QObject* sender, const char* signal, QAbstractState* target);
QT_CORE_C_EXPORT QAbstractTransition* qt_core_c_QState_addTransition_target(QState* this_ptr, QAbstractState* target);
QT_CORE_C_EXPORT void qt_core_c_QState_addTransition_transition(QState* this_ptr, QAbstractTransition* transition);
QT_CORE_C_EXPORT void qt_core_c_QState_assignProperty(QState* this_ptr, QObject* object, const char* name, const QVariant* value);
QT_CORE_C_EXPORT QState::ChildMode qt_core_c_QState_childMode(const QState* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QState_delete(QState* this_ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QState_errorState(const QState* this_ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QState_initialState(const QState* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QState_metaObject(const QState* this_ptr);
QT_CORE_C_EXPORT QState* qt_core_c_QState_new_childMode(QState::ChildMode childMode);
QT_CORE_C_EXPORT QState* qt_core_c_QState_new_childMode_parent(QState::ChildMode childMode, QState* parent);
QT_CORE_C_EXPORT QState* qt_core_c_QState_new_no_args();
QT_CORE_C_EXPORT QState* qt_core_c_QState_new_parent(QState* parent);
QT_CORE_C_EXPORT void qt_core_c_QState_removeTransition(QState* this_ptr, QAbstractTransition* transition);
QT_CORE_C_EXPORT void qt_core_c_QState_setChildMode(QState* this_ptr, QState::ChildMode mode);
QT_CORE_C_EXPORT void qt_core_c_QState_setErrorState(QState* this_ptr, QAbstractState* state);
QT_CORE_C_EXPORT void qt_core_c_QState_setInitialState(QState* this_ptr, QAbstractState* state);
QT_CORE_C_EXPORT void qt_core_c_QState_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QState_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QState_transitions_to_output(const QState* this_ptr, QList< QAbstractTransition* >* output);

} // extern "C"

#endif // QT_CORE_C_QSTATE_H
