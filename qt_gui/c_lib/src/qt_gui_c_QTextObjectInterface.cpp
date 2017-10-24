#include "qt_gui_c_QTextObjectInterface.h"

void qt_gui_c_QTextObjectInterface_delete(QTextObjectInterface* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTextObjectInterface_drawObject(QTextObjectInterface* this_ptr, QPainter* painter, const QRectF* rect, QTextDocument* doc, int posInDocument, const QTextFormat* format) {
  this_ptr->drawObject(painter, *rect, doc, posInDocument, *format);
}

void qt_gui_c_QTextObjectInterface_intrinsicSize_to_output(QTextObjectInterface* this_ptr, QTextDocument* doc, int posInDocument, const QTextFormat* format, QSizeF* output) {
  new(output) QSizeF(this_ptr->intrinsicSize(doc, posInDocument, *format));
}

