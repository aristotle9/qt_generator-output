#ifndef QT_GUI_C_QTEXTDOCUMENTFRAGMENT_H
#define QT_GUI_C_QTEXTDOCUMENTFRAGMENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_constructor_document(const QTextDocument* document, QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_constructor_no_args(QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_constructor_range(const QTextCursor* range, QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_constructor_rhs(const QTextDocumentFragment* rhs, QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_destructor(QTextDocumentFragment* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_fromHtml_to_output_html(const QString* html, QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_fromHtml_to_output_html_resourceProvider(const QString* html, const QTextDocument* resourceProvider, QTextDocumentFragment* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_fromPlainText_to_output(const QString* plainText, QTextDocumentFragment* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextDocumentFragment_isEmpty(const QTextDocumentFragment* this_ptr);
QT_GUI_C_EXPORT QTextDocumentFragment* qt_gui_c_QTextDocumentFragment_operator_assign(QTextDocumentFragment* this_ptr, const QTextDocumentFragment* rhs);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_toHtml_to_output_encoding(const QTextDocumentFragment* this_ptr, const QByteArray* encoding, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_toHtml_to_output_no_args(const QTextDocumentFragment* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextDocumentFragment_toPlainText_to_output(const QTextDocumentFragment* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTDOCUMENTFRAGMENT_H
