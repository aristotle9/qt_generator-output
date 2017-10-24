#ifndef QT_GUI_C_QPIXMAP_H
#define QT_GUI_C_QPIXMAP_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_G_dynamic_cast_QPixmap_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QPixmap_G_operator_shl(QDataStream* arg1, const QPixmap* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_G_operator_shl_to_output(const QDebug* arg1, const QPixmap* arg2, QDebug* output);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPixmap_G_static_cast_QPaintDevice_ptr(QPixmap* ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_G_static_cast_QPixmap_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_G_swap(QPixmap* value1, QPixmap* value2);
QT_GUI_C_EXPORT qint64 qt_gui_c_QPixmap_cacheKey(const QPixmap* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_convert_to_QVariant_to_output(const QPixmap* this_ptr, QVariant* output);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_copy_as_ptr_no_args(const QPixmap* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_copy_as_ptr_rect(const QPixmap* this_ptr, const QRect* rect);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_copy_as_ptr_x_y_width_height(const QPixmap* this_ptr, int x, int y, int width, int height);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QPixmap_createHeuristicMask_as_ptr_clipTight(const QPixmap* this_ptr, bool clipTight);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QPixmap_createHeuristicMask_as_ptr_no_args(const QPixmap* this_ptr);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QPixmap_createMaskFromColor_as_ptr_maskColor(const QPixmap* this_ptr, const QColor* maskColor);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QPixmap_createMaskFromColor_as_ptr_maskColor_mode(const QPixmap* this_ptr, const QColor* maskColor, const Qt::MaskMode* mode);
QT_GUI_C_EXPORT int qt_gui_c_QPixmap_defaultDepth();
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_delete(QPixmap* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPixmap_depth(const QPixmap* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_detach(QPixmap* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPixmap_devType(const QPixmap* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPixmap_devicePixelRatio(const QPixmap* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_fill_device_ofs(QPixmap* this_ptr, const QPaintDevice* device, const QPoint* ofs);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_fill_device_xofs_yofs(QPixmap* this_ptr, const QPaintDevice* device, int xofs, int yofs);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_fill_fillColor(QPixmap* this_ptr, const QColor* fillColor);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_fill_no_args(QPixmap* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget(QObject* widget);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_rect(QObject* widget, const QRect* rect);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x(QObject* widget, int x);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y(QObject* widget, int x, int y);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y_w(QObject* widget, int x, int y, int w);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y_w_h(QObject* widget, int x, int y, int w, int h);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1(unsigned long long arg1);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x(unsigned long long arg1, int x);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y(unsigned long long arg1, int x, int y);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y_w(unsigned long long arg1, int x, int y, int w);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y_w_h(unsigned long long arg1, int x, int y, int w, int h);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_hasAlpha(const QPixmap* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_hasAlphaChannel(const QPixmap* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPixmap_height(const QPixmap* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_isDetached(const QPixmap* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_isNull(const QPixmap* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_isQBitmap(const QPixmap* this_ptr);
QT_GUI_C_EXPORT QBitmap* qt_gui_c_QPixmap_mask_as_ptr(const QPixmap* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_new_QPixmap(const QPixmap* arg1);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_new_QSize(const QSize* arg1);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_new_int_int(int w, int h);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_new_no_args();
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_operator_assign(QPixmap* this_ptr, const QPixmap* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_operator_not(const QPixmap* this_ptr);
QT_GUI_C_EXPORT QPaintEngine* qt_gui_c_QPixmap_paintEngine(const QPixmap* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_rect_to_output(const QPixmap* this_ptr, QRect* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_save_device(const QPixmap* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_save_device_format(const QPixmap* this_ptr, QIODevice* device, const char* format);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_save_device_format_quality(const QPixmap* this_ptr, QIODevice* device, const char* format, int quality);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_save_fileName(const QPixmap* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_save_fileName_format(const QPixmap* this_ptr, const QString* fileName, const char* format);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmap_save_fileName_format_quality(const QPixmap* this_ptr, const QString* fileName, const char* format, int quality);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaledToHeight_as_ptr_h(const QPixmap* this_ptr, int h);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaledToHeight_as_ptr_h_mode(const QPixmap* this_ptr, int h, const Qt::TransformationMode* mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaledToWidth_as_ptr_w(const QPixmap* this_ptr, int w);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaledToWidth_as_ptr_w_mode(const QPixmap* this_ptr, int w, const Qt::TransformationMode* mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_s(const QPixmap* this_ptr, const QSize* s);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_s_aspectMode(const QPixmap* this_ptr, const QSize* s, const Qt::AspectRatioMode* aspectMode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_s_aspectMode_mode(const QPixmap* this_ptr, const QSize* s, const Qt::AspectRatioMode* aspectMode, const Qt::TransformationMode* mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_w_h(const QPixmap* this_ptr, int w, int h);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_w_h_aspectMode(const QPixmap* this_ptr, int w, int h, const Qt::AspectRatioMode* aspectMode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_w_h_aspectMode_mode(const QPixmap* this_ptr, int w, int h, const Qt::AspectRatioMode* aspectMode, const Qt::TransformationMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_scroll_dx_dy_rect(QPixmap* this_ptr, int dx, int dy, const QRect* rect);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_scroll_dx_dy_rect_exposed(QPixmap* this_ptr, int dx, int dy, const QRect* rect, QRegion* exposed);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_scroll_dx_dy_x_y_width_height(QPixmap* this_ptr, int dx, int dy, int x, int y, int width, int height);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_scroll_dx_dy_x_y_width_height_exposed(QPixmap* this_ptr, int dx, int dy, int x, int y, int width, int height, QRegion* exposed);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_setDevicePixelRatio(QPixmap* this_ptr, double scaleFactor);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_setMask(QPixmap* this_ptr, const QBitmap* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_size_to_output(const QPixmap* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_swap(QPixmap* this_ptr, QPixmap* other);
QT_GUI_C_EXPORT QImage* qt_gui_c_QPixmap_toImage_as_ptr(const QPixmap* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QMatrix(const QPixmap* this_ptr, const QMatrix* arg1);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QMatrix_Qt_TransformationMode(const QPixmap* this_ptr, const QMatrix* arg1, const Qt::TransformationMode* mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QTransform(const QPixmap* this_ptr, const QTransform* arg1);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QTransform_Qt_TransformationMode(const QPixmap* this_ptr, const QTransform* arg1, const Qt::TransformationMode* mode);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_trueMatrix_to_output_QMatrix_int_int(const QMatrix* m, int w, int h, QMatrix* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixmap_trueMatrix_to_output_QTransform_int_int(const QTransform* m, int w, int h, QTransform* output);
QT_GUI_C_EXPORT int qt_gui_c_QPixmap_width(const QPixmap* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPIXMAP_H
