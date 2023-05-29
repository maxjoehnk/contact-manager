import 'package:json_annotation/json_annotation.dart';

import 'email.dart';

part 'organization.g.dart';

@JsonSerializable()
class Organization {
  final String id;
  final String name;
  final List<EmailContact> email;

  Organization({required this.id, required this.name, required this.email});

  factory Organization.fromJson(Map<String, dynamic> json) =>
      _$OrganizationFromJson(json);

  Map<String, dynamic> toJson() => _$OrganizationToJson(this);
}

@JsonSerializable()
class CreateOrganization {
  final String name;
  final List<EmailContact> email;

  CreateOrganization({required this.name, required this.email});

  Map<String, dynamic> toJson() => _$CreateOrganizationToJson(this);
}
