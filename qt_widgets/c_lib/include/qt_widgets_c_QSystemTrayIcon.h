#ifndef QT_WIDGETS_C_QSYSTEMTRAYICON_H
#define QT_WIDGETS_C_QSYSTEMTRAYICON_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSystemTrayIcon_G_static_cast_QObject_ptr(QSystemTrayIcon* ptr);
QT_WIDGETS_C_EXPORT QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_G_static_cast_QSystemTrayIcon_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QSystemTrayIcon_contextMenu(const QSystemTrayIcon* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_delete(QSystemTrayIcon* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_geometry_to_output(const QSystemTrayIcon* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_hide(QSystemTrayIcon* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_icon_to_output(const QSystemTrayIcon* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSystemTrayIcon_isSystemTrayAvailable();
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSystemTrayIcon_isVisible(const QSystemTrayIcon* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSystemTrayIcon_metaObject(const QSystemTrayIcon* this_ptr);
QT_WIDGETS_C_EXPORT QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_icon(const QIcon* icon);
QT_WIDGETS_C_EXPORT QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_icon_parent(const QIcon* icon, QObject* parent);
QT_WIDGETS_C_EXPORT QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_no_args();
QT_WIDGETS_C_EXPORT QSystemTrayIcon* qt_widgets_c_QSystemTrayIcon_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSystemTrayIcon_qt_metacall(QSystemTrayIcon* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSystemTrayIcon_qt_metacast(QSystemTrayIcon* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_setContextMenu(QSystemTrayIcon* this_ptr, QMenu* menu);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_setIcon(QSystemTrayIcon* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_setToolTip(QSystemTrayIcon* this_ptr, const QString* tip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_setVisible(QSystemTrayIcon* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_show(QSystemTrayIcon* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QIcon(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QIcon_int(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QIcon* icon, int msecs);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QSystemTrayIcon_MessageIcon(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QSystemTrayIcon::MessageIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_showMessage_QString_QString_QSystemTrayIcon_MessageIcon_int(QSystemTrayIcon* this_ptr, const QString* title, const QString* msg, const QSystemTrayIcon::MessageIcon* icon, int msecs);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSystemTrayIcon_supportsMessages();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_toolTip_to_output(const QSystemTrayIcon* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSystemTrayIcon_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSYSTEMTRAYICON_H
