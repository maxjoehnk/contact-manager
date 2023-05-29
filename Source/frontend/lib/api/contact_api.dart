import 'http.dart';
import 'models/contact.dart';
import 'models/organization.dart';
import 'models/person.dart';

class ContactApi {
  final HttpApi httpApi;

  ContactApi(this.httpApi);

  Future<List<Contact>> getContacts() async {
    List<dynamic> res = await httpApi.get("v1/contacts");

    return res.map((e) => Contact.fromJson(e)).toList();
  }

  Future<String> createPerson(CreatePerson person) async {
    return await httpApi.post("v1/contacts/persons", person.toJson());
  }

  Future<String> createOrganization(CreateOrganization organization) async {
    return await httpApi.post(
        "v1/contacts/organizations", organization.toJson());
  }

  Future<void> updatePerson(Person update) async {
    await httpApi.put("v1/contacts/persons/${update.id}", update.toJson());
  }

  Future<void> deletePerson(Person person) async {
    await httpApi.delete("v1/contacts/persons/${person.id}");
  }

  Future<void> deleteOrganization(Organization organization) async {
    await httpApi.delete("v1/contacts/organizations/${organization.id}");
  }
}
