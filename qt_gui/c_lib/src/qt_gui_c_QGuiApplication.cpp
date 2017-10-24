#include "qt_gui_c_QGuiApplication.h"

QCoreApplication* qt_gui_c_QGuiApplication_G_static_cast_QCoreApplication_ptr(QGuiApplication* ptr) {
  return static_cast<QCoreApplication*>(ptr);
}

QGuiApplication* qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QCoreApplication(QCoreApplication* ptr) {
  return static_cast<QGuiApplication*>(ptr);
}

QGuiApplication* qt_gui_c_QGuiApplication_G_static_cast_QGuiApplication_ptr_QObject(QObject* ptr) {
  return static_cast<QGuiApplication*>(ptr);
}

QObject* qt_gui_c_QGuiApplication_G_static_cast_QObject_ptr(QGuiApplication* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_gui_c_QGuiApplication_allWindows_to_output(QList< QWindow* >* output) {
  new(output) QList< QWindow* >(QGuiApplication::allWindows());
}

void qt_gui_c_QGuiApplication_applicationDisplayName_to_output(QString* output) {
  new(output) QString(QGuiApplication::applicationDisplayName());
}

void qt_gui_c_QGuiApplication_changeOverrideCursor(const QCursor* arg1) {
  QGuiApplication::changeOverrideCursor(*arg1);
}

QClipboard* qt_gui_c_QGuiApplication_clipboard() {
  return QGuiApplication::clipboard();
}

void qt_gui_c_QGuiApplication_delete(QGuiApplication* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QGuiApplication_desktopFileName_to_output(QString* output) {
  new(output) QString(QGuiApplication::desktopFileName());
}

bool qt_gui_c_QGuiApplication_desktopSettingsAware() {
  return QGuiApplication::desktopSettingsAware();
}

double qt_gui_c_QGuiApplication_devicePixelRatio(const QGuiApplication* this_ptr) {
  return this_ptr->devicePixelRatio();
}

int qt_gui_c_QGuiApplication_exec() {
  return QGuiApplication::exec();
}

QObject* qt_gui_c_QGuiApplication_focusObject() {
  return QGuiApplication::focusObject();
}

QWindow* qt_gui_c_QGuiApplication_focusWindow() {
  return QGuiApplication::focusWindow();
}

void qt_gui_c_QGuiApplication_font_to_output(QFont* output) {
  new(output) QFont(QGuiApplication::font());
}

QInputMethod* qt_gui_c_QGuiApplication_inputMethod() {
  return QGuiApplication::inputMethod();
}

bool qt_gui_c_QGuiApplication_isFallbackSessionManagementEnabled() {
  return QGuiApplication::isFallbackSessionManagementEnabled();
}

bool qt_gui_c_QGuiApplication_isLeftToRight() {
  return QGuiApplication::isLeftToRight();
}

bool qt_gui_c_QGuiApplication_isRightToLeft() {
  return QGuiApplication::isRightToLeft();
}

bool qt_gui_c_QGuiApplication_isSavingSession(const QGuiApplication* this_ptr) {
  return this_ptr->isSavingSession();
}

bool qt_gui_c_QGuiApplication_isSessionRestored(const QGuiApplication* this_ptr) {
  return this_ptr->isSessionRestored();
}

const QMetaObject* qt_gui_c_QGuiApplication_metaObject(const QGuiApplication* this_ptr) {
  return this_ptr->metaObject();
}

QWindow* qt_gui_c_QGuiApplication_modalWindow() {
  return QGuiApplication::modalWindow();
}

QGuiApplication* qt_gui_c_QGuiApplication_new_argc_argv(int* argc, char** argv) {
  return new QGuiApplication(*argc, argv);
}

QGuiApplication* qt_gui_c_QGuiApplication_new_argc_argv_arg3(int* argc, char** argv, int arg3) {
  return new QGuiApplication(*argc, argv, arg3);
}

bool qt_gui_c_QGuiApplication_notify(QGuiApplication* this_ptr, QObject* arg1, QEvent* arg2) {
  return this_ptr->notify(arg1, arg2);
}

QCursor* qt_gui_c_QGuiApplication_overrideCursor() {
  return QGuiApplication::overrideCursor();
}

void qt_gui_c_QGuiApplication_palette_to_output(QPalette* output) {
  new(output) QPalette(QGuiApplication::palette());
}

void (*qt_gui_c_QGuiApplication_platformFunction(const QByteArray* function))() {
  return QGuiApplication::platformFunction(*function);
}

void qt_gui_c_QGuiApplication_platformName_to_output(QString* output) {
  new(output) QString(QGuiApplication::platformName());
}

QScreen* qt_gui_c_QGuiApplication_primaryScreen() {
  return QGuiApplication::primaryScreen();
}

int qt_gui_c_QGuiApplication_qt_metacall(QGuiApplication* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QGuiApplication_qt_metacast(QGuiApplication* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_gui_c_QGuiApplication_quitOnLastWindowClosed() {
  return QGuiApplication::quitOnLastWindowClosed();
}

void qt_gui_c_QGuiApplication_restoreOverrideCursor() {
  QGuiApplication::restoreOverrideCursor();
}

void qt_gui_c_QGuiApplication_screens_to_output(QList< QScreen* >* output) {
  new(output) QList< QScreen* >(QGuiApplication::screens());
}

void qt_gui_c_QGuiApplication_sessionId_to_output(const QGuiApplication* this_ptr, QString* output) {
  new(output) QString(this_ptr->sessionId());
}

void qt_gui_c_QGuiApplication_sessionKey_to_output(const QGuiApplication* this_ptr, QString* output) {
  new(output) QString(this_ptr->sessionKey());
}

void qt_gui_c_QGuiApplication_setApplicationDisplayName(const QString* name) {
  QGuiApplication::setApplicationDisplayName(*name);
}

void qt_gui_c_QGuiApplication_setDesktopFileName(const QString* name) {
  QGuiApplication::setDesktopFileName(*name);
}

void qt_gui_c_QGuiApplication_setDesktopSettingsAware(bool on) {
  QGuiApplication::setDesktopSettingsAware(on);
}

void qt_gui_c_QGuiApplication_setFallbackSessionManagementEnabled(bool arg1) {
  QGuiApplication::setFallbackSessionManagementEnabled(arg1);
}

void qt_gui_c_QGuiApplication_setFont(const QFont* arg1) {
  QGuiApplication::setFont(*arg1);
}

void qt_gui_c_QGuiApplication_setLayoutDirection(const Qt::LayoutDirection* direction) {
  QGuiApplication::setLayoutDirection(*direction);
}

void qt_gui_c_QGuiApplication_setOverrideCursor(const QCursor* arg1) {
  QGuiApplication::setOverrideCursor(*arg1);
}

void qt_gui_c_QGuiApplication_setPalette(const QPalette* pal) {
  QGuiApplication::setPalette(*pal);
}

void qt_gui_c_QGuiApplication_setQuitOnLastWindowClosed(bool quit) {
  QGuiApplication::setQuitOnLastWindowClosed(quit);
}

void qt_gui_c_QGuiApplication_setWindowIcon(const QIcon* icon) {
  QGuiApplication::setWindowIcon(*icon);
}

QStyleHints* qt_gui_c_QGuiApplication_styleHints() {
  return QGuiApplication::styleHints();
}

void qt_gui_c_QGuiApplication_sync() {
  QGuiApplication::sync();
}

QWindow* qt_gui_c_QGuiApplication_topLevelAt(const QPoint* pos) {
  return QGuiApplication::topLevelAt(*pos);
}

void qt_gui_c_QGuiApplication_topLevelWindows_to_output(QList< QWindow* >* output) {
  new(output) QList< QWindow* >(QGuiApplication::topLevelWindows());
}

void qt_gui_c_QGuiApplication_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGuiApplication::trUtf8(s, c, n));
}

void qt_gui_c_QGuiApplication_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGuiApplication::tr(s, c, n));
}

void qt_gui_c_QGuiApplication_windowIcon_to_output(QIcon* output) {
  new(output) QIcon(QGuiApplication::windowIcon());
}

