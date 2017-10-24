#include "qt_gui_c_QTextDocumentWriter.h"

QTextCodec* qt_gui_c_QTextDocumentWriter_codec(const QTextDocumentWriter* this_ptr) {
  return this_ptr->codec();
}

void qt_gui_c_QTextDocumentWriter_constructor_device_format(QIODevice* device, const QByteArray* format, QTextDocumentWriter* output) {
  new(output) QTextDocumentWriter(device, *format);
}

void qt_gui_c_QTextDocumentWriter_constructor_fileName(const QString* fileName, QTextDocumentWriter* output) {
  new(output) QTextDocumentWriter(*fileName);
}

void qt_gui_c_QTextDocumentWriter_constructor_fileName_format(const QString* fileName, const QByteArray* format, QTextDocumentWriter* output) {
  new(output) QTextDocumentWriter(*fileName, *format);
}

void qt_gui_c_QTextDocumentWriter_constructor_no_args(QTextDocumentWriter* output) {
  new(output) QTextDocumentWriter();
}

void qt_gui_c_QTextDocumentWriter_destructor(QTextDocumentWriter* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

QIODevice* qt_gui_c_QTextDocumentWriter_device(const QTextDocumentWriter* this_ptr) {
  return this_ptr->device();
}

void qt_gui_c_QTextDocumentWriter_fileName_to_output(const QTextDocumentWriter* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_gui_c_QTextDocumentWriter_format_to_output(const QTextDocumentWriter* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->format());
}

void qt_gui_c_QTextDocumentWriter_setCodec(QTextDocumentWriter* this_ptr, QTextCodec* codec) {
  this_ptr->setCodec(codec);
}

void qt_gui_c_QTextDocumentWriter_setDevice(QTextDocumentWriter* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_gui_c_QTextDocumentWriter_setFileName(QTextDocumentWriter* this_ptr, const QString* fileName) {
  this_ptr->setFileName(*fileName);
}

void qt_gui_c_QTextDocumentWriter_setFormat(QTextDocumentWriter* this_ptr, const QByteArray* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QTextDocumentWriter_supportedDocumentFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTextDocumentWriter::supportedDocumentFormats());
}

bool qt_gui_c_QTextDocumentWriter_write_document(QTextDocumentWriter* this_ptr, const QTextDocument* document) {
  return this_ptr->write(document);
}

bool qt_gui_c_QTextDocumentWriter_write_fragment(QTextDocumentWriter* this_ptr, const QTextDocumentFragment* fragment) {
  return this_ptr->write(*fragment);
}

