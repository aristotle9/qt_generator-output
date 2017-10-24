#ifndef QT_GUI_C_QDESKTOPSERVICES_H
#define QT_GUI_C_QDESKTOPSERVICES_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QDesktopServices_delete(QDesktopServices* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QDesktopServices_openUrl(const QUrl* url);
QT_GUI_C_EXPORT void qt_gui_c_QDesktopServices_setUrlHandler(const QString* scheme, QObject* receiver, const char* method);
QT_GUI_C_EXPORT void qt_gui_c_QDesktopServices_unsetUrlHandler(const QString* scheme);

} // extern "C"

#endif // QT_GUI_C_QDESKTOPSERVICES_H
