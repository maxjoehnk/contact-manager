import 'package:json_annotation/json_annotation.dart';

import 'contact.dart';

part 'distribution_list.g.dart';

@JsonSerializable(createToJson: false)
class DistributionList {
  final String id;
  final String name;
  final List<Contact> contacts;

  DistributionList(
      {required this.id, required this.name, required this.contacts});

  factory DistributionList.fromJson(Map<String, dynamic> json) =>
      _$DistributionListFromJson(json);
}

@JsonSerializable(createFactory: false)
class CreateDistributionList {
  final String name;
  final List<Contact> contacts;

  CreateDistributionList({required this.name, required this.contacts});

  Map<String, dynamic> toJson() => _$CreateDistributionListToJson(this);
}
