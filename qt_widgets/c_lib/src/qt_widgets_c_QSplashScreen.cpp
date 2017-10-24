#include "qt_widgets_c_QSplashScreen.h"

QSplashScreen* qt_widgets_c_QSplashScreen_G_dynamic_cast_QSplashScreen_ptr(QWidget* ptr) {
  return dynamic_cast<QSplashScreen*>(ptr);
}

QObject* qt_widgets_c_QSplashScreen_G_static_cast_QObject_ptr(QSplashScreen* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QSplashScreen_G_static_cast_QPaintDevice_ptr(QSplashScreen* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QSplashScreen* qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QObject(QObject* ptr) {
  return static_cast<QSplashScreen*>(ptr);
}

QSplashScreen* qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QSplashScreen*>(ptr);
}

QSplashScreen* qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QWidget(QWidget* ptr) {
  return static_cast<QSplashScreen*>(ptr);
}

QWidget* qt_widgets_c_QSplashScreen_G_static_cast_QWidget_ptr(QSplashScreen* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QSplashScreen_clearMessage(QSplashScreen* this_ptr) {
  this_ptr->clearMessage();
}

void qt_widgets_c_QSplashScreen_delete(QSplashScreen* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QSplashScreen_finish(QSplashScreen* this_ptr, QWidget* w) {
  this_ptr->finish(w);
}

void qt_widgets_c_QSplashScreen_message_to_output(const QSplashScreen* this_ptr, QString* output) {
  new(output) QString(this_ptr->message());
}

const QMetaObject* qt_widgets_c_QSplashScreen_metaObject(const QSplashScreen* this_ptr) {
  return this_ptr->metaObject();
}

QPixmap* qt_widgets_c_QSplashScreen_pixmap_as_ptr(const QSplashScreen* this_ptr) {
  return new QPixmap(this_ptr->pixmap());
}

int qt_widgets_c_QSplashScreen_qt_metacall(QSplashScreen* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QSplashScreen_qt_metacast(QSplashScreen* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QSplashScreen_repaint(QSplashScreen* this_ptr) {
  this_ptr->repaint();
}

void qt_widgets_c_QSplashScreen_setPixmap(QSplashScreen* this_ptr, const QPixmap* pixmap) {
  this_ptr->setPixmap(*pixmap);
}

void qt_widgets_c_QSplashScreen_showMessage_message(QSplashScreen* this_ptr, const QString* message) {
  this_ptr->showMessage(*message);
}

void qt_widgets_c_QSplashScreen_showMessage_message_alignment(QSplashScreen* this_ptr, const QString* message, int alignment) {
  this_ptr->showMessage(*message, alignment);
}

void qt_widgets_c_QSplashScreen_showMessage_message_alignment_color(QSplashScreen* this_ptr, const QString* message, int alignment, const QColor* color) {
  this_ptr->showMessage(*message, alignment, *color);
}

void qt_widgets_c_QSplashScreen_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSplashScreen::trUtf8(s, c, n));
}

void qt_widgets_c_QSplashScreen_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QSplashScreen::tr(s, c, n));
}

