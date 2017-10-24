#include "qt_gui_c_QTextFormat.h"

QDataStream* qt_gui_c_QTextFormat_G_operator_shl_QDataStream_QTextFormat(QDataStream* arg1, const QTextFormat* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_gui_c_QTextFormat_G_operator_shl_QDataStream_QTextLength(QDataStream* arg1, const QTextLength* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QTextFormat_G_operator_shl_to_output_QDebug_QTextFormat(const QDebug* arg1, const QTextFormat* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_gui_c_QTextFormat_G_operator_shl_to_output_QDebug_QTextLength(const QDebug* arg1, const QTextLength* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QTextFormat_G_operator_shr_QDataStream_QTextFormat(QDataStream* arg1, QTextFormat* arg2) {
  return &operator>>(*arg1, *arg2);
}

QDataStream* qt_gui_c_QTextFormat_G_operator_shr_QDataStream_QTextLength(QDataStream* arg1, QTextLength* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QTextFormat_G_swap_QTextBlockFormat_QTextBlockFormat(QTextBlockFormat* value1, QTextBlockFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextCharFormat_QTextCharFormat(QTextCharFormat* value1, QTextCharFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextFormat_QTextFormat(QTextFormat* value1, QTextFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextFrameFormat_QTextFrameFormat(QTextFrameFormat* value1, QTextFrameFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextImageFormat_QTextImageFormat(QTextImageFormat* value1, QTextImageFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextListFormat_QTextListFormat(QTextListFormat* value1, QTextListFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextTableCellFormat_QTextTableCellFormat(QTextTableCellFormat* value1, QTextTableCellFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_G_swap_QTextTableFormat_QTextTableFormat(QTextTableFormat* value1, QTextTableFormat* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QTextFormat_background_to_output(const QTextFormat* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->background());
}

bool qt_gui_c_QTextFormat_boolProperty(const QTextFormat* this_ptr, int propertyId) {
  return this_ptr->boolProperty(propertyId);
}

void qt_gui_c_QTextFormat_brushProperty_to_output(const QTextFormat* this_ptr, int propertyId, QBrush* output) {
  new(output) QBrush(this_ptr->brushProperty(propertyId));
}

void qt_gui_c_QTextFormat_clearBackground(QTextFormat* this_ptr) {
  this_ptr->clearBackground();
}

void qt_gui_c_QTextFormat_clearForeground(QTextFormat* this_ptr) {
  this_ptr->clearForeground();
}

void qt_gui_c_QTextFormat_clearProperty(QTextFormat* this_ptr, int propertyId) {
  this_ptr->clearProperty(propertyId);
}

void qt_gui_c_QTextFormat_colorProperty_to_output(const QTextFormat* this_ptr, int propertyId, QColor* output) {
  new(output) QColor(this_ptr->colorProperty(propertyId));
}

void qt_gui_c_QTextFormat_constructor_no_args(QTextFormat* output) {
  new(output) QTextFormat();
}

void qt_gui_c_QTextFormat_constructor_rhs(const QTextFormat* rhs, QTextFormat* output) {
  new(output) QTextFormat(*rhs);
}

void qt_gui_c_QTextFormat_constructor_type(int type, QTextFormat* output) {
  new(output) QTextFormat(type);
}

void qt_gui_c_QTextFormat_convert_to_QVariant_to_output(const QTextFormat* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QTextFormat_destructor(QTextFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

double qt_gui_c_QTextFormat_doubleProperty(const QTextFormat* this_ptr, int propertyId) {
  return this_ptr->doubleProperty(propertyId);
}

void qt_gui_c_QTextFormat_foreground_to_output(const QTextFormat* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->foreground());
}

bool qt_gui_c_QTextFormat_hasProperty(const QTextFormat* this_ptr, int propertyId) {
  return this_ptr->hasProperty(propertyId);
}

int qt_gui_c_QTextFormat_intProperty(const QTextFormat* this_ptr, int propertyId) {
  return this_ptr->intProperty(propertyId);
}

bool qt_gui_c_QTextFormat_isBlockFormat(const QTextFormat* this_ptr) {
  return this_ptr->isBlockFormat();
}

bool qt_gui_c_QTextFormat_isCharFormat(const QTextFormat* this_ptr) {
  return this_ptr->isCharFormat();
}

bool qt_gui_c_QTextFormat_isEmpty(const QTextFormat* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_gui_c_QTextFormat_isFrameFormat(const QTextFormat* this_ptr) {
  return this_ptr->isFrameFormat();
}

bool qt_gui_c_QTextFormat_isImageFormat(const QTextFormat* this_ptr) {
  return this_ptr->isImageFormat();
}

bool qt_gui_c_QTextFormat_isListFormat(const QTextFormat* this_ptr) {
  return this_ptr->isListFormat();
}

bool qt_gui_c_QTextFormat_isTableCellFormat(const QTextFormat* this_ptr) {
  return this_ptr->isTableCellFormat();
}

bool qt_gui_c_QTextFormat_isTableFormat(const QTextFormat* this_ptr) {
  return this_ptr->isTableFormat();
}

bool qt_gui_c_QTextFormat_isValid(const QTextFormat* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QTextFormat_lengthProperty_to_output(const QTextFormat* this_ptr, int propertyId, QTextLength* output) {
  new(output) QTextLength(this_ptr->lengthProperty(propertyId));
}

void qt_gui_c_QTextFormat_lengthVectorProperty_to_output(const QTextFormat* this_ptr, int propertyId, QVector< QTextLength >* output) {
  new(output) QVector< QTextLength >(this_ptr->lengthVectorProperty(propertyId));
}

void qt_gui_c_QTextFormat_merge(QTextFormat* this_ptr, const QTextFormat* other) {
  this_ptr->merge(*other);
}

int qt_gui_c_QTextFormat_objectIndex(const QTextFormat* this_ptr) {
  return this_ptr->objectIndex();
}

int qt_gui_c_QTextFormat_objectType(const QTextFormat* this_ptr) {
  return this_ptr->objectType();
}

QTextFormat* qt_gui_c_QTextFormat_operator_assign(QTextFormat* this_ptr, const QTextFormat* rhs) {
  return &this_ptr->operator=(*rhs);
}

bool qt_gui_c_QTextFormat_operator_eq(const QTextFormat* this_ptr, const QTextFormat* rhs) {
  return this_ptr->operator==(*rhs);
}

bool qt_gui_c_QTextFormat_operator_neq(const QTextFormat* this_ptr, const QTextFormat* rhs) {
  return this_ptr->operator!=(*rhs);
}

void qt_gui_c_QTextFormat_penProperty_to_output(const QTextFormat* this_ptr, int propertyId, QPen* output) {
  new(output) QPen(this_ptr->penProperty(propertyId));
}

void qt_gui_c_QTextFormat_properties_to_output(const QTextFormat* this_ptr, QMap< int, QVariant >* output) {
  new(output) QMap< int, QVariant >(this_ptr->properties());
}

int qt_gui_c_QTextFormat_propertyCount(const QTextFormat* this_ptr) {
  return this_ptr->propertyCount();
}

void qt_gui_c_QTextFormat_property_to_output(const QTextFormat* this_ptr, int propertyId, QVariant* output) {
  new(output) QVariant(this_ptr->property(propertyId));
}

void qt_gui_c_QTextFormat_setBackground(QTextFormat* this_ptr, const QBrush* brush) {
  this_ptr->setBackground(*brush);
}

void qt_gui_c_QTextFormat_setForeground(QTextFormat* this_ptr, const QBrush* brush) {
  this_ptr->setForeground(*brush);
}

void qt_gui_c_QTextFormat_setLayoutDirection(QTextFormat* this_ptr, const Qt::LayoutDirection* direction) {
  this_ptr->setLayoutDirection(*direction);
}

void qt_gui_c_QTextFormat_setObjectIndex(QTextFormat* this_ptr, int object) {
  this_ptr->setObjectIndex(object);
}

void qt_gui_c_QTextFormat_setObjectType(QTextFormat* this_ptr, int type) {
  this_ptr->setObjectType(type);
}

void qt_gui_c_QTextFormat_setProperty_propertyId_lengths(QTextFormat* this_ptr, int propertyId, const QVector< QTextLength >* lengths) {
  this_ptr->setProperty(propertyId, *lengths);
}

void qt_gui_c_QTextFormat_setProperty_propertyId_value(QTextFormat* this_ptr, int propertyId, const QVariant* value) {
  this_ptr->setProperty(propertyId, *value);
}

void qt_gui_c_QTextFormat_stringProperty_to_output(const QTextFormat* this_ptr, int propertyId, QString* output) {
  new(output) QString(this_ptr->stringProperty(propertyId));
}

void qt_gui_c_QTextFormat_swap(QTextFormat* this_ptr, QTextFormat* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QTextFormat_toBlockFormat_to_output(const QTextFormat* this_ptr, QTextBlockFormat* output) {
  new(output) QTextBlockFormat(this_ptr->toBlockFormat());
}

void qt_gui_c_QTextFormat_toCharFormat_to_output(const QTextFormat* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->toCharFormat());
}

void qt_gui_c_QTextFormat_toFrameFormat_to_output(const QTextFormat* this_ptr, QTextFrameFormat* output) {
  new(output) QTextFrameFormat(this_ptr->toFrameFormat());
}

void qt_gui_c_QTextFormat_toImageFormat_to_output(const QTextFormat* this_ptr, QTextImageFormat* output) {
  new(output) QTextImageFormat(this_ptr->toImageFormat());
}

void qt_gui_c_QTextFormat_toListFormat_to_output(const QTextFormat* this_ptr, QTextListFormat* output) {
  new(output) QTextListFormat(this_ptr->toListFormat());
}

void qt_gui_c_QTextFormat_toTableCellFormat_to_output(const QTextFormat* this_ptr, QTextTableCellFormat* output) {
  new(output) QTextTableCellFormat(this_ptr->toTableCellFormat());
}

void qt_gui_c_QTextFormat_toTableFormat_to_output(const QTextFormat* this_ptr, QTextTableFormat* output) {
  new(output) QTextTableFormat(this_ptr->toTableFormat());
}

int qt_gui_c_QTextFormat_type(const QTextFormat* this_ptr) {
  return this_ptr->type();
}

