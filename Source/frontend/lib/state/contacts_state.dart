import 'dart:developer';

import 'package:flutter/foundation.dart';
import 'package:frontend/api/contact_api.dart';
import 'package:frontend/api/models/contact.dart';
import 'package:frontend/api/models/organization.dart';
import 'package:frontend/api/models/person.dart';

class ContactsState extends ChangeNotifier {
  final ContactApi _api;
  List<Contact> _contacts = [];
  bool _loading = false;

  ContactsState(this._api) {
    refresh();
  }

  List<Contact> get contacts => _contacts;

  bool get loading => _loading;

  Future<void> refresh() async {
    _loading = true;
    notifyListeners();

    try {
      _contacts = await _api.getContacts();
    } catch (e) {
      log(e.toString());
    } finally {
      _loading = false;
    }
    notifyListeners();
  }

  Future<void> createPerson(CreatePerson person) async {
    await _api.createPerson(person);
    await refresh();
  }

  Future<void> createOrganization(CreateOrganization organization) async {
    await _api.createOrganization(organization);
    await refresh();
  }

  Future<void> updatePerson(Person update) async {
    await _api.updatePerson(update);
    await refresh();
  }

  Future<void> deletePerson(Person person) async {
    await _api.deletePerson(person);
    await refresh();
  }

  Future<void> deleteOrganization(Organization organization) async {
    await _api.deleteOrganization(organization);
    await refresh();
  }
}
