import 'package:json_annotation/json_annotation.dart';

import 'email.dart';

part 'person.g.dart';

@JsonSerializable()
class Person {
  final String id;
  final String? firstName;
  final String? lastName;
  final List<EmailContact> email;

  Person(
      {required this.id,
      required this.firstName,
      required this.lastName,
      required this.email});

  factory Person.fromJson(Map<String, dynamic> json) => _$PersonFromJson(json);

  Map<String, dynamic> toJson() => _$PersonToJson(this);
}

@JsonSerializable()
class CreatePerson {
  final String? firstName;
  final String? lastName;
  final List<EmailContact> email;

  CreatePerson(
      {required this.firstName, required this.lastName, required this.email});

  Map<String, dynamic> toJson() => _$CreatePersonToJson(this);
}
