#ifndef QT_WIDGETS_C_QGRAPHICSTEXTITEM_H
#define QT_WIDGETS_C_QGRAPHICSTEXTITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsObject(QGraphicsObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsItem_ptr(QGraphicsTextItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsObject* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsObject_ptr(QGraphicsTextItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsItem(QGraphicsItem* ptr);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsObject(QGraphicsObject* ptr);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QGraphicsTextItem_G_static_cast_QObject_ptr(QGraphicsTextItem* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_adjustSize(QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_boundingRect_to_output(const QGraphicsTextItem* this_ptr, QRectF* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsTextItem_contains(const QGraphicsTextItem* this_ptr, const QPointF* point);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_defaultTextColor_to_output(const QGraphicsTextItem* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_delete(QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT QTextDocument* qt_widgets_c_QGraphicsTextItem_document(const QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_font_to_output(const QGraphicsTextItem* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsTextItem_isObscuredBy(const QGraphicsTextItem* this_ptr, const QGraphicsItem* item);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QGraphicsTextItem_metaObject(const QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_no_args();
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_parent(QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_text_parent(const QString* text, QGraphicsItem* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_opaqueArea_to_output(const QGraphicsTextItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsTextItem_openExternalLinks(const QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_paint(QGraphicsTextItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsTextItem_qt_metacall(QGraphicsTextItem* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QGraphicsTextItem_qt_metacast(QGraphicsTextItem* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setDefaultTextColor(QGraphicsTextItem* this_ptr, const QColor* c);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setDocument(QGraphicsTextItem* this_ptr, QTextDocument* document);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setFont(QGraphicsTextItem* this_ptr, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setHtml(QGraphicsTextItem* this_ptr, const QString* html);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setOpenExternalLinks(QGraphicsTextItem* this_ptr, bool open);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setPlainText(QGraphicsTextItem* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setTabChangesFocus(QGraphicsTextItem* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setTextCursor(QGraphicsTextItem* this_ptr, const QTextCursor* cursor);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_setTextWidth(QGraphicsTextItem* this_ptr, double width);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_shape_to_output(const QGraphicsTextItem* this_ptr, QPainterPath* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QGraphicsTextItem_tabChangesFocus(const QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT QTextCursor* qt_widgets_c_QGraphicsTextItem_textCursor_as_ptr(const QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QGraphicsTextItem_textWidth(const QGraphicsTextItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_toHtml_to_output(const QGraphicsTextItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_toPlainText_to_output(const QGraphicsTextItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QGraphicsTextItem_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QGraphicsTextItem_type(const QGraphicsTextItem* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QGRAPHICSTEXTITEM_H
