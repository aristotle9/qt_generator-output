#include "qt_widgets_c_QLabel.h"

QLabel* qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QLabel*>(ptr);
}

QLabel* qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QLabel*>(ptr);
}

QFrame* qt_widgets_c_QLabel_G_static_cast_QFrame_ptr(QLabel* ptr) {
  return static_cast<QFrame*>(ptr);
}

QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QFrame(QFrame* ptr) {
  return static_cast<QLabel*>(ptr);
}

QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QObject(QObject* ptr) {
  return static_cast<QLabel*>(ptr);
}

QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QLabel*>(ptr);
}

QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QWidget(QWidget* ptr) {
  return static_cast<QLabel*>(ptr);
}

QObject* qt_widgets_c_QLabel_G_static_cast_QObject_ptr(QLabel* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QLabel_G_static_cast_QPaintDevice_ptr(QLabel* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QLabel_G_static_cast_QWidget_ptr(QLabel* ptr) {
  return static_cast<QWidget*>(ptr);
}

QWidget* qt_widgets_c_QLabel_buddy(const QLabel* this_ptr) {
  return this_ptr->buddy();
}

void qt_widgets_c_QLabel_clear(QLabel* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QLabel_delete(QLabel* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QLabel_hasScaledContents(const QLabel* this_ptr) {
  return this_ptr->hasScaledContents();
}

bool qt_widgets_c_QLabel_hasSelectedText(const QLabel* this_ptr) {
  return this_ptr->hasSelectedText();
}

int qt_widgets_c_QLabel_heightForWidth(const QLabel* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

int qt_widgets_c_QLabel_indent(const QLabel* this_ptr) {
  return this_ptr->indent();
}

int qt_widgets_c_QLabel_margin(const QLabel* this_ptr) {
  return this_ptr->margin();
}

const QMetaObject* qt_widgets_c_QLabel_metaObject(const QLabel* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QLabel_minimumSizeHint_to_output(const QLabel* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QMovie* qt_widgets_c_QLabel_movie(const QLabel* this_ptr) {
  return this_ptr->movie();
}

bool qt_widgets_c_QLabel_openExternalLinks(const QLabel* this_ptr) {
  return this_ptr->openExternalLinks();
}

const QPicture* qt_widgets_c_QLabel_picture(const QLabel* this_ptr) {
  return this_ptr->picture();
}

const QPixmap* qt_widgets_c_QLabel_pixmap(const QLabel* this_ptr) {
  return this_ptr->pixmap();
}

int qt_widgets_c_QLabel_qt_metacall(QLabel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QLabel_qt_metacast(QLabel* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QLabel_selectedText_to_output(const QLabel* this_ptr, QString* output) {
  new(output) QString(this_ptr->selectedText());
}

int qt_widgets_c_QLabel_selectionStart(const QLabel* this_ptr) {
  return this_ptr->selectionStart();
}

void qt_widgets_c_QLabel_setBuddy(QLabel* this_ptr, QWidget* arg1) {
  this_ptr->setBuddy(arg1);
}

void qt_widgets_c_QLabel_setIndent(QLabel* this_ptr, int arg1) {
  this_ptr->setIndent(arg1);
}

void qt_widgets_c_QLabel_setMargin(QLabel* this_ptr, int arg1) {
  this_ptr->setMargin(arg1);
}

void qt_widgets_c_QLabel_setMovie(QLabel* this_ptr, QMovie* movie) {
  this_ptr->setMovie(movie);
}

void qt_widgets_c_QLabel_setNum_double(QLabel* this_ptr, double arg1) {
  this_ptr->setNum(arg1);
}

void qt_widgets_c_QLabel_setNum_int(QLabel* this_ptr, int arg1) {
  this_ptr->setNum(arg1);
}

void qt_widgets_c_QLabel_setOpenExternalLinks(QLabel* this_ptr, bool open) {
  this_ptr->setOpenExternalLinks(open);
}

void qt_widgets_c_QLabel_setPicture(QLabel* this_ptr, const QPicture* arg1) {
  this_ptr->setPicture(*arg1);
}

void qt_widgets_c_QLabel_setPixmap(QLabel* this_ptr, const QPixmap* arg1) {
  this_ptr->setPixmap(*arg1);
}

void qt_widgets_c_QLabel_setScaledContents(QLabel* this_ptr, bool arg1) {
  this_ptr->setScaledContents(arg1);
}

void qt_widgets_c_QLabel_setSelection(QLabel* this_ptr, int arg1, int arg2) {
  this_ptr->setSelection(arg1, arg2);
}

void qt_widgets_c_QLabel_setText(QLabel* this_ptr, const QString* arg1) {
  this_ptr->setText(*arg1);
}

void qt_widgets_c_QLabel_setTextFormat(QLabel* this_ptr, const Qt::TextFormat* arg1) {
  this_ptr->setTextFormat(*arg1);
}

void qt_widgets_c_QLabel_setWordWrap(QLabel* this_ptr, bool on) {
  this_ptr->setWordWrap(on);
}

void qt_widgets_c_QLabel_sizeHint_to_output(const QLabel* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QLabel_text_to_output(const QLabel* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_widgets_c_QLabel_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLabel::trUtf8(s, c, n));
}

void qt_widgets_c_QLabel_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QLabel::tr(s, c, n));
}

bool qt_widgets_c_QLabel_wordWrap(const QLabel* this_ptr) {
  return this_ptr->wordWrap();
}

