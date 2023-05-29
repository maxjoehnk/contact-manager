import 'email.dart';
import 'organization.dart';
import 'person.dart';

class Contact {
  final String id;
  final Person? person;
  final Organization? organization;

  List<EmailContact> get emails {
    if (person != null) {
      return person!.email;
    } else {
      return organization!.email;
    }
  }

  Contact({required this.id, this.person, this.organization});

  factory Contact.person(Person person) {
    return Contact(id: person.id, person: person);
  }

  factory Contact.organization(Organization organization) {
    return Contact(id: organization.id, organization: organization);
  }

  factory Contact.fromJson(Map<String, dynamic> json) {
    if (json['type'] == 'person') {
      return Contact.person(Person.fromJson(json));
    } else {
      return Contact.organization(Organization.fromJson(json));
    }
  }

  Map<String, dynamic> toJson() {
    if (person != null) {
      return person!.toJson();
    } else {
      return organization!.toJson();
    }
  }
}
