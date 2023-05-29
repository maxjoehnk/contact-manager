// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'organization.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Organization _$OrganizationFromJson(Map<String, dynamic> json) => Organization(
      id: json['id'] as String,
      name: json['name'] as String,
      email: (json['email'] as List<dynamic>)
          .map((e) => EmailContact.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$OrganizationToJson(Organization instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'email': instance.email,
    };

CreateOrganization _$CreateOrganizationFromJson(Map<String, dynamic> json) =>
    CreateOrganization(
      name: json['name'] as String,
      email: (json['email'] as List<dynamic>)
          .map((e) => EmailContact.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$CreateOrganizationToJson(CreateOrganization instance) =>
    <String, dynamic>{
      'name': instance.name,
      'email': instance.email,
    };
