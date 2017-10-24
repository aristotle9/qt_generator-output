#include "qt_gui_c_QTextItem.h"

double qt_gui_c_QTextItem_ascent(const QTextItem* this_ptr) {
  return this_ptr->ascent();
}

double qt_gui_c_QTextItem_descent(const QTextItem* this_ptr) {
  return this_ptr->descent();
}

void qt_gui_c_QTextItem_destructor(QTextItem* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextItem_font_to_output(const QTextItem* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

unsigned int qt_gui_c_QTextItem_renderFlags(const QTextItem* this_ptr) {
  return uint(this_ptr->renderFlags());
}

void qt_gui_c_QTextItem_text_to_output(const QTextItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

double qt_gui_c_QTextItem_width(const QTextItem* this_ptr) {
  return this_ptr->width();
}

