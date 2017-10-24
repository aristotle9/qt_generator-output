#ifndef QT_GUI_C_QWHATSTHISCLICKEDEVENT_H
#define QT_GUI_C_QWHATSTHISCLICKEDEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QEvent_ptr(QWhatsThisClickedEvent* ptr);
QT_GUI_C_EXPORT QWhatsThisClickedEvent* qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QWhatsThisClickedEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QWhatsThisClickedEvent_delete(QWhatsThisClickedEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QWhatsThisClickedEvent_href_to_output(const QWhatsThisClickedEvent* this_ptr, QString* output);
QT_GUI_C_EXPORT QWhatsThisClickedEvent* qt_gui_c_QWhatsThisClickedEvent_new(const QString* href);

} // extern "C"

#endif // QT_GUI_C_QWHATSTHISCLICKEDEVENT_H
