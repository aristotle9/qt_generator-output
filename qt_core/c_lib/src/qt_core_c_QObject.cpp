#include "qt_core_c_QObject.h"

void qt_core_c_QObject_G_operator_shl_to_output(const QDebug* arg1, const QObject* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

bool qt_core_c_QObject_blockSignals(QObject* this_ptr, bool b) {
  return this_ptr->blockSignals(b);
}

const QList< QObject* >* qt_core_c_QObject_children(const QObject* this_ptr) {
  return &this_ptr->children();
}

void qt_core_c_QObject_connect_to_output_sender_signal_member(const QObject* this_ptr, const QObject* sender, const char* signal, const char* member, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(this_ptr->connect(sender, signal, member));
}

void qt_core_c_QObject_connect_to_output_sender_signal_member_type(const QObject* this_ptr, const QObject* sender, const char* signal, const char* member, const Qt::ConnectionType* type, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(this_ptr->connect(sender, signal, member, *type));
}

void qt_core_c_QObject_connect_to_output_sender_signal_receiver_member(const QObject* sender, const char* signal, const QObject* receiver, const char* member, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QObject::connect(sender, signal, receiver, member));
}

void qt_core_c_QObject_connect_to_output_sender_signal_receiver_member_arg5(const QObject* sender, const char* signal, const QObject* receiver, const char* member, const Qt::ConnectionType* arg5, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QObject::connect(sender, signal, receiver, member, *arg5));
}

void qt_core_c_QObject_connect_to_output_sender_signal_receiver_method(const QObject* sender, const QMetaMethod* signal, const QObject* receiver, const QMetaMethod* method, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QObject::connect(sender, *signal, receiver, *method));
}

void qt_core_c_QObject_connect_to_output_sender_signal_receiver_method_type(const QObject* sender, const QMetaMethod* signal, const QObject* receiver, const QMetaMethod* method, const Qt::ConnectionType* type, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QObject::connect(sender, *signal, receiver, *method, *type));
}

void qt_core_c_QObject_delete(QObject* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QObject_deleteLater(QObject* this_ptr) {
  this_ptr->deleteLater();
}

void qt_core_c_QObject_destroyed_arg1(QObject* this_ptr, QObject* arg1) {
  this_ptr->destroyed(arg1);
}

void qt_core_c_QObject_destroyed_no_args(QObject* this_ptr) {
  this_ptr->destroyed();
}

bool qt_core_c_QObject_disconnect_QMetaObject_Connection(const QMetaObject::Connection* arg1) {
  return QObject::disconnect(*arg1);
}

bool qt_core_c_QObject_disconnect_QObject(const QObject* this_ptr, const QObject* receiver) {
  return this_ptr->disconnect(receiver);
}

bool qt_core_c_QObject_disconnect_QObject_QMetaMethod_QObject_QMetaMethod(const QObject* sender, const QMetaMethod* signal, const QObject* receiver, const QMetaMethod* member) {
  return QObject::disconnect(sender, *signal, receiver, *member);
}

bool qt_core_c_QObject_disconnect_QObject_char(const QObject* this_ptr, const QObject* receiver, const char* member) {
  return this_ptr->disconnect(receiver, member);
}

bool qt_core_c_QObject_disconnect_QObject_char_QObject_char(const QObject* sender, const char* signal, const QObject* receiver, const char* member) {
  return QObject::disconnect(sender, signal, receiver, member);
}

bool qt_core_c_QObject_disconnect_char(const QObject* this_ptr, const char* signal) {
  return this_ptr->disconnect(signal);
}

bool qt_core_c_QObject_disconnect_char_QObject(const QObject* this_ptr, const char* signal, const QObject* receiver) {
  return this_ptr->disconnect(signal, receiver);
}

bool qt_core_c_QObject_disconnect_char_QObject_char(const QObject* this_ptr, const char* signal, const QObject* receiver, const char* member) {
  return this_ptr->disconnect(signal, receiver, member);
}

bool qt_core_c_QObject_disconnect_no_args(const QObject* this_ptr) {
  return this_ptr->disconnect();
}

void qt_core_c_QObject_dumpObjectInfo(QObject* this_ptr) {
  this_ptr->dumpObjectInfo();
}

void qt_core_c_QObject_dumpObjectInfo_const(const QObject* this_ptr) {
  this_ptr->dumpObjectInfo();
}

void qt_core_c_QObject_dumpObjectTree(QObject* this_ptr) {
  this_ptr->dumpObjectTree();
}

void qt_core_c_QObject_dumpObjectTree_const(const QObject* this_ptr) {
  this_ptr->dumpObjectTree();
}

void qt_core_c_QObject_dynamicPropertyNames_to_output(const QObject* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->dynamicPropertyNames());
}

bool qt_core_c_QObject_event(QObject* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

bool qt_core_c_QObject_eventFilter(QObject* this_ptr, QObject* watched, QEvent* event) {
  return this_ptr->eventFilter(watched, event);
}

bool qt_core_c_QObject_inherits(const QObject* this_ptr, const char* classname) {
  return this_ptr->inherits(classname);
}

void qt_core_c_QObject_installEventFilter(QObject* this_ptr, QObject* filterObj) {
  this_ptr->installEventFilter(filterObj);
}

bool qt_core_c_QObject_isWidgetType(const QObject* this_ptr) {
  return this_ptr->isWidgetType();
}

bool qt_core_c_QObject_isWindowType(const QObject* this_ptr) {
  return this_ptr->isWindowType();
}

void qt_core_c_QObject_killTimer(QObject* this_ptr, int id) {
  this_ptr->killTimer(id);
}

const QMetaObject* qt_core_c_QObject_metaObject(const QObject* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QObject_moveToThread(QObject* this_ptr, QThread* thread) {
  this_ptr->moveToThread(thread);
}

QObject* qt_core_c_QObject_new_no_args() {
  return new QObject();
}

QObject* qt_core_c_QObject_new_parent(QObject* parent) {
  return new QObject(parent);
}

void qt_core_c_QObject_objectName_to_output(const QObject* this_ptr, QString* output) {
  new(output) QString(this_ptr->objectName());
}

QObject* qt_core_c_QObject_parent(const QObject* this_ptr) {
  return this_ptr->parent();
}

void qt_core_c_QObject_property_to_output(const QObject* this_ptr, const char* name, QVariant* output) {
  new(output) QVariant(this_ptr->property(name));
}

void qt_core_c_QObject_removeEventFilter(QObject* this_ptr, QObject* obj) {
  this_ptr->removeEventFilter(obj);
}

void qt_core_c_QObject_setObjectName(QObject* this_ptr, const QString* name) {
  this_ptr->setObjectName(*name);
}

void qt_core_c_QObject_setParent(QObject* this_ptr, QObject* parent) {
  this_ptr->setParent(parent);
}

bool qt_core_c_QObject_setProperty(QObject* this_ptr, const char* name, const QVariant* value) {
  return this_ptr->setProperty(name, *value);
}

bool qt_core_c_QObject_signalsBlocked(const QObject* this_ptr) {
  return this_ptr->signalsBlocked();
}

int qt_core_c_QObject_startTimer_interval(QObject* this_ptr, int interval) {
  return this_ptr->startTimer(interval);
}

int qt_core_c_QObject_startTimer_interval_timerType(QObject* this_ptr, int interval, const Qt::TimerType* timerType) {
  return this_ptr->startTimer(interval, *timerType);
}

QThread* qt_core_c_QObject_thread(const QObject* this_ptr) {
  return this_ptr->thread();
}

void qt_core_c_QObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QObject::trUtf8(s, c, n));
}

void qt_core_c_QObject_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QObject::tr(s, c, n));
}

