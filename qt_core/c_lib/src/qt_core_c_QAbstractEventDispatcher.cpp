#include "qt_core_c_QAbstractEventDispatcher.h"

QAbstractEventDispatcher* qt_core_c_QAbstractEventDispatcher_G_dynamic_cast_QAbstractEventDispatcher_ptr(QObject* ptr) {
  return dynamic_cast<QAbstractEventDispatcher*>(ptr);
}

QAbstractEventDispatcher* qt_core_c_QAbstractEventDispatcher_G_static_cast_QAbstractEventDispatcher_ptr(QObject* ptr) {
  return static_cast<QAbstractEventDispatcher*>(ptr);
}

QObject* qt_core_c_QAbstractEventDispatcher_G_static_cast_QObject_ptr(QAbstractEventDispatcher* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QAbstractEventDispatcher_TimerInfo_constructor(int id, int i, const Qt::TimerType* t, QAbstractEventDispatcher::TimerInfo* output) {
  new(output) QAbstractEventDispatcher::TimerInfo(id, i, *t);
}

void qt_core_c_QAbstractEventDispatcher_TimerInfo_destructor(QAbstractEventDispatcher::TimerInfo* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QAbstractEventDispatcher_TimerInfo_interval(const QAbstractEventDispatcher::TimerInfo* this_ptr) {
  return this_ptr->interval;
}

void qt_core_c_QAbstractEventDispatcher_TimerInfo_set_interval(QAbstractEventDispatcher::TimerInfo* this_ptr, int value) {
  this_ptr->interval = value;
}

void qt_core_c_QAbstractEventDispatcher_TimerInfo_set_timerId(QAbstractEventDispatcher::TimerInfo* this_ptr, int value) {
  this_ptr->timerId = value;
}

void qt_core_c_QAbstractEventDispatcher_TimerInfo_set_timerType(QAbstractEventDispatcher::TimerInfo* this_ptr, const Qt::TimerType* value) {
  this_ptr->timerType = *value;
}

int qt_core_c_QAbstractEventDispatcher_TimerInfo_timerId(const QAbstractEventDispatcher::TimerInfo* this_ptr) {
  return this_ptr->timerId;
}

const Qt::TimerType* qt_core_c_QAbstractEventDispatcher_TimerInfo_timerType(const QAbstractEventDispatcher::TimerInfo* this_ptr) {
  return &this_ptr->timerType;
}

Qt::TimerType* qt_core_c_QAbstractEventDispatcher_TimerInfo_timerType_mut(QAbstractEventDispatcher::TimerInfo* this_ptr) {
  return &this_ptr->timerType;
}

void qt_core_c_QAbstractEventDispatcher_closingDown(QAbstractEventDispatcher* this_ptr) {
  this_ptr->closingDown();
}

void qt_core_c_QAbstractEventDispatcher_delete(QAbstractEventDispatcher* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QAbstractEventDispatcher_filterNativeEvent(QAbstractEventDispatcher* this_ptr, const QByteArray* eventType, void* message, long* result) {
  return this_ptr->filterNativeEvent(*eventType, message, result);
}

void qt_core_c_QAbstractEventDispatcher_flush(QAbstractEventDispatcher* this_ptr) {
  this_ptr->flush();
}

bool qt_core_c_QAbstractEventDispatcher_hasPendingEvents(QAbstractEventDispatcher* this_ptr) {
  return this_ptr->hasPendingEvents();
}

void qt_core_c_QAbstractEventDispatcher_installNativeEventFilter(QAbstractEventDispatcher* this_ptr, QAbstractNativeEventFilter* filterObj) {
  this_ptr->installNativeEventFilter(filterObj);
}

QAbstractEventDispatcher* qt_core_c_QAbstractEventDispatcher_instance_no_args() {
  return QAbstractEventDispatcher::instance();
}

QAbstractEventDispatcher* qt_core_c_QAbstractEventDispatcher_instance_thread(QThread* thread) {
  return QAbstractEventDispatcher::instance(thread);
}

void qt_core_c_QAbstractEventDispatcher_interrupt(QAbstractEventDispatcher* this_ptr) {
  this_ptr->interrupt();
}

const QMetaObject* qt_core_c_QAbstractEventDispatcher_metaObject(const QAbstractEventDispatcher* this_ptr) {
  return this_ptr->metaObject();
}

void qt_core_c_QAbstractEventDispatcher_registerSocketNotifier(QAbstractEventDispatcher* this_ptr, QSocketNotifier* notifier) {
  this_ptr->registerSocketNotifier(notifier);
}

int qt_core_c_QAbstractEventDispatcher_registerTimer_interval_timerType_object(QAbstractEventDispatcher* this_ptr, int interval, const Qt::TimerType* timerType, QObject* object) {
  return this_ptr->registerTimer(interval, *timerType, object);
}

void qt_core_c_QAbstractEventDispatcher_registerTimer_timerId_interval_timerType_object(QAbstractEventDispatcher* this_ptr, int timerId, int interval, const Qt::TimerType* timerType, QObject* object) {
  this_ptr->registerTimer(timerId, interval, *timerType, object);
}

void qt_core_c_QAbstractEventDispatcher_registeredTimers_to_output(const QAbstractEventDispatcher* this_ptr, QObject* object, QList< QAbstractEventDispatcher::TimerInfo >* output) {
  new(output) QList< QAbstractEventDispatcher::TimerInfo >(this_ptr->registeredTimers(object));
}

int qt_core_c_QAbstractEventDispatcher_remainingTime(QAbstractEventDispatcher* this_ptr, int timerId) {
  return this_ptr->remainingTime(timerId);
}

void qt_core_c_QAbstractEventDispatcher_removeNativeEventFilter(QAbstractEventDispatcher* this_ptr, QAbstractNativeEventFilter* filterObj) {
  this_ptr->removeNativeEventFilter(filterObj);
}

void qt_core_c_QAbstractEventDispatcher_startingUp(QAbstractEventDispatcher* this_ptr) {
  this_ptr->startingUp();
}

void qt_core_c_QAbstractEventDispatcher_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractEventDispatcher::trUtf8(s, c, n));
}

void qt_core_c_QAbstractEventDispatcher_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractEventDispatcher::tr(s, c, n));
}

void qt_core_c_QAbstractEventDispatcher_unregisterSocketNotifier(QAbstractEventDispatcher* this_ptr, QSocketNotifier* notifier) {
  this_ptr->unregisterSocketNotifier(notifier);
}

bool qt_core_c_QAbstractEventDispatcher_unregisterTimer(QAbstractEventDispatcher* this_ptr, int timerId) {
  return this_ptr->unregisterTimer(timerId);
}

bool qt_core_c_QAbstractEventDispatcher_unregisterTimers(QAbstractEventDispatcher* this_ptr, QObject* object) {
  return this_ptr->unregisterTimers(object);
}

void qt_core_c_QAbstractEventDispatcher_wakeUp(QAbstractEventDispatcher* this_ptr) {
  this_ptr->wakeUp();
}

