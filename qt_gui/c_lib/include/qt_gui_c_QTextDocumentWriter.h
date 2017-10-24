#ifndef QT_GUI_C_QTEXTDOCUMENTWRITER_H
#define QT_GUI_C_QTEXTDOCUMENTWRITER_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextCodec* qt_gui_c_QTextDocumentWriter_codec(const QTextDocumentWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_constructor_device_format(QIODevice* device, const QByteArray* format, QTextDocumentWriter* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_constructor_fileName(const QString* fileName, QTextDocumentWriter* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_constructor_fileName_format(const QString* fileName, const QByteArray* format, QTextDocumentWriter* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_constructor_no_args(QTextDocumentWriter* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_destructor(QTextDocumentWriter* this_ptr);
QT_GUI_C_EXPORT QIODevice* qt_gui_c_QTextDocumentWriter_device(const QTextDocumentWriter* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_fileName_to_output(const QTextDocumentWriter* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_format_to_output(const QTextDocumentWriter* this_ptr, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_setCodec(QTextDocumentWriter* this_ptr, QTextCodec* codec);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_setDevice(QTextDocumentWriter* this_ptr, QIODevice* device);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_setFileName(QTextDocumentWriter* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_setFormat(QTextDocumentWriter* this_ptr, const QByteArray* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentWriter_supportedDocumentFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocumentWriter_write_document(QTextDocumentWriter* this_ptr, const QTextDocument* document);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocumentWriter_write_fragment(QTextDocumentWriter* this_ptr, const QTextDocumentFragment* fragment);

} // extern "C"

#endif // QT_GUI_C_QTEXTDOCUMENTWRITER_H
