#ifndef QT_CORE_C_QVARIANT_H
#define QT_CORE_C_QVARIANT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QVariant_G_operator_shl_QDataStream_QVariant(QDataStream* s, const QVariant* p);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QVariant_G_operator_shl_QDataStream_QVariant_Type(QDataStream* s, const QVariant::Type* p);
QT_CORE_C_EXPORT void qt_core_c_QVariant_G_operator_shl_to_output(const QDebug* arg1, const QVariant::Type* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QVariant_G_operator_shr_QDataStream_QVariant(QDataStream* s, QVariant* p);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QVariant_G_operator_shr_QDataStream_QVariant_Type(QDataStream* s, QVariant::Type* p);
QT_CORE_C_EXPORT void qt_core_c_QVariant_G_swap(QVariant* value1, QVariant* value2);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_canConvert(const QVariant* this_ptr, int targetTypeId);
QT_CORE_C_EXPORT void qt_core_c_QVariant_clear(QVariant* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QBitArray(const QBitArray* bitarray, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QByteArray(const QByteArray* bytearray, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QChar(const QChar* qchar, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QDataStream(QDataStream* s, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QDate(const QDate* date, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QDateTime(const QDateTime* datetime, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QEasingCurve(const QEasingCurve* easing, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QHash_QString_QVariant(const QHash< QString, QVariant >* hash, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QJsonArray(const QJsonArray* jsonArray, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QJsonDocument(const QJsonDocument* jsonDocument, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QJsonObject(const QJsonObject* jsonObject, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QJsonValue(const QJsonValue* jsonValue, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QLatin1String(const QLatin1String* string, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QLine(const QLine* line, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QLineF(const QLineF* line, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QList_QVariant(const QList< QVariant >* list, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QLocale(const QLocale* locale, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QMap_QString_QVariant(const QMap< QString, QVariant >* map, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QModelIndex(const QModelIndex* modelIndex, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QPersistentModelIndex(const QPersistentModelIndex* modelIndex, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QPoint(const QPoint* pt, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QPointF(const QPointF* pt, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QRect(const QRect* rect, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QRectF(const QRectF* rect, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QRegExp(const QRegExp* regExp, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QRegularExpression(const QRegularExpression* re, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QSize(const QSize* size, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QSizeF(const QSizeF* size, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QString(const QString* string, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QStringList(const QStringList* stringlist, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QTime(const QTime* time, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QUrl(const QUrl* url, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QUuid(const QUuid* uuid, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QVariant(const QVariant* other, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_QVariant_Type(QVariant::Type type, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_bool(bool b, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_char(const char* str, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_double(double d, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_float(float f, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_int(int i, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_int_void(int typeId, const void* copy, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_int_void_unsigned_int(int typeId, const void* copy, unsigned int flags, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_no_args(QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_qlonglong(qlonglong ll, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_qulonglong(qulonglong ull, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_constructor_unsigned_int(unsigned int ui, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_destructor(QVariant* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_isNull(const QVariant* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_isValid(const QVariant* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVariant_load(QVariant* this_ptr, QDataStream* ds);
QT_CORE_C_EXPORT QVariant::Type qt_core_c_QVariant_nameToType(const char* name);
QT_CORE_C_EXPORT QVariant* qt_core_c_QVariant_operator_assign(QVariant* this_ptr, const QVariant* other);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_operator_eq(const QVariant* this_ptr, const QVariant* v);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_operator_ge(const QVariant* this_ptr, const QVariant* v);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_operator_gt(const QVariant* this_ptr, const QVariant* v);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_operator_le(const QVariant* this_ptr, const QVariant* v);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_operator_lt(const QVariant* this_ptr, const QVariant* v);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_operator_neq(const QVariant* this_ptr, const QVariant* v);
QT_CORE_C_EXPORT void qt_core_c_QVariant_save(const QVariant* this_ptr, QDataStream* ds);
QT_CORE_C_EXPORT void qt_core_c_QVariant_swap(QVariant* this_ptr, QVariant* other);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toBitArray_to_output(const QVariant* this_ptr, QBitArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QVariant_toBool(const QVariant* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toByteArray_to_output(const QVariant* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toChar_to_output(const QVariant* this_ptr, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toDateTime_to_output(const QVariant* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toDate_to_output(const QVariant* this_ptr, QDate* output);
QT_CORE_C_EXPORT double qt_core_c_QVariant_toDouble_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QVariant_toDouble_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toEasingCurve_to_output(const QVariant* this_ptr, QEasingCurve* output);
QT_CORE_C_EXPORT float qt_core_c_QVariant_toFloat_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT float qt_core_c_QVariant_toFloat_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toHash_to_output(const QVariant* this_ptr, QHash< QString, QVariant >* output);
QT_CORE_C_EXPORT int qt_core_c_QVariant_toInt_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QVariant_toInt_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toJsonArray_to_output(const QVariant* this_ptr, QJsonArray* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toJsonDocument_to_output(const QVariant* this_ptr, QJsonDocument* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toJsonObject_to_output(const QVariant* this_ptr, QJsonObject* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toJsonValue_to_output(const QVariant* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toLineF_to_output(const QVariant* this_ptr, QLineF* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toLine_to_output(const QVariant* this_ptr, QLine* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toList_to_output(const QVariant* this_ptr, QList< QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toLocale_to_output(const QVariant* this_ptr, QLocale* output);
QT_CORE_C_EXPORT qlonglong qt_core_c_QVariant_toLongLong_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT qlonglong qt_core_c_QVariant_toLongLong_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toMap_to_output(const QVariant* this_ptr, QMap< QString, QVariant >* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toModelIndex_to_output(const QVariant* this_ptr, QModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toPersistentModelIndex_to_output(const QVariant* this_ptr, QPersistentModelIndex* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toPointF_to_output(const QVariant* this_ptr, QPointF* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toPoint_to_output(const QVariant* this_ptr, QPoint* output);
QT_CORE_C_EXPORT double qt_core_c_QVariant_toReal_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT double qt_core_c_QVariant_toReal_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toRectF_to_output(const QVariant* this_ptr, QRectF* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toRect_to_output(const QVariant* this_ptr, QRect* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toRegExp_to_output(const QVariant* this_ptr, QRegExp* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toRegularExpression_to_output(const QVariant* this_ptr, QRegularExpression* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toSizeF_to_output(const QVariant* this_ptr, QSizeF* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toSize_to_output(const QVariant* this_ptr, QSize* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toStringList_to_output(const QVariant* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toString_to_output(const QVariant* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toTime_to_output(const QVariant* this_ptr, QTime* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QVariant_toUInt_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QVariant_toUInt_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT qulonglong qt_core_c_QVariant_toULongLong_no_args(const QVariant* this_ptr);
QT_CORE_C_EXPORT qulonglong qt_core_c_QVariant_toULongLong_ok(const QVariant* this_ptr, bool* ok);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toUrl_to_output(const QVariant* this_ptr, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QVariant_toUuid_to_output(const QVariant* this_ptr, QUuid* output);
QT_CORE_C_EXPORT QVariant::Type qt_core_c_QVariant_type(const QVariant* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QVariant_typeName(const QVariant* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QVariant_typeToName(int typeId);
QT_CORE_C_EXPORT int qt_core_c_QVariant_userType(const QVariant* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QVARIANT_H
