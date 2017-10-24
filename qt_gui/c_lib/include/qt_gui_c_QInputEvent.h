#ifndef QT_GUI_C_QINPUTEVENT_H
#define QT_GUI_C_QINPUTEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QInputEvent_G_static_cast_QEvent_ptr(QInputEvent* ptr);
QT_GUI_C_EXPORT QInputEvent* qt_gui_c_QInputEvent_G_static_cast_QInputEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputEvent_delete(QInputEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputEvent_setTimestamp(QInputEvent* this_ptr, unsigned long atimestamp);
QT_GUI_C_EXPORT unsigned long qt_gui_c_QInputEvent_timestamp(const QInputEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QINPUTEVENT_H
