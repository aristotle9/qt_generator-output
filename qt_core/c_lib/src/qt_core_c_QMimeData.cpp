#include "qt_core_c_QMimeData.h"

QMimeData* qt_core_c_QMimeData_G_dynamic_cast_QMimeData_ptr(QObject* ptr) {
  return dynamic_cast<QMimeData*>(ptr);
}

QMimeData* qt_core_c_QMimeData_G_static_cast_QMimeData_ptr(QObject* ptr) {
  return static_cast<QMimeData*>(ptr);
}

QObject* qt_core_c_QMimeData_G_static_cast_QObject_ptr(QMimeData* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_core_c_QMimeData_clear(QMimeData* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QMimeData_colorData_to_output(const QMimeData* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->colorData());
}

void qt_core_c_QMimeData_data_to_output(const QMimeData* this_ptr, const QString* mimetype, QByteArray* output) {
  new(output) QByteArray(this_ptr->data(*mimetype));
}

void qt_core_c_QMimeData_delete(QMimeData* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QMimeData_formats_to_output(const QMimeData* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->formats());
}

bool qt_core_c_QMimeData_hasColor(const QMimeData* this_ptr) {
  return this_ptr->hasColor();
}

bool qt_core_c_QMimeData_hasFormat(const QMimeData* this_ptr, const QString* mimetype) {
  return this_ptr->hasFormat(*mimetype);
}

bool qt_core_c_QMimeData_hasHtml(const QMimeData* this_ptr) {
  return this_ptr->hasHtml();
}

bool qt_core_c_QMimeData_hasImage(const QMimeData* this_ptr) {
  return this_ptr->hasImage();
}

bool qt_core_c_QMimeData_hasText(const QMimeData* this_ptr) {
  return this_ptr->hasText();
}

bool qt_core_c_QMimeData_hasUrls(const QMimeData* this_ptr) {
  return this_ptr->hasUrls();
}

void qt_core_c_QMimeData_html_to_output(const QMimeData* this_ptr, QString* output) {
  new(output) QString(this_ptr->html());
}

void qt_core_c_QMimeData_imageData_to_output(const QMimeData* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->imageData());
}

const QMetaObject* qt_core_c_QMimeData_metaObject(const QMimeData* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_core_c_QMimeData_new() {
  return new QMimeData();
}

void qt_core_c_QMimeData_removeFormat(QMimeData* this_ptr, const QString* mimetype) {
  this_ptr->removeFormat(*mimetype);
}

void qt_core_c_QMimeData_setColorData(QMimeData* this_ptr, const QVariant* color) {
  this_ptr->setColorData(*color);
}

void qt_core_c_QMimeData_setData(QMimeData* this_ptr, const QString* mimetype, const QByteArray* data) {
  this_ptr->setData(*mimetype, *data);
}

void qt_core_c_QMimeData_setHtml(QMimeData* this_ptr, const QString* html) {
  this_ptr->setHtml(*html);
}

void qt_core_c_QMimeData_setImageData(QMimeData* this_ptr, const QVariant* image) {
  this_ptr->setImageData(*image);
}

void qt_core_c_QMimeData_setText(QMimeData* this_ptr, const QString* text) {
  this_ptr->setText(*text);
}

void qt_core_c_QMimeData_setUrls(QMimeData* this_ptr, const QList< QUrl >* urls) {
  this_ptr->setUrls(*urls);
}

void qt_core_c_QMimeData_text_to_output(const QMimeData* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_core_c_QMimeData_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMimeData::trUtf8(s, c, n));
}

void qt_core_c_QMimeData_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMimeData::tr(s, c, n));
}

void qt_core_c_QMimeData_urls_to_output(const QMimeData* this_ptr, QList< QUrl >* output) {
  new(output) QList< QUrl >(this_ptr->urls());
}

