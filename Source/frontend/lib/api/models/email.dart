import 'package:json_annotation/json_annotation.dart';

part 'email.g.dart';

@JsonSerializable()
class EmailContact {
  final String label;
  final String email;

  EmailContact({required this.label, required this.email});

  factory EmailContact.fromJson(Map<String, dynamic> json) =>
      _$EmailContactFromJson(json);

  Map<String, dynamic> toJson() => _$EmailContactToJson(this);
}
