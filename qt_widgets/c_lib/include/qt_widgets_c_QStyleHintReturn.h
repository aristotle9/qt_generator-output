#ifndef QT_WIDGETS_C_QSTYLEHINTRETURN_H
#define QT_WIDGETS_C_QSTYLEHINTRETURN_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleHintReturn_delete(QStyleHintReturn* this_ptr);
QT_WIDGETS_C_EXPORT QStyleHintReturn* qt_widgets_c_QStyleHintReturn_new_no_args();
QT_WIDGETS_C_EXPORT QStyleHintReturn* qt_widgets_c_QStyleHintReturn_new_version(int version);
QT_WIDGETS_C_EXPORT QStyleHintReturn* qt_widgets_c_QStyleHintReturn_new_version_type(int version, int type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleHintReturn_set_type(QStyleHintReturn* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QStyleHintReturn_set_version(QStyleHintReturn* this_ptr, int value);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleHintReturn_type(const QStyleHintReturn* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QStyleHintReturn_version(const QStyleHintReturn* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSTYLEHINTRETURN_H
