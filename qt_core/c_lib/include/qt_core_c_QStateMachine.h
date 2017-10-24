#ifndef QT_CORE_C_QSTATEMACHINE_H
#define QT_CORE_C_QSTATEMACHINE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QStateMachine::SignalEvent* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_SignalEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QStateMachine::WrappedEvent* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_WrappedEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QState(QState* ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QStateMachine_G_static_cast_QAbstractState_ptr(QStateMachine* ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_SignalEvent(QStateMachine::SignalEvent* ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_WrappedEvent(QStateMachine::WrappedEvent* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QStateMachine_G_static_cast_QObject_ptr(QStateMachine* ptr);
QT_CORE_C_EXPORT QStateMachine::SignalEvent* qt_core_c_QStateMachine_G_static_cast_QStateMachine_SignalEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QStateMachine::WrappedEvent* qt_core_c_QStateMachine_G_static_cast_QStateMachine_WrappedEvent_ptr(QEvent* ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QState(QState* ptr);
QT_CORE_C_EXPORT QState* qt_core_c_QStateMachine_G_static_cast_QState_ptr(QStateMachine* ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_SignalEvent_arguments_to_output(const QStateMachine::SignalEvent* this_ptr, QList< QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_SignalEvent_delete(QStateMachine::SignalEvent* this_ptr);
QT_CORE_C_EXPORT QStateMachine::SignalEvent* qt_core_c_QStateMachine_SignalEvent_new(QObject* sender, int signalIndex, const QList< QVariant >* arguments);
QT_CORE_C_EXPORT QObject* qt_core_c_QStateMachine_SignalEvent_sender(const QStateMachine::SignalEvent* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QStateMachine_SignalEvent_signalIndex(const QStateMachine::SignalEvent* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_WrappedEvent_delete(QStateMachine::WrappedEvent* this_ptr);
QT_CORE_C_EXPORT QEvent* qt_core_c_QStateMachine_WrappedEvent_event(const QStateMachine::WrappedEvent* this_ptr);
QT_CORE_C_EXPORT QStateMachine::WrappedEvent* qt_core_c_QStateMachine_WrappedEvent_new(QObject* object, QEvent* event);
QT_CORE_C_EXPORT QObject* qt_core_c_QStateMachine_WrappedEvent_object(const QStateMachine::WrappedEvent* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_addDefaultAnimation(QStateMachine* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_addState(QStateMachine* this_ptr, QAbstractState* state);
QT_CORE_C_EXPORT bool qt_core_c_QStateMachine_cancelDelayedEvent(QStateMachine* this_ptr, int id);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_clearError(QStateMachine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_configuration_to_output(const QStateMachine* this_ptr, QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_defaultAnimations_to_output(const QStateMachine* this_ptr, QList< QAbstractAnimation* >* output);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_delete(QStateMachine* this_ptr);
QT_CORE_C_EXPORT QStateMachine::Error qt_core_c_QStateMachine_error(const QStateMachine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_errorString_to_output(const QStateMachine* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QStateMachine_eventFilter(QStateMachine* this_ptr, QObject* watched, QEvent* event);
QT_CORE_C_EXPORT bool qt_core_c_QStateMachine_isAnimated(const QStateMachine* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QStateMachine_isRunning(const QStateMachine* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QStateMachine_metaObject(const QStateMachine* this_ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_new_childMode(const QState::ChildMode* childMode);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_new_childMode_parent(const QState::ChildMode* childMode, QObject* parent);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_new_no_args();
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QStateMachine_new_parent(QObject* parent);
QT_CORE_C_EXPORT int qt_core_c_QStateMachine_postDelayedEvent(QStateMachine* this_ptr, QEvent* event, int delay);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_postEvent_event(QStateMachine* this_ptr, QEvent* event);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_postEvent_event_priority(QStateMachine* this_ptr, QEvent* event, QStateMachine::EventPriority priority);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_removeDefaultAnimation(QStateMachine* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_removeState(QStateMachine* this_ptr, QAbstractState* state);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_setAnimated(QStateMachine* this_ptr, bool enabled);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_setGlobalRestorePolicy(QStateMachine* this_ptr, const QState::RestorePolicy* restorePolicy);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_setRunning(QStateMachine* this_ptr, bool running);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_start(QStateMachine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_stop(QStateMachine* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStateMachine_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSTATEMACHINE_H
