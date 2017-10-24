#ifndef QT_GUI_C_QRESIZEEVENT_H
#define QT_GUI_C_QRESIZEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QResizeEvent_G_static_cast_QEvent_ptr(QResizeEvent* ptr);
QT_GUI_C_EXPORT QResizeEvent* qt_gui_c_QResizeEvent_G_static_cast_QResizeEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QResizeEvent_delete(QResizeEvent* this_ptr);
QT_GUI_C_EXPORT QResizeEvent* qt_gui_c_QResizeEvent_new(const QSize* size, const QSize* oldSize);
QT_GUI_C_EXPORT const QSize* qt_gui_c_QResizeEvent_oldSize(const QResizeEvent* this_ptr);
QT_GUI_C_EXPORT const QSize* qt_gui_c_QResizeEvent_size(const QResizeEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QRESIZEEVENT_H
