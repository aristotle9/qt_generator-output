#include "qt_core_c_QCoreApplication.h"

QCoreApplication* qt_core_c_QCoreApplication_G_dynamic_cast_QCoreApplication_ptr(QObject* ptr) {
  return dynamic_cast<QCoreApplication*>(ptr);
}

void qt_core_c_QCoreApplication_G_qAddPostRoutine(void (*arg1)()) {
  qAddPostRoutine(arg1);
}

void qt_core_c_QCoreApplication_G_qAddPreRoutine(void (*arg1)()) {
  qAddPreRoutine(arg1);
}

void qt_core_c_QCoreApplication_G_qRemovePostRoutine(void (*arg1)()) {
  qRemovePostRoutine(arg1);
}

QCoreApplication* qt_core_c_QCoreApplication_G_static_cast_QCoreApplication_ptr(QObject* ptr) {
  return static_cast<QCoreApplication*>(ptr);
}

QObject* qt_core_c_QCoreApplication_G_static_cast_QObject_ptr(QCoreApplication* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QCoreApplication_addLibraryPath(const QString* arg1) {
  QCoreApplication::addLibraryPath(*arg1);
}

void qt_core_c_QCoreApplication_applicationDirPath_to_output(QString* output) {
  new(output) QString(QCoreApplication::applicationDirPath());
}

void qt_core_c_QCoreApplication_applicationFilePath_to_output(QString* output) {
  new(output) QString(QCoreApplication::applicationFilePath());
}

void qt_core_c_QCoreApplication_applicationName_to_output(QString* output) {
  new(output) QString(QCoreApplication::applicationName());
}

qint64 qt_core_c_QCoreApplication_applicationPid() {
  return QCoreApplication::applicationPid();
}

void qt_core_c_QCoreApplication_applicationVersion_to_output(QString* output) {
  new(output) QString(QCoreApplication::applicationVersion());
}

void qt_core_c_QCoreApplication_arguments_to_output(QStringList* output) {
  new(output) QStringList(QCoreApplication::arguments());
}

bool qt_core_c_QCoreApplication_closingDown() {
  return QCoreApplication::closingDown();
}

void qt_core_c_QCoreApplication_delete(QCoreApplication* this_ptr) {
  delete this_ptr;
}

QAbstractEventDispatcher* qt_core_c_QCoreApplication_eventDispatcher() {
  return QCoreApplication::eventDispatcher();
}

int qt_core_c_QCoreApplication_exec() {
  return QCoreApplication::exec();
}

void qt_core_c_QCoreApplication_exit_no_args() {
  QCoreApplication::exit();
}

void qt_core_c_QCoreApplication_exit_retcode(int retcode) {
  QCoreApplication::exit(retcode);
}

void qt_core_c_QCoreApplication_flush() {
  QCoreApplication::flush();
}

bool qt_core_c_QCoreApplication_hasPendingEvents() {
  return QCoreApplication::hasPendingEvents();
}

void qt_core_c_QCoreApplication_installNativeEventFilter(QCoreApplication* this_ptr, QAbstractNativeEventFilter* filterObj) {
  this_ptr->installNativeEventFilter(filterObj);
}

bool qt_core_c_QCoreApplication_installTranslator(QTranslator* messageFile) {
  return QCoreApplication::installTranslator(messageFile);
}

QCoreApplication* qt_core_c_QCoreApplication_instance() {
  return QCoreApplication::instance();
}

bool qt_core_c_QCoreApplication_isQuitLockEnabled() {
  return QCoreApplication::isQuitLockEnabled();
}

bool qt_core_c_QCoreApplication_isSetuidAllowed() {
  return QCoreApplication::isSetuidAllowed();
}

void qt_core_c_QCoreApplication_libraryPaths_to_output(QStringList* output) {
  new(output) QStringList(QCoreApplication::libraryPaths());
}

const QMetaObject* qt_core_c_QCoreApplication_metaObject(const QCoreApplication* this_ptr) {
  return this_ptr->metaObject();
}

QCoreApplication* qt_core_c_QCoreApplication_new_argc_argv(int* argc, char** argv) {
  return new QCoreApplication(*argc, argv);
}

QCoreApplication* qt_core_c_QCoreApplication_new_argc_argv_arg3(int* argc, char** argv, int arg3) {
  return new QCoreApplication(*argc, argv, arg3);
}

bool qt_core_c_QCoreApplication_notify(QCoreApplication* this_ptr, QObject* arg1, QEvent* arg2) {
  return this_ptr->notify(arg1, arg2);
}

void qt_core_c_QCoreApplication_organizationDomain_to_output(QString* output) {
  new(output) QString(QCoreApplication::organizationDomain());
}

void qt_core_c_QCoreApplication_organizationName_to_output(QString* output) {
  new(output) QString(QCoreApplication::organizationName());
}

void qt_core_c_QCoreApplication_postEvent_receiver_event(QObject* receiver, QEvent* event) {
  QCoreApplication::postEvent(receiver, event);
}

void qt_core_c_QCoreApplication_postEvent_receiver_event_priority(QObject* receiver, QEvent* event, int priority) {
  QCoreApplication::postEvent(receiver, event, priority);
}

void qt_core_c_QCoreApplication_quit() {
  QCoreApplication::quit();
}

void qt_core_c_QCoreApplication_removeLibraryPath(const QString* arg1) {
  QCoreApplication::removeLibraryPath(*arg1);
}

void qt_core_c_QCoreApplication_removeNativeEventFilter(QCoreApplication* this_ptr, QAbstractNativeEventFilter* filterObj) {
  this_ptr->removeNativeEventFilter(filterObj);
}

void qt_core_c_QCoreApplication_removePostedEvents_receiver(QObject* receiver) {
  QCoreApplication::removePostedEvents(receiver);
}

void qt_core_c_QCoreApplication_removePostedEvents_receiver_eventType(QObject* receiver, int eventType) {
  QCoreApplication::removePostedEvents(receiver, eventType);
}

bool qt_core_c_QCoreApplication_removeTranslator(QTranslator* messageFile) {
  return QCoreApplication::removeTranslator(messageFile);
}

bool qt_core_c_QCoreApplication_sendEvent(QObject* receiver, QEvent* event) {
  return QCoreApplication::sendEvent(receiver, event);
}

void qt_core_c_QCoreApplication_sendPostedEvents_no_args() {
  QCoreApplication::sendPostedEvents();
}

void qt_core_c_QCoreApplication_sendPostedEvents_receiver(QObject* receiver) {
  QCoreApplication::sendPostedEvents(receiver);
}

void qt_core_c_QCoreApplication_sendPostedEvents_receiver_event_type(QObject* receiver, int event_type) {
  QCoreApplication::sendPostedEvents(receiver, event_type);
}

void qt_core_c_QCoreApplication_setApplicationName(const QString* application) {
  QCoreApplication::setApplicationName(*application);
}

void qt_core_c_QCoreApplication_setApplicationVersion(const QString* version) {
  QCoreApplication::setApplicationVersion(*version);
}

void qt_core_c_QCoreApplication_setAttribute_attribute(const Qt::ApplicationAttribute* attribute) {
  QCoreApplication::setAttribute(*attribute);
}

void qt_core_c_QCoreApplication_setAttribute_attribute_on(const Qt::ApplicationAttribute* attribute, bool on) {
  QCoreApplication::setAttribute(*attribute, on);
}

void qt_core_c_QCoreApplication_setEventDispatcher(QAbstractEventDispatcher* eventDispatcher) {
  QCoreApplication::setEventDispatcher(eventDispatcher);
}

void qt_core_c_QCoreApplication_setLibraryPaths(const QStringList* arg1) {
  QCoreApplication::setLibraryPaths(*arg1);
}

void qt_core_c_QCoreApplication_setOrganizationDomain(const QString* orgDomain) {
  QCoreApplication::setOrganizationDomain(*orgDomain);
}

void qt_core_c_QCoreApplication_setOrganizationName(const QString* orgName) {
  QCoreApplication::setOrganizationName(*orgName);
}

void qt_core_c_QCoreApplication_setQuitLockEnabled(bool enabled) {
  QCoreApplication::setQuitLockEnabled(enabled);
}

void qt_core_c_QCoreApplication_setSetuidAllowed(bool allow) {
  QCoreApplication::setSetuidAllowed(allow);
}

bool qt_core_c_QCoreApplication_startingUp() {
  return QCoreApplication::startingUp();
}

bool qt_core_c_QCoreApplication_testAttribute(const Qt::ApplicationAttribute* attribute) {
  return QCoreApplication::testAttribute(*attribute);
}

void qt_core_c_QCoreApplication_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCoreApplication::trUtf8(s, c, n));
}

void qt_core_c_QCoreApplication_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCoreApplication::tr(s, c, n));
}

void qt_core_c_QCoreApplication_translate_to_output_context_key(const char* context, const char* key, QString* output) {
  new(output) QString(QCoreApplication::translate(context, key));
}

void qt_core_c_QCoreApplication_translate_to_output_context_key_disambiguation(const char* context, const char* key, const char* disambiguation, QString* output) {
  new(output) QString(QCoreApplication::translate(context, key, disambiguation));
}

void qt_core_c_QCoreApplication_translate_to_output_context_key_disambiguation_n(const char* context, const char* key, const char* disambiguation, int n, QString* output) {
  new(output) QString(QCoreApplication::translate(context, key, disambiguation, n));
}

