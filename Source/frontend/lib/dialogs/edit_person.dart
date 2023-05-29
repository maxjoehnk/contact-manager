import 'package:flutter/material.dart';
import 'package:frontend/api/models/person.dart';
import 'package:frontend/state/contacts_state.dart';
import 'package:frontend/widgets/form_field.dart';
import 'package:provider/provider.dart';

class EditPersonDialog extends StatefulWidget {
  final Person? person;

  const EditPersonDialog({this.person, Key? key}) : super(key: key);

  @override
  State<EditPersonDialog> createState() => _EditPersonDialogState();
}

class _EditPersonDialogState extends State<EditPersonDialog> {
  final _formKey = GlobalKey<FormState>();
  final _firstNameController = TextEditingController();
  final _lastNameController = TextEditingController();

  @override
  void initState() {
    super.initState();
    if (widget.person != null) {
      _firstNameController.text = widget.person!.firstName ?? "";
      _lastNameController.text = widget.person!.lastName ?? "";
    }
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: Form(
        key: _formKey,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 400, maxHeight: 600),
          child: Container(
            padding: const EdgeInsets.all(16),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              mainAxisAlignment: MainAxisAlignment.start,
              children: [
                Text(
                    widget.person == null
                        ? "Kontakt hinzufügen"
                        : "Kontakt bearbeiten",
                    style: Theme.of(context).textTheme.titleLarge),
                const SizedBox(height: 16),
                Row(children: [
                  ContactFormField(
                    controller: _firstNameController,
                    label: "Vorname",
                    submit: _submit,
                    autofocus: true,
                  ),
                  ContactFormField(
                      controller: _lastNameController,
                      label: "Nachname",
                      submit: _submit),
                ]),
                const SizedBox(height: 16),
                Row(
                  children: [
                    if (widget.person != null)
                      TextButton(
                          onPressed: () => _deletePerson(),
                          child: Text("Löschen",
                              style: TextStyle(
                                  color: Theme.of(context).colorScheme.error))),
                    const Spacer(),
                    ElevatedButton(
                        onPressed: _submit, child: const Text("Speichern"))
                  ],
                )
              ],
            ),
          ),
        ),
      ),
    );
  }

  Future<void> _deletePerson() async {
    var state = Provider.of<ContactsState>(context, listen: false);
    await state.deletePerson(widget.person!);
    Navigator.of(context).pop();
  }

  void _submit() {
    if (_formKey.currentState!.validate()) {
      _savePerson();
    }
  }

  void _savePerson() async {
    var state = Provider.of<ContactsState>(context, listen: false);
    if (widget.person != null) {
      await state.updatePerson(_assembleUpdate());
    } else {
      await state.createPerson(_assembleCreate());
    }

    Navigator.of(context).pop();
  }

  CreatePerson _assembleCreate() {
    return CreatePerson(
      firstName: _firstNameController.text,
      lastName: _lastNameController.text,
      email: [],
    );
  }

  Person _assembleUpdate() {
    return Person(
      id: widget.person!.id,
      firstName: _firstNameController.text,
      lastName: _lastNameController.text,
      email: [],
    );
  }
}
