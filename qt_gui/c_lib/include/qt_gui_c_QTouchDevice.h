#ifndef QT_GUI_C_QTOUCHDEVICE_H
#define QT_GUI_C_QTOUCHDEVICE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_G_operator_shl_to_output(const QDebug* arg1, const QTouchDevice* arg2, QDebug* output);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTouchDevice_capabilities(const QTouchDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_delete(QTouchDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_devices_to_output(QList< const QTouchDevice* >* output);
QT_GUI_C_EXPORT int qt_gui_c_QTouchDevice_maximumTouchPoints(const QTouchDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_name_to_output(const QTouchDevice* this_ptr, QString* output);
QT_GUI_C_EXPORT QTouchDevice* qt_gui_c_QTouchDevice_new();
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_setCapabilities(QTouchDevice* this_ptr, unsigned int caps);
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_setMaximumTouchPoints(QTouchDevice* this_ptr, int max);
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_setName(QTouchDevice* this_ptr, const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QTouchDevice_setType(QTouchDevice* this_ptr, QTouchDevice::DeviceType devType);
QT_GUI_C_EXPORT QTouchDevice::DeviceType qt_gui_c_QTouchDevice_type(const QTouchDevice* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTOUCHDEVICE_H
