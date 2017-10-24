#ifndef QT_WIDGETS_C_QCOLORMAP_H
#define QT_WIDGETS_C_QCOLORMAP_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_cleanup();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_colorAt_to_output(const QColormap* this_ptr, unsigned int pixel, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_colormap_to_output(const QColormap* this_ptr, QVector< QColor >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_constructor(const QColormap* colormap, QColormap* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QColormap_depth(const QColormap* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_destructor(QColormap* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_initialize();
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_instance_to_output_no_args(QColormap* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QColormap_instance_to_output_screen(int screen, QColormap* output);
QT_WIDGETS_C_EXPORT QColormap::Mode qt_widgets_c_QColormap_mode(const QColormap* this_ptr);
QT_WIDGETS_C_EXPORT QColormap* qt_widgets_c_QColormap_operator_assign(QColormap* this_ptr, const QColormap* colormap);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QColormap_pixel(const QColormap* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QColormap_size(const QColormap* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QCOLORMAP_H
