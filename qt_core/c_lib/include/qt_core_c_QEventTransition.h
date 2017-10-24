#ifndef QT_CORE_C_QEVENTTRANSITION_H
#define QT_CORE_C_QEVENTTRANSITION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr);
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractTransition* qt_core_c_QEventTransition_G_static_cast_QAbstractTransition_ptr(QEventTransition* ptr);
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr);
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QEventTransition_G_static_cast_QObject_ptr(QEventTransition* ptr);
QT_CORE_C_EXPORT void qt_core_c_QEventTransition_delete(QEventTransition* this_ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QEventTransition_eventSource(const QEventTransition* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QEventTransition_metaObject(const QEventTransition* this_ptr);
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_new_no_args();
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_new_object_type(QObject* object, const QEvent::Type* type);
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_new_object_type_sourceState(QObject* object, const QEvent::Type* type, QState* sourceState);
QT_CORE_C_EXPORT QEventTransition* qt_core_c_QEventTransition_new_sourceState(QState* sourceState);
QT_CORE_C_EXPORT void qt_core_c_QEventTransition_setEventSource(QEventTransition* this_ptr, QObject* object);
QT_CORE_C_EXPORT void qt_core_c_QEventTransition_setEventType(QEventTransition* this_ptr, const QEvent::Type* type);
QT_CORE_C_EXPORT void qt_core_c_QEventTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QEventTransition_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QEVENTTRANSITION_H
