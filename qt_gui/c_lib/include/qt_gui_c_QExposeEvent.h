#ifndef QT_GUI_C_QEXPOSEEVENT_H
#define QT_GUI_C_QEXPOSEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QExposeEvent_G_static_cast_QEvent_ptr(QExposeEvent* ptr);
QT_GUI_C_EXPORT QExposeEvent* qt_gui_c_QExposeEvent_G_static_cast_QExposeEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QExposeEvent_delete(QExposeEvent* this_ptr);
QT_GUI_C_EXPORT QExposeEvent* qt_gui_c_QExposeEvent_new(const QRegion* rgn);
QT_GUI_C_EXPORT const QRegion* qt_gui_c_QExposeEvent_region(const QExposeEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QEXPOSEEVENT_H
