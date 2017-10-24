#ifndef QT_WIDGETS_C_QFILEICONPROVIDER_H
#define QT_WIDGETS_C_QFILEICONPROVIDER_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileIconProvider_delete(QFileIconProvider* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileIconProvider_icon_to_output_info(const QFileIconProvider* this_ptr, const QFileInfo* info, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileIconProvider_icon_to_output_type(const QFileIconProvider* this_ptr, QFileIconProvider::IconType type, QIcon* output);
QT_WIDGETS_C_EXPORT QFileIconProvider* qt_widgets_c_QFileIconProvider_new();
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QFileIconProvider_options(const QFileIconProvider* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileIconProvider_setOptions(QFileIconProvider* this_ptr, unsigned int options);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFileIconProvider_type_to_output(const QFileIconProvider* this_ptr, const QFileInfo* info, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QFILEICONPROVIDER_H
