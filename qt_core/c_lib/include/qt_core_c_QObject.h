#ifndef QT_CORE_C_QOBJECT_H
#define QT_CORE_C_QOBJECT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QObject_G_operator_shl_to_output(const QDebug* arg1, const QObject* arg2, QDebug* output);
QT_CORE_C_EXPORT bool qt_core_c_QObject_blockSignals(QObject* this_ptr, bool b);
QT_CORE_C_EXPORT const QList< QObject* >* qt_core_c_QObject_children(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_connect_to_output_sender_signal_member(const QObject* this_ptr, const QObject* sender, const char* signal, const char* member, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_connect_to_output_sender_signal_member_type(const QObject* this_ptr, const QObject* sender, const char* signal, const char* member, const Qt::ConnectionType* type, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_connect_to_output_sender_signal_receiver_member(const QObject* sender, const char* signal, const QObject* receiver, const char* member, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_connect_to_output_sender_signal_receiver_member_arg5(const QObject* sender, const char* signal, const QObject* receiver, const char* member, const Qt::ConnectionType* arg5, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_connect_to_output_sender_signal_receiver_method(const QObject* sender, const QMetaMethod* signal, const QObject* receiver, const QMetaMethod* method, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_connect_to_output_sender_signal_receiver_method_type(const QObject* sender, const QMetaMethod* signal, const QObject* receiver, const QMetaMethod* method, const Qt::ConnectionType* type, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_delete(QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_deleteLater(QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_destroyed_arg1(QObject* this_ptr, QObject* arg1);
QT_CORE_C_EXPORT void qt_core_c_QObject_destroyed_no_args(QObject* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_QMetaObject_Connection(const QMetaObject::Connection* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_QObject(const QObject* this_ptr, const QObject* receiver);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_QObject_QMetaMethod_QObject_QMetaMethod(const QObject* sender, const QMetaMethod* signal, const QObject* receiver, const QMetaMethod* member);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_QObject_char(const QObject* this_ptr, const QObject* receiver, const char* member);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_QObject_char_QObject_char(const QObject* sender, const char* signal, const QObject* receiver, const char* member);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_char(const QObject* this_ptr, const char* signal);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_char_QObject(const QObject* this_ptr, const char* signal, const QObject* receiver);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_char_QObject_char(const QObject* this_ptr, const char* signal, const QObject* receiver, const char* member);
QT_CORE_C_EXPORT bool qt_core_c_QObject_disconnect_no_args(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_dumpObjectInfo(QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_dumpObjectInfo_const(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_dumpObjectTree(QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_dumpObjectTree_const(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_dynamicPropertyNames_to_output(const QObject* this_ptr, QList< QByteArray >* output);
QT_CORE_C_EXPORT bool qt_core_c_QObject_event(QObject* this_ptr, QEvent* event);
QT_CORE_C_EXPORT bool qt_core_c_QObject_eventFilter(QObject* this_ptr, QObject* watched, QEvent* event);
QT_CORE_C_EXPORT bool qt_core_c_QObject_inherits(const QObject* this_ptr, const char* classname);
QT_CORE_C_EXPORT void qt_core_c_QObject_installEventFilter(QObject* this_ptr, QObject* filterObj);
QT_CORE_C_EXPORT bool qt_core_c_QObject_isWidgetType(const QObject* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QObject_isWindowType(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_killTimer(QObject* this_ptr, int id);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QObject_metaObject(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_moveToThread(QObject* this_ptr, QThread* thread);
QT_CORE_C_EXPORT QObject* qt_core_c_QObject_new_no_args();
QT_CORE_C_EXPORT QObject* qt_core_c_QObject_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QObject_objectName_to_output(const QObject* this_ptr, QString* output);
QT_CORE_C_EXPORT QObject* qt_core_c_QObject_parent(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_property_to_output(const QObject* this_ptr, const char* name, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_removeEventFilter(QObject* this_ptr, QObject* obj);
QT_CORE_C_EXPORT void qt_core_c_QObject_setObjectName(QObject* this_ptr, const QString* name);
QT_CORE_C_EXPORT void qt_core_c_QObject_setParent(QObject* this_ptr, QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QObject_setProperty(QObject* this_ptr, const char* name, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QObject_signalsBlocked(const QObject* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QObject_startTimer_interval(QObject* this_ptr, int interval);
QT_CORE_C_EXPORT int qt_core_c_QObject_startTimer_interval_timerType(QObject* this_ptr, int interval, const Qt::TimerType* timerType);
QT_CORE_C_EXPORT QThread* qt_core_c_QObject_thread(const QObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QObject_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QOBJECT_H
