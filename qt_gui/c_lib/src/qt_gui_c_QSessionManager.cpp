#include "qt_gui_c_QSessionManager.h"

QObject* qt_gui_c_QSessionManager_G_static_cast_QObject_ptr(QSessionManager* ptr) {
  return static_cast<QObject*>(ptr);
}

QSessionManager* qt_gui_c_QSessionManager_G_static_cast_QSessionManager_ptr(QObject* ptr) {
  return static_cast<QSessionManager*>(ptr);
}

bool qt_gui_c_QSessionManager_allowsErrorInteraction(QSessionManager* this_ptr) {
  return this_ptr->allowsErrorInteraction();
}

bool qt_gui_c_QSessionManager_allowsInteraction(QSessionManager* this_ptr) {
  return this_ptr->allowsInteraction();
}

void qt_gui_c_QSessionManager_cancel(QSessionManager* this_ptr) {
  this_ptr->cancel();
}

void qt_gui_c_QSessionManager_discardCommand_to_output(const QSessionManager* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->discardCommand());
}

bool qt_gui_c_QSessionManager_isPhase2(const QSessionManager* this_ptr) {
  return this_ptr->isPhase2();
}

const QMetaObject* qt_gui_c_QSessionManager_metaObject(const QSessionManager* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QSessionManager_qt_metacall(QSessionManager* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QSessionManager_qt_metacast(QSessionManager* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QSessionManager_release(QSessionManager* this_ptr) {
  this_ptr->release();
}

void qt_gui_c_QSessionManager_requestPhase2(QSessionManager* this_ptr) {
  this_ptr->requestPhase2();
}

void qt_gui_c_QSessionManager_restartCommand_to_output(const QSessionManager* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->restartCommand());
}

QSessionManager::RestartHint qt_gui_c_QSessionManager_restartHint(const QSessionManager* this_ptr) {
  return this_ptr->restartHint();
}

void qt_gui_c_QSessionManager_sessionId_to_output(const QSessionManager* this_ptr, QString* output) {
  new(output) QString(this_ptr->sessionId());
}

void qt_gui_c_QSessionManager_sessionKey_to_output(const QSessionManager* this_ptr, QString* output) {
  new(output) QString(this_ptr->sessionKey());
}

void qt_gui_c_QSessionManager_setDiscardCommand(QSessionManager* this_ptr, const QStringList* arg1) {
  this_ptr->setDiscardCommand(*arg1);
}

void qt_gui_c_QSessionManager_setManagerProperty_QString_QString(QSessionManager* this_ptr, const QString* name, const QString* value) {
  this_ptr->setManagerProperty(*name, *value);
}

void qt_gui_c_QSessionManager_setManagerProperty_QString_QStringList(QSessionManager* this_ptr, const QString* name, const QStringList* value) {
  this_ptr->setManagerProperty(*name, *value);
}

void qt_gui_c_QSessionManager_setRestartCommand(QSessionManager* this_ptr, const QStringList* arg1) {
  this_ptr->setRestartCommand(*arg1);
}

void qt_gui_c_QSessionManager_setRestartHint(QSessionManager* this_ptr, QSessionManager::RestartHint arg1) {
  this_ptr->setRestartHint(arg1);
}

void qt_gui_c_QSessionManager_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSessionManager::trUtf8(s, c, n));
}

void qt_gui_c_QSessionManager_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSessionManager::tr(s, c, n));
}

