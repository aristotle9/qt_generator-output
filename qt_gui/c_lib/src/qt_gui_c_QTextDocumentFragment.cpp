#include "qt_gui_c_QTextDocumentFragment.h"

void qt_gui_c_QTextDocumentFragment_constructor_document(const QTextDocument* document, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(document);
}

void qt_gui_c_QTextDocumentFragment_constructor_no_args(QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment();
}

void qt_gui_c_QTextDocumentFragment_constructor_range(const QTextCursor* range, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(*range);
}

void qt_gui_c_QTextDocumentFragment_constructor_rhs(const QTextDocumentFragment* rhs, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(*rhs);
}

void qt_gui_c_QTextDocumentFragment_destructor(QTextDocumentFragment* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextDocumentFragment_fromHtml_to_output_html(const QString* html, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(QTextDocumentFragment::fromHtml(*html));
}

void qt_gui_c_QTextDocumentFragment_fromHtml_to_output_html_resourceProvider(const QString* html, const QTextDocument* resourceProvider, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(QTextDocumentFragment::fromHtml(*html, resourceProvider));
}

void qt_gui_c_QTextDocumentFragment_fromPlainText_to_output(const QString* plainText, QTextDocumentFragment* output) {
  new(output) QTextDocumentFragment(QTextDocumentFragment::fromPlainText(*plainText));
}

bool qt_gui_c_QTextDocumentFragment_isEmpty(const QTextDocumentFragment* this_ptr) {
  return this_ptr->isEmpty();
}

QTextDocumentFragment* qt_gui_c_QTextDocumentFragment_operator_assign(QTextDocumentFragment* this_ptr, const QTextDocumentFragment* rhs) {
  return &this_ptr->operator=(*rhs);
}

void qt_gui_c_QTextDocumentFragment_toHtml_to_output_encoding(const QTextDocumentFragment* this_ptr, const QByteArray* encoding, QString* output) {
  new(output) QString(this_ptr->toHtml(*encoding));
}

void qt_gui_c_QTextDocumentFragment_toHtml_to_output_no_args(const QTextDocumentFragment* this_ptr, QString* output) {
  new(output) QString(this_ptr->toHtml());
}

void qt_gui_c_QTextDocumentFragment_toPlainText_to_output(const QTextDocumentFragment* this_ptr, QString* output) {
  new(output) QString(this_ptr->toPlainText());
}

