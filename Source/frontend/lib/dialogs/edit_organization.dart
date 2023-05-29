import 'package:flutter/material.dart';
import 'package:frontend/api/models/organization.dart';
import 'package:frontend/state/contacts_state.dart';
import 'package:frontend/widgets/form_field.dart';
import 'package:provider/provider.dart';

class EditOrganizationDialog extends StatefulWidget {
  final Organization? organization;

  const EditOrganizationDialog({Key? key, this.organization}) : super(key: key);

  @override
  State<EditOrganizationDialog> createState() => _EditOrganizationDialogState();
}

class _EditOrganizationDialogState extends State<EditOrganizationDialog> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();

  @override
  void initState() {
    super.initState();
    if (widget.organization != null) {
      _nameController.text = widget.organization!.name;
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
                    widget.organization == null
                        ? "Organisation hinzufügen"
                        : "Organisation bearbeiten",
                    style: Theme.of(context).textTheme.titleLarge),
                const SizedBox(height: 16),
                Row(children: [
                  ContactFormField(
                    controller: _nameController,
                    label: "Name",
                    autofocus: true,
                    required: true,
                    submit: _submit,
                    validator: (value) {
                      if (value == null || value.isEmpty) {
                        return 'Bitte geben Sie einen Namen ein.';
                      }
                      return null;
                    },
                  )
                ]),
                const SizedBox(height: 16),
                Row(
                  children: [
                    if (widget.organization != null)
                      TextButton(
                          onPressed: () => _delete(),
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

  void _submit() {
    if (_formKey.currentState!.validate()) {
      _save();
    }
  }

  void _save() async {
    var state = Provider.of<ContactsState>(context, listen: false);
    var organization = CreateOrganization(
      name: _nameController.text,
      email: [],
    );
    await state.createOrganization(organization);
    _close();
  }

  Future<void> _delete() async {
    var state = Provider.of<ContactsState>(context, listen: false);
    await state.deleteOrganization(widget.organization!);
    _close();
  }

  void _close() {
    Navigator.of(context).pop();
  }
}
