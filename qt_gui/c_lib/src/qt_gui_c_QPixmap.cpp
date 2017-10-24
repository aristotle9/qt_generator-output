#include "qt_gui_c_QPixmap.h"

QPixmap* qt_gui_c_QPixmap_G_dynamic_cast_QPixmap_ptr(QPaintDevice* ptr) {
  return dynamic_cast<QPixmap*>(ptr);
}

QDataStream* qt_gui_c_QPixmap_G_operator_shl(QDataStream* arg1, const QPixmap* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QPixmap_G_operator_shl_to_output(const QDebug* arg1, const QPixmap* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QPaintDevice* qt_gui_c_QPixmap_G_static_cast_QPaintDevice_ptr(QPixmap* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPixmap* qt_gui_c_QPixmap_G_static_cast_QPixmap_ptr(QPaintDevice* ptr) {
  return static_cast<QPixmap*>(ptr);
}

void qt_gui_c_QPixmap_G_swap(QPixmap* value1, QPixmap* value2) {
  swap(*value1, *value2);
}

qint64 qt_gui_c_QPixmap_cacheKey(const QPixmap* this_ptr) {
  return this_ptr->cacheKey();
}

void qt_gui_c_QPixmap_convert_to_QVariant_to_output(const QPixmap* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

QPixmap* qt_gui_c_QPixmap_copy_as_ptr_no_args(const QPixmap* this_ptr) {
  return new QPixmap(this_ptr->copy());
}

QPixmap* qt_gui_c_QPixmap_copy_as_ptr_rect(const QPixmap* this_ptr, const QRect* rect) {
  return new QPixmap(this_ptr->copy(*rect));
}

QPixmap* qt_gui_c_QPixmap_copy_as_ptr_x_y_width_height(const QPixmap* this_ptr, int x, int y, int width, int height) {
  return new QPixmap(this_ptr->copy(x, y, width, height));
}

QBitmap* qt_gui_c_QPixmap_createHeuristicMask_as_ptr_clipTight(const QPixmap* this_ptr, bool clipTight) {
  return new QBitmap(this_ptr->createHeuristicMask(clipTight));
}

QBitmap* qt_gui_c_QPixmap_createHeuristicMask_as_ptr_no_args(const QPixmap* this_ptr) {
  return new QBitmap(this_ptr->createHeuristicMask());
}

QBitmap* qt_gui_c_QPixmap_createMaskFromColor_as_ptr_maskColor(const QPixmap* this_ptr, const QColor* maskColor) {
  return new QBitmap(this_ptr->createMaskFromColor(*maskColor));
}

QBitmap* qt_gui_c_QPixmap_createMaskFromColor_as_ptr_maskColor_mode(const QPixmap* this_ptr, const QColor* maskColor, const Qt::MaskMode* mode) {
  return new QBitmap(this_ptr->createMaskFromColor(*maskColor, *mode));
}

int qt_gui_c_QPixmap_defaultDepth() {
  return QPixmap::defaultDepth();
}

void qt_gui_c_QPixmap_delete(QPixmap* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QPixmap_depth(const QPixmap* this_ptr) {
  return this_ptr->depth();
}

void qt_gui_c_QPixmap_detach(QPixmap* this_ptr) {
  this_ptr->detach();
}

int qt_gui_c_QPixmap_devType(const QPixmap* this_ptr) {
  return this_ptr->devType();
}

double qt_gui_c_QPixmap_devicePixelRatio(const QPixmap* this_ptr) {
  return this_ptr->devicePixelRatio();
}

void qt_gui_c_QPixmap_fill_device_ofs(QPixmap* this_ptr, const QPaintDevice* device, const QPoint* ofs) {
  this_ptr->fill(device, *ofs);
}

void qt_gui_c_QPixmap_fill_device_xofs_yofs(QPixmap* this_ptr, const QPaintDevice* device, int xofs, int yofs) {
  this_ptr->fill(device, xofs, yofs);
}

void qt_gui_c_QPixmap_fill_fillColor(QPixmap* this_ptr, const QColor* fillColor) {
  this_ptr->fill(*fillColor);
}

void qt_gui_c_QPixmap_fill_no_args(QPixmap* this_ptr) {
  this_ptr->fill();
}

QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget(QObject* widget) {
  return new QPixmap(QPixmap::grabWidget(widget));
}

QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_rect(QObject* widget, const QRect* rect) {
  return new QPixmap(QPixmap::grabWidget(widget, *rect));
}

QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x(QObject* widget, int x) {
  return new QPixmap(QPixmap::grabWidget(widget, x));
}

QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y(QObject* widget, int x, int y) {
  return new QPixmap(QPixmap::grabWidget(widget, x, y));
}

QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y_w(QObject* widget, int x, int y, int w) {
  return new QPixmap(QPixmap::grabWidget(widget, x, y, w));
}

QPixmap* qt_gui_c_QPixmap_grabWidget_as_ptr_widget_x_y_w_h(QObject* widget, int x, int y, int w, int h) {
  return new QPixmap(QPixmap::grabWidget(widget, x, y, w, h));
}

QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1(unsigned long long arg1) {
  return new QPixmap(QPixmap::grabWindow(arg1));
}

QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x(unsigned long long arg1, int x) {
  return new QPixmap(QPixmap::grabWindow(arg1, x));
}

QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y(unsigned long long arg1, int x, int y) {
  return new QPixmap(QPixmap::grabWindow(arg1, x, y));
}

QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y_w(unsigned long long arg1, int x, int y, int w) {
  return new QPixmap(QPixmap::grabWindow(arg1, x, y, w));
}

QPixmap* qt_gui_c_QPixmap_grabWindow_as_ptr_arg1_x_y_w_h(unsigned long long arg1, int x, int y, int w, int h) {
  return new QPixmap(QPixmap::grabWindow(arg1, x, y, w, h));
}

bool qt_gui_c_QPixmap_hasAlpha(const QPixmap* this_ptr) {
  return this_ptr->hasAlpha();
}

bool qt_gui_c_QPixmap_hasAlphaChannel(const QPixmap* this_ptr) {
  return this_ptr->hasAlphaChannel();
}

int qt_gui_c_QPixmap_height(const QPixmap* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QPixmap_isDetached(const QPixmap* this_ptr) {
  return this_ptr->isDetached();
}

bool qt_gui_c_QPixmap_isNull(const QPixmap* this_ptr) {
  return this_ptr->isNull();
}

bool qt_gui_c_QPixmap_isQBitmap(const QPixmap* this_ptr) {
  return this_ptr->isQBitmap();
}

QBitmap* qt_gui_c_QPixmap_mask_as_ptr(const QPixmap* this_ptr) {
  return new QBitmap(this_ptr->mask());
}

QPixmap* qt_gui_c_QPixmap_new_QPixmap(const QPixmap* arg1) {
  return new QPixmap(*arg1);
}

QPixmap* qt_gui_c_QPixmap_new_QSize(const QSize* arg1) {
  return new QPixmap(*arg1);
}

QPixmap* qt_gui_c_QPixmap_new_int_int(int w, int h) {
  return new QPixmap(w, h);
}

QPixmap* qt_gui_c_QPixmap_new_no_args() {
  return new QPixmap();
}

QPixmap* qt_gui_c_QPixmap_operator_assign(QPixmap* this_ptr, const QPixmap* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QPixmap_operator_not(const QPixmap* this_ptr) {
  return this_ptr->operator!();
}

QPaintEngine* qt_gui_c_QPixmap_paintEngine(const QPixmap* this_ptr) {
  return this_ptr->paintEngine();
}

void qt_gui_c_QPixmap_rect_to_output(const QPixmap* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rect());
}

bool qt_gui_c_QPixmap_save_device(const QPixmap* this_ptr, QIODevice* device) {
  return this_ptr->save(device);
}

bool qt_gui_c_QPixmap_save_device_format(const QPixmap* this_ptr, QIODevice* device, const char* format) {
  return this_ptr->save(device, format);
}

bool qt_gui_c_QPixmap_save_device_format_quality(const QPixmap* this_ptr, QIODevice* device, const char* format, int quality) {
  return this_ptr->save(device, format, quality);
}

bool qt_gui_c_QPixmap_save_fileName(const QPixmap* this_ptr, const QString* fileName) {
  return this_ptr->save(*fileName);
}

bool qt_gui_c_QPixmap_save_fileName_format(const QPixmap* this_ptr, const QString* fileName, const char* format) {
  return this_ptr->save(*fileName, format);
}

bool qt_gui_c_QPixmap_save_fileName_format_quality(const QPixmap* this_ptr, const QString* fileName, const char* format, int quality) {
  return this_ptr->save(*fileName, format, quality);
}

QPixmap* qt_gui_c_QPixmap_scaledToHeight_as_ptr_h(const QPixmap* this_ptr, int h) {
  return new QPixmap(this_ptr->scaledToHeight(h));
}

QPixmap* qt_gui_c_QPixmap_scaledToHeight_as_ptr_h_mode(const QPixmap* this_ptr, int h, const Qt::TransformationMode* mode) {
  return new QPixmap(this_ptr->scaledToHeight(h, *mode));
}

QPixmap* qt_gui_c_QPixmap_scaledToWidth_as_ptr_w(const QPixmap* this_ptr, int w) {
  return new QPixmap(this_ptr->scaledToWidth(w));
}

QPixmap* qt_gui_c_QPixmap_scaledToWidth_as_ptr_w_mode(const QPixmap* this_ptr, int w, const Qt::TransformationMode* mode) {
  return new QPixmap(this_ptr->scaledToWidth(w, *mode));
}

QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_s(const QPixmap* this_ptr, const QSize* s) {
  return new QPixmap(this_ptr->scaled(*s));
}

QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_s_aspectMode(const QPixmap* this_ptr, const QSize* s, const Qt::AspectRatioMode* aspectMode) {
  return new QPixmap(this_ptr->scaled(*s, *aspectMode));
}

QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_s_aspectMode_mode(const QPixmap* this_ptr, const QSize* s, const Qt::AspectRatioMode* aspectMode, const Qt::TransformationMode* mode) {
  return new QPixmap(this_ptr->scaled(*s, *aspectMode, *mode));
}

QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_w_h(const QPixmap* this_ptr, int w, int h) {
  return new QPixmap(this_ptr->scaled(w, h));
}

QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_w_h_aspectMode(const QPixmap* this_ptr, int w, int h, const Qt::AspectRatioMode* aspectMode) {
  return new QPixmap(this_ptr->scaled(w, h, *aspectMode));
}

QPixmap* qt_gui_c_QPixmap_scaled_as_ptr_w_h_aspectMode_mode(const QPixmap* this_ptr, int w, int h, const Qt::AspectRatioMode* aspectMode, const Qt::TransformationMode* mode) {
  return new QPixmap(this_ptr->scaled(w, h, *aspectMode, *mode));
}

void qt_gui_c_QPixmap_scroll_dx_dy_rect(QPixmap* this_ptr, int dx, int dy, const QRect* rect) {
  this_ptr->scroll(dx, dy, *rect);
}

void qt_gui_c_QPixmap_scroll_dx_dy_rect_exposed(QPixmap* this_ptr, int dx, int dy, const QRect* rect, QRegion* exposed) {
  this_ptr->scroll(dx, dy, *rect, exposed);
}

void qt_gui_c_QPixmap_scroll_dx_dy_x_y_width_height(QPixmap* this_ptr, int dx, int dy, int x, int y, int width, int height) {
  this_ptr->scroll(dx, dy, x, y, width, height);
}

void qt_gui_c_QPixmap_scroll_dx_dy_x_y_width_height_exposed(QPixmap* this_ptr, int dx, int dy, int x, int y, int width, int height, QRegion* exposed) {
  this_ptr->scroll(dx, dy, x, y, width, height, exposed);
}

void qt_gui_c_QPixmap_setDevicePixelRatio(QPixmap* this_ptr, double scaleFactor) {
  this_ptr->setDevicePixelRatio(scaleFactor);
}

void qt_gui_c_QPixmap_setMask(QPixmap* this_ptr, const QBitmap* arg1) {
  this_ptr->setMask(*arg1);
}

void qt_gui_c_QPixmap_size_to_output(const QPixmap* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_gui_c_QPixmap_swap(QPixmap* this_ptr, QPixmap* other) {
  this_ptr->swap(*other);
}

QImage* qt_gui_c_QPixmap_toImage_as_ptr(const QPixmap* this_ptr) {
  return new QImage(this_ptr->toImage());
}

QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QMatrix(const QPixmap* this_ptr, const QMatrix* arg1) {
  return new QPixmap(this_ptr->transformed(*arg1));
}

QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QMatrix_Qt_TransformationMode(const QPixmap* this_ptr, const QMatrix* arg1, const Qt::TransformationMode* mode) {
  return new QPixmap(this_ptr->transformed(*arg1, *mode));
}

QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QTransform(const QPixmap* this_ptr, const QTransform* arg1) {
  return new QPixmap(this_ptr->transformed(*arg1));
}

QPixmap* qt_gui_c_QPixmap_transformed_as_ptr_QTransform_Qt_TransformationMode(const QPixmap* this_ptr, const QTransform* arg1, const Qt::TransformationMode* mode) {
  return new QPixmap(this_ptr->transformed(*arg1, *mode));
}

void qt_gui_c_QPixmap_trueMatrix_to_output_QMatrix_int_int(const QMatrix* m, int w, int h, QMatrix* output) {
  new(output) QMatrix(QPixmap::trueMatrix(*m, w, h));
}

void qt_gui_c_QPixmap_trueMatrix_to_output_QTransform_int_int(const QTransform* m, int w, int h, QTransform* output) {
  new(output) QTransform(QPixmap::trueMatrix(*m, w, h));
}

int qt_gui_c_QPixmap_width(const QPixmap* this_ptr) {
  return this_ptr->width();
}

