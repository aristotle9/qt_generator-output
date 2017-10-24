#ifndef QT_GUI_C_QSCREENORIENTATIONCHANGEEVENT_H
#define QT_GUI_C_QSCREENORIENTATIONCHANGEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QEvent_ptr(QScreenOrientationChangeEvent* ptr);
QT_GUI_C_EXPORT QScreenOrientationChangeEvent* qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QScreenOrientationChangeEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QScreenOrientationChangeEvent_delete(QScreenOrientationChangeEvent* this_ptr);
QT_GUI_C_EXPORT QScreenOrientationChangeEvent* qt_gui_c_QScreenOrientationChangeEvent_new(QScreen* screen, const Qt::ScreenOrientation* orientation);
QT_GUI_C_EXPORT QScreen* qt_gui_c_QScreenOrientationChangeEvent_screen(const QScreenOrientationChangeEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QSCREENORIENTATIONCHANGEEVENT_H
