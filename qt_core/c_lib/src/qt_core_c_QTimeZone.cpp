#include "qt_core_c_QTimeZone.h"

void qt_core_c_QTimeZone_G_operator_shl_to_output(const QDebug* dbg, const QTimeZone* tz, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *tz));
}

QDataStream* qt_core_c_QTimeZone_G_operator_shr(QDataStream* ds, QTimeZone* tz) {
  return &operator>>(*ds, *tz);
}

void qt_core_c_QTimeZone_G_swap(QTimeZone* value1, QTimeZone* value2) {
  swap(*value1, *value2);
}

const QString* qt_core_c_QTimeZone_OffsetData_abbreviation(const QTimeZone::OffsetData* this_ptr) {
  return &this_ptr->abbreviation;
}

QString* qt_core_c_QTimeZone_OffsetData_abbreviation_mut(QTimeZone::OffsetData* this_ptr) {
  return &this_ptr->abbreviation;
}

const QDateTime* qt_core_c_QTimeZone_OffsetData_atUtc(const QTimeZone::OffsetData* this_ptr) {
  return &this_ptr->atUtc;
}

QDateTime* qt_core_c_QTimeZone_OffsetData_atUtc_mut(QTimeZone::OffsetData* this_ptr) {
  return &this_ptr->atUtc;
}

int qt_core_c_QTimeZone_OffsetData_daylightTimeOffset(const QTimeZone::OffsetData* this_ptr) {
  return this_ptr->daylightTimeOffset;
}

void qt_core_c_QTimeZone_OffsetData_destructor(QTimeZone::OffsetData* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QTimeZone_OffsetData_offsetFromUtc(const QTimeZone::OffsetData* this_ptr) {
  return this_ptr->offsetFromUtc;
}

void qt_core_c_QTimeZone_OffsetData_set_abbreviation(QTimeZone::OffsetData* this_ptr, const QString* value) {
  this_ptr->abbreviation = *value;
}

void qt_core_c_QTimeZone_OffsetData_set_atUtc(QTimeZone::OffsetData* this_ptr, const QDateTime* value) {
  this_ptr->atUtc = *value;
}

void qt_core_c_QTimeZone_OffsetData_set_daylightTimeOffset(QTimeZone::OffsetData* this_ptr, int value) {
  this_ptr->daylightTimeOffset = value;
}

void qt_core_c_QTimeZone_OffsetData_set_offsetFromUtc(QTimeZone::OffsetData* this_ptr, int value) {
  this_ptr->offsetFromUtc = value;
}

void qt_core_c_QTimeZone_OffsetData_set_standardTimeOffset(QTimeZone::OffsetData* this_ptr, int value) {
  this_ptr->standardTimeOffset = value;
}

int qt_core_c_QTimeZone_OffsetData_standardTimeOffset(const QTimeZone::OffsetData* this_ptr) {
  return this_ptr->standardTimeOffset;
}

void qt_core_c_QTimeZone_abbreviation_to_output(const QTimeZone* this_ptr, const QDateTime* atDateTime, QString* output) {
  new(output) QString(this_ptr->abbreviation(*atDateTime));
}

void qt_core_c_QTimeZone_availableTimeZoneIds_to_output_country(const QLocale::Country* country, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTimeZone::availableTimeZoneIds(*country));
}

void qt_core_c_QTimeZone_availableTimeZoneIds_to_output_no_args(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTimeZone::availableTimeZoneIds());
}

void qt_core_c_QTimeZone_availableTimeZoneIds_to_output_offsetSeconds(int offsetSeconds, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTimeZone::availableTimeZoneIds(offsetSeconds));
}

void qt_core_c_QTimeZone_comment_to_output(const QTimeZone* this_ptr, QString* output) {
  new(output) QString(this_ptr->comment());
}

void qt_core_c_QTimeZone_constructor_ianaId(const QByteArray* ianaId, QTimeZone* output) {
  new(output) QTimeZone(*ianaId);
}

void qt_core_c_QTimeZone_constructor_no_args(QTimeZone* output) {
  new(output) QTimeZone();
}

void qt_core_c_QTimeZone_constructor_offsetSeconds(int offsetSeconds, QTimeZone* output) {
  new(output) QTimeZone(offsetSeconds);
}

void qt_core_c_QTimeZone_constructor_other(const QTimeZone* other, QTimeZone* output) {
  new(output) QTimeZone(*other);
}

void qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation(const QByteArray* zoneId, int offsetSeconds, const QString* name, const QString* abbreviation, QTimeZone* output) {
  new(output) QTimeZone(*zoneId, offsetSeconds, *name, *abbreviation);
}

void qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation_country(const QByteArray* zoneId, int offsetSeconds, const QString* name, const QString* abbreviation, const QLocale::Country* country, QTimeZone* output) {
  new(output) QTimeZone(*zoneId, offsetSeconds, *name, *abbreviation, *country);
}

void qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation_country_comment(const QByteArray* zoneId, int offsetSeconds, const QString* name, const QString* abbreviation, const QLocale::Country* country, const QString* comment, QTimeZone* output) {
  new(output) QTimeZone(*zoneId, offsetSeconds, *name, *abbreviation, *country, *comment);
}

int qt_core_c_QTimeZone_daylightTimeOffset(const QTimeZone* this_ptr, const QDateTime* atDateTime) {
  return this_ptr->daylightTimeOffset(*atDateTime);
}

void qt_core_c_QTimeZone_destructor(QTimeZone* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QTimeZone_displayName_to_output_atDateTime(const QTimeZone* this_ptr, const QDateTime* atDateTime, QString* output) {
  new(output) QString(this_ptr->displayName(*atDateTime));
}

void qt_core_c_QTimeZone_displayName_to_output_atDateTime_nameType(const QTimeZone* this_ptr, const QDateTime* atDateTime, const QTimeZone::NameType* nameType, QString* output) {
  new(output) QString(this_ptr->displayName(*atDateTime, *nameType));
}

void qt_core_c_QTimeZone_displayName_to_output_atDateTime_nameType_locale(const QTimeZone* this_ptr, const QDateTime* atDateTime, const QTimeZone::NameType* nameType, const QLocale* locale, QString* output) {
  new(output) QString(this_ptr->displayName(*atDateTime, *nameType, *locale));
}

void qt_core_c_QTimeZone_displayName_to_output_timeType(const QTimeZone* this_ptr, const QTimeZone::TimeType* timeType, QString* output) {
  new(output) QString(this_ptr->displayName(*timeType));
}

void qt_core_c_QTimeZone_displayName_to_output_timeType_nameType(const QTimeZone* this_ptr, const QTimeZone::TimeType* timeType, const QTimeZone::NameType* nameType, QString* output) {
  new(output) QString(this_ptr->displayName(*timeType, *nameType));
}

void qt_core_c_QTimeZone_displayName_to_output_timeType_nameType_locale(const QTimeZone* this_ptr, const QTimeZone::TimeType* timeType, const QTimeZone::NameType* nameType, const QLocale* locale, QString* output) {
  new(output) QString(this_ptr->displayName(*timeType, *nameType, *locale));
}

bool qt_core_c_QTimeZone_hasDaylightTime(const QTimeZone* this_ptr) {
  return this_ptr->hasDaylightTime();
}

bool qt_core_c_QTimeZone_hasTransitions(const QTimeZone* this_ptr) {
  return this_ptr->hasTransitions();
}

void qt_core_c_QTimeZone_ianaIdToWindowsId_to_output(const QByteArray* ianaId, QByteArray* output) {
  new(output) QByteArray(QTimeZone::ianaIdToWindowsId(*ianaId));
}

void qt_core_c_QTimeZone_id_to_output(const QTimeZone* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->id());
}

bool qt_core_c_QTimeZone_isDaylightTime(const QTimeZone* this_ptr, const QDateTime* atDateTime) {
  return this_ptr->isDaylightTime(*atDateTime);
}

bool qt_core_c_QTimeZone_isTimeZoneIdAvailable(const QByteArray* ianaId) {
  return QTimeZone::isTimeZoneIdAvailable(*ianaId);
}

bool qt_core_c_QTimeZone_isValid(const QTimeZone* this_ptr) {
  return this_ptr->isValid();
}

void qt_core_c_QTimeZone_nextTransition_to_output(const QTimeZone* this_ptr, const QDateTime* afterDateTime, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->nextTransition(*afterDateTime));
}

void qt_core_c_QTimeZone_offsetData_to_output(const QTimeZone* this_ptr, const QDateTime* forDateTime, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->offsetData(*forDateTime));
}

int qt_core_c_QTimeZone_offsetFromUtc(const QTimeZone* this_ptr, const QDateTime* atDateTime) {
  return this_ptr->offsetFromUtc(*atDateTime);
}

QTimeZone* qt_core_c_QTimeZone_operator_assign(QTimeZone* this_ptr, const QTimeZone* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QTimeZone_operator_eq(const QTimeZone* this_ptr, const QTimeZone* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QTimeZone_operator_neq(const QTimeZone* this_ptr, const QTimeZone* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QTimeZone_previousTransition_to_output(const QTimeZone* this_ptr, const QDateTime* beforeDateTime, QTimeZone::OffsetData* output) {
  new(output) QTimeZone::OffsetData(this_ptr->previousTransition(*beforeDateTime));
}

int qt_core_c_QTimeZone_standardTimeOffset(const QTimeZone* this_ptr, const QDateTime* atDateTime) {
  return this_ptr->standardTimeOffset(*atDateTime);
}

void qt_core_c_QTimeZone_swap(QTimeZone* this_ptr, QTimeZone* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QTimeZone_systemTimeZoneId_to_output(QByteArray* output) {
  new(output) QByteArray(QTimeZone::systemTimeZoneId());
}

void qt_core_c_QTimeZone_systemTimeZone_to_output(QTimeZone* output) {
  new(output) QTimeZone(QTimeZone::systemTimeZone());
}

void qt_core_c_QTimeZone_transitions_to_output(const QTimeZone* this_ptr, const QDateTime* fromDateTime, const QDateTime* toDateTime, QVector< QTimeZone::OffsetData >* output) {
  new(output) QVector< QTimeZone::OffsetData >(this_ptr->transitions(*fromDateTime, *toDateTime));
}

void qt_core_c_QTimeZone_utc_to_output(QTimeZone* output) {
  new(output) QTimeZone(QTimeZone::utc());
}

void qt_core_c_QTimeZone_windowsIdToDefaultIanaId_to_output_windowsId(const QByteArray* windowsId, QByteArray* output) {
  new(output) QByteArray(QTimeZone::windowsIdToDefaultIanaId(*windowsId));
}

void qt_core_c_QTimeZone_windowsIdToDefaultIanaId_to_output_windowsId_country(const QByteArray* windowsId, const QLocale::Country* country, QByteArray* output) {
  new(output) QByteArray(QTimeZone::windowsIdToDefaultIanaId(*windowsId, *country));
}

void qt_core_c_QTimeZone_windowsIdToIanaIds_to_output_windowsId(const QByteArray* windowsId, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTimeZone::windowsIdToIanaIds(*windowsId));
}

void qt_core_c_QTimeZone_windowsIdToIanaIds_to_output_windowsId_country(const QByteArray* windowsId, const QLocale::Country* country, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTimeZone::windowsIdToIanaIds(*windowsId, *country));
}

