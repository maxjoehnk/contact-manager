import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:frontend/api/models/contact.dart';
import 'package:frontend/state/contacts_state.dart';
import 'package:provider/provider.dart';

import 'api/models/distribution_list.dart';
import 'dialogs/edit_organization.dart';
import 'dialogs/edit_person.dart';

class ContactListView extends StatelessWidget {
  final DistributionList? distributionList;

  const ContactListView(this.distributionList, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (distributionList != null) {
      return ContactList(contacts: distributionList!.contacts);
    }
    return Consumer<ContactsState>(
      builder: (context, state, child) {
        if (state.loading) {
          return const Center(child: CircularProgressIndicator());
        }
        return ContactList(contacts: state.contacts);
      },
    );
  }
}

class ContactList extends StatelessWidget {
  final List<Contact> contacts;

  const ContactList({required this.contacts, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
        itemCount: contacts.length,
        itemBuilder: (context, index) {
          var contact = contacts[index];
          return GestureDetector(
            behavior: HitTestBehavior.opaque,
            onSecondaryTap: () {
              log("open menu");
              final RenderBox renderBox =
                  context.findRenderObject() as RenderBox;
              final offset = renderBox.localToGlobal(Offset.zero);
              final left = offset.dx;
              final bottom = offset.dy;
              final top = bottom + renderBox.size.height;
              final right = left + renderBox.size.width;

              showMenu(
                  context: context,
                  position: RelativeRect.fromLTRB(left, top, right, bottom),
                  items: [PopupMenuItem(child: Text("Test"))]);
            },
            child: ListTile(
                leading: CircleAvatar(
                  child: Text(_getInitials(contact)),
                ),
                title: Text(contact.person == null
                    ? contact.organization!.name
                    : "${contact.person!.firstName ?? ""} ${contact.person!.lastName ?? ""}"),
                subtitle: contact.emails.isEmpty
                    ? null
                    : Text(contact.emails.first.email),
                onTap: () {
                  if (contact.person != null) {
                    showDialog(
                        context: context,
                        builder: (context) {
                          return EditPersonDialog(
                            person: contact.person!,
                          );
                        });
                  } else {
                    showDialog(
                        context: context,
                        builder: (context) {
                          return EditOrganizationDialog(
                            organization: contact.organization!,
                          );
                        });
                  }
                  log("edit contact");
                },
                trailing: IconButton(
                  icon: const Icon(Icons.email),
                  onPressed: () {
                    log("send email");
                  },
                )),
          );
        });
  }

  String _getInitials(Contact contact) {
    if (contact.organization != null) {
      return contact.organization!.name.substring(0, 2);
    }
    if (contact.person!.firstName == null && contact.person!.lastName == null) {
      return "?";
    }
    return "${contact.person!.firstName?[0] ?? ""}${contact.person!.lastName?[0] ?? ""}";
  }
}
