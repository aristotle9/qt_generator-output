#ifndef QT_CORE_C_QABSTRACTTRANSITION_H
#define QT_CORE_C_QABSTRACTTRANSITION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAbstractTransition* qt_core_c_QAbstractTransition_G_dynamic_cast_QAbstractTransition_ptr(QObject* ptr);
QT_CORE_C_EXPORT QAbstractTransition* qt_core_c_QAbstractTransition_G_static_cast_QAbstractTransition_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAbstractTransition_G_static_cast_QObject_ptr(QAbstractTransition* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_addAnimation(QAbstractTransition* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_animations_to_output(const QAbstractTransition* this_ptr, QList< QAbstractAnimation* >* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_delete(QAbstractTransition* this_ptr);
QT_CORE_C_EXPORT QStateMachine* qt_core_c_QAbstractTransition_machine(const QAbstractTransition* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAbstractTransition_metaObject(const QAbstractTransition* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_removeAnimation(QAbstractTransition* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_setTargetState(QAbstractTransition* this_ptr, QAbstractState* target);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_setTargetStates(QAbstractTransition* this_ptr, const QList< QAbstractState* >* targets);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_setTransitionType(QAbstractTransition* this_ptr, QAbstractTransition::TransitionType type);
QT_CORE_C_EXPORT QState* qt_core_c_QAbstractTransition_sourceState(const QAbstractTransition* this_ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QAbstractTransition_targetState(const QAbstractTransition* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_targetStates_to_output(const QAbstractTransition* this_ptr, QList< QAbstractState* >* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAbstractTransition_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT QAbstractTransition::TransitionType qt_core_c_QAbstractTransition_transitionType(const QAbstractTransition* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QABSTRACTTRANSITION_H
