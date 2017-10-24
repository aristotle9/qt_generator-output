#ifndef QT_GUI_C_QPLATFORMSURFACEEVENT_H
#define QT_GUI_C_QPLATFORMSURFACEEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QEvent* qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QEvent_ptr(QPlatformSurfaceEvent* ptr);
QT_GUI_C_EXPORT QPlatformSurfaceEvent* qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QPlatformSurfaceEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPlatformSurfaceEvent_delete(QPlatformSurfaceEvent* this_ptr);
QT_GUI_C_EXPORT QPlatformSurfaceEvent* qt_gui_c_QPlatformSurfaceEvent_new(QPlatformSurfaceEvent::SurfaceEventType surfaceEventType);
QT_GUI_C_EXPORT QPlatformSurfaceEvent::SurfaceEventType qt_gui_c_QPlatformSurfaceEvent_surfaceEventType(const QPlatformSurfaceEvent* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPLATFORMSURFACEEVENT_H
