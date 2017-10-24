#include "qt_gui_c_QDesktopServices.h"

void qt_gui_c_QDesktopServices_delete(QDesktopServices* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QDesktopServices_openUrl(const QUrl* url) {
  return QDesktopServices::openUrl(*url);
}

void qt_gui_c_QDesktopServices_setUrlHandler(const QString* scheme, QObject* receiver, const char* method) {
  QDesktopServices::setUrlHandler(*scheme, receiver, method);
}

void qt_gui_c_QDesktopServices_unsetUrlHandler(const QString* scheme) {
  QDesktopServices::unsetUrlHandler(*scheme);
}

