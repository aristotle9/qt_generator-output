#ifndef QT_GUI_C_QPAINTEVENT_H
#define QT_GUI_C_QPAINTEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QPaintEvent_G_static_cast_QEvent_ptr(QPaintEvent* ptr);
QT_GUI_C_EXPORT QPaintEvent* qt_gui_c_QPaintEvent_G_static_cast_QPaintEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintEvent_delete(QPaintEvent* this_ptr);
QT_GUI_C_EXPORT QPaintEvent* qt_gui_c_QPaintEvent_new_paintRect(const QRect* paintRect);
QT_GUI_C_EXPORT QPaintEvent* qt_gui_c_QPaintEvent_new_paintRegion(const QRegion* paintRegion);
QT_GUI_C_EXPORT const QRect* qt_gui_c_QPaintEvent_rect(const QPaintEvent* this_ptr);
QT_GUI_C_EXPORT const QRegion* qt_gui_c_QPaintEvent_region(const QPaintEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPAINTEVENT_H
