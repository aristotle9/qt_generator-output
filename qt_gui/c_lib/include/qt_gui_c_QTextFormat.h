#ifndef QT_GUI_C_QTEXTFORMAT_H
#define QT_GUI_C_QTEXTFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QTextFormat_G_operator_shl_QDataStream_QTextFormat(QDataStream* arg1, const QTextFormat* arg2);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QTextFormat_G_operator_shl_QDataStream_QTextLength(QDataStream* arg1, const QTextLength* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_operator_shl_to_output_QDebug_QTextFormat(const QDebug* arg1, const QTextFormat* arg2, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_operator_shl_to_output_QDebug_QTextLength(const QDebug* arg1, const QTextLength* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QTextFormat_G_operator_shr_QDataStream_QTextFormat(QDataStream* arg1, QTextFormat* arg2);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QTextFormat_G_operator_shr_QDataStream_QTextLength(QDataStream* arg1, QTextLength* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextBlockFormat_QTextBlockFormat(QTextBlockFormat* value1, QTextBlockFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextCharFormat_QTextCharFormat(QTextCharFormat* value1, QTextCharFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextFormat_QTextFormat(QTextFormat* value1, QTextFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextFrameFormat_QTextFrameFormat(QTextFrameFormat* value1, QTextFrameFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextImageFormat_QTextImageFormat(QTextImageFormat* value1, QTextImageFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextListFormat_QTextListFormat(QTextListFormat* value1, QTextListFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextTableCellFormat_QTextTableCellFormat(QTextTableCellFormat* value1, QTextTableCellFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_G_swap_QTextTableFormat_QTextTableFormat(QTextTableFormat* value1, QTextTableFormat* value2);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_background_to_output(const QTextFormat* this_ptr, QBrush* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_boolProperty(const QTextFormat* this_ptr, int propertyId);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_brushProperty_to_output(const QTextFormat* this_ptr, int propertyId, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_clearBackground(QTextFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_clearForeground(QTextFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_clearProperty(QTextFormat* this_ptr, int propertyId);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_colorProperty_to_output(const QTextFormat* this_ptr, int propertyId, QColor* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_constructor_no_args(QTextFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_constructor_rhs(const QTextFormat* rhs, QTextFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_constructor_type(int type, QTextFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_convert_to_QVariant_to_output(const QTextFormat* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_destructor(QTextFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextFormat_doubleProperty(const QTextFormat* this_ptr, int propertyId);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_foreground_to_output(const QTextFormat* this_ptr, QBrush* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_hasProperty(const QTextFormat* this_ptr, int propertyId);
QT_GUI_C_EXPORT int qt_gui_c_QTextFormat_intProperty(const QTextFormat* this_ptr, int propertyId);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isBlockFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isCharFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isEmpty(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isFrameFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isImageFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isListFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isTableCellFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isTableFormat(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_isValid(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_lengthProperty_to_output(const QTextFormat* this_ptr, int propertyId, QTextLength* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_lengthVectorProperty_to_output(const QTextFormat* this_ptr, int propertyId, QVector< QTextLength >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_merge(QTextFormat* this_ptr, const QTextFormat* other);
QT_GUI_C_EXPORT int qt_gui_c_QTextFormat_objectIndex(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextFormat_objectType(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextFormat_operator_assign(QTextFormat* this_ptr, const QTextFormat* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_operator_eq(const QTextFormat* this_ptr, const QTextFormat* rhs);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFormat_operator_neq(const QTextFormat* this_ptr, const QTextFormat* rhs);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_penProperty_to_output(const QTextFormat* this_ptr, int propertyId, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_properties_to_output(const QTextFormat* this_ptr, QMap< int, QVariant >* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextFormat_propertyCount(const QTextFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_property_to_output(const QTextFormat* this_ptr, int propertyId, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setBackground(QTextFormat* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setForeground(QTextFormat* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setLayoutDirection(QTextFormat* this_ptr, const Qt::LayoutDirection* direction);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setObjectIndex(QTextFormat* this_ptr, int object);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setObjectType(QTextFormat* this_ptr, int type);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setProperty_propertyId_lengths(QTextFormat* this_ptr, int propertyId, const QVector< QTextLength >* lengths);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_setProperty_propertyId_value(QTextFormat* this_ptr, int propertyId, const QVariant* value);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_stringProperty_to_output(const QTextFormat* this_ptr, int propertyId, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_swap(QTextFormat* this_ptr, QTextFormat* other);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toBlockFormat_to_output(const QTextFormat* this_ptr, QTextBlockFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toCharFormat_to_output(const QTextFormat* this_ptr, QTextCharFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toFrameFormat_to_output(const QTextFormat* this_ptr, QTextFrameFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toImageFormat_to_output(const QTextFormat* this_ptr, QTextImageFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toListFormat_to_output(const QTextFormat* this_ptr, QTextListFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toTableCellFormat_to_output(const QTextFormat* this_ptr, QTextTableCellFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFormat_toTableFormat_to_output(const QTextFormat* this_ptr, QTextTableFormat* output);
QT_GUI_C_EXPORT int qt_gui_c_QTextFormat_type(const QTextFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTFORMAT_H
