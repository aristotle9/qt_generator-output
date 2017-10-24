#ifndef QT_GUI_C_QSTATUSTIPEVENT_H
#define QT_GUI_C_QSTATUSTIPEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QStatusTipEvent_G_static_cast_QEvent_ptr(QStatusTipEvent* ptr);
QT_GUI_C_EXPORT QStatusTipEvent* qt_gui_c_QStatusTipEvent_G_static_cast_QStatusTipEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStatusTipEvent_delete(QStatusTipEvent* this_ptr);
QT_GUI_C_EXPORT QStatusTipEvent* qt_gui_c_QStatusTipEvent_new(const QString* tip);
QT_GUI_C_EXPORT void qt_gui_c_QStatusTipEvent_tip_to_output(const QStatusTipEvent* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QSTATUSTIPEVENT_H
