#include "qt_gui_c_QPlatformSurfaceEvent.h"

QEvent* qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QEvent_ptr(QPlatformSurfaceEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QPlatformSurfaceEvent* qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QPlatformSurfaceEvent_ptr(QEvent* ptr) {
  return static_cast<QPlatformSurfaceEvent*>(ptr);
}

void qt_gui_c_QPlatformSurfaceEvent_delete(QPlatformSurfaceEvent* this_ptr) {
  delete this_ptr;
}

QPlatformSurfaceEvent* qt_gui_c_QPlatformSurfaceEvent_new(QPlatformSurfaceEvent::SurfaceEventType surfaceEventType) {
  return new QPlatformSurfaceEvent(surfaceEventType);
}

QPlatformSurfaceEvent::SurfaceEventType qt_gui_c_QPlatformSurfaceEvent_surfaceEventType(const QPlatformSurfaceEvent* this_ptr) {
  return this_ptr->surfaceEventType();
}

