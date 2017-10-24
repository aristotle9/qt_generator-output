#ifndef QT_GUI_C_QPAINTDEVICE_H
#define QT_GUI_C_QPAINTDEVICE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_colorCount(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPaintDevice_delete(QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_depth(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_devType(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_devicePixelRatio(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPaintDevice_devicePixelRatioF(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPaintDevice_devicePixelRatioFScale();
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_height(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_heightMM(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_logicalDpiX(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_logicalDpiY(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT QPaintEngine* qt_gui_c_QPaintDevice_paintEngine(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPaintDevice_paintingActive(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_physicalDpiX(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_physicalDpiY(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_width(const QPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPaintDevice_widthMM(const QPaintDevice* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPAINTDEVICE_H
