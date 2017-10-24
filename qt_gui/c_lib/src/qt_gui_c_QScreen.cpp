#include "qt_gui_c_QScreen.h"

void qt_gui_c_QScreen_G_operator_shl_to_output(const QDebug* arg1, const QScreen* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, arg2));
}

QObject* qt_gui_c_QScreen_G_static_cast_QObject_ptr(QScreen* ptr) {
  return static_cast<QObject*>(ptr);
}

QScreen* qt_gui_c_QScreen_G_static_cast_QScreen_ptr(QObject* ptr) {
  return static_cast<QScreen*>(ptr);
}

int qt_gui_c_QScreen_angleBetween(const QScreen* this_ptr, const Qt::ScreenOrientation* a, const Qt::ScreenOrientation* b) {
  return this_ptr->angleBetween(*a, *b);
}

void qt_gui_c_QScreen_availableGeometry_to_output(const QScreen* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->availableGeometry());
}

void qt_gui_c_QScreen_availableSize_to_output(const QScreen* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->availableSize());
}

void qt_gui_c_QScreen_availableVirtualGeometry_to_output(const QScreen* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->availableVirtualGeometry());
}

void qt_gui_c_QScreen_availableVirtualSize_to_output(const QScreen* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->availableVirtualSize());
}

void qt_gui_c_QScreen_delete(QScreen* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QScreen_depth(const QScreen* this_ptr) {
  return this_ptr->depth();
}

double qt_gui_c_QScreen_devicePixelRatio(const QScreen* this_ptr) {
  return this_ptr->devicePixelRatio();
}

void qt_gui_c_QScreen_geometry_to_output(const QScreen* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window(QScreen* this_ptr, unsigned long long window) {
  return new QPixmap(this_ptr->grabWindow(window));
}

QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x(QScreen* this_ptr, unsigned long long window, int x) {
  return new QPixmap(this_ptr->grabWindow(window, x));
}

QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y(QScreen* this_ptr, unsigned long long window, int x, int y) {
  return new QPixmap(this_ptr->grabWindow(window, x, y));
}

QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y_w(QScreen* this_ptr, unsigned long long window, int x, int y, int w) {
  return new QPixmap(this_ptr->grabWindow(window, x, y, w));
}

QPixmap* qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y_w_h(QScreen* this_ptr, unsigned long long window, int x, int y, int w, int h) {
  return new QPixmap(this_ptr->grabWindow(window, x, y, w, h));
}

bool qt_gui_c_QScreen_isLandscape(const QScreen* this_ptr, const Qt::ScreenOrientation* orientation) {
  return this_ptr->isLandscape(*orientation);
}

bool qt_gui_c_QScreen_isPortrait(const QScreen* this_ptr, const Qt::ScreenOrientation* orientation) {
  return this_ptr->isPortrait(*orientation);
}

double qt_gui_c_QScreen_logicalDotsPerInch(const QScreen* this_ptr) {
  return this_ptr->logicalDotsPerInch();
}

double qt_gui_c_QScreen_logicalDotsPerInchX(const QScreen* this_ptr) {
  return this_ptr->logicalDotsPerInchX();
}

double qt_gui_c_QScreen_logicalDotsPerInchY(const QScreen* this_ptr) {
  return this_ptr->logicalDotsPerInchY();
}

void qt_gui_c_QScreen_manufacturer_to_output(const QScreen* this_ptr, QString* output) {
  new(output) QString(this_ptr->manufacturer());
}

void qt_gui_c_QScreen_mapBetween_to_output(const QScreen* this_ptr, const Qt::ScreenOrientation* a, const Qt::ScreenOrientation* b, const QRect* rect, QRect* output) {
  new(output) QRect(this_ptr->mapBetween(*a, *b, *rect));
}

const QMetaObject* qt_gui_c_QScreen_metaObject(const QScreen* this_ptr) {
  return this_ptr->metaObject();
}

void qt_gui_c_QScreen_model_to_output(const QScreen* this_ptr, QString* output) {
  new(output) QString(this_ptr->model());
}

void qt_gui_c_QScreen_name_to_output(const QScreen* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

double qt_gui_c_QScreen_physicalDotsPerInch(const QScreen* this_ptr) {
  return this_ptr->physicalDotsPerInch();
}

double qt_gui_c_QScreen_physicalDotsPerInchX(const QScreen* this_ptr) {
  return this_ptr->physicalDotsPerInchX();
}

double qt_gui_c_QScreen_physicalDotsPerInchY(const QScreen* this_ptr) {
  return this_ptr->physicalDotsPerInchY();
}

void qt_gui_c_QScreen_physicalSize_to_output(const QScreen* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->physicalSize());
}

int qt_gui_c_QScreen_qt_metacall(QScreen* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QScreen_qt_metacast(QScreen* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

double qt_gui_c_QScreen_refreshRate(const QScreen* this_ptr) {
  return this_ptr->refreshRate();
}

void qt_gui_c_QScreen_serialNumber_to_output(const QScreen* this_ptr, QString* output) {
  new(output) QString(this_ptr->serialNumber());
}

void qt_gui_c_QScreen_size_to_output(const QScreen* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_gui_c_QScreen_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScreen::trUtf8(s, c, n));
}

void qt_gui_c_QScreen_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScreen::tr(s, c, n));
}

void qt_gui_c_QScreen_transformBetween_to_output(const QScreen* this_ptr, const Qt::ScreenOrientation* a, const Qt::ScreenOrientation* b, const QRect* target, QTransform* output) {
  new(output) QTransform(this_ptr->transformBetween(*a, *b, *target));
}

void qt_gui_c_QScreen_virtualGeometry_to_output(const QScreen* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->virtualGeometry());
}

void qt_gui_c_QScreen_virtualSiblings_to_output(const QScreen* this_ptr, QList< QScreen* >* output) {
  new(output) QList< QScreen* >(this_ptr->virtualSiblings());
}

void qt_gui_c_QScreen_virtualSize_to_output(const QScreen* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->virtualSize());
}

