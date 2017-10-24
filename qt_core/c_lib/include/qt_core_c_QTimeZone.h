#ifndef QT_CORE_C_QTIMEZONE_H
#define QT_CORE_C_QTIMEZONE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QTimeZone_G_operator_shl_to_output(const QDebug* dbg, const QTimeZone* tz, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QTimeZone_G_operator_shr(QDataStream* ds, QTimeZone* tz);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_G_swap(QTimeZone* value1, QTimeZone* value2);
QT_CORE_C_EXPORT const QString* qt_core_c_QTimeZone_OffsetData_abbreviation(const QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT QString* qt_core_c_QTimeZone_OffsetData_abbreviation_mut(QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT const QDateTime* qt_core_c_QTimeZone_OffsetData_atUtc(const QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT QDateTime* qt_core_c_QTimeZone_OffsetData_atUtc_mut(QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeZone_OffsetData_daylightTimeOffset(const QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_OffsetData_destructor(QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimeZone_OffsetData_offsetFromUtc(const QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_OffsetData_set_abbreviation(QTimeZone::OffsetData* this_ptr, const QString* value);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_OffsetData_set_atUtc(QTimeZone::OffsetData* this_ptr, const QDateTime* value);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_OffsetData_set_daylightTimeOffset(QTimeZone::OffsetData* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_OffsetData_set_offsetFromUtc(QTimeZone::OffsetData* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_OffsetData_set_standardTimeOffset(QTimeZone::OffsetData* this_ptr, int value);
QT_CORE_C_EXPORT int qt_core_c_QTimeZone_OffsetData_standardTimeOffset(const QTimeZone::OffsetData* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_abbreviation_to_output(const QTimeZone* this_ptr, const QDateTime* atDateTime, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_availableTimeZoneIds_to_output_country(const QLocale::Country* country, QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_availableTimeZoneIds_to_output_no_args(QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_availableTimeZoneIds_to_output_offsetSeconds(int offsetSeconds, QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_comment_to_output(const QTimeZone* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_ianaId(const QByteArray* ianaId, QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_no_args(QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_offsetSeconds(int offsetSeconds, QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_other(const QTimeZone* other, QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation(const QByteArray* zoneId, int offsetSeconds, const QString* name, const QString* abbreviation, QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation_country(const QByteArray* zoneId, int offsetSeconds, const QString* name, const QString* abbreviation, const QLocale::Country* country, QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_constructor_zoneId_offsetSeconds_name_abbreviation_country_comment(const QByteArray* zoneId, int offsetSeconds, const QString* name, const QString* abbreviation, const QLocale::Country* country, const QString* comment, QTimeZone* output);
QT_CORE_C_EXPORT int qt_core_c_QTimeZone_daylightTimeOffset(const QTimeZone* this_ptr, const QDateTime* atDateTime);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_destructor(QTimeZone* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_displayName_to_output_atDateTime(const QTimeZone* this_ptr, const QDateTime* atDateTime, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_displayName_to_output_atDateTime_nameType(const QTimeZone* this_ptr, const QDateTime* atDateTime, const QTimeZone::NameType* nameType, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_displayName_to_output_atDateTime_nameType_locale(const QTimeZone* this_ptr, const QDateTime* atDateTime, const QTimeZone::NameType* nameType, const QLocale* locale, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_displayName_to_output_timeType(const QTimeZone* this_ptr, const QTimeZone::TimeType* timeType, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_displayName_to_output_timeType_nameType(const QTimeZone* this_ptr, const QTimeZone::TimeType* timeType, const QTimeZone::NameType* nameType, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_displayName_to_output_timeType_nameType_locale(const QTimeZone* this_ptr, const QTimeZone::TimeType* timeType, const QTimeZone::NameType* nameType, const QLocale* locale, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_hasDaylightTime(const QTimeZone* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_hasTransitions(const QTimeZone* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_ianaIdToWindowsId_to_output(const QByteArray* ianaId, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_id_to_output(const QTimeZone* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_isDaylightTime(const QTimeZone* this_ptr, const QDateTime* atDateTime);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_isTimeZoneIdAvailable(const QByteArray* ianaId);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_isValid(const QTimeZone* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_nextTransition_to_output(const QTimeZone* this_ptr, const QDateTime* afterDateTime, QTimeZone::OffsetData* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_offsetData_to_output(const QTimeZone* this_ptr, const QDateTime* forDateTime, QTimeZone::OffsetData* output);
QT_CORE_C_EXPORT int qt_core_c_QTimeZone_offsetFromUtc(const QTimeZone* this_ptr, const QDateTime* atDateTime);
QT_CORE_C_EXPORT QTimeZone* qt_core_c_QTimeZone_operator_assign(QTimeZone* this_ptr, const QTimeZone* other);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_operator_eq(const QTimeZone* this_ptr, const QTimeZone* other);
QT_CORE_C_EXPORT bool qt_core_c_QTimeZone_operator_neq(const QTimeZone* this_ptr, const QTimeZone* other);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_previousTransition_to_output(const QTimeZone* this_ptr, const QDateTime* beforeDateTime, QTimeZone::OffsetData* output);
QT_CORE_C_EXPORT int qt_core_c_QTimeZone_standardTimeOffset(const QTimeZone* this_ptr, const QDateTime* atDateTime);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_swap(QTimeZone* this_ptr, QTimeZone* other);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_systemTimeZoneId_to_output(QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_systemTimeZone_to_output(QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_transitions_to_output(const QTimeZone* this_ptr, const QDateTime* fromDateTime, const QDateTime* toDateTime, QVector< QTimeZone::OffsetData >* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_utc_to_output(QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_windowsIdToDefaultIanaId_to_output_windowsId(const QByteArray* windowsId, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_windowsIdToDefaultIanaId_to_output_windowsId_country(const QByteArray* windowsId, const QLocale::Country* country, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_windowsIdToIanaIds_to_output_windowsId(const QByteArray* windowsId, QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QTimeZone_windowsIdToIanaIds_to_output_windowsId_country(const QByteArray* windowsId, const QLocale::Country* country, QList< QByteArray >* output);

} // extern "C"

#endif // QT_CORE_C_QTIMEZONE_H
