#ifndef QT_GUI_C_QAPPLICATIONSTATECHANGEEVENT_H
#define QT_GUI_C_QAPPLICATIONSTATECHANGEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QApplicationStateChangeEvent* qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QApplicationStateChangeEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QEvent_ptr(QApplicationStateChangeEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QApplicationStateChangeEvent_delete(QApplicationStateChangeEvent* this_ptr);
QT_GUI_C_EXPORT QApplicationStateChangeEvent* qt_gui_c_QApplicationStateChangeEvent_new(const Qt::ApplicationState* state);

} // extern "C"

#endif // QT_GUI_C_QAPPLICATIONSTATECHANGEEVENT_H
