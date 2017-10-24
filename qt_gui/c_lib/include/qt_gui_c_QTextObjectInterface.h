#ifndef QT_GUI_C_QTEXTOBJECTINTERFACE_H
#define QT_GUI_C_QTEXTOBJECTINTERFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextObjectInterface_delete(QTextObjectInterface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextObjectInterface_drawObject(QTextObjectInterface* this_ptr, QPainter* painter, const QRectF* rect, QTextDocument* doc, int posInDocument, const QTextFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextObjectInterface_intrinsicSize_to_output(QTextObjectInterface* this_ptr, QTextDocument* doc, int posInDocument, const QTextFormat* format, QSizeF* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTOBJECTINTERFACE_H
