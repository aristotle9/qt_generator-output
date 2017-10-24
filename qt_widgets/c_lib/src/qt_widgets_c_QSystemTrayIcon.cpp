#include "qt_widgets_c_QSystemTrayIcon.h"

QObject* qt_widgets_c_QSystemTrayIcon_G_static_cast_QObject_ptr(QSystemTrayIcon* ptr) {
  return static_cast<QObject*>(ptr);
}

QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_G_static_cast_QSystemTrayIcon_ptr(QObject* ptr) {
  return static_cast<QSystemTrayIcon*>(ptr);
}

QMenu* qt_widgets_c_QSystemTrayIcon_contextMenu(const QSystemTrayIcon* this_ptr) {
  return this_ptr->contextMenu();
}

void qt_widgets_c_QSystemTrayIcon_delete(QSystemTrayIcon* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QSystemTrayIcon_geometry_to_output(const QSystemTrayIcon* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

void qt_widgets_c_QSystemTrayIcon_hide(QSystemTrayIcon* this_ptr) {
  this_ptr->hide();
}

void qt_widgets_c_QSystemTrayIcon_icon_to_output(const QSystemTrayIcon* this_ptr, QIcon* output) {
  new(output) QIcon(this_ptr->icon());
}

bool qt_widgets_c_QSystemTrayIcon_isSystemTrayAvailable() {
  return QSystemTrayIcon::isSystemTrayAvailable();
}

bool qt_widgets_c_QSystemTrayIcon_isVisible(const QSystemTrayIcon* this_ptr) {
  return this_ptr->isVisible();
}

const QMetaObject* qt_widgets_c_QSystemTrayIcon_metaObject(const QSystemTrayIcon* this_ptr) {
  return this_ptr->metaObject();
}

QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_icon(const QIcon* icon) {
  return new QSystemTrayIcon(*icon);
}

QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_icon_parent(const QIcon* icon, QObject* parent) {
  return new QSystemTrayIcon(*icon, parent);
}

QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_no_args() {
  return new QSystemTrayIcon();
}

QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_parent(QObject* parent) {
  return new QSystemTrayIcon(parent);
}

int qt_widgets_c_QSystemTrayIcon_qt_metacall(QSystemTrayIcon* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSystemTrayIcon_qt_metacast(QSystemTrayIcon* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSystemTrayIcon_setContextMenu(QSystemTrayIcon* this_ptr, QMenu* menu) {
  this_ptr->setContextMenu(menu);
}

void qt_widgets_c_QSystemTrayIcon_setIcon(QSystemTrayIcon* this_ptr, const QIcon* icon) {
  this_ptr->setIcon(*icon);
}

void qt_widgets_c_QSystemTrayIcon_setToolTip(QSystemTrayIcon* this_ptr, const QString* tip) {
  this_ptr->setToolTip(*tip);
}

void qt_widgets_c_QSystemTrayIcon_setVisible(QSystemTrayIcon* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QSystemTrayIcon_show(QSystemTrayIcon* this_ptr) {
  this_ptr->show();
}

void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg) {
  this_ptr->showMessage(*title, *msg);
}

void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QIcon(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QIcon* icon) {
  this_ptr->showMessage(*title, *msg, *icon);
}

void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QIcon_int(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QIcon* icon, int msecs) {
  this_ptr->showMessage(*title, *msg, *icon, msecs);
}

void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QSystemTrayIcon_MessageIcon(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QSystemTrayIcon::MessageIcon* icon) {
  this_ptr->showMessage(*title, *msg, *icon);
}

void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QSystemTrayIcon_MessageIcon_int(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QSystemTrayIcon::MessageIcon* icon, int msecs) {
  this_ptr->showMessage(*title, *msg, *icon, msecs);
}

bool qt_widgets_c_QSystemTrayIcon_supportsMessages() {
  return QSystemTrayIcon::supportsMessages();
}

void qt_widgets_c_QSystemTrayIcon_toolTip_to_output(const QSystemTrayIcon* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

void qt_widgets_c_QSystemTrayIcon_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSystemTrayIcon::trUtf8(s, c, n));
}

void qt_widgets_c_QSystemTrayIcon_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSystemTrayIcon::tr(s, c, n));
}

