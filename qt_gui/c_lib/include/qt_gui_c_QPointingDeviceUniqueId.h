#ifndef QT_GUI_C_QPOINTINGDEVICEUNIQUEID_H
#define QT_GUI_C_QPOINTINGDEVICEUNIQUEID_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPointingDeviceUniqueId_constructor(QPointingDeviceUniqueId* output);
QT_GUI_C_EXPORT void qt_gui_c_QPointingDeviceUniqueId_destructor(QPointingDeviceUniqueId* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPointingDeviceUniqueId_fromNumericId_to_output(qint64 id, QPointingDeviceUniqueId* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPointingDeviceUniqueId_isValid(const QPointingDeviceUniqueId* this_ptr);
QT_GUI_C_EXPORT qint64 qt_gui_c_QPointingDeviceUniqueId_numericId(const QPointingDeviceUniqueId* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPOINTINGDEVICEUNIQUEID_H
