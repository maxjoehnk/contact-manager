// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'distribution_list.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DistributionList _$DistributionListFromJson(Map<String, dynamic> json) =>
    DistributionList(
      id: json['id'] as String,
      name: json['name'] as String,
      contacts: (json['contacts'] as List<dynamic>)
          .map((e) => Contact.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$CreateDistributionListToJson(
        CreateDistributionList instance) =>
    <String, dynamic>{
      'name': instance.name,
      'contacts': instance.contacts,
    };
