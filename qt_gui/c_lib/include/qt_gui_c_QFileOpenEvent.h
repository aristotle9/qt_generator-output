#ifndef QT_GUI_C_QFILEOPENEVENT_H
#define QT_GUI_C_QFILEOPENEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QFileOpenEvent_G_static_cast_QEvent_ptr(QFileOpenEvent* ptr);
QT_GUI_C_EXPORT QFileOpenEvent* qt_gui_c_QFileOpenEvent_G_static_cast_QFileOpenEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFileOpenEvent_delete(QFileOpenEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFileOpenEvent_file_to_output(const QFileOpenEvent* this_ptr, QString* output);
QT_GUI_C_EXPORT QFileOpenEvent* qt_gui_c_QFileOpenEvent_new_file(const QString* file);
QT_GUI_C_EXPORT QFileOpenEvent* qt_gui_c_QFileOpenEvent_new_url(const QUrl* url);
QT_GUI_C_EXPORT void qt_gui_c_QFileOpenEvent_url_to_output(const QFileOpenEvent* this_ptr, QUrl* output);

} // extern "C"

#endif // QT_GUI_C_QFILEOPENEVENT_H
