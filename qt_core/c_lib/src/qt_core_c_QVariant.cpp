#include "qt_core_c_QVariant.h"

QDataStream* qt_core_c_QVariant_G_operator_shl_QDataStream_QVariant(QDataStream* s, const QVariant* p) {
  return &operator<<(*s, *p);
}

QDataStream* qt_core_c_QVariant_G_operator_shl_QDataStream_QVariant_Type(QDataStream* s, const QVariant::Type* p) {
  return &operator<<(*s, *p);
}

void qt_core_c_QVariant_G_operator_shl_to_output(const QDebug* arg1, const QVariant::Type* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QVariant_G_operator_shr_QDataStream_QVariant(QDataStream* s, QVariant* p) {
  return &operator>>(*s, *p);
}

QDataStream* qt_core_c_QVariant_G_operator_shr_QDataStream_QVariant_Type(QDataStream* s, QVariant::Type* p) {
  return &operator>>(*s, *p);
}

void qt_core_c_QVariant_G_swap(QVariant* value1, QVariant* value2) {
  swap(*value1, *value2);
}

bool qt_core_c_QVariant_canConvert(const QVariant* this_ptr, int targetTypeId) {
  return this_ptr->canConvert(targetTypeId);
}

void qt_core_c_QVariant_clear(QVariant* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QVariant_constructor_QBitArray(const QBitArray* bitarray, QVariant* output) {
  new(output) QVariant(*bitarray);
}

void qt_core_c_QVariant_constructor_QByteArray(const QByteArray* bytearray, QVariant* output) {
  new(output) QVariant(*bytearray);
}

void qt_core_c_QVariant_constructor_QChar(const QChar* qchar, QVariant* output) {
  new(output) QVariant(*qchar);
}

void qt_core_c_QVariant_constructor_QDataStream(QDataStream* s, QVariant* output) {
  new(output) QVariant(*s);
}

void qt_core_c_QVariant_constructor_QDate(const QDate* date, QVariant* output) {
  new(output) QVariant(*date);
}

void qt_core_c_QVariant_constructor_QDateTime(const QDateTime* datetime, QVariant* output) {
  new(output) QVariant(*datetime);
}

void qt_core_c_QVariant_constructor_QEasingCurve(const QEasingCurve* easing, QVariant* output) {
  new(output) QVariant(*easing);
}

void qt_core_c_QVariant_constructor_QHash_QString_QVariant(const QHash< QString, QVariant >* hash, QVariant* output) {
  new(output) QVariant(*hash);
}

void qt_core_c_QVariant_constructor_QJsonArray(const QJsonArray* jsonArray, QVariant* output) {
  new(output) QVariant(*jsonArray);
}

void qt_core_c_QVariant_constructor_QJsonDocument(const QJsonDocument* jsonDocument, QVariant* output) {
  new(output) QVariant(*jsonDocument);
}

void qt_core_c_QVariant_constructor_QJsonObject(const QJsonObject* jsonObject, QVariant* output) {
  new(output) QVariant(*jsonObject);
}

void qt_core_c_QVariant_constructor_QJsonValue(const QJsonValue* jsonValue, QVariant* output) {
  new(output) QVariant(*jsonValue);
}

void qt_core_c_QVariant_constructor_QLatin1String(const QLatin1String* string, QVariant* output) {
  new(output) QVariant(*string);
}

void qt_core_c_QVariant_constructor_QLine(const QLine* line, QVariant* output) {
  new(output) QVariant(*line);
}

void qt_core_c_QVariant_constructor_QLineF(const QLineF* line, QVariant* output) {
  new(output) QVariant(*line);
}

void qt_core_c_QVariant_constructor_QList_QVariant(const QList< QVariant >* list, QVariant* output) {
  new(output) QVariant(*list);
}

void qt_core_c_QVariant_constructor_QLocale(const QLocale* locale, QVariant* output) {
  new(output) QVariant(*locale);
}

void qt_core_c_QVariant_constructor_QMap_QString_QVariant(const QMap< QString, QVariant >* map, QVariant* output) {
  new(output) QVariant(*map);
}

void qt_core_c_QVariant_constructor_QModelIndex(const QModelIndex* modelIndex, QVariant* output) {
  new(output) QVariant(*modelIndex);
}

void qt_core_c_QVariant_constructor_QPersistentModelIndex(const QPersistentModelIndex* modelIndex, QVariant* output) {
  new(output) QVariant(*modelIndex);
}

void qt_core_c_QVariant_constructor_QPoint(const QPoint* pt, QVariant* output) {
  new(output) QVariant(*pt);
}

void qt_core_c_QVariant_constructor_QPointF(const QPointF* pt, QVariant* output) {
  new(output) QVariant(*pt);
}

void qt_core_c_QVariant_constructor_QRect(const QRect* rect, QVariant* output) {
  new(output) QVariant(*rect);
}

void qt_core_c_QVariant_constructor_QRectF(const QRectF* rect, QVariant* output) {
  new(output) QVariant(*rect);
}

void qt_core_c_QVariant_constructor_QRegExp(const QRegExp* regExp, QVariant* output) {
  new(output) QVariant(*regExp);
}

void qt_core_c_QVariant_constructor_QRegularExpression(const QRegularExpression* re, QVariant* output) {
  new(output) QVariant(*re);
}

void qt_core_c_QVariant_constructor_QSize(const QSize* size, QVariant* output) {
  new(output) QVariant(*size);
}

void qt_core_c_QVariant_constructor_QSizeF(const QSizeF* size, QVariant* output) {
  new(output) QVariant(*size);
}

void qt_core_c_QVariant_constructor_QString(const QString* string, QVariant* output) {
  new(output) QVariant(*string);
}

void qt_core_c_QVariant_constructor_QStringList(const QStringList* stringlist, QVariant* output) {
  new(output) QVariant(*stringlist);
}

void qt_core_c_QVariant_constructor_QTime(const QTime* time, QVariant* output) {
  new(output) QVariant(*time);
}

void qt_core_c_QVariant_constructor_QUrl(const QUrl* url, QVariant* output) {
  new(output) QVariant(*url);
}

void qt_core_c_QVariant_constructor_QUuid(const QUuid* uuid, QVariant* output) {
  new(output) QVariant(*uuid);
}

void qt_core_c_QVariant_constructor_QVariant(const QVariant* other, QVariant* output) {
  new(output) QVariant(*other);
}

void qt_core_c_QVariant_constructor_QVariant_Type(QVariant::Type type, QVariant* output) {
  new(output) QVariant(type);
}

void qt_core_c_QVariant_constructor_bool(bool b, QVariant* output) {
  new(output) QVariant(b);
}

void qt_core_c_QVariant_constructor_char(const char* str, QVariant* output) {
  new(output) QVariant(str);
}

void qt_core_c_QVariant_constructor_double(double d, QVariant* output) {
  new(output) QVariant(d);
}

void qt_core_c_QVariant_constructor_float(float f, QVariant* output) {
  new(output) QVariant(f);
}

void qt_core_c_QVariant_constructor_int(int i, QVariant* output) {
  new(output) QVariant(i);
}

void qt_core_c_QVariant_constructor_int_void(int typeId, const void* copy, QVariant* output) {
  new(output) QVariant(typeId, copy);
}

void qt_core_c_QVariant_constructor_int_void_unsigned_int(int typeId, const void* copy, unsigned int flags, QVariant* output) {
  new(output) QVariant(typeId, copy, flags);
}

void qt_core_c_QVariant_constructor_no_args(QVariant* output) {
  new(output) QVariant();
}

void qt_core_c_QVariant_constructor_qlonglong(qlonglong ll, QVariant* output) {
  new(output) QVariant(ll);
}

void qt_core_c_QVariant_constructor_qulonglong(qulonglong ull, QVariant* output) {
  new(output) QVariant(ull);
}

void qt_core_c_QVariant_constructor_unsigned_int(unsigned int ui, QVariant* output) {
  new(output) QVariant(ui);
}

void qt_core_c_QVariant_destructor(QVariant* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QVariant_isNull(const QVariant* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QVariant_isValid(const QVariant* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QVariant_load(QVariant* this_ptr, QDataStream* ds) {
  this_ptr->load(*ds);
}

QVariant::Type qt_core_c_QVariant_nameToType(const char* name) {
  return QVariant::nameToType(name);
}

QVariant* qt_core_c_QVariant_operator_assign(QVariant* this_ptr, const QVariant* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QVariant_operator_eq(const QVariant* this_ptr, const QVariant* v) {
  return this_ptr->operator==(*v);
}

bool qt_core_c_QVariant_operator_ge(const QVariant* this_ptr, const QVariant* v) {
  return this_ptr->operator>=(*v);
}

bool qt_core_c_QVariant_operator_gt(const QVariant* this_ptr, const QVariant* v) {
  return this_ptr->operator>(*v);
}

bool qt_core_c_QVariant_operator_le(const QVariant* this_ptr, const QVariant* v) {
  return this_ptr->operator<=(*v);
}

bool qt_core_c_QVariant_operator_lt(const QVariant* this_ptr, const QVariant* v) {
  return this_ptr->operator<(*v);
}

bool qt_core_c_QVariant_operator_neq(const QVariant* this_ptr, const QVariant* v) {
  return this_ptr->operator!=(*v);
}

void qt_core_c_QVariant_save(const QVariant* this_ptr, QDataStream* ds) {
  this_ptr->save(*ds);
}

void qt_core_c_QVariant_swap(QVariant* this_ptr, QVariant* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QVariant_toBitArray_to_output(const QVariant* this_ptr, QBitArray* output) {
  new(output) QBitArray(this_ptr->toBitArray());
}

bool qt_core_c_QVariant_toBool(const QVariant* this_ptr) {
  return this_ptr->toBool();
}

void qt_core_c_QVariant_toByteArray_to_output(const QVariant* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toByteArray());
}

void qt_core_c_QVariant_toChar_to_output(const QVariant* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toChar());
}

void qt_core_c_QVariant_toDateTime_to_output(const QVariant* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->toDateTime());
}

void qt_core_c_QVariant_toDate_to_output(const QVariant* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->toDate());
}

double qt_core_c_QVariant_toDouble_no_args(const QVariant* this_ptr) {
  return this_ptr->toDouble();
}

double qt_core_c_QVariant_toDouble_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toDouble(ok);
}

void qt_core_c_QVariant_toEasingCurve_to_output(const QVariant* this_ptr, QEasingCurve* output) {
  new(output) QEasingCurve(this_ptr->toEasingCurve());
}

float qt_core_c_QVariant_toFloat_no_args(const QVariant* this_ptr) {
  return this_ptr->toFloat();
}

float qt_core_c_QVariant_toFloat_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toFloat(ok);
}

void qt_core_c_QVariant_toHash_to_output(const QVariant* this_ptr, QHash< QString, QVariant >* output) {
  new(output) QHash< QString, QVariant >(this_ptr->toHash());
}

int qt_core_c_QVariant_toInt_no_args(const QVariant* this_ptr) {
  return this_ptr->toInt();
}

int qt_core_c_QVariant_toInt_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toInt(ok);
}

void qt_core_c_QVariant_toJsonArray_to_output(const QVariant* this_ptr, QJsonArray* output) {
  new(output) QJsonArray(this_ptr->toJsonArray());
}

void qt_core_c_QVariant_toJsonDocument_to_output(const QVariant* this_ptr, QJsonDocument* output) {
  new(output) QJsonDocument(this_ptr->toJsonDocument());
}

void qt_core_c_QVariant_toJsonObject_to_output(const QVariant* this_ptr, QJsonObject* output) {
  new(output) QJsonObject(this_ptr->toJsonObject());
}

void qt_core_c_QVariant_toJsonValue_to_output(const QVariant* this_ptr, QJsonValue* output) {
  new(output) QJsonValue(this_ptr->toJsonValue());
}

void qt_core_c_QVariant_toLineF_to_output(const QVariant* this_ptr, QLineF* output) {
  new(output) QLineF(this_ptr->toLineF());
}

void qt_core_c_QVariant_toLine_to_output(const QVariant* this_ptr, QLine* output) {
  new(output) QLine(this_ptr->toLine());
}

void qt_core_c_QVariant_toList_to_output(const QVariant* this_ptr, QList< QVariant >* output) {
  new(output) QList< QVariant >(this_ptr->toList());
}

void qt_core_c_QVariant_toLocale_to_output(const QVariant* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->toLocale());
}

qlonglong qt_core_c_QVariant_toLongLong_no_args(const QVariant* this_ptr) {
  return this_ptr->toLongLong();
}

qlonglong qt_core_c_QVariant_toLongLong_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toLongLong(ok);
}

void qt_core_c_QVariant_toMap_to_output(const QVariant* this_ptr, QMap< QString, QVariant >* output) {
  new(output) QMap< QString, QVariant >(this_ptr->toMap());
}

void qt_core_c_QVariant_toModelIndex_to_output(const QVariant* this_ptr, QModelIndex* output) {
  new(output) QModelIndex(this_ptr->toModelIndex());
}

void qt_core_c_QVariant_toPersistentModelIndex_to_output(const QVariant* this_ptr, QPersistentModelIndex* output) {
  new(output) QPersistentModelIndex(this_ptr->toPersistentModelIndex());
}

void qt_core_c_QVariant_toPointF_to_output(const QVariant* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->toPointF());
}

void qt_core_c_QVariant_toPoint_to_output(const QVariant* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->toPoint());
}

double qt_core_c_QVariant_toReal_no_args(const QVariant* this_ptr) {
  return this_ptr->toReal();
}

double qt_core_c_QVariant_toReal_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toReal(ok);
}

void qt_core_c_QVariant_toRectF_to_output(const QVariant* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->toRectF());
}

void qt_core_c_QVariant_toRect_to_output(const QVariant* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->toRect());
}

void qt_core_c_QVariant_toRegExp_to_output(const QVariant* this_ptr, QRegExp* output) {
  new(output) QRegExp(this_ptr->toRegExp());
}

void qt_core_c_QVariant_toRegularExpression_to_output(const QVariant* this_ptr, QRegularExpression* output) {
  new(output) QRegularExpression(this_ptr->toRegularExpression());
}

void qt_core_c_QVariant_toSizeF_to_output(const QVariant* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->toSizeF());
}

void qt_core_c_QVariant_toSize_to_output(const QVariant* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->toSize());
}

void qt_core_c_QVariant_toStringList_to_output(const QVariant* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->toStringList());
}

void qt_core_c_QVariant_toString_to_output(const QVariant* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

void qt_core_c_QVariant_toTime_to_output(const QVariant* this_ptr, QTime* output) {
  new(output) QTime(this_ptr->toTime());
}

unsigned int qt_core_c_QVariant_toUInt_no_args(const QVariant* this_ptr) {
  return this_ptr->toUInt();
}

unsigned int qt_core_c_QVariant_toUInt_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toUInt(ok);
}

qulonglong qt_core_c_QVariant_toULongLong_no_args(const QVariant* this_ptr) {
  return this_ptr->toULongLong();
}

qulonglong qt_core_c_QVariant_toULongLong_ok(const QVariant* this_ptr, bool* ok) {
  return this_ptr->toULongLong(ok);
}

void qt_core_c_QVariant_toUrl_to_output(const QVariant* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->toUrl());
}

void qt_core_c_QVariant_toUuid_to_output(const QVariant* this_ptr, QUuid* output) {
  new(output) QUuid(this_ptr->toUuid());
}

QVariant::Type qt_core_c_QVariant_type(const QVariant* this_ptr) {
  return this_ptr->type();
}

const char* qt_core_c_QVariant_typeName(const QVariant* this_ptr) {
  return this_ptr->typeName();
}

const char* qt_core_c_QVariant_typeToName(int typeId) {
  return QVariant::typeToName(typeId);
}

int qt_core_c_QVariant_userType(const QVariant* this_ptr) {
  return this_ptr->userType();
}

